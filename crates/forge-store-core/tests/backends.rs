use forge_store_core::backends::{
    compatforge_poll_status, compatforge_selected_install, decode_compatforge_reply,
    fixture_remote_is_trusted, flatpak_action_completed, flatpak_record_matches,
    package_display_version, parse_flatpak_list, CompatForgeRequest, FlatpakInvocation,
    PackageRequest,
};
use forge_store_core::catalogue::Artifact;
use forge_store_core::queue::Action;
use serde_json::json;

#[test]
fn fixture_remote_requires_exact_image_url() {
    assert!(fixture_remote_is_trusted(
        "forge-store-fixture file:///usr/share/forge-store/flatpak/repo-v1\n"
    ));
    assert!(fixture_remote_is_trusted("other https://example.org\nforge-store-fixture file:///usr/share/forge-store/flatpak/repo-v2\n"));
    assert!(!fixture_remote_is_trusted(
        "forge-store-fixture https://example.org/repo\n"
    ));
    assert!(!fixture_remote_is_trusted(
        "forge-store-fixture file:///tmp/repo-v1\n"
    ));
}

fn sample() -> Artifact {
    Artifact {
        target: "7z2601-x64.exe".into(),
        url: "https://www.7-zip.org/a/7z2601-x64.exe".into(),
        sha256: "d64a0468f5b5b0b0fc5b2188450bcd655b70809d97b1c4535f2884635094377d".into(),
        size: 1658851,
    }
}

#[test]
fn windows_install_must_match_reviewed_service_recipe() {
    let definition = json!({"application":{"schemaVersion":"1","id":"7zip","version":"26.01","installer":{
        "fileName":"7z2601-x64.exe","sha256":"d64a0468f5b5b0b0fc5b2188450bcd655b70809d97b1c4535f2884635094377d","arguments":["/S"]}}});
    let request = CompatForgeRequest::install(
        "7zip",
        "26.01",
        &sample(),
        "/home/forge/.cache/forge-store/d64a",
        &definition,
        Some((":0", "/run/user/1000/xauth_test")),
    )
    .unwrap();
    assert_eq!(request["operation"], "jobs.submit");
    assert_eq!(request["payload"]["applicationId"], "7zip");
    assert_eq!(request["payload"]["kind"], "install");
    assert_eq!(request["payload"]["environmentOverrides"]["DISPLAY"], ":0");
    assert_eq!(
        request["payload"]["environmentOverrides"]["XAUTHORITY"],
        "/run/user/1000/xauth_test"
    );
    assert!(CompatForgeRequest::install(
        "7zip",
        "26.02",
        &sample(),
        "/home/forge/.cache/forge-store/d64a",
        &definition,
        None
    )
    .is_err());
    let mut mismatched = definition.clone();
    mismatched["application"]["installer"]["sha256"] = json!("00".repeat(32));
    assert!(CompatForgeRequest::install(
        "7zip",
        "26.01",
        &sample(),
        "/home/forge/.cache/forge-store/d64a",
        &mismatched,
        None
    )
    .is_err());
    assert!(CompatForgeRequest::install(
        "7zip",
        "26.01",
        &sample(),
        "/home/forge/.cache/forge-store/d64a",
        &definition,
        Some(("evil.example:0", "/run/user/1000/xauth_test"))
    )
    .is_err());
}

#[test]
fn flatpak_actions_use_fixed_arguments() {
    let reference = "app/org.forgeos.StoreFixture/x86_64/stable";
    let install =
        FlatpakInvocation::new("forge-store-fixture", reference, Action::Install).unwrap();
    assert_eq!(
        install.args,
        [
            "--user",
            "install",
            "--noninteractive",
            "--assumeyes",
            "forge-store-fixture",
            reference
        ]
    );
    let update = FlatpakInvocation::new("forge-store-fixture", reference, Action::Update).unwrap();
    assert_eq!(
        update.args,
        [
            "--user",
            "update",
            "--noninteractive",
            "--assumeyes",
            reference
        ]
    );
    assert!(FlatpakInvocation::new(
        "forge-store-fixture",
        "app/evil;touch /tmp/pwn/x86_64/stable",
        Action::Install
    )
    .is_err());
    assert!(FlatpakInvocation::new("forge-store-fixture", reference, Action::Rollback).is_err());
    assert!(FlatpakInvocation::new("flathub", reference, Action::Install).is_err());
}

#[test]
fn flatpak_installed_identity_binds_ref_arch_branch_and_remote() {
    let reference = "app/org.forgeos.StoreFixture/x86_64/stable";
    let row = json!({"application_id":"org.forgeos.StoreFixture","arch":"x86_64",
        "branch":"stable","origin":"forge-store-fixture","version":"1.0"});
    assert!(flatpak_record_matches(
        &row,
        reference,
        "forge-store-fixture"
    ));
    for (field, value) in [
        ("arch", "aarch64"),
        ("branch", "beta"),
        ("origin", "evil-remote"),
        ("application_id", "org.forgeos.Other"),
    ] {
        let mut wrong = row.clone();
        wrong[field] = value.into();
        assert!(!flatpak_record_matches(
            &wrong,
            reference,
            "forge-store-fixture"
        ));
    }
}

