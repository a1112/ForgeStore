use forge_store_core::backends::{
    decode_native_reply, native_status_matches, FlatpakInvocation, NativeRequest,
};
use forge_store_core::catalogue::Catalog;
use forge_store_core::queue::{Action, Backend, JobQueue, JobState};
use serde_json::{json, Value};

fn catalog(delivery: Value, schema: u32) -> Value {
    json!({"schemaVersion":schema,"entries":[{"id":"org.forge.test",
        "name":{"zhCN":"测试","en":"Test"},"summary":{"zhCN":"测试工具","en":"Test tool"},
        "publisher":"Reviewed publisher","license":"GPL-3.0-or-later","version":"1.2-1",
        "origin":"https://example.org/releases/","permissions":[],
        "compatibility":{"status":"unknown","evidence":null},"delivery":delivery}]})
}

fn deb() -> Value {
    json!({"backend":"ubuntu-deb","reviewedPackageId":"org.forge.test","package":"test-app",
        "version":"1.2-1","architecture":"amd64","distribution":"ubuntu","release":"26.04"})
}

fn snap() -> Value {
    json!({"backend":"snap","reviewedPackageId":"org.forge.test","name":"test-app",
        "snapId":"abcdefghijklmnopqrstuvwx12345678","revision":123,"channel":"latest/stable",
        "confinement":"strict"})
}

fn parses(value: &Value) -> bool {
    Catalog::parse(&serde_json::to_vec(value).unwrap()).is_ok()
}

#[test]
fn v2_accepts_fixed_native_identities_but_v1_does_not() {
    for delivery in [deb(), snap()] {
        assert!(parses(&catalog(delivery.clone(), 2)));
        assert!(!parses(&catalog(delivery, 1)));
    }
}

#[test]
fn v2_flatpak_requires_pinned_commit_and_reviewed_remote() {
    let mut delivery = json!({"backend":"flatpak","remote":"flathub",
        "reference":"app/org.example.Test/x86_64/stable","commit":"a".repeat(64)});
    assert!(parses(&catalog(delivery.clone(), 2)));
    assert!(!parses(&catalog(delivery.clone(), 1)));
    delivery.as_object_mut().unwrap().remove("commit");
    assert!(!parses(&catalog(delivery.clone(), 2)));
    delivery["remote"] = json!("forge-store-fixture");
    assert!(parses(&catalog(delivery.clone(), 1)));
    delivery["commit"] = json!("a".repeat(64));
    assert!(parses(&catalog(delivery.clone(), 2)));
    delivery["remote"] = json!("unreviewed");
    assert!(!parses(&catalog(delivery, 2)));
}

#[test]
fn legacy_catalogue_cannot_acquire_a_null_commit_field() {
    let d = json!({"backend":"flatpak","remote":"forge-store-fixture",
        "reference":"app/org.example.Test/x86_64/stable","commit":null});
    assert!(!parses(&catalog(d, 1)));
}

#[test]
fn native_probe_rejects_duplicate_missing_and_unbounded_backends() {
    use forge_store_core::backends::native_probe;
    let p = json!({"backends":[{"backend":"ubuntu-deb","available":true,"detail":""},
        {"backend":"snap","available":false,"detail":"AppArmor unavailable"}]});
    assert_eq!(native_probe(&p).unwrap().len(), 2);
    let mut duplicate = p.clone();
    duplicate["backends"][1]["backend"] = json!("ubuntu-deb");
    assert!(native_probe(&duplicate).is_err());
    let mut large = p;
    large["backends"][0]["detail"] = json!("a".repeat(513));
    assert!(native_probe(&large).is_err());
}

#[test]
fn native_catalogue_rejects_option_injection_unknown_fields_and_identity_changes() {
    for (field, value) in [
        ("package", json!("--allow-unauthenticated")),
        ("version", json!("1.2;id")),
        ("architecture", json!("x86_64")),
        ("distribution", json!("arch")),
        ("reviewedPackageId", json!("different")),
        ("command", json!("sudo apt install")),
    ] {
        let mut d = deb();
        d[field] = value;
        assert!(!parses(&catalog(d, 2)), "{field}");
    }
    for (field, value) in [
        ("name", json!("--devmode")),
        ("revision", json!(true)),
        ("revision", json!(0)),
        ("revision", json!(-1)),
        ("confinement", json!("classic")),
        ("snapId", json!("a")),
        ("channel", json!("stable --devmode")),
    ] {
        let mut d = snap();
        d[field] = value;
        assert!(!parses(&catalog(d, 2)), "{field}");
    }
}

