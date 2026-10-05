# Ubuntu application backends

This branch adds catalogue schema v2 and Store jobs for the independently approved Ubuntu profile. The existing v1 Windows/fixture catalogue, candidate-v22 TUF root and metadata, Windows acceptance ledger and Arch image remain unchanged. Parser/unit tests are not GUI or publication evidence.

## Catalogue

All existing application fields remain closed. v1 remains compatible and fixture-only for Flatpak. The v2 deliveries are:

```json
{"backend":"ubuntu-deb","reviewedPackageId":"org.forge.example","package":"example-app","version":"1.2-1","architecture":"amd64","distribution":"ubuntu","release":"26.04"}
```

An optional `artifact` has the existing signed target/HTTPS URL/SHA-256/size fields. It must also bind to a TUF target, is limited to 256 MiB to match the OS service and is staged as a private digest-named `.deb`. Without an artifact, the administrator-reviewed OS policy uses APT's exact `package=version`. No catalogue entry grants privileged installation authority or modifies that policy.

```json
{"backend":"snap","reviewedPackageId":"org.forge.example","name":"example-app","snapId":"abcdefghijklmnopqrstuvwx12345678","revision":123,"channel":"latest/stable","confinement":"strict"}
```

Only strict confinement is accepted. Snap names, IDs, positive revision bounds and track/risk channels are validated. Classic/devmode flags and caller commands are not supported.

```json
{"backend":"flatpak","remote":"flathub","reference":"app/org.example.App/x86_64/stable","commit":"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"}
```

v2 Flatpak requires a lowercase exact commit and either the existing fixed-image fixture remote or `flathub` at `https://dl.flathub.org/repo/`. The Ubuntu profile must provision the reviewed remote with upstream verification; Store never disables GPG verification or adds remotes from application text. Installation remains in the ordinary user's Flatpak installation.

