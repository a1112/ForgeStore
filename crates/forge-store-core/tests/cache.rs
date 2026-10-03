use bytes::Bytes;
use forge_store_core::cache::{reviewed_redirect, VerifiedCache};
use forge_store_core::catalogue::Artifact;
use futures_util::stream;
use sha2::{Digest, Sha256};
use std::io;
use tokio_util::sync::CancellationToken;

fn artifact(bytes: &[u8]) -> Artifact {
    Artifact {
        target: "fixture.bin".into(),
        url: "https://example.org/fixture.bin".into(),
        sha256: hex::encode(Sha256::digest(bytes)),
        size: bytes.len() as u64,
    }
}

#[test]
fn redirect_policy_allows_pinned_release_hosts_and_rejects_other_destinations() {
    let start = url::Url::parse("https://www.7-zip.org/a/7z2601-x64.exe").unwrap();
    let github = reviewed_redirect(
        &start,
        &start,
        "https://github.com/ip7z/7zip/releases/download/26.01/7z2601-x64.exe",
    )
    .unwrap();
    assert_eq!(github.host_str(), Some("github.com"));
    assert!(reviewed_redirect(
        &start,
        &github,
        "https://release-assets.githubusercontent.com/file?sig=abc"
    )
    .is_ok());
    for target in [
        "http://github.com/file",
        "https://127.0.0.1/file",
        "https://www.7-zip.org.evil.example/file",
        "https://evil.example/file",
        "https://github.com/file?unreviewed=1",
    ] {
        assert!(
            reviewed_redirect(&start, &github, target).is_err(),
            "{target}"
        );
    }
    let sumatra = url::Url::parse("https://www.sumatrapdfreader.org/dl/file.exe").unwrap();
    assert!(reviewed_redirect(
        &sumatra,
        &sumatra,
        "https://files.sumatrapdfreader.org/software/file.exe"
    )
    .is_ok());
}

#[tokio::test]
#[ignore = "requires the reviewed upstream release URL"]
async fn live_official_7zip_redirect_reaches_verified_bytes() {
    let temporary = tempfile::tempdir().unwrap();
    let cache = VerifiedCache::open(temporary.path()).unwrap();
    let artifact = Artifact {
        target: "7z2601-x64.exe".into(),
        url: "https://www.7-zip.org/a/7z2601-x64.exe".into(),
        sha256: "d64a0468f5b5b0b0fc5b2188450bcd655b70809d97b1c4535f2884635094377d".into(),
        size: 1658851,
    };
    let downloaded = cache
        .download(&artifact, &CancellationToken::new())
        .await
        .unwrap();
    assert_eq!(downloaded.metadata().unwrap().len(), artifact.size);
    assert_eq!(
        hex::encode(Sha256::digest(std::fs::read(downloaded).unwrap())),
        artifact.sha256
    );
}

#[tokio::test]
async fn publishes_only_a_fully_verified_artifact() {
    let tmp = tempfile::tempdir().unwrap();
    let cache = VerifiedCache::open(tmp.path()).unwrap();
    let bytes = b"signed payload";
    let path = cache
        .ingest(
            &artifact(bytes),
            stream::iter([Ok::<Bytes, io::Error>(Bytes::from_static(bytes))]),
            &CancellationToken::new(),
        )
        .await
        .unwrap();
    assert_eq!(std::fs::read(path).unwrap(), bytes);
}

#[tokio::test]
async fn tamper_and_cancellation_never_publish() {
    let tmp = tempfile::tempdir().unwrap();
    let cache = VerifiedCache::open(tmp.path()).unwrap();
    let bytes = b"signed payload";
    let declared = artifact(bytes);
    assert!(cache
        .ingest(
            &declared,
            stream::iter([Ok::<Bytes, io::Error>(Bytes::from_static(b"tampered"))]),
            &CancellationToken::new()
        )
        .await
        .is_err());
    assert!(!tmp.path().join(&declared.sha256).exists());
    let cancel = CancellationToken::new();
    cancel.cancel();
    assert!(cache
        .ingest(
            &declared,
            stream::iter([Ok::<Bytes, io::Error>(Bytes::from_static(bytes))]),
            &cancel
        )
        .await
        .is_err());
    assert!(!tmp.path().join(&declared.sha256).exists());
}

