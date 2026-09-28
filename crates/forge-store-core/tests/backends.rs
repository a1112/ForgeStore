use forge_store_core::backends::{
    compatforge_poll_status, decode_compatforge_reply, fixture_remote_is_trusted,
    CompatForgeRequest, FlatpakInvocation, PackageRequest,
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
    )
    .unwrap();
    assert_eq!(request["operation"], "jobs.submit");
    assert_eq!(request["payload"]["applicationId"], "7zip");
    assert_eq!(request["payload"]["kind"], "install");
    assert!(CompatForgeRequest::install(
        "7zip",
        "26.02",
        &sample(),
        "/home/forge/.cache/forge-store/d64a",
        &definition
    )
    .is_err());
    let mut mismatched = definition;
    mismatched["application"]["installer"]["sha256"] = json!("00".repeat(32));
    assert!(CompatForgeRequest::install(
        "7zip",
        "26.01",
        &sample(),
        "/home/forge/.cache/forge-store/d64a",
        &mismatched
    )
    .is_err());
}

#[test]
fn flatpak_actions_use_fixed_arguments() {
    let reference = "app/org.forgeos.StoreFixture/x86_64/stable";
    let install = FlatpakInvocation::new("flathub", reference, Action::Install).unwrap();
    assert_eq!(
        install.args,
        [
            "--user",
            "install",
            "--noninteractive",
            "--assumeyes",
            "flathub",
            reference
        ]
    );
    let update = FlatpakInvocation::new("flathub", reference, Action::Update).unwrap();
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
        "flathub",
        "app/evil;touch /tmp/pwn/x86_64/stable",
        Action::Install
    )
    .is_err());
    assert!(FlatpakInvocation::new("flathub", reference, Action::Rollback).is_err());
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
