use forge_store_core::catalogue::{CompatibilityStatus, Delivery};
use forge_store_core::trusted_catalogue::TrustedCatalogue;
use std::path::Path;
use tough::{IntoVec, RepositoryLoader, TargetName};
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
    let v3 = load("candidate-v3").await.unwrap();
    assert_eq!(v3.entries.len(), 5);
    for entry in &v3.entries {
        if matches!(entry.delivery, Delivery::Compatforge { .. }) {
            assert!(matches!(
                entry.compatibility.status,
                CompatibilityStatus::Tested
            ));
            assert_eq!(
                entry.compatibility.evidence.as_deref(),
                Some(format!("windows-rolling-20260929.acceptance.json#{}", entry.id).as_str())
            );
        }
    }
    assert!(load("candidate-v2").await.is_err());

    let tuf = source.join("catalogue/candidate-v3/tuf");
    let root = std::fs::read(tuf.join("trusted-root.json")).unwrap();
    let repository = RepositoryLoader::new(
        &root,
        Url::from_directory_path(tuf.join("metadata")).unwrap(),
        Url::from_directory_path(tuf.join("targets")).unwrap(),
    )
    .load()
    .await
    .unwrap();
    let evidence = repository
        .read_target(&TargetName::new("windows-rolling-20260929.acceptance.json").unwrap())
        .await
        .unwrap()
        .unwrap()
        .into_vec()
        .await
        .unwrap();
    let evidence: serde_json::Value = serde_json::from_slice(&evidence).unwrap();
    for entry in &v3.entries {
        if let Delivery::Compatforge { artifact, .. } = &entry.delivery {
            let acceptance = evidence["applications"]
                .as_array()
                .unwrap()
                .iter()
                .find(|app| app["id"] == entry.id)
                .unwrap();
            assert_eq!(acceptance["version"], entry.version);
            assert_eq!(acceptance["sha256"], artifact.sha256);
            assert_eq!(acceptance["size"], artifact.size);
            assert_eq!(acceptance["guiFileOperation"], "passed");
        }
    }

    let v4 = load("candidate-v4").await.unwrap();
    assert_eq!(v4.entries.len(), 6);
    let sqlite = v4
        .entries
        .iter()
        .find(|entry| entry.id == "sqlitestudio")
        .unwrap();
    assert_eq!(sqlite.version, "3.4.17");
    assert!(matches!(
        sqlite.compatibility.status,
        CompatibilityStatus::Tested
    ));
    assert_eq!(
        sqlite.compatibility.evidence.as_deref(),
        Some("windows-sqlitestudio-20260929.acceptance.json#sqlitestudio")
    );
    let sqlite_tuf = source.join("catalogue/candidate-v4/tuf");
    let sqlite_repository = RepositoryLoader::new(
        &std::fs::read(sqlite_tuf.join("trusted-root.json")).unwrap(),
        Url::from_directory_path(sqlite_tuf.join("metadata")).unwrap(),
        Url::from_directory_path(sqlite_tuf.join("targets")).unwrap(),
    )
    .load()
    .await
    .unwrap();
    let sqlite_evidence = sqlite_repository
        .read_target(&TargetName::new("windows-sqlitestudio-20260929.acceptance.json").unwrap())
        .await
        .unwrap()
        .unwrap()
        .into_vec()
        .await
        .unwrap();
    let sqlite_evidence: serde_json::Value = serde_json::from_slice(&sqlite_evidence).unwrap();
    let sqlite_acceptance = &sqlite_evidence["applications"][0];
    assert_eq!(sqlite_acceptance["id"], "sqlitestudio");
    assert_eq!(sqlite_acceptance["guiFileOperation"], "passed");
    assert_eq!(sqlite_acceptance["sqliteRows"], serde_json::json!([["ok"]]));
    if let Delivery::Compatforge { artifact, .. } = &sqlite.delivery {
        assert_eq!(sqlite_acceptance["sha256"], artifact.sha256);
        assert_eq!(sqlite_acceptance["size"], artifact.size);
    } else {
        panic!("SQLiteStudio must use CompatForge delivery");
    }
    assert!(load("candidate-v3").await.is_err());
}
