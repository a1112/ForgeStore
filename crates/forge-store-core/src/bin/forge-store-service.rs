#[cfg(not(unix))]
fn main() {
    eprintln!("ForgeStore service requires Linux");
    std::process::exit(1);
}

#[cfg(unix)]
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    linux::main().await
}

#[cfg(unix)]
mod linux {
    use forge_store_core::backends::{
        bounded_process_output, compatforge_poll_status, compatforge_selected_install,
        fixture_remote_is_trusted, flatpak_action_completed, flatpak_record_matches,
        native_identity_matches, native_probe, native_status_matches, package_display_version,
        parse_flatpak_columns, wait_managed_child, CompatForgeClient, CompatForgeRequest,
        FlatpakInvocation, NativeClient, NativeRequest, PackageClient, PackageRequest,
    };
    use forge_store_core::cache::VerifiedCache;
    use forge_store_core::catalogue::{AppEntry, Catalog, Delivery};
    use forge_store_core::queue::{Action, Backend, Job, JobQueue, JobState};
    use forge_store_core::service_api::{parse_request, response, StoreRequest, MAX_LINE_BYTES};
    use forge_store_core::trusted_catalogue::TrustedCatalogue;
    use serde_json::{json, Value};
    use std::os::unix::fs::FileTypeExt;
    use std::path::{Path, PathBuf};
    use std::sync::{Arc, Mutex};
    use std::time::Duration;
    use tokio::io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader};
    use tokio::net::{UnixListener, UnixStream};
    use tokio::sync::Notify;
    use tokio_util::sync::CancellationToken;
    use url::Url;

    struct Paths {
        share: PathBuf,
        state: PathBuf,
        cache: PathBuf,
        socket: PathBuf,
    }
    impl Paths {
        fn from_args() -> Result<Self, String> {
            let args = std::env::args().collect::<Vec<_>>();
            if args.len() == 3 && args[1] == "--test-root" {
                let root = PathBuf::from(&args[2]);
                if !root.is_absolute() {
                    return Err("test root must be absolute".into());
                }
                return Ok(Self {
                    share: root.join("share"),
                    state: root.join("state"),
                    cache: root.join("cache"),
                    socket: root.join("runtime/store.sock"),
                });
            }
            if args.len() != 1 {
                return Err("usage: forge-store-service [--test-root ABSOLUTE_PATH]".into());
            }
            let home = PathBuf::from(std::env::var("HOME").map_err(|_| "HOME is unavailable")?);
            let runtime = PathBuf::from(
                std::env::var("XDG_RUNTIME_DIR").map_err(|_| "XDG_RUNTIME_DIR is unavailable")?,
            );
            if !home.is_absolute() || !runtime.is_absolute() {
                return Err("invalid HOME or runtime path".into());
            }
            Ok(Self {
                share: PathBuf::from("/usr/share/forge-store"),
                state: home.join(".local/state/forge-store"),
                cache: home.join(".cache/forge-store"),
                socket: runtime.join("forge-store/store.sock"),
            })
        }
    }

    struct Store {
        catalogue: Catalog,
        queue: JobQueue,
        cache: VerifiedCache,
        compat: CompatForgeClient,
        notify: Notify,
        running: Mutex<Option<(String, CancellationToken)>>,
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    enum RunOutcome {
        Completed,
        Cancelled,
        Interrupted,
        ProviderFailed(String),
    }

    pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
        let paths = Paths::from_args()?;
        private_directory(&paths.state)?;
        prepare_cache_directory(&paths.cache)?;
        let socket_dir = paths.socket.parent().ok_or("invalid socket path")?;
        private_directory(socket_dir)?;
        let metadata = Url::from_directory_path(paths.share.join("tuf/metadata"))
            .map_err(|_| "invalid metadata path")?;
        let targets = Url::from_directory_path(paths.share.join("tuf/targets"))
            .map_err(|_| "invalid targets path")?;
        let catalogue = TrustedCatalogue::load(
            &paths.share.join("tuf/trusted-root.json"),
            metadata,
            targets,
            &paths.state.join("tuf-datastore"),
        )
        .await?;
        let store = Arc::new(Store {
            catalogue,
            queue: JobQueue::open(&paths.state.join("jobs.sqlite"))?,
            cache: VerifiedCache::open(&paths.cache)?,
            compat: CompatForgeClient::system(&paths.cache.join("requests")),
            notify: Notify::new(),
            running: Mutex::new(None),
        });
        // A private parent directory plus socket mode and SO_PEERCRED protect the user API.
        if paths.socket.exists() {
            let old = std::fs::symlink_metadata(&paths.socket)?;
            if !old.file_type().is_socket() {
                return Err("socket path is not a socket".into());
            }
            std::fs::remove_file(&paths.socket)?;
        }
        let listener = UnixListener::bind(&paths.socket)?;
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(&paths.socket, std::fs::Permissions::from_mode(0o600))?;
        let worker = store.clone();
        tokio::spawn(async move {
            worker_loop(worker).await;
        });
        store.notify.notify_one();
        loop {
            let (stream, _) = listener.accept().await?;
            let client = store.clone();
            tokio::spawn(async move {
                let _ = serve_connection(client, stream).await;
            });
        }
    }

    fn private_directory(path: &Path) -> Result<(), Box<dyn std::error::Error>> {
        use std::os::unix::fs::PermissionsExt;
        if path.exists() && std::fs::symlink_metadata(path)?.file_type().is_symlink() {
            return Err("private directory is a symlink".into());
        }
        std::fs::create_dir_all(path)?;
        std::fs::set_permissions(path, std::fs::Permissions::from_mode(0o700))?;
        Ok(())
    }

    fn prepare_cache_directory(path: &Path) -> Result<(), Box<dyn std::error::Error>> {
        private_directory(path.parent().ok_or("cache path has no parent")?)?;
        private_directory(path)
    }

    async fn serve_connection(
        store: Arc<Store>,
        stream: UnixStream,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        if stream.peer_cred()?.uid() != unsafe { libc::geteuid() } {
            return Err("foreign peer".into());
        }
        let mut stream = stream;
        let mut line = Vec::new();
        {
            let reader = BufReader::new(&mut stream);
            reader
                .take((MAX_LINE_BYTES + 1) as u64)
                .read_until(b'\n', &mut line)
                .await?;
        }
        let result = match parse_request(&line) {
            Ok(request) => {
                let id = request.request_id().to_string();
                let output = handle(&store, request).await;
                response(&id, output.clone())
            }
            Err(_) => response(
                "invalid",
                Err(("invalid-request", "Malformed or unsupported request")),
            ),
        };
        let mut encoded = serde_json::to_vec(&result)?;
        if encoded.len() + 1 > 1024 * 1024 {
            let id = result
                .get("requestId")
                .and_then(Value::as_str)
                .unwrap_or("invalid");
            encoded = serde_json::to_vec(&response(
                id,
                Err(("snapshot-too-large", "Snapshot exceeds client limit")),
            ))?;
        }
        encoded.push(b'\n');
        stream.write_all(&encoded).await?;
        Ok(())
    }

    async fn handle(
        store: &Store,
        request: StoreRequest,
    ) -> Result<Value, (&'static str, &'static str)> {
        match request {
            StoreRequest::Snapshot { .. } => snapshot(store).await,
            StoreRequest::Enqueue { app_id, action, .. } => {
                let entry = store
                    .catalogue
                    .entries
                    .iter()
                    .find(|entry| entry.id == app_id)
                    .ok_or(("not-found", "Application is absent from signed catalogue"))?;
                let backend = match &entry.delivery {
                    Delivery::Compatforge { .. } => Backend::Compatforge,
                    Delivery::Flatpak { .. } => Backend::Flatpak,
                    Delivery::ForgePackage { .. } => Backend::ForgePackage,
                    Delivery::UbuntuDeb { .. } => Backend::UbuntuDeb,
                    Delivery::Snap { .. } => Backend::Snap,
                };
                if matches!(
                    (backend, action),
                    (Backend::Flatpak, Action::Rollback)
                        | (Backend::UbuntuDeb | Backend::Snap, Action::Rollback)
                        | (Backend::ForgePackage, Action::Uninstall)
                ) {
                    return Err(("unsupported", "Backend does not support this action"));
                }
                let job = store
                    .queue
                    .enqueue_reviewed(&app_id, backend, action, &entry.delivery)
                    .map_err(|_| ("conflict", "Application already has an active operation"))?;
                store.notify.notify_one();
                Ok(json!({"jobId":job.id}))
            }
            StoreRequest::Cancel { job_id, .. } => {
                let job = store
                    .queue
                    .get(&job_id)
                    .map_err(|_| ("internal", "Queue unavailable"))?
                    .ok_or(("not-found", "Job not found"))?;
                if matches!(
                    job.backend,
                    Backend::ForgePackage | Backend::UbuntuDeb | Backend::Snap
                ) && job.state == JobState::Running
                {
                    return Err((
                        "unsupported",
                        "Package transaction cannot be cancelled after starting",
                    ));
                }
                store
                    .queue
                    .cancel(&job_id)
                    .map_err(|_| ("conflict", "Job is not cancellable"))?;
                if let Ok(guard) = store.running.lock() {
                    if let Some((id, token)) = guard.as_ref() {
                        if id == &job_id {
                            token.cancel();
                        }
                    }
                }
                Ok(json!({"jobId":job_id}))
            }
            StoreRequest::Retry { job_id, .. } => {
                let job = store
                    .queue
                    .retry(&job_id)
                    .map_err(|_| ("conflict", "Job is not retryable"))?;
                store.notify.notify_one();
                Ok(json!({"jobId":job.id}))
            }
        }
    }

    async fn snapshot(store: &Store) -> Result<Value, (&'static str, &'static str)> {
        let (recent, jobs_has_more) = store
            .queue
            .list_recent(64)
            .map_err(|_| ("storage", "Job database is unavailable"))?;
        let jobs = recent
            .into_iter()
            .map(|job| {
                json!({
            "id":job.id,"appId":job.app_id,"backend":backend_name(job.backend),
            "action":action_name(job.action),"state":state_name(job.state),
            "nativePending":job.native_pending,"detail":job.detail.as_deref().map(|text| bounded_text(text, 512))})
            })
            .collect::<Vec<_>>();
        // Leave room for IPC encoding before the UI's five-second request deadline.
        let (compat, flatpak, package, native) =
            tokio::time::timeout(Duration::from_secs(4), async {
                tokio::join!(
                    probe_compat(store),
                    probe_flatpak(store),
                    probe_package(store),
                    probe_native(store)
                )
            })
            .await
            .map_err(|_| ("snapshot-timeout", "Backend snapshot exceeded total budget"))?;
        let mut installed = Vec::new();
        installed.extend(compat.1);
        installed.extend(flatpak.1);
        installed.extend(package.1);
        installed.extend(native.1);
        Ok(
            json!({"catalogue":store.catalogue,"jobs":jobs,"jobsHasMore":jobs_has_more,"installed":installed,"backends":{
                "compatforge":{"available":compat.0,"reason":if compat.0 { "" } else { "CompatForge service unavailable" }},
                "flatpak":{"available":flatpak.0,"reason":if flatpak.0 { "" } else { "Flatpak unavailable" }},
                "forgePackage":{"available":package.0,"reason":if package.0 { "" } else { "ForgeOS package service unavailable" }},
                "ubuntu-deb":native.0[0],"snap":native.0[1]
            }}),
        )
    }

    async fn probe_compat(store: &Store) -> (bool, Vec<Value>) {
        if !store.compat.available() {
            return (false, Vec::new());
        }
        let result = tokio::time::timeout(
            Duration::from_secs(3),
            store.compat.operation("applications.list", json!({})),
        )
        .await;
        let Ok(Ok(result)) = result else {
            return (false, Vec::new());
        };
        let Some(records) = result.as_array() else {
            return (false, Vec::new());
        };
        let installed = records.iter().filter_map(|record| {
            let id = record.get("application")?.get("id")?.as_str()?;
            if !store.catalogue.entries.iter().any(|entry|
                entry.id == id && matches!(entry.delivery, Delivery::Compatforge { .. })) { return None; }
            let (version, can_rollback) = compatforge_selected_install(record, id)?;
            Some(json!({"appId":id,"backend":"compatforge","version":version,"canRollback":can_rollback}))
        }).collect();
        (true, installed)
    }

    async fn probe_native(store: &Store) -> ([Value; 2], Vec<Value>) {
        let unavailable = json!({"available":false,"reason":"Authenticated Ubuntu native package service unavailable"});
        let failed = ([unavailable.clone(), unavailable], Vec::new());
        let request = NativeRequest::probe(&format!("probe-{}", uuid::Uuid::new_v4())).unwrap();
        let Ok(Ok(result)) =
            tokio::time::timeout(Duration::from_secs(3), NativeClient.call(request)).await
        else {
            return failed;
        };
        let Ok(probed) = native_probe(&result) else {
            return failed;
        };
        let mut platforms = [Value::Null, Value::Null];
        for (backend, available, detail) in probed {
            platforms[usize::from(backend == "snap")] =
                json!({"available":available,"reason":detail});
        }
        let entries = store
            .catalogue
            .entries
            .iter()
            .filter(|entry| {
                matches!(
                    entry.delivery,
                    Delivery::UbuntuDeb { .. } | Delivery::Snap { .. }
                )
            })
            .cloned()
            .collect::<Vec<_>>();
        let collect = async move {
            use futures_util::{stream, StreamExt};
            stream::iter(entries.into_iter().map(|entry| async move {
                let (id, backend) = match &entry.delivery {
                    Delivery::UbuntuDeb { reviewed_package_id, .. } => (reviewed_package_id, "ubuntu-deb"),
                    Delivery::Snap { reviewed_package_id, .. } => (reviewed_package_id, "snap"),
                    _ => return None,
                };
                let req = NativeRequest::status(&format!("status-{}", uuid::Uuid::new_v4()), id).ok()?;
                let value = NativeClient.call(req).await.ok()?;
                native_status_matches(&value, &entry.delivery, true).ok()?;
                Some(json!({"appId":entry.id,"backend":backend,"version":value["version"],"canRollback":false}))
            })).buffer_unordered(16).filter_map(|row| async move {row}).collect::<Vec<_>>().await
        };
        match tokio::time::timeout(Duration::from_secs(3), collect).await {
            Ok(installed) => (platforms, installed),
            Err(_) => failed,
        }
    }

    async fn probe_flatpak(store: &Store) -> (bool, Vec<Value>) {
        probe_flatpak_with_program(
            &store.catalogue.entries,
            Path::new("/usr/bin/flatpak"),
            Duration::from_millis(3500),
        )
        .await
    }

    async fn probe_flatpak_with_program(
        entries: &[AppEntry],
        program: &Path,
        budget: Duration,
    ) -> (bool, Vec<Value>) {
        if !program.is_file() {
            return (false, Vec::new());
        }
        let deadline = tokio::time::Instant::now() + budget;
        // The one deadline covers remotes, list and all bounded concurrent info calls.
        let collected = async {
            let remotes = flatpak_query(
                program,
                &["remotes", "--user", "--columns=name,url"],
                64 * 1024,
                deadline,
            )
            .await?;
            if !remotes.status.success() {
                return Err("Flatpak remote query failed".into());
            }
            let remote_text = String::from_utf8_lossy(&remotes.stdout);
            let trusted_fixture = fixture_remote_is_trusted(&remote_text);
            let trusted_flathub =
                forge_store_core::backends::flatpak_remote_is_trusted(&remote_text, "flathub");
            if !trusted_fixture && !trusted_flathub {
                return Err("Flatpak remote is unavailable".into());
            }
            let listed = flatpak_query(
                program,
                &[
                    "--user",
                    "list",
                    "--app",
                    "--columns=application,arch,branch,origin,version",
                ],
                1024 * 1024,
                deadline,
            )
            .await?;
            if !listed.status.success() {
                return Err("Flatpak list query failed".into());
            }
            let rows = parse_flatpak_columns(&listed.stdout)?;
            let queries = entries
                .iter()
                .filter_map(|entry| {
                    let Delivery::Flatpak {
                        remote,
                        reference,
                        commit,
                    } = &entry.delivery
                    else {
                        return None;
                    };
                    if remote == "flathub" && !trusted_flathub
                        || remote == "forge-store-fixture" && !trusted_fixture
                    {
                        return None;
                    }
                    let row = rows
                        .iter()
                        .find(|row| flatpak_record_matches(row, reference, remote))?;
                    Some((
                        entry.id.clone(),
                        reference.clone(),
                        commit.clone(),
                        row.get("version")
                            .and_then(Value::as_str)
                            .unwrap_or("")
                            .to_string(),
                    ))
                })
                .collect::<Vec<_>>();
            use futures_util::{stream, StreamExt};
            let installed = stream::iter(queries.into_iter().map(|(id, reference, commit, version)| async move {
                if let Some(expected) = commit {
                    if flatpak_commit_info(program, &reference, deadline).await.ok().as_ref() != Some(&expected) { return None; }
                }
                Some(json!({"appId":id,"backend":"flatpak","version":version,"canRollback":false}))
            })).buffer_unordered(4).filter_map(|row| async move { row }).collect::<Vec<_>>().await;
            Ok::<_, Box<dyn std::error::Error + Send + Sync>>(installed)
        };
        match tokio::time::timeout_at(deadline, collected).await {
            Ok(Ok(installed)) => (true, installed),
            _ => (false, Vec::new()),
        }
    }

    async fn flatpak_query(
        program: &Path,
        args: &[&str],
        limit: usize,
        deadline: tokio::time::Instant,
    ) -> Result<std::process::Output, Box<dyn std::error::Error + Send + Sync>> {
        if tokio::time::Instant::now() >= deadline {
            return Err("Flatpak query total budget exhausted".into());
        }
        let mut command = tokio::process::Command::new(program);
        command.args(args).env("LC_ALL", "C").env("LANGUAGE", "C");
        Ok(bounded_process_output(
            &mut command,
            limit,
            16 * 1024,
            deadline.saturating_duration_since(tokio::time::Instant::now()),
        )
        .await?)
    }

    async fn flatpak_commit_info(
        program: &Path,
        reference: &str,
        deadline: tokio::time::Instant,
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        let info = flatpak_query(
            program,
            &["info", "--user", "--show-commit", reference],
            128,
            deadline,
        )
        .await?;
        if !info.status.success() {
            return Err("Flatpak commit lookup failed".into());
        }
        let commit = std::str::from_utf8(&info.stdout)?.trim();
        if commit.len() != 64 || !commit.bytes().all(|b| b.is_ascii_hexdigit()) {
            return Err("Flatpak commit ID is invalid".into());
        }
        Ok(commit.to_ascii_lowercase())
    }

    async fn probe_package(store: &Store) -> (bool, Vec<Value>) {
        if !Path::new("/run/forge-package/forge-package.sock").exists() {
            return (false, Vec::new());
        }
        let request = PackageRequest::list(&format!("store-{}", uuid::Uuid::new_v4())).unwrap();
        let result =
            tokio::time::timeout(Duration::from_secs(3), PackageClient::new().call(request)).await;
        let Ok(Ok(result)) = result else {
            return (false, Vec::new());
        };
        let Some(apps) = result.get("applications").and_then(Value::as_array) else {
            return (false, Vec::new());
        };
        let installed = apps
            .iter()
            .filter_map(|app| {
                let id = app.get("appId")?.as_str()?;
                let entry = store
                    .catalogue
                    .entries
                    .iter()
                    .find(|entry| entry.id == id)?;
                let Delivery::ForgePackage { artifact } = &entry.delivery else {
                    return None;
                };
                let active = app.get("active")?.as_str()?;
                let version = package_display_version(active, &artifact.sha256, &entry.version)?;
                Some(
                    json!({"appId":id,"backend":"forge-package","version":version,
                "canRollback":app.get("previous").is_some_and(|v| !v.is_null())}),
                )
            })
            .collect();
        (true, installed)
    }

    async fn worker_loop(store: Arc<Store>) {
        loop {
            match store.queue.start_next() {
                Ok(Some(job)) => {
                    let token = CancellationToken::new();
                    if let Err(error) =
                        register_running_token(&store.queue, &store.running, &job, &token)
                    {
                        eprintln!("job {} token registration failed: {error}", job.id);
                        let written = if job.native_pending {
                            store.queue.interrupt(
                                &job.id,
                                "worker registration failed while native outcome remains unknown",
                            )
                        } else {
                            store
                                .queue
                                .finish(&job.id, false, Some("worker registration failed"))
                        };
                        if let Err(write_error) = written {
                            eprintln!("job {} terminal state write failed: {write_error}", job.id);
                        }
                        continue;
                    }
                    let outcome = run_job(&store, &job, &token).await;
                    if let Ok(mut guard) = store.running.lock() {
                        *guard = None;
                    }
                    let detail = outcome
                        .as_ref()
                        .err()
                        .map(|e| bounded_detail(&e.to_string()));
                    let finished = match (store.queue.get(&job.id), &outcome) {
                        (Ok(Some(_)), Ok(RunOutcome::ProviderFailed(detail))) => store.queue.finish(&job.id,false,Some(detail)),
                        (Ok(Some(_)), Err(_)) if job.native_pending => store.queue.interrupt(&job.id,detail.as_deref().unwrap_or("Native replay remains unresolved")),
                        (Ok(Some(_)), Ok(RunOutcome::Interrupted)) => store.queue.interrupt(&job.id,"Native outcome unknown; retry replays the same authenticated provider request"),
                        (Ok(Some(current)), Ok(RunOutcome::Cancelled))
                            if current.state == JobState::Cancelling =>
                        {
                            store.queue.acknowledge_cancel(&job.id)
                        }
                        (Ok(Some(current)), Ok(RunOutcome::Completed))
                            if current.state == JobState::Cancelling =>
                        {
                            store.queue.finish_after_cancel(
                                &job.id,
                                true,
                                Some("provider completed before cancellation"),
                            )
                        }
                        (Ok(Some(current)), Err(_)) if current.state == JobState::Cancelling => {
                            store
                                .queue
                                .finish_after_cancel(&job.id, false, detail.as_deref())
                        }
                        (Ok(Some(_)), Ok(RunOutcome::Completed)) => {
                            store.queue.finish(&job.id, true, None)
                        }
                        (Ok(Some(_)), Ok(RunOutcome::Cancelled)) => store.queue.finish(
                            &job.id,
                            false,
                            Some("backend cancelled without a store cancellation request"),
                        ),
                        (Ok(Some(_)), Err(_)) => {
                            store.queue.finish(&job.id, false, detail.as_deref())
                        }
                        (Err(error), _) => {
                            eprintln!("job {} state read failed: {error}", job.id);
                            continue;
                        }
                        (Ok(None), _) => {
                            eprintln!("job {} disappeared", job.id);
                            continue;
                        }
                    };
                    if let Err(error) = finished {
                        eprintln!("job {} terminal state write failed: {error}", job.id);
                    }
                }
                Ok(None) => store.notify.notified().await,
                Err(_) => tokio::time::sleep(Duration::from_secs(5)).await,
            }
        }
    }

    fn register_running_token(
        queue: &JobQueue,
        running: &Mutex<Option<(String, CancellationToken)>>,
        job: &Job,
        token: &CancellationToken,
    ) -> Result<(), String> {
        let mut guard = running.lock().map_err(|_| "running job lock poisoned")?;
        *guard = Some((job.id.clone(), token.clone()));
        // Hold the lock while re-reading. A cancel racing after this read waits
        // for the lock, then finds and cancels the newly registered token.
        let current = queue
            .get(&job.id)
            .map_err(|e| e.to_string())?
            .ok_or("running job disappeared")?;
        if current.state == JobState::Cancelling {
            token.cancel();
        }
        Ok(())
    }

    async fn run_job(
        store: &Store,
        job: &Job,
        cancel: &CancellationToken,
    ) -> Result<RunOutcome, Box<dyn std::error::Error + Send + Sync>> {
        let entry = store
            .catalogue
            .entries
            .iter()
            .find(|entry| entry.id == job.app_id)
            .ok_or("application disappeared from signed catalogue")?;
        job.verify_delivery(&entry.delivery)?;
        match &entry.delivery {
            Delivery::Compatforge { artifact, .. } => {
                run_compat(store, entry, artifact, job.action, cancel).await
            }
            Delivery::Flatpak {
                remote,
                reference,
                commit,
            } => run_flatpak(remote, reference, commit.as_deref(), job.action, cancel).await,
            Delivery::ForgePackage { artifact } => {
                run_package(store, entry, artifact, job.action, cancel).await
            }
            Delivery::UbuntuDeb { .. } | Delivery::Snap { .. } => {
                run_native(store, entry, job, cancel).await
            }
        }
    }

    async fn run_compat(
        store: &Store,
        entry: &AppEntry,
        artifact: &forge_store_core::catalogue::Artifact,
        action: Action,
        cancel: &CancellationToken,
    ) -> Result<RunOutcome, Box<dyn std::error::Error + Send + Sync>> {
        let id = &entry.id;
        match action {
            Action::Install | Action::Update => {
                let definition = store
                    .compat
                    .operation("applications.get", json!({"id":id}))
                    .await?;
                let downloaded = store.cache.download(artifact, cancel).await;
                if cancel.is_cancelled() {
                    return Ok(RunOutcome::Cancelled);
                }
                downloaded?;
                let cached = store.cache.stage_compat_installer(artifact).await?;
                let display = std::env::var("DISPLAY").ok();
                let xauthority = std::env::var("XAUTHORITY").ok();
                let expected_appearance = match &entry.delivery {
                    Delivery::Compatforge {
                        wine_appearance, ..
                    } => *wine_appearance,
                    _ => return Err("application is not a CompatForge delivery".into()),
                };
                let request = CompatForgeRequest::install_with_appearance(
                    id,
                    &entry.version,
                    artifact,
                    cached.to_str().ok_or("cache path is not UTF-8")?,
                    &definition,
                    display.as_deref().zip(xauthority.as_deref()),
                    expected_appearance,
                )?;
                let submitted = store.compat.call(request).await?;
                let job_id = submitted
                    .get("id")
                    .and_then(Value::as_str)
                    .ok_or("missing CompatForge job ID")?;
                let mut cancel_sent = false;
                loop {
                    if cancel.is_cancelled() && !cancel_sent {
                        // A failed cancel does not prove the provider stopped. Continue polling.
                        match store
                            .compat
                            .operation("jobs.cancel", json!({"id":job_id}))
                            .await
                        {
                            Ok(_) => cancel_sent = true,
                            Err(error) => {
                                eprintln!(
                                    "CompatForge job {job_id} cancellation rejected: {error}"
                                );
                                cancel_sent = true;
                            }
                        }
                    }
                    let result = store
                        .compat
                        .operation("jobs.poll", json!({"id":job_id,"timeoutMilliseconds":1000}))
                        .await?;
                    match compatforge_poll_status(&result, job_id, id)? {
                        "succeeded" => return Ok(RunOutcome::Completed),
                        "cancelled" => return Ok(RunOutcome::Cancelled),
                        "failed" => {
                            return Err(format!(
                                "CompatForge job failed: {}",
                                result
                                    .get("job")
                                    .and_then(|job| job.get("error"))
                                    .and_then(Value::as_str)
                                    .unwrap_or("no details")
                            )
                            .into())
                        }
                        "preparing" | "running" | "cancelling" => {}
                        _ => return Err("invalid CompatForge job status".into()),
                    }
                }
            }
            Action::Uninstall => {
                store
                    .compat
                    .operation("applications.uninstall", json!({"id":id}))
                    .await?;
                Ok(RunOutcome::Completed)
            }
            Action::Rollback => {
                let history = store
                    .compat
                    .operation("applications.generations", json!({"id":id}))
                    .await?;
                let selected = history.get("selectedGeneration").and_then(Value::as_str);
                let generation = history
                    .get("generations")
                    .and_then(Value::as_array)
                    .and_then(|items| {
                        items
                            .iter()
                            .filter(|item| {
                                item.get("status") == Some(&json!("ready"))
                                    && item.get("id").and_then(Value::as_str) != selected
                            })
                            .max_by_key(|item| {
                                item.get("createdAtMilliseconds")
                                    .and_then(Value::as_u64)
                                    .unwrap_or(0)
                            })
                    })
                    .and_then(|item| item.get("id").and_then(Value::as_str))
                    .ok_or("no ready prior generation")?;
                store
                    .compat
                    .operation(
                        "applications.rollback",
                        json!({"applicationId":id,"generationId":generation}),
                    )
                    .await?;
                Ok(RunOutcome::Completed)
            }
        }
    }

    async fn run_flatpak(
        remote: &str,
        reference: &str,
        commit: Option<&str>,
        action: Action,
        cancel: &CancellationToken,
    ) -> Result<RunOutcome, Box<dyn std::error::Error + Send + Sync>> {
        run_flatpak_with_program(
            Path::new("/usr/bin/flatpak"),
            remote,
            reference,
            commit,
            action,
            cancel,
        )
        .await
    }

    async fn run_flatpak_with_program(
        program: &Path,
        remote: &str,
        reference: &str,
        commit: Option<&str>,
        action: Action,
        cancel: &CancellationToken,
    ) -> Result<RunOutcome, Box<dyn std::error::Error + Send + Sync>> {
        if remote != "forge-store-fixture" && !(remote == "flathub" && commit.is_some()) {
            return Err("Flatpak remote is not in the trusted local fixture allowlist".into());
        }
        let remotes = flatpak_query(
            program,
            &["remotes", "--user", "--columns=name,url"],
            64 * 1024,
            tokio::time::Instant::now() + Duration::from_secs(3),
        )
        .await?;
        if !remotes.status.success()
            || remotes.stdout.len() > 64 * 1024
            || !forge_store_core::backends::flatpak_remote_is_trusted(
                &String::from_utf8_lossy(&remotes.stdout),
                remote,
            )
        {
            return Err("Flatpak fixture remote URL differs from the image".into());
        }
        let before = flatpak_installed_commit_with_program(program, remote, reference).await?;
        match action {
            Action::Install if before.is_some() && commit.is_none() => {
                return Err("Flatpak application is already installed".into())
            }
            Action::Update | Action::Uninstall if before.is_none() => {
                return Err("Flatpak application is not installed from the reviewed remote".into())
            }
            _ => {}
        }
        // A prior initial install can leave the tip after a failed pin. The exact
        // reviewed list identity was confirmed above; retry resumes only pin update.
        if action == Action::Install && commit.is_some() && before.as_deref() == commit {
            return Ok(RunOutcome::Completed);
        }
        let invocation_action = if action == Action::Install && before.is_some() && commit.is_some()
        {
            Action::Update
        } else {
            action
        };
        let invocation = match commit {
            Some(commit) => {
                FlatpakInvocation::pinned(remote, reference, invocation_action, commit)?
            }
            None => FlatpakInvocation::new(remote, reference, invocation_action)?,
        };
        if cancel.is_cancelled() {
            return Ok(RunOutcome::Cancelled);
        }
        let mut command = tokio::process::Command::new(program);
        command
            .args(&invocation.args)
            .kill_on_drop(true)
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null());
        let mut child = command.spawn()?;
        let waited = wait_managed_child(&mut child, cancel, Duration::from_secs(1800)).await?;
        let mut status = waited.status;
        let mut cancelled = waited.cancelled;
        let mut timed_out = waited.timed_out;
        // The documented CLI pins only update, not install. Keep the job running through
        // initial deployment and the exact-commit update; never advertise the interim tip.
        if !cancelled && !timed_out && status.success() && invocation_action == Action::Install {
            if let Some(commit) = commit {
                let pin = FlatpakInvocation::pinned(remote, reference, Action::Update, commit)?;
                let mut pinned = tokio::process::Command::new(program)
                    .args(pin.args)
                    .stdout(std::process::Stdio::null())
                    .stderr(std::process::Stdio::null())
                    .kill_on_drop(true)
                    .spawn()?;
                let waited =
                    wait_managed_child(&mut pinned, cancel, Duration::from_secs(1800)).await?;
                status = waited.status;
                cancelled = waited.cancelled;
                timed_out = waited.timed_out;
            }
        }
        let after = flatpak_installed_commit_with_program(program, remote, reference).await?;
        let pinned = commit.is_none_or(|expected| {
            action == Action::Uninstall || after.as_deref() == Some(expected)
        });
        if timed_out {
            return Err(format!(
                "Flatpak operation timed out; observed={after:?}; expected={commit:?}"
            )
            .into());
        }
        if cancelled {
            if pinned && flatpak_action_completed(action, before.as_deref(), after.as_deref()) {
                return Ok(RunOutcome::Completed);
            }
            if before == after {
                return Ok(RunOutcome::Cancelled);
            }
            return Err(format!("Flatpak state is ambiguous after cancellation; before={before:?}; observed={after:?}; expected={commit:?}").into());
        }
        if !status.success() {
            return Err(format!(
                "Flatpak exited with {status}; observed={after:?}; expected={commit:?}"
            )
            .into());
        }
        if !pinned
            || action == Action::Uninstall && after.is_some()
            || matches!(action, Action::Install | Action::Update) && after.is_none()
        {
            return Err(format!("Flatpak reported success without expected installed state; observed={after:?}; expected={commit:?}").into());
        }
        Ok(RunOutcome::Completed)
    }

    async fn flatpak_installed_commit_with_program(
        program: &Path,
        remote: &str,
        reference: &str,
    ) -> Result<Option<String>, Box<dyn std::error::Error + Send + Sync>> {
        let deadline = tokio::time::Instant::now() + Duration::from_secs(5);
        let listed = flatpak_query(
            program,
            &[
                "--user",
                "list",
                "--app",
                "--columns=application,arch,branch,origin,version",
            ],
            1024 * 1024,
            deadline,
        )
        .await?;
        if !listed.status.success() {
            return Err("Flatpak installed list is unavailable".into());
        }
        let rows = parse_flatpak_columns(&listed.stdout)?;
        let matches = rows
            .iter()
            .filter(|row| flatpak_record_matches(row, reference, remote))
            .count();
        if matches > 1 {
            return Err("duplicate Flatpak installed reference".into());
        }
        if matches == 0 {
            return Ok(None);
        }
        Ok(Some(
            flatpak_commit_info(program, reference, deadline).await?,
        ))
    }

    async fn run_package(
        store: &Store,
        entry: &AppEntry,
        artifact: &forge_store_core::catalogue::Artifact,
        action: Action,
        cancel: &CancellationToken,
    ) -> Result<RunOutcome, Box<dyn std::error::Error + Send + Sync>> {
        let client = PackageClient::new();
        let id = || format!("store-{}", uuid::Uuid::new_v4());
        match action {
            Action::Install | Action::Update => {
                let downloaded = store.cache.download(artifact, cancel).await;
                if cancel.is_cancelled() {
                    return Ok(RunOutcome::Cancelled);
                }
                downloaded?;
                let staged = store.cache.stage_forge_package(artifact).await?;
                let path = staged.to_str().ok_or("package staging path is not UTF-8")?;
                client
                    .call(PackageRequest::install(&id(), path, artifact)?)
                    .await?;
                client
                    .call(PackageRequest::activate(
                        &id(),
                        &entry.id,
                        &artifact.sha256,
                    )?)
                    .await?;
                Ok(RunOutcome::Completed)
            }
            Action::Rollback => {
                client
                    .call(PackageRequest::rollback(&id(), &entry.id)?)
                    .await?;
                Ok(RunOutcome::Completed)
            }
            Action::Uninstall => Err("ForgeOS package uninstall is unavailable".into()),
        }
    }

    async fn run_native(
        store: &Store,
        entry: &AppEntry,
        job: &Job,
        cancel: &CancellationToken,
    ) -> Result<RunOutcome, Box<dyn std::error::Error + Send + Sync>> {
        let (id, artifact) = match &entry.delivery {
            Delivery::UbuntuDeb {
                reviewed_package_id,
                artifact,
                ..
            } => (reviewed_package_id, artifact.as_ref()),
            Delivery::Snap {
                reviewed_package_id,
                ..
            } => (reviewed_package_id, None),
            _ => return Err("not a native delivery".into()),
        };
        let operation = match job.action {
            Action::Install => "install",
            Action::Update => "update",
            Action::Uninstall => "uninstall",
            Action::Rollback => return Err("Native rollback is unavailable".into()),
        };
        let preflight = NativeClient
            .call(NativeRequest::status(
                &format!("preflight-{}", uuid::Uuid::new_v4()),
                id,
            )?)
            .await?;
        native_identity_matches(&preflight, &entry.delivery)?;
        if matches!(job.action, Action::Install | Action::Update) {
            if let Some(artifact) = artifact {
                let download = store.cache.download(artifact, cancel).await;
                if cancel.is_cancelled() {
                    return Ok(RunOutcome::Cancelled);
                }
                download?;
                store.cache.stage_native_deb(artifact).await?;
            }
        }
        if cancel.is_cancelled() {
            return Ok(RunOutcome::Cancelled);
        }
        let request = NativeRequest::mutation(
            &job.id,
            operation,
            id,
            if job.action == Action::Uninstall {
                None
            } else {
                artifact
            },
        )?;
        // Do not select on cancellation or kill dpkg. Disconnect/timeout cannot prove
        // the privileged provider stopped. Durable request ID permits safe replay.
        let result = match NativeClient.call(request).await {
            Ok(value) => value,
            Err(forge_store_core::backends::BackendError::Service { code, message }) => {
                if matches!(code.as_str(), "native_failed" | "identity_mismatch") {
                    eprintln!(
                        "Native provider {} terminal failure: {code}: {message}",
                        job.id
                    );
                    return Ok(RunOutcome::ProviderFailed(format!(
                        "Native provider terminal failure: {code}: {message}"
                    )));
                }
                if matches!(
                    code.as_str(),
                    "interrupted"
                        | "unknown-outcome"
                        | "pending"
                        | "recovery-required"
                        | "io"
                        | "unavailable"
                ) {
                    return Ok(RunOutcome::Interrupted);
                }
                return Err(
                    format!("Native provider rejected operation: {code}: {message}").into(),
                );
            }
            Err(error) => {
                eprintln!("Native request {} outcome unknown: {error}", job.id);
                return Ok(RunOutcome::Interrupted);
            }
        };
        let installed = job.action != Action::Uninstall;
        if native_status_matches(&result, &entry.delivery, installed).is_err() {
            return Ok(RunOutcome::Interrupted);
        }
        let status_req = NativeRequest::status(&format!("confirm-{}", uuid::Uuid::new_v4()), id)?;
        let checked = NativeClient.call(status_req).await;
        match checked {
            Ok(value) if native_status_matches(&value, &entry.delivery, installed).is_ok() => {
                Ok(RunOutcome::Completed)
            }
            _ => Ok(RunOutcome::Interrupted),
        }
    }

    fn bounded_detail(message: &str) -> String {
        bounded_text(message, 4096)
    }

    fn bounded_text(message: &str, limit: usize) -> String {
        let mut end = message.len().min(limit);
        while !message.is_char_boundary(end) {
            end -= 1;
        }
        message[..end].to_string()
    }

    fn backend_name(value: Backend) -> &'static str {
        match value {
            Backend::Compatforge => "compatforge",
            Backend::Flatpak => "flatpak",
            Backend::ForgePackage => "forge-package",
            Backend::UbuntuDeb => "ubuntu-deb",
            Backend::Snap => "snap",
        }
    }
    fn action_name(value: Action) -> &'static str {
        match value {
            Action::Install => "install",
            Action::Update => "update",
            Action::Uninstall => "uninstall",
            Action::Rollback => "rollback",
        }
    }
    fn state_name(value: JobState) -> &'static str {
        match value {
            JobState::Queued => "queued",
            JobState::Running => "running",
            JobState::Cancelling => "cancelling",
            JobState::Interrupted => "interrupted",
            JobState::Succeeded => "succeeded",
            JobState::Failed => "failed",
            JobState::Cancelled => "cancelled",
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[tokio::test]
        async fn replay_rejects_catalogue_backend_change_before_any_provider_call() {
            let temp = tempfile::tempdir().unwrap();
            let queue = JobQueue::open(&temp.path().join("jobs.sqlite")).unwrap();
            let old = queue
                .enqueue("org.forge.test", Backend::UbuntuDeb, Action::Install)
                .unwrap();
            queue.start_next().unwrap();
            queue.interrupt(&old.id, "native provider unknown").unwrap();
            queue.retry(&old.id).unwrap();
            let replay = queue.start_next().unwrap().unwrap();
            let entry:AppEntry=serde_json::from_value(json!({"id":"org.forge.test",
                "name":{"zhCN":"test","en":"test"},"summary":{"zhCN":"test","en":"test"},
                "publisher":"test","license":"MIT","version":"1","origin":"https://example.org/",
                "permissions":[],"compatibility":{"status":"unknown","evidence":null},
                "delivery":{"backend":"flatpak","remote":"forge-store-fixture","reference":"app/org.example.Test/x86_64/stable"}})).unwrap();
            let store = Store {
                catalogue: Catalog {
                    schema_version: 1,
                    entries: vec![entry],
                },
                queue,
                cache: VerifiedCache::open(&temp.path().join("cache")).unwrap(),
                compat: CompatForgeClient::system(&temp.path().join("requests")),
                notify: Notify::new(),
                running: Mutex::new(None),
            };
            let error = run_job(&store, &replay, &CancellationToken::new())
                .await
                .unwrap_err();
            assert_eq!(
                error.to_string(),
                "job delivery differs from signed catalogue"
            );
            assert!(store.queue.get(&old.id).unwrap().unwrap().native_pending);
        }

        fn fake_flatpak(root: &Path, count: usize, info_delay: f64) -> PathBuf {
            use std::os::unix::fs::PermissionsExt;
            let program = root.join("flatpak-test");
            let script = format!(
                r#"#!/usr/bin/python3
import pathlib,sys,time,os
root=pathlib.Path(__file__).parent
args=sys.argv[1:]
with (root/'calls').open('a') as f: f.write(' '.join(args)+'\n')
with (root/'pids').open('a') as f: f.write(str(os.getpid())+'\n')
state=root/'commit'
if 'remotes' in args:
 print('forge-store-fixture\tfile:///usr/share/forge-store/flatpak/repo-v1')
elif 'list' in args:
 if state.exists():
  for i in range({count}): print('org.example.Test'+str(i)+'\tx86_64\tstable\tforge-store-fixture\t1.0')
elif 'info' in args:
 time.sleep({info_delay})
 print(state.read_text())
elif 'install' in args:
 state.write_text('a'*64)
elif 'update' in args:
 fail=root/'fail-pin'
 if fail.exists():
  fail.unlink()
  sys.exit(1)
 state.write_text('b'*64)
"#
            );
            std::fs::write(&program, script).unwrap();
            std::fs::set_permissions(&program, std::fs::Permissions::from_mode(0o700)).unwrap();
            program
        }

        fn flatpak_entries(count: usize) -> Vec<AppEntry> {
            (0..count).map(|i| serde_json::from_value(json!({"id":format!("org.example.Test{i}"),
                "name":{"zhCN":"test","en":"test"},"summary":{"zhCN":"test","en":"test"},
                "publisher":"test","license":"MIT","version":"1","origin":"https://example.org/",
                "permissions":[],"compatibility":{"status":"unknown","evidence":null},
                "delivery":{"backend":"flatpak","remote":"forge-store-fixture",
                    "reference":format!("app/org.example.Test{i}/x86_64/stable"),"commit":"b".repeat(64)}})).unwrap()).collect()
        }

        #[tokio::test]
        async fn flatpak_pin_retry_recovers_partial_initial_deployment() {
            let temp = tempfile::tempdir().unwrap();
            let program = fake_flatpak(temp.path(), 1, 0.0);
            std::fs::write(temp.path().join("fail-pin"), "fail once").unwrap();
            let expected = "b".repeat(64);
            let cancel = CancellationToken::new();
            let initial = run_flatpak_with_program(
                &program,
                "forge-store-fixture",
                "app/org.example.Test0/x86_64/stable",
                Some(&expected),
                Action::Install,
                &cancel,
            )
            .await
            .unwrap_err();
            assert!(initial.to_string().contains("observed=Some"));
            assert_eq!(
                std::fs::read_to_string(temp.path().join("commit")).unwrap(),
                "a".repeat(64)
            );
            let retry = run_flatpak_with_program(
                &program,
                "forge-store-fixture",
                "app/org.example.Test0/x86_64/stable",
                Some(&expected),
                Action::Install,
                &cancel,
            )
            .await
            .unwrap();
            assert_eq!(retry, RunOutcome::Completed);
            assert_eq!(
                std::fs::read_to_string(temp.path().join("commit")).unwrap(),
                expected
            );
            let calls = std::fs::read_to_string(temp.path().join("calls")).unwrap();
            assert_eq!(
                calls
                    .lines()
                    .filter(|line| line.contains(" install "))
                    .count(),
                1
            );
            let again = run_flatpak_with_program(
                &program,
                "forge-store-fixture",
                "app/org.example.Test0/x86_64/stable",
                Some(&expected),
                Action::Install,
                &cancel,
            )
            .await
            .unwrap();
            assert_eq!(again, RunOutcome::Completed);
        }

        #[tokio::test]
        async fn flatpak_snapshot_reuses_list_queries_commits_concurrently_and_has_total_budget() {
            let temp = tempfile::tempdir().unwrap();
            let program = fake_flatpak(temp.path(), 8, 0.08);
            std::fs::write(temp.path().join("commit"), "b".repeat(64)).unwrap();
            let start = std::time::Instant::now();
            let (available, installed) = probe_flatpak_with_program(
                &flatpak_entries(8),
                &program,
                Duration::from_millis(600),
            )
            .await;
            assert!(available);
            assert_eq!(installed.len(), 8);
            assert!(
                start.elapsed() < Duration::from_millis(600),
                "snapshot exceeded total budget"
            );
            let calls = std::fs::read_to_string(temp.path().join("calls")).unwrap();
            assert_eq!(
                calls.lines().filter(|line| line.contains(" list ")).count(),
                1
            );
            let slow = fake_flatpak(temp.path(), 8, 1.0);
            let start = std::time::Instant::now();
            let (available, installed) =
                probe_flatpak_with_program(&flatpak_entries(8), &slow, Duration::from_millis(150))
                    .await;
            assert!(!available);
            assert!(installed.is_empty());
            assert!(
                start.elapsed() < Duration::from_millis(400),
                "slow lookup exceeded total budget"
            );
            tokio::time::sleep(Duration::from_millis(50)).await;
            for pid in std::fs::read_to_string(temp.path().join("pids"))
                .unwrap()
                .lines()
            {
                assert!(
                    !Path::new(&format!("/proc/{pid}")).exists(),
                    "query child was not stopped/reaped"
                );
            }
        }

        #[test]
        fn cancellation_between_start_and_token_registration_is_delivered() {
            let directory = tempfile::tempdir().unwrap();
            let queue = JobQueue::open(&directory.path().join("jobs.sqlite")).unwrap();
            let job = queue
                .enqueue("7zip", Backend::Compatforge, Action::Install)
                .unwrap();
            queue.start_next().unwrap();
            queue.cancel(&job.id).unwrap();
            let running = Mutex::new(None);
            let token = CancellationToken::new();
            register_running_token(&queue, &running, &job, &token).unwrap();
            assert!(token.is_cancelled());
        }

        #[test]
        fn cache_parent_is_private_for_package_service_staging() {
            use std::os::unix::fs::PermissionsExt;
            let directory = tempfile::tempdir().unwrap();
            let cache = directory.path().join(".cache/forge-store");
            std::fs::create_dir_all(cache.parent().unwrap()).unwrap();
            std::fs::set_permissions(
                cache.parent().unwrap(),
                std::fs::Permissions::from_mode(0o755),
            )
            .unwrap();
            prepare_cache_directory(&cache).unwrap();
            assert_eq!(
                std::fs::metadata(cache.parent().unwrap())
                    .unwrap()
                    .permissions()
                    .mode()
                    & 0o777,
                0o700
            );
            assert_eq!(
                std::fs::metadata(cache).unwrap().permissions().mode() & 0o777,
                0o700
            );
        }
    }
}
