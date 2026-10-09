use forge_store_core::catalogue::{Catalog, Delivery};

fn example() -> String {
    r#"{"schemaVersion":1,"entries":[{"id":"7zip","name":{"zhCN":"7-Zip","en":"7-Zip"},"summary":{"zhCN":"压缩工具","en":"Archive tool"},"publisher":"Igor Pavlov","license":"LGPL-2.1-or-later","version":"24.09","origin":"https://www.7-zip.org/","permissions":[],"compatibility":{"status":"tested","evidence":"2026-09-28-real-vm-gui"},"delivery":{"backend":"compatforge","reviewedApplicationId":"7zip","artifact":{"target":"artifacts/7zip/7z2409-x64.exe","url":"https://www.7-zip.org/a/7z2409-x64.exe","sha256":"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa","size":1024}}}]}"#.to_string()
}

#[test]
fn valid_reviewed_windows_app_is_accepted() {
    let catalog = Catalog::parse(example().as_bytes()).unwrap();
    assert_eq!(catalog.entries[0].id, "7zip");
    assert!(matches!(
        catalog.entries[0].delivery,
        Delivery::Compatforge { .. }
    ));
}

#[test]
fn classic_appearance_is_a_closed_signed_delivery_field() {
    let mut source: serde_json::Value = serde_json::from_str(&example()).unwrap();
    source["entries"][0]["delivery"]["wineAppearance"] = serde_json::json!("classic");
    let catalog =
        Catalog::parse(&serde_json::to_vec(&source).unwrap()).expect("reviewed classic appearance");
    assert_eq!(
        serde_json::to_value(catalog).unwrap()["entries"][0]["delivery"]["wineAppearance"],
        "classic"
    );
    source["entries"][0]["delivery"]["wineAppearance"] = serde_json::json!("script");
    assert!(Catalog::parse(&serde_json::to_vec(&source).unwrap()).is_err());
}

#[test]
fn duplicate_application_ids_are_rejected() {
    let source = example();
    let entry = serde_json::from_str::<serde_json::Value>(&source).unwrap()["entries"][0].clone();
    let mut catalog: serde_json::Value = serde_json::from_str(&source).unwrap();
    catalog["entries"].as_array_mut().unwrap().push(entry);
    assert!(Catalog::parse(serde_json::to_vec(&catalog).unwrap().as_slice()).is_err());
}

#[test]
fn unsafe_artifact_url_is_rejected() {
    for url in [
        "http://example.com/file.exe",
        "https://127.0.0.1/file.exe",
        "https://localhost/file.exe",
        "file:///tmp/file.exe",
        "https://user:pass@example.com/file.exe",
    ] {
        let mut catalog: serde_json::Value = serde_json::from_str(&example()).unwrap();
        catalog["entries"][0]["delivery"]["artifact"]["url"] = url.into();
        assert!(
            Catalog::parse(serde_json::to_vec(&catalog).unwrap().as_slice()).is_err(),
            "{url}"
        );
    }
}

#[test]
fn invalid_hash_and_oversized_catalogue_are_rejected() {
    let mut catalog: serde_json::Value = serde_json::from_str(&example()).unwrap();
    catalog["entries"][0]["delivery"]["artifact"]["sha256"] = "abcd".into();
    assert!(Catalog::parse(serde_json::to_vec(&catalog).unwrap().as_slice()).is_err());
    assert!(Catalog::parse(&vec![b' '; 1024 * 1024 + 1]).is_err());
}

#[test]
fn forge_package_may_use_only_the_image_fixture_directory() {
    let mut catalog: serde_json::Value = serde_json::from_str(&example()).unwrap();
    let sha = "a".repeat(64);
    catalog["entries"][0]["id"] = "org.forgeos.storefixture".into();
    catalog["entries"][0]["delivery"] = serde_json::json!({"backend":"forge-package","artifact":{
        "target":"fixture.forgepkg","url":format!("file:///usr/share/forge-store/fixtures/{sha}.forgepkg"),
        "sha256":sha,"size":1024}});
    assert!(Catalog::parse(serde_json::to_vec(&catalog).unwrap().as_slice()).is_ok());
    catalog["entries"][0]["delivery"]["artifact"]["url"] =
        "file:///home/forge/evil.forgepkg".into();
    assert!(Catalog::parse(serde_json::to_vec(&catalog).unwrap().as_slice()).is_err());
}

#[test]
fn reviewed_local_flatpak_fixture_is_allowed() {
    let mut catalog: serde_json::Value = serde_json::from_str(&example()).unwrap();
    catalog["entries"][0]["id"] = "org.forgeos.storefixture".into();
    catalog["entries"][0]["delivery"] = serde_json::json!({"backend":"flatpak",
        "remote":"forge-store-fixture","reference":"app/org.forgeos.StoreFixture/x86_64/stable"});
    assert!(Catalog::parse(serde_json::to_vec(&catalog).unwrap().as_slice()).is_ok());
    catalog["entries"][0]["delivery"]["remote"] = "unknown-remote".into();
    assert!(Catalog::parse(serde_json::to_vec(&catalog).unwrap().as_slice()).is_err());
    catalog["entries"][0]["delivery"]["remote"] = "flathub".into();
    assert!(Catalog::parse(serde_json::to_vec(&catalog).unwrap().as_slice()).is_err());
}