The [upstream CLI](https://docs.flatpak.org/en/latest/flatpak-command-reference.html) has machine `list --columns` output, not `list --json`. Store parses bounded five-column output, distinguishes empty success from malformed output and rejects duplicate identities. The legacy JSON parser remains available for existing tests/clients.

The documented [commit downgrade/update](https://docs.flatpak.org/en/latest/tips-and-tricks.html#downgrading) uses `update --commit`, because `install` has no commit option. For initial installation Store performs install, then exact-commit update and independent `info --show-commit` readback before reporting success. Initial install can briefly deploy the remote tip; this is not an atomic pinned installation. No application is automatically launched. Failure retains the observed commit in the job detail, and the snapshot does not advertise a mismatching commit as accepted. Update uses the exact commit directly. If initial install already deployed the reviewed reference but pinning failed, retry verifies its remote/ref/architecture/branch and skips initial install, applying only the exact-commit update. An already matching commit is an idempotent success. Uninstall preserves user application data.

## Native package boundary and recovery

The fixed socket is `/run/forge-native-package/forge-native-package.sock`. Requests are one bounded JSON line (4096 bytes), numeric schema 1, ASCII request ID, with `probe`, `status{id}`, or install/update/uninstall by reviewed ID; local artifact requests add only SHA-256 and size, never paths or command arguments. The client checks the server's root UID using Unix peer credentials. Replies are bounded and must match the request/schema/closed status envelope.

The administrator-reviewed policy is owned by ForgeOS. Fresh peer/Polkit checks, staging validation, actual package queries, native locks and durable provider journals are enforced there. Store checks returned identity before mutation (allowing an older installed version during update), then every native identity field, installed status and exact version/revision against the signed delivery after mutation. A successful mutation is confirmed by a second status query. Failed/partial/package-version-mismatched states are not counted as installed or accepted. Native backends always report `canRollback:false`.

Snapshots expose `ubuntu-deb` and `snap` separately, including real probe unavailability. New queued operations can be cancelled; running native jobs reject cancellation, so neither dpkg nor a detached provider is killed and falsely called cancelled. Unknown outcomes (I/O, timeout, interrupted journal, malformed or mismatched completion) remain `interrupted` and block new operations for the same application. Retrying an interrupted native job reuses its original job/provider request ID, letting the OS journal replay the exact prior operation. The migrated SQLite `native_pending` flag persists across retry/restart: a queued replay cannot be cancelled as though its provider had stopped, and missing catalogue/preflight failure cannot release the unknown outcome. Only verified success or the helper's confirmed terminal failure releases it. Provider preflight rejection for a new request is a failed job; failed-job retries use a new ID. If policy/catalogue changes prevent replay of an unresolved request, administrator recovery is required; the Store does not clear the OS journal.

Flatpak query pipes are bounded while reading, before allocating their content: remotes stdout 64 KiB, installed list stdout 1 MiB, commit stdout 128 bytes, and every stderr 16 KiB. Each query has a deadline, stops/reaps its ordinary-user child on overflow or timeout, and also kills/reaps on abandoned snapshots. Snapshots reuse the one validated installed list, query exact commits with at most four concurrent children, and share a 3.5-second Flatpak budget. All backend probes together have a four-second deadline before the UI's five-second timeout. Timeout or invalid output never creates accepted installed rows.

## Validation in this change

Queue recovery also binds the original delivery: newly enqueued jobs retain a SHA-256 of the complete reviewed delivery. Dispatch checks the persisted backend and digest before any provider call, and retries retain that digest. Changing a signed catalogue entry to another backend, or changing the same backend's package/version/artifact/commit, cannot repurpose an unresolved provider request. Older native jobs without the digest fail closed and retain unresolved state; old Windows/fixture history remains present and same-backend legacy operations remain supported.

Both Flatpak initial installation and exact-commit update use the same cancellation/deadline lifecycle: at most 1800 seconds per step, then a stop attempt and at most ten seconds to reap the ordinary-user child. Cancellation is resolved by actual installed commit after the child stops, and timeout is recorded as failure with the observed state. Privileged native dpkg/APT jobs never use this child-killing helper.

Independent review found the original borrowed native-probe stream was not Send under the Linux service's spawned connection future. Native probe now collects owned catalogue entries before constructing async query futures; Windows compilation alone could not discover that Linux-only failure. A Linux service regression test additionally checks that a pending native replay with a changed Flatpak catalogue entry fails before any provider call. Builder verification and this observed red/green are separate from Windows-host tests.

Baseline Windows-host Rust tests: 40 passed, one existing network test ignored. Added 14 catalogue/protocol/queue/staging/Flatpak-column tests first; observed v2 acceptance failures, legacy-null-field and null-envelope failures, OS size-boundary failure, false queued-replay cancellation and missing API failures before implementation. Windows-host full suite: 54 passed, one existing network test ignored; fmt/check/Clippy all targets run locked/offline. Windows builds do not compile the Linux-only service module; Linux builder checks are a separate required gate.

Qt tests add v2 snapshot publishing, translated Ubuntu/Snap labels and rejection of the unsafe running-job cancel button. Both new UI assertions failed before the implementation. On the host Qt 6.11/MSVC offscreen tests use `QT_QUICK_CONTROLS_STYLE=Basic`: the pre-existing Windows native-theme/offscreen button-metrics test fails with that theme, while Basic's baseline and changed suites pass. This is a test environment setting and does not change the product theme. Full installation, actual window actions, application function, service restart and market acceptance are recorded separately after the isolated Ubuntu runtime exists.

Review regression tests observed the real Linux control flow fail before the fixes: pending native replay dispatched to a different backend; partial Flatpak deployment could not retry its failed pin; eight pinned applications exceeded the snapshot budget. The fixes passed those same tests, including no repeated list lookup, bounded concurrent slow info calls, and stopped/reaped PID checks. Cross-platform real subprocess tests additionally cover a million-byte stdout or stderr and a sleeping process deadline. Windows-host final suite after review: 58 passed, one existing network test ignored; fmt/check/Clippy all targets passed locked/offline. `FORGE_TEST_PYTHON` can name the Windows test Python interpreter; Unix uses `/usr/bin/python3`. Linux checks and release artifact hashes are recorded separately by the builder and do not count as GUI/market acceptance.