#[test]
fn local_deb_artifact_obeys_os_staging_size_boundary() {
    let mut d = deb();
    d["artifact"] = json!({"target":"test.deb","url":"https://example.org/test.deb",
        "sha256":"a".repeat(64),"size":256*1024*1024+1});
    assert!(!parses(&catalog(d, 2)));
}

#[test]
fn native_requests_are_closed_bounded_and_never_contain_paths() {
    let req = NativeRequest::mutation("store-123", "install", "org.forge.test", None).unwrap();
    let value: Value = serde_json::from_slice(&req.to_line().unwrap()).unwrap();
    assert_eq!(
        value,
        json!({"schemaVersion":1,"requestId":"store-123","operation":"install","id":"org.forge.test"})
    );
    for id in ["", "../../a", "含中文", "abc\n", "a.b"] {
        assert!(NativeRequest::mutation(id, "install", "org.forge.test", None).is_err());
    }
    assert!(NativeRequest::mutation("123", "rollback", "org.forge.test", None).is_err());
}

fn status() -> Value {
    json!({"id":"org.forge.test","backend":"ubuntu-deb","installed":true,"version":"1.2-1",
        "revision":null,"matchesPolicy":true,"identity":{"package":"test-app","architecture":"amd64",
        "distribution":"ubuntu","release":"26.04"}})
}

#[test]
fn native_reply_rejects_schema_request_framing_and_unexpected_fields() {
    let mut reply = json!({"schemaVersion":1,"requestId":"123","ok":true,"result":status()});
    let encode = |v: &Value| {
        let mut bytes = serde_json::to_vec(v).unwrap();
        bytes.push(b'\n');
        bytes
    };
    assert!(decode_native_reply(&encode(&reply), "123").is_ok());
    assert!(decode_native_reply(&encode(&reply), "other").is_err());
    reply["schemaVersion"] = json!("1");
    assert!(decode_native_reply(&encode(&reply), "123").is_err());
    reply["schemaVersion"] = json!(1);
    reply["extra"] = json!(true);
    assert!(decode_native_reply(&encode(&reply), "123").is_err());
    assert!(decode_native_reply(&vec![b' '; 4097], "123").is_err());
    assert!(decode_native_reply(b"{}\n{}\n", "123").is_err());
    let error = json!({"schemaVersion":1,"requestId":"123","ok":false,"error":{"code":"unauthorized","message":"denied"},"result":null});
    assert!(matches!(
        decode_native_reply(&encode(&error), "123"),
        Err(forge_store_core::backends::BackendError::Invalid(_))
    ));
}

#[test]
fn native_installed_status_binds_every_catalogue_identity_and_pin() {
    let parsed = Catalog::parse(&serde_json::to_vec(&catalog(deb(), 2)).unwrap()).unwrap();
    let delivery = &parsed.entries[0].delivery;
    assert!(native_status_matches(&status(), delivery, true).is_ok());
    for (field, val) in [
        ("id", json!("other")),
        ("backend", json!("snap")),
        ("version", json!("1.3")),
        ("matchesPolicy", json!(false)),
        ("revision", json!(12)),
        ("installed", json!(false)),
    ] {
        let mut s = status();
        s[field] = val;
        assert!(
            native_status_matches(&s, delivery, true).is_err(),
            "{field}"
        );
    }
    for (field, val) in [
        ("package", json!("other")),
        ("architecture", json!("arm64")),
        ("distribution", json!("arch")),
        ("release", json!("24.04")),
        ("extra", json!("value")),
    ] {
        let mut s = status();
        s["identity"][field] = val;
        assert!(
            native_status_matches(&s, delivery, true).is_err(),
            "{field}"
        );
    }
}

#[test]
fn native_preflight_allows_old_version_but_never_other_package_identity() {
    use forge_store_core::backends::native_identity_matches;
    let parsed = Catalog::parse(&serde_json::to_vec(&catalog(deb(), 2)).unwrap()).unwrap();
    let mut s = status();
    s["version"] = json!("1.1-1");
    s["matchesPolicy"] = json!(false);
    assert!(native_identity_matches(&s, &parsed.entries[0].delivery).is_ok());
    s["identity"]["package"] = json!("unrelated-app");
    assert!(native_identity_matches(&s, &parsed.entries[0].delivery).is_err());
}