#[tokio::test]
async fn forge_package_staging_is_a_distinct_private_file() {
    let tmp = tempfile::tempdir().unwrap();
    let cache = VerifiedCache::open(tmp.path()).unwrap();
    let bytes = b"fixture package";
    let declared = artifact(bytes);
    let cached = cache
        .ingest(
            &declared,
            stream::iter([Ok::<Bytes, io::Error>(Bytes::from_static(bytes))]),
            &CancellationToken::new(),
        )
        .await
        .unwrap();
    let staged = cache.stage_forge_package(&declared).await.unwrap();
    assert_eq!(
        staged,
        tmp.path()
            .join("staging")
            .join(format!("{}.forgepkg", declared.sha256))
    );
    assert_eq!(std::fs::read(&staged).unwrap(), bytes);
    assert_ne!(std::fs::metadata(&cached).unwrap().len(), 0);
    #[cfg(unix)]
    {
        use std::os::unix::fs::{MetadataExt, PermissionsExt};
        assert_eq!(std::fs::metadata(&staged).unwrap().nlink(), 1);
        assert_eq!(
            std::fs::metadata(&staged).unwrap().permissions().mode() & 0o777,
            0o600
        );
        assert_eq!(
            std::fs::metadata(tmp.path().join("staging"))
                .unwrap()
                .permissions()
                .mode()
                & 0o777,
            0o700
        );
    }
}

#[tokio::test]
async fn verified_cached_artifact_does_not_need_network() {
    let tmp = tempfile::tempdir().unwrap();
    let cache = VerifiedCache::open(tmp.path()).unwrap();
    let bytes = b"offline fixture";
    let declared = artifact(bytes);
    let local = cache
        .ingest(
            &declared,
            stream::iter([Ok::<Bytes, io::Error>(Bytes::from_static(bytes))]),
            &CancellationToken::new(),
        )
        .await
        .unwrap();
    assert_eq!(
        cache
            .download(&declared, &CancellationToken::new())
            .await
            .unwrap(),
        local
    );
}

#[tokio::test]
async fn compat_installer_alias_preserves_reviewed_basename_and_digest() {
    let tmp = tempfile::tempdir().unwrap();
    let cache = VerifiedCache::open(tmp.path()).unwrap();
    let bytes = b"fixture installer";
    let mut declared = artifact(bytes);
    declared.target = "7z2601-x64.exe".into();
    cache
        .ingest(
            &declared,
            stream::iter([Ok::<Bytes, io::Error>(Bytes::from_static(bytes))]),
            &CancellationToken::new(),
        )
        .await
        .unwrap();
    let alias = cache.stage_compat_installer(&declared).await.unwrap();
    assert_eq!(alias.file_name().unwrap(), "7z2601-x64.exe");
    assert_eq!(std::fs::read(&alias).unwrap(), bytes);
    std::fs::write(&alias, b"tampered").unwrap();
    assert!(cache.stage_compat_installer(&declared).await.is_err());
}

#[tokio::test]
async fn compat_msi_alias_preserves_pin_and_rejects_other_extensions() {
    let tmp = tempfile::tempdir().unwrap();
    let cache = VerifiedCache::open(tmp.path()).unwrap();
    let bytes = b"opaque MSI cache fixture; Core performs MSI format validation";
    let mut declared = artifact(bytes);
    declared.target = "qalculate-5.12.0-x64.msi".into();
    cache
        .ingest(
            &declared,
            stream::iter([Ok::<Bytes, io::Error>(Bytes::from_static(bytes))]),
            &CancellationToken::new(),
        )
        .await
        .unwrap();
    let alias = cache.stage_compat_installer(&declared).await.unwrap();
    assert_eq!(alias.file_name().unwrap(), "qalculate-5.12.0-x64.msi");
    assert_eq!(std::fs::read(&alias).unwrap(), bytes);
    for name in [
        "canary.msix",
        "canary.mst",
        "canary.msi.exe.tmp",
        "../canary.msi",
        "canary.msi/child",
    ] {
        declared.target = name.into();
        assert!(
            cache.stage_compat_installer(&declared).await.is_err(),
            "{name}"
        );
    }
    declared.target = "qalculate-5.12.0-x64.msi".into();
    std::fs::write(&alias, b"tampered").unwrap();
    assert!(cache.stage_compat_installer(&declared).await.is_err());
}

#[test]
fn private_and_reserved_addresses_are_not_public_sources() {
    use forge_store_core::cache::is_public_address;
    for ip in [
        "127.0.0.1",
        "10.0.0.1",
        "172.16.1.1",
        "192.168.1.1",
        "169.254.1.1",
        "100.64.0.1",
        "192.0.2.1",
        "198.51.100.2",
        "203.0.113.1",
        "::1",
        "fc00::1",
        "fe80::1",
        "::ffff:127.0.0.1",
    ] {
        assert!(!is_public_address(ip.parse().unwrap()), "{ip}");
    }
    assert!(is_public_address("1.1.1.1".parse().unwrap()));
    assert!(is_public_address("2606:4700:4700::1111".parse().unwrap()));
}
