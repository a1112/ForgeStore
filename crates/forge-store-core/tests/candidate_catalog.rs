use forge_store_core::catalogue::Delivery;
use forge_store_core::trusted_catalogue::TrustedCatalogue;
use std::path::Path;
use url::Url;

#[tokio::test]
async fn reviewed_candidate_upgrade_and_rollback_protection() {
    let source = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .unwrap();
    let datastore = tempfile::tempdir().unwrap();
    let state = datastore.path().join("state");
    let load = |version: &str| {
        let tuf = source.join("catalogue").join(version).join("tuf");
        let state = state.clone();
        async move {
            TrustedCatalogue::load(
                &tuf.join("trusted-root.json"),
                Url::from_directory_path(tuf.join("metadata")).unwrap(),
                Url::from_directory_path(tuf.join("targets")).unwrap(),
                &state,
            )
            .await
        }
    };
    let v1 = load("candidate-v1").await.unwrap();
    assert_eq!(v1.entries.len(), 5);
    let v2 = load("candidate-v2").await.unwrap();
    let pkg = v2
        .entries
        .iter()
        .find(|entry| entry.id == "org.forgeos.storefixture")
        .unwrap();
    assert_eq!(pkg.version, "2.0.0");
    assert!(matches!(pkg.delivery, Delivery::ForgePackage { .. }));
    assert!(load("candidate-v1").await.is_err());
}