#[test]
fn unresolved_job_cannot_change_backend_or_same_backend_delivery_on_retry() {
    let old = Catalog::parse(&serde_json::to_vec(&catalog(deb(), 2)).unwrap()).unwrap();
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("queue.sqlite");
    let q = JobQueue::open(&path).unwrap();
    let job = q
        .enqueue_reviewed(
            "org.forge.test",
            Backend::UbuntuDeb,
            Action::Install,
            &old.entries[0].delivery,
        )
        .unwrap();
    q.start_next().unwrap();
    q.interrupt(&job.id, "provider pending").unwrap();
    drop(q);
    let q = JobQueue::open(&path).unwrap();
    let retry = q.retry(&job.id).unwrap();
    assert!(retry.verify_delivery(&old.entries[0].delivery).is_ok());
    for mut replacement in [
        deb(),
        snap(),
        json!({"backend":"flatpak","remote":"flathub",
        "reference":"app/org.example.Test/x86_64/stable","commit":"a".repeat(64)}),
    ] {
        if replacement["backend"] == "ubuntu-deb" {
            replacement["version"] = json!("2.0-1");
        }
        let current =
            Catalog::parse(&serde_json::to_vec(&catalog(replacement, 2)).unwrap()).unwrap();
        assert!(retry.verify_delivery(&current.entries[0].delivery).is_err());
    }
    assert!(q.get(&job.id).unwrap().unwrap().native_pending);
}

#[test]
fn queue_binding_migration_preserves_old_history_and_keeps_unbound_native_unknown() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("legacy.sqlite");
    let conn = rusqlite::Connection::open(&path).unwrap();
    conn.execute_batch("CREATE TABLE jobs(sequence INTEGER PRIMARY KEY AUTOINCREMENT,id TEXT NOT NULL UNIQUE,app_id TEXT NOT NULL,backend TEXT NOT NULL,action TEXT NOT NULL,state TEXT NOT NULL,detail TEXT); INSERT INTO jobs(id,app_id,backend,action,state,detail) VALUES ('old-windows','7zip','compatforge','install','succeeded','original evidence'),('old-native','org.forge.test','ubuntu-deb','install','running',NULL);").unwrap();
    drop(conn);
    let q = JobQueue::open(&path).unwrap();
    let windows = q.get("old-windows").unwrap().unwrap();
    assert_eq!(windows.state, JobState::Succeeded);
    assert_eq!(windows.detail.as_deref(), Some("original evidence"));
    assert!(windows.delivery_sha256.is_none());
    assert!(!windows.native_pending);
    let native = q.get("old-native").unwrap().unwrap();
    assert_eq!(native.state, JobState::Interrupted);
    let d = Catalog::parse(&serde_json::to_vec(&catalog(deb(), 2)).unwrap()).unwrap();
    assert!(native.native_pending);
    assert!(native.verify_delivery(&d.entries[0].delivery).is_err());
    assert_eq!(q.list().unwrap().len(), 2);
}

#[tokio::test]
async fn managed_child_wait_bounds_timeout_and_honors_cancellation() {
    use forge_store_core::backends::wait_managed_child;
    use std::time::Duration;
    use tokio_util::sync::CancellationToken;
    fn sleeping_child() -> tokio::process::Child {
        #[cfg(windows)]
        let mut command = {
            let mut c = tokio::process::Command::new("powershell.exe");
            c.args([
                "-NoProfile",
                "-NonInteractive",
                "-Command",
                "Start-Sleep -Seconds 30",
            ]);
            c
        };
        #[cfg(not(windows))]
        let mut command = {
            let mut c = tokio::process::Command::new("/bin/sleep");
            c.arg("30");
            c
        };
        command
            .kill_on_drop(true)
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .spawn()
            .unwrap()
    }
    let mut child = sleeping_child();
    let cancel = CancellationToken::new();
    let outcome = wait_managed_child(&mut child, &cancel, Duration::from_millis(20))
        .await
        .unwrap();
    assert!(outcome.timed_out);
    assert!(!outcome.cancelled);
    assert!(child.try_wait().unwrap().is_some());
    let mut child = sleeping_child();
    cancel.cancel();
    let outcome = wait_managed_child(&mut child, &cancel, Duration::from_secs(30))
        .await
        .unwrap();
    assert!(outcome.cancelled);
    assert!(!outcome.timed_out);
    assert!(child.try_wait().unwrap().is_some());
}