#[test]
fn flatpak_json_list_accepts_empty_success_and_rejects_malformed_output() {
    assert!(parse_flatpak_list(b"").unwrap().is_empty());
    assert!(parse_flatpak_list(b"\n").unwrap().is_empty());
    assert!(parse_flatpak_list(b"[]\n").unwrap().is_empty());
    let records = parse_flatpak_list(br#"[{"application_id":"org.forgeos.StoreFixture","arch":"x86_64","branch":"stable","origin":"forge-store-fixture","version":"1"}]"#).unwrap();
    assert_eq!(records.len(), 1);
    assert!(parse_flatpak_list(b"not json").is_err());
}

#[cfg(unix)]
#[test]
#[ignore = "run in the pinned Arch image with Flatpak 1.18 installed"]
fn pinned_flatpak_cli_columns_parse_under_chinese_desktop_locale() {
    let result = std::process::Command::new("/usr/bin/flatpak")
        .args([
            "--user",
            "list",
            "--app",
            "--json",
            "--columns=application,arch,branch,origin,version",
        ])
        .env("LANG", "zh_CN.UTF-8")
        .env("LC_ALL", "C")
        .env("LANGUAGE", "C")
        .output()
        .unwrap();
    assert!(
        result.status.success(),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
    parse_flatpak_list(&result.stdout).unwrap();
}

#[test]
fn old_forge_package_digest_is_labeled_without_unverified_semantic_version() {
    let old = "a".repeat(64);
    let current = "b".repeat(64);
    assert_eq!(
        package_display_version(&current, &current, "2.0.0").as_deref(),
        Some("2.0.0")
    );
    assert_eq!(
        package_display_version(&old, &current, "2.0.0").as_deref(),
        Some("SHA-256 aaaaaaaaaaaa")
    );
    assert!(package_display_version("invalid", &current, "2.0.0").is_none());
}

#[test]
fn flatpak_cancel_resolution_uses_provider_commit_state() {
    assert!(flatpak_action_completed(Action::Install, None, Some("new")));
    assert!(!flatpak_action_completed(Action::Install, None, None));
    assert!(flatpak_action_completed(
        Action::Update,
        Some("old"),
        Some("new")
    ));
    assert!(!flatpak_action_completed(
        Action::Update,
        Some("old"),
        Some("old")
    ));
    assert!(flatpak_action_completed(
        Action::Uninstall,
        Some("old"),
        None
    ));
    assert!(!flatpak_action_completed(
        Action::Uninstall,
        Some("old"),
        Some("old")
    ));
}

#[test]
fn compatforge_installed_version_tracks_selected_generation_after_rollback() {
    let summary = json!({"installed":true,"application":{"id":"7zip","version":"26.01"},
        "generations":{"applicationId":"7zip","selectedGeneration":"old",
            "generations":[
                {"id":"old","status":"ready","definition":{"id":"7zip","version":"25.01"}},
                {"id":"new","status":"ready","definition":{"id":"7zip","version":"26.01"}}]}});
    assert_eq!(
        compatforge_selected_install(&summary, "7zip"),
        Some(("25.01".into(), true))
    );
    assert_eq!(compatforge_selected_install(&summary, "other"), None);
    let mut bad = summary;
    bad["generations"]["generations"][0]["definition"]["id"] = "other".into();
    assert_eq!(compatforge_selected_install(&bad, "7zip"), None);
}

#[test]
fn package_service_request_is_canonical_ascii_and_bounded() {
    let path = format!(
        "/home/forge/.cache/forge-store/staging/{}.forgepkg",
        sample().sha256
    );
    let request = PackageRequest::install("r1", &path, &sample()).unwrap();
    let encoded = request.to_canonical_line().unwrap();
    assert_eq!(encoded, format!("{{\"operation\":\"install\",\"path\":\"{}\",\"requestId\":\"r1\",\"schemaVersion\":1,\"sha256\":\"{}\",\"size\":1658851}}\n", path, sample().sha256).as_bytes());
    assert!(PackageRequest::status("bad id", "org.forgeos.storefixture").is_err());
}

#[test]
fn compatforge_reply_must_match_request_and_operation() {
    let value = json!({"schemaVersion":"1","requestId":"store-1","operation":"applications.get",
        "result":{"application":{"id":"7zip"}}});
    assert_eq!(
        decode_compatforge_reply(
            &serde_json::to_vec(&value).unwrap(),
            "store-1",
            "applications.get"
        )
        .unwrap()["application"]["id"],
        "7zip"
    );
    assert!(decode_compatforge_reply(
        &serde_json::to_vec(&value).unwrap(),
        "store-2",
        "applications.get"
    )
    .is_err());
    assert!(decode_compatforge_reply(
        &serde_json::to_vec(&value).unwrap(),
        "store-1",
        "jobs.submit"
    )
    .is_err());
}

#[test]
fn compatforge_poll_uses_nested_job_and_checks_identity() {
    let result = json!({"job":{"schemaVersion":"1","id":"job-1","applicationId":"7zip","status":"succeeded"},
        "events":[],"streamEnded":true});
    assert_eq!(
        compatforge_poll_status(&result, "job-1", "7zip").unwrap(),
        "succeeded"
    );
    assert!(compatforge_poll_status(&result, "job-2", "7zip").is_err());
    assert!(compatforge_poll_status(&result, "job-1", "other").is_err());
    assert!(compatforge_poll_status(&json!({"status":"succeeded"}), "job-1", "7zip").is_err());
}
