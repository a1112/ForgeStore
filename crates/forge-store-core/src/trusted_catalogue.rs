use crate::catalogue::{
    safe_https_url, Artifact, Catalog, CatalogError, Delivery, MAX_CATALOG_BYTES,
};
use std::path::Path;
use thiserror::Error;
use tough::{ExpirationEnforcement, IntoVec, Limits, RepositoryLoader, TargetName};
use url::Url;

#[derive(Debug, Error)]
pub enum TrustError {
    #[error("trusted root or datastore: {0}")]
    Io(#[from] std::io::Error),
    #[error("invalid catalogue: {0}")]
    Catalog(#[from] CatalogError),
    #[error("TUF verification failed: {0}")]
    Tuf(#[source] Box<tough::error::Error>),
    #[error("missing or mismatched signed target binding")]
    TargetBinding,
    #[error("metadata source is not an allowed directory URL")]
    Source,
}

impl From<tough::error::Error> for TrustError {
    fn from(error: tough::error::Error) -> Self {
        Self::Tuf(Box::new(error))
    }
}

pub struct TrustedCatalogue;

impl TrustedCatalogue {
    pub async fn load(
        root_path: &Path,
        metadata: Url,
        targets: Url,
        datastore: &Path,
    ) -> Result<Catalog, TrustError> {
        check_source(&metadata)?;
        check_source(&targets)?;
        if metadata.scheme() != targets.scheme()
            || (metadata.scheme() == "https" && metadata.host_str() != targets.host_str())
        {
            return Err(TrustError::Source);
        }
        let root = tokio::fs::read(root_path).await?;
        if root.len() > 1024 * 1024 || root.is_empty() {
            return Err(TrustError::Source);
        }
        tokio::fs::create_dir_all(datastore).await?;
        let repository = RepositoryLoader::new(&root, metadata, targets)
            .datastore(datastore)
            .expiration_enforcement(ExpirationEnforcement::Safe)
            .limits(Limits {
                max_root_size: 1024 * 1024,
                max_timestamp_size: 64 * 1024,
                max_snapshot_size: 256 * 1024,
                max_targets_size: 1024 * 1024,
                max_root_updates: 32,
            })
            .load()
            .await?;
        let catalog_target = TargetName::new("catalogue.json")?;
        let signed_catalog = repository
            .all_targets()
            .find(|(name, _)| *name == &catalog_target)
            .map(|(_, target)| target)
            .ok_or(TrustError::TargetBinding)?;
        if signed_catalog.length > MAX_CATALOG_BYTES as u64 {
            return Err(TrustError::TargetBinding);
        }
        let bytes = repository
            .read_target(&catalog_target)
            .await?
            .ok_or(TrustError::TargetBinding)?
            .into_vec()
            .await?;
        let catalog = Catalog::parse(&bytes)?;
        for app in &catalog.entries {
            let artifact = match &app.delivery {
                Delivery::Compatforge { artifact, .. } | Delivery::ForgePackage { artifact } => {
                    artifact
                }
                Delivery::UbuntuDeb {
                    artifact: Some(artifact),
                    ..
                } => artifact,
                Delivery::Flatpak { .. }
                | Delivery::Snap { .. }
                | Delivery::UbuntuDeb { artifact: None, .. } => continue,
            };
            check_artifact_target(&repository, artifact)?;
        }
        Ok(catalog)
    }
}

fn check_source(url: &Url) -> Result<(), TrustError> {
    if !url.path().ends_with('/') || url.query().is_some() || url.fragment().is_some() {
        return Err(TrustError::Source);
    }
    if url.scheme() == "https" {
        safe_https_url(url.as_str())?;
    } else if url.scheme() != "file" || url.host_str().is_some() || url.to_file_path().is_err() {
        return Err(TrustError::Source);
    }
    Ok(())
}

fn check_artifact_target(
    repository: &tough::Repository,
    artifact: &Artifact,
) -> Result<(), TrustError> {
    let target_name = TargetName::new(&artifact.target)?;
    let signed = repository
        .all_targets()
        .find(|(name, _)| *name == &target_name)
        .map(|(_, target)| target)
        .ok_or(TrustError::TargetBinding)?;
    let digest = hex::decode(&artifact.sha256).map_err(|_| TrustError::TargetBinding)?;
    if signed.length != artifact.size || signed.hashes.sha256.as_ref() != digest.as_slice() {
        return Err(TrustError::TargetBinding);
    }
    Ok(())
}
