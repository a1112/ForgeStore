use forge_store_core::trusted_catalogue::TrustedCatalogue;
use std::path::Path;
use url::Url;

fn fixture_urls(name: &str) -> (std::path::PathBuf, Url, Url) {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures")
        .join(name);
    (
        root.join("trusted-root.json"),
        Url::from_directory_path(root.join("metadata")).unwrap(),
        Url::from_directory_path(root.join("targets")).unwrap(),
    )
}

#[tokio::test]
async fn signed_catalogue_loads_and_tampered_target_fails() {
    let tmp = tempfile::tempdir().unwrap();
    let (root, metadata, targets) = fixture_urls("v1");
    let catalog = TrustedCatalogue::load(
        &root,
        metadata.clone(),
        targets.clone(),
        &tmp.path().join("state"),
    )
    .await
    .unwrap();
    assert_eq!(catalog.entries[0].id, "7zip");

    let clone = tempfile::tempdir().unwrap();
    std::fs::create_dir(clone.path().join("targets")).unwrap();
    let signed_target = std::fs::read_dir(targets.to_file_path().unwrap())
        .unwrap()
        .map(|entry| entry.unwrap().path())
        .find(|path| {
            path.file_name()
                .unwrap()
                .to_string_lossy()
                .ends_with(".catalogue.json")
        })
        .unwrap();
    std::fs::write(
        clone
            .path()
            .join("targets")
            .join(signed_target.file_name().unwrap()),
        b"tampered",
    )
    .unwrap();
    let bad_targets = Url::from_directory_path(clone.path().join("targets")).unwrap();
    assert!(
        TrustedCatalogue::load(&root, metadata, bad_targets, &tmp.path().join("state"))
            .await
            .is_err()
    );
}

#[tokio::test]
async fn expired_metadata_is_rejected() {
    let tmp = tempfile::tempdir().unwrap();
    let (root, metadata, targets) = fixture_urls("expired");
    assert!(
        TrustedCatalogue::load(&root, metadata, targets, &tmp.path().join("state"))
            .await
            .is_err()
    );
}

#[tokio::test]
async fn persistent_datastore_rejects_signed_rollback() {
    let tmp = tempfile::tempdir().unwrap();
    let (root, metadata, targets) = fixture_urls("v2");
    TrustedCatalogue::load(&root, metadata, targets, &tmp.path().join("state"))
        .await
        .unwrap();
    let (root, metadata, targets) = fixture_urls("v1");
    assert!(
        TrustedCatalogue::load(&root, metadata, targets, &tmp.path().join("state"))
            .await
            .is_err()
    );
}