#[test]
fn native_interruption_survives_restart_and_retry_reuses_provider_identity() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("queue.sqlite");
    let q = JobQueue::open(&path).unwrap();
    let job = q
        .enqueue("org.forge.test", Backend::UbuntuDeb, Action::Install)
        .unwrap();
    q.start_next().unwrap().unwrap();
    q.interrupt(&job.id, "unknown provider outcome").unwrap();
    assert_eq!(
        q.get(&job.id).unwrap().unwrap().state,
        JobState::Interrupted
    );
    assert!(q
        .enqueue("org.forge.test", Backend::Snap, Action::Install)
        .is_err());
    drop(q);
    let q = JobQueue::open(&path).unwrap();
    let retry = q.retry(&job.id).unwrap();
    assert_eq!(retry.id, job.id);
    assert_eq!(retry.state, JobState::Queued);
    assert!(
        q.cancel(&retry.id).is_err(),
        "replayed unknown native job cannot be labelled cancelled"
    );
    assert!(q
        .enqueue("org.forge.other", Backend::Snap, Action::Rollback)
        .is_err());
}

#[test]
fn pinned_flatpak_update_enforces_exact_commit() {
    let commit = "a".repeat(64);
    let invocation = FlatpakInvocation::pinned(
        "flathub",
        "app/org.example.Test/x86_64/stable",
        Action::Update,
        &commit,
    )
    .unwrap();
    assert!(invocation
        .args
        .iter()
        .any(|a| a == &format!("--commit={commit}")));
    assert!(!invocation.args.iter().any(|a| a == "--no-gpg-verify"));
    assert!(FlatpakInvocation::pinned(
        "evil",
        "app/org.example.Test/x86_64/stable",
        Action::Install,
        &commit
    )
    .is_err());
}

#[tokio::test]
async fn local_deb_staging_is_private_hash_bound_and_separate_from_forgepkg() {
    use bytes::Bytes;
    use forge_store_core::cache::VerifiedCache;
    use forge_store_core::catalogue::Artifact;
    use futures_util::stream;
    use sha2::{Digest, Sha256};
    use tokio_util::sync::CancellationToken;
    let bytes = b"verified deb bytes";
    let a = Artifact {
        target: "test.deb".into(),
        url: "https://example.org/test.deb".into(),
        sha256: hex::encode(Sha256::digest(bytes)),
        size: bytes.len() as u64,
    };
    let dir = tempfile::tempdir().unwrap();
    let cache = VerifiedCache::open(dir.path()).unwrap();
    cache
        .ingest(
            &a,
            stream::iter([Ok(Bytes::from_static(bytes))]),
            &CancellationToken::new(),
        )
        .await
        .unwrap();
    let path = cache.stage_native_deb(&a).await.unwrap();
    assert_eq!(
        path,
        dir.path().join("staging").join(format!("{}.deb", a.sha256))
    );
    assert_eq!(std::fs::read(&path).unwrap(), bytes);
    std::fs::write(&path, b"substituted").unwrap();
    assert!(cache.stage_native_deb(&a).await.is_err());
}

#[test]
fn upstream_flatpak_machine_columns_preserve_identity_and_empty_version() {
    use forge_store_core::backends::{flatpak_record_matches, parse_flatpak_columns};
    let rows=parse_flatpak_columns(b"org.example.Test\tx86_64\tstable\tflathub\t1.2\norg.example.Other\tx86_64\tstable\tflathub\t\n").unwrap();
    assert_eq!(rows.len(), 2);
    assert!(flatpak_record_matches(
        &rows[0],
        "app/org.example.Test/x86_64/stable",
        "flathub"
    ));
    assert_eq!(rows[1]["version"], "");
    assert!(parse_flatpak_columns(b"").unwrap().is_empty());
    assert!(parse_flatpak_columns(b"Name\tVersion\n").is_err());
    assert!(
        parse_flatpak_columns(b"org.example.Test\tx86_64\tstable\tflathub\t1.2\tunknown\n")
            .is_err()
    );
    assert!(parse_flatpak_columns(b"org.example.Test\tx86_64\tstable\tflathub\t1.2\norg.example.Test\tx86_64\tstable\tflathub\t1.2\n").is_err());
}
