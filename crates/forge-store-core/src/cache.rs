use crate::catalogue::{Artifact, CatalogError, MAX_ARTIFACT_BYTES, MAX_FORGEPKG_BYTES};
use bytes::Bytes;
use futures_util::{Stream, StreamExt, TryStreamExt};
use sha2::{Digest, Sha256};
use std::io;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr};
use std::path::{Path, PathBuf};
use std::time::Duration;
use thiserror::Error;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio_util::sync::CancellationToken;
use uuid::Uuid;

#[derive(Debug, Error)]
pub enum CacheError {
    #[error("invalid artifact: {0}")]
    Invalid(#[from] CatalogError),
    #[error("file operation: {0}")]
    Io(#[from] io::Error),
    #[error("HTTP operation: {0}")]
    Http(#[from] reqwest::Error),
    #[error("artifact is cancelled")]
    Cancelled,
    #[error("artifact length or digest mismatch")]
    Mismatch,
    #[error("local fixture rejected: {0}")]
    LocalFixture(&'static str),
    #[error("artifact length mismatch")]
    LengthMismatch,
    #[error("artifact SHA-256 mismatch")]
    DigestMismatch,
    #[error("artifact source resolved to a non-public address")]
    PrivateAddress,
    #[error("artifact source redirected or returned HTTP error")]
    HttpStatus,
}

pub struct VerifiedCache {
    root: PathBuf,
}

impl VerifiedCache {
    pub fn open(root: &Path) -> Result<Self, CacheError> {
        if root.exists() && std::fs::symlink_metadata(root)?.file_type().is_symlink() {
            return Err(CacheError::Io(io::Error::new(
                io::ErrorKind::InvalidInput,
                "cache root is a symlink",
            )));
        }
        std::fs::create_dir_all(root)?;
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            std::fs::set_permissions(root, std::fs::Permissions::from_mode(0o700))?;
        }
        Ok(Self {
            root: root.to_path_buf(),
        })
    }

    pub async fn ingest<S>(
        &self,
        artifact: &Artifact,
        stream: S,
        cancel: &CancellationToken,
    ) -> Result<PathBuf, CacheError>
    where
        S: Stream<Item = Result<Bytes, io::Error>> + Send,
    {
        artifact.validate(MAX_ARTIFACT_BYTES)?;
        if cancel.is_cancelled() {
            return Err(CacheError::Cancelled);
        }
        let output = self.root.join(&artifact.sha256);
        if output.exists() && file_matches(&output, artifact).await? {
            return Ok(output);
        }
        let part = self.root.join(format!(".part-{}", Uuid::new_v4()));
        let outcome = self
            .ingest_to(artifact, stream, cancel, &part, &output)
            .await;
        if outcome.is_err() {
            let _ = tokio::fs::remove_file(&part).await;
        }
        outcome
    }

    async fn ingest_to<S>(
        &self,
        artifact: &Artifact,
        stream: S,
        cancel: &CancellationToken,
        part: &Path,
        output: &Path,
    ) -> Result<PathBuf, CacheError>
    where
        S: Stream<Item = Result<Bytes, io::Error>> + Send,
    {
        let mut file = tokio::fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(part)
            .await?;
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            file.set_permissions(std::fs::Permissions::from_mode(0o600))
                .await?;
        }
        let mut hash = Sha256::new();
        let mut length = 0_u64;
        futures_util::pin_mut!(stream);
        loop {
            let next = tokio::select! {
                _ = cancel.cancelled() => return Err(CacheError::Cancelled),
                next = stream.next() => next,
            };
            let Some(chunk) = next else { break };
            let chunk = chunk?;
            length = length
                .checked_add(chunk.len() as u64)
                .ok_or(CacheError::LengthMismatch)?;
            if length > artifact.size {
                return Err(CacheError::LengthMismatch);
            }
            file.write_all(&chunk).await?;
            hash.update(&chunk);
        }
        if cancel.is_cancelled() {
            return Err(CacheError::Cancelled);
        }
        if length != artifact.size {
            return Err(CacheError::LengthMismatch);
        }
        if hex::encode(hash.finalize()) != artifact.sha256.to_ascii_lowercase() {
            return Err(CacheError::DigestMismatch);
        }
        file.sync_all().await?;
        drop(file);
        if output.exists() && !file_matches(output, artifact).await? {
            return Err(CacheError::Mismatch);
        }
        tokio::fs::rename(part, output).await?;
        Ok(output.to_path_buf())
    }

    pub async fn download(
        &self,
        artifact: &Artifact,
        cancel: &CancellationToken,
    ) -> Result<PathBuf, CacheError> {
        let url = artifact.validate(MAX_ARTIFACT_BYTES)?;
        if cancel.is_cancelled() {
            return Err(CacheError::Cancelled);
        }
        let cached = self.root.join(&artifact.sha256);
        if cached.exists() && file_matches(&cached, artifact).await? {
            return Ok(cached);
        }
        #[cfg(unix)]
        if url.scheme() == "file" {
            let path = url
                .to_file_path()
                .map_err(|_| CacheError::LocalFixture("file URL could not be decoded"))?;
            return self.ingest_local_file(&path, artifact, true, cancel).await;
        }
        #[cfg(not(unix))]
        if url.scheme() == "file" {
            return Err(CacheError::Mismatch);
        }
        let host = url.host_str().ok_or(CacheError::PrivateAddress)?;
        let port = url
            .port_or_known_default()
            .ok_or(CacheError::PrivateAddress)?;
        let addresses = tokio::time::timeout(
            Duration::from_secs(10),
            tokio::net::lookup_host((host, port)),
        )
        .await
        .map_err(|_| io::Error::new(io::ErrorKind::TimedOut, "DNS timed out"))??
        .collect::<Vec<SocketAddr>>();
        if addresses.is_empty() || addresses.iter().any(|addr| !is_public_address(addr.ip())) {
            return Err(CacheError::PrivateAddress);
        }
        let client = reqwest::Client::builder()
            .redirect(reqwest::redirect::Policy::none())
            .no_proxy()
            .resolve_to_addrs(host, &addresses)
            .connect_timeout(Duration::from_secs(10))
            .timeout(Duration::from_secs(120))
            .build()?;
        let response = tokio::select! {
            _ = cancel.cancelled() => return Err(CacheError::Cancelled),
            response = client.get(url).send() => response?,
        };
        if response.status() != reqwest::StatusCode::OK {
            return Err(CacheError::HttpStatus);
        }
        if response
            .content_length()
            .is_some_and(|size| size != artifact.size)
        {
            return Err(CacheError::Mismatch);
        }
        let chunks = response.bytes_stream().map_err(io::Error::other);
        self.ingest(artifact, chunks, cancel).await
    }

    pub async fn stage_forge_package(&self, artifact: &Artifact) -> Result<PathBuf, CacheError> {
        artifact.validate(MAX_FORGEPKG_BYTES)?;
        let cached = self.root.join(&artifact.sha256);
        if !file_matches(&cached, artifact).await? {
            return Err(CacheError::Mismatch);
        }
        let staging = self.root.join("staging");
        if staging.exists()
            && tokio::fs::symlink_metadata(&staging)
                .await?
                .file_type()
                .is_symlink()
        {
            return Err(CacheError::Mismatch);
        }
        tokio::fs::create_dir_all(&staging).await?;
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            tokio::fs::set_permissions(&staging, std::fs::Permissions::from_mode(0o700)).await?;
        }
        let output = staging.join(format!("{}.forgepkg", artifact.sha256));
        if output.exists() {
            return if file_matches(&output, artifact).await? {
                Ok(output)
            } else {
                Err(CacheError::Mismatch)
            };
        }
        let part = staging.join(format!(".part-{}", Uuid::new_v4()));
        let outcome = async {
            tokio::fs::copy(&cached, &part).await?;
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                tokio::fs::set_permissions(&part, std::fs::Permissions::from_mode(0o600)).await?;
            }
            if !file_matches(&part, artifact).await? {
                return Err(CacheError::Mismatch);
            }
            tokio::fs::OpenOptions::new()
                .write(true)
                .open(&part)
                .await?
                .sync_all()
                .await?;
            tokio::fs::rename(&part, &output).await?;
            Ok(output)
        }
        .await;
        if outcome.is_err() {
            let _ = tokio::fs::remove_file(part).await;
        }
        outcome
    }

    pub async fn stage_compat_installer(&self, artifact: &Artifact) -> Result<PathBuf, CacheError> {
        artifact.validate_remote(MAX_ARTIFACT_BYTES)?;
        let name = artifact.target.as_str();
        if name.len() > 128
            || name.starts_with('.')
            || !name.ends_with(".exe")
            || !name
                .bytes()
                .all(|byte| byte.is_ascii_alphanumeric() || b"._-".contains(&byte))
        {
            return Err(CacheError::Mismatch);
        }
        let cached = self.root.join(&artifact.sha256);
        if !file_matches(&cached, artifact).await? {
            return Err(CacheError::Mismatch);
        }
        let parent = self.root.join("windows");
        private_dir(&parent).await?;
        let version = parent.join(&artifact.sha256);
        private_dir(&version).await?;
        let output = version.join(name);
        if output.exists() {
            return if file_matches(&output, artifact).await? {
                Ok(output)
            } else {
                Err(CacheError::Mismatch)
            };
        }
        let part = version.join(format!(".part-{}", Uuid::new_v4()));
        let outcome = async {
            tokio::fs::copy(&cached, &part).await?;
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                tokio::fs::set_permissions(&part, std::fs::Permissions::from_mode(0o600)).await?;
            }
            if !file_matches(&part, artifact).await? {
                return Err(CacheError::Mismatch);
            }
            tokio::fs::OpenOptions::new()
                .write(true)
                .open(&part)
                .await?
                .sync_all()
                .await?;
            tokio::fs::rename(&part, &output).await?;
            Ok(output)
        }
        .await;
        if outcome.is_err() {
            let _ = tokio::fs::remove_file(part).await;
        }
        outcome
    }

    #[cfg(unix)]
    async fn ingest_local_file(
        &self,
        path: &Path,
        artifact: &Artifact,
        require_root: bool,
        cancel: &CancellationToken,
    ) -> Result<PathBuf, CacheError> {
        use std::os::unix::fs::{MetadataExt, PermissionsExt};
        let metadata = tokio::fs::symlink_metadata(path).await?;
        if !metadata.file_type().is_file() {
            eprintln!(
                "ForgeStore local fixture is not regular: {}",
                path.display()
            );
            return Err(CacheError::LocalFixture("not a regular file"));
        }
        if metadata.len() != artifact.size {
            eprintln!(
                "ForgeStore local fixture length {} differs from signed {}",
                metadata.len(),
                artifact.size
            );
            return Err(CacheError::LocalFixture("wrong file length"));
        }
        if metadata.nlink() != 1 {
            eprintln!("ForgeStore local fixture has {} links", metadata.nlink());
            return Err(CacheError::LocalFixture("hard-linked file"));
        }
        if metadata.permissions().mode() & 0o022 != 0 {
            eprintln!(
                "ForgeStore local fixture mode {:o} is writable",
                metadata.permissions().mode()
            );
            return Err(CacheError::LocalFixture("file is group or world writable"));
        }
        if require_root && metadata.uid() != 0 {
            eprintln!(
                "ForgeStore local fixture owner UID {} is not root",
                metadata.uid()
            );
            return Err(CacheError::LocalFixture("file is not root-owned"));
        }
        if require_root {
            let allowed = format!(
                "/usr/share/forge-store/fixtures/{}.forgepkg",
                artifact.sha256
            );
            if path != Path::new(&allowed) {
                eprintln!(
                    "ForgeStore local fixture path {} differs from {}",
                    path.display(),
                    allowed
                );
                return Err(CacheError::LocalFixture(
                    "path differs from signed image fixture path",
                ));
            }
            for directory in path
                .ancestors()
                .skip(1)
                .take_while(|part| *part != Path::new("/"))
            {
                let info = tokio::fs::symlink_metadata(directory).await?;
                if !info.file_type().is_dir()
                    || info.uid() != 0
                    || info.permissions().mode() & 0o022 != 0
                {
                    eprintln!(
                        "ForgeStore local fixture ancestor {}: dir={} uid={} mode={:o}",
                        directory.display(),
                        info.file_type().is_dir(),
                        info.uid(),
                        info.permissions().mode()
                    );
                    return Err(CacheError::LocalFixture(
                        "ancestor directory ownership or mode is unsafe",
                    ));
                }
            }
        }
        let file = tokio::fs::File::open(path).await?;
        let stream = tokio_util::io::ReaderStream::new(file);
        self.ingest(artifact, stream, cancel).await
    }
}

async fn private_dir(path: &Path) -> Result<(), CacheError> {
    if path.exists()
        && tokio::fs::symlink_metadata(path)
            .await?
            .file_type()
            .is_symlink()
    {
        return Err(CacheError::Mismatch);
    }
    tokio::fs::create_dir_all(path).await?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        tokio::fs::set_permissions(path, std::fs::Permissions::from_mode(0o700)).await?;
    }
    Ok(())
}

async fn file_matches(path: &Path, artifact: &Artifact) -> Result<bool, io::Error> {
    let metadata = tokio::fs::symlink_metadata(path).await?;
    if !metadata.file_type().is_file() || metadata.len() != artifact.size {
        return Ok(false);
    }
    let mut file = tokio::fs::File::open(path).await?;
    let mut hash = Sha256::new();
    let mut buffer = [0_u8; 64 * 1024];
    loop {
        let count = file.read(&mut buffer).await?;
        if count == 0 {
            break;
        }
        hash.update(&buffer[..count]);
    }
    Ok(hex::encode(hash.finalize()) == artifact.sha256.to_ascii_lowercase())
}

pub fn is_public_address(ip: IpAddr) -> bool {
    match ip {
        IpAddr::V4(ip) => public_v4(ip),
        IpAddr::V6(ip) => public_v6(ip),
    }
}

fn public_v4(ip: Ipv4Addr) -> bool {
    let [a, b, c, _] = ip.octets();
    if matches!(
        a,
        0 | 10 | 127 | 169 | 172 | 192 | 198 | 203 | 224..=255 | 100
    ) {
        match a {
            0 | 10 | 127 | 224..=255 => return false,
            100 if (64..=127).contains(&b) => return false,
            169 if b == 254 => return false,
            172 if (16..=31).contains(&b) => return false,
            192 if (b == 168)
                || (b == 0 && c == 0)
                || (b == 0 && c == 2)
                || (b == 88 && c == 99) =>
            {
                return false
            }
            198 if (b == 18 || b == 19) || (b == 51 && c == 100) => return false,
            203 if b == 0 && c == 113 => return false,
            _ => {}
        }
    }
    true
}

fn public_v6(ip: Ipv6Addr) -> bool {
    if let Some(mapped) = ip.to_ipv4_mapped() {
        return public_v4(mapped);
    }
    let segments = ip.segments();
    (segments[0] & 0xe000) == 0x2000 && !(segments[0] == 0x2001 && segments[1] == 0x0db8)
}

#[cfg(all(test, unix))]
mod local_file_tests {
    use super::*;
    use std::os::unix::fs::symlink;

    #[tokio::test]
    async fn local_fixture_verifies_content_and_rejects_symlink() {
        let tmp = tempfile::tempdir().unwrap();
        let source = tmp.path().join("signed.forgepkg");
        tokio::fs::write(&source, b"package fixture").await.unwrap();
        use std::os::unix::fs::PermissionsExt;
        tokio::fs::set_permissions(&source, std::fs::Permissions::from_mode(0o644))
            .await
            .unwrap();
        let digest = hex::encode(Sha256::digest(b"package fixture"));
        let artifact = Artifact {
            target: "signed.forgepkg".into(),
            url: format!("file:///usr/share/forge-store/fixtures/{digest}.forgepkg"),
            sha256: digest,
            size: 15,
        };
        let cache = VerifiedCache::open(&tmp.path().join("cache")).unwrap();
        let file = cache
            .ingest_local_file(&source, &artifact, false, &CancellationToken::new())
            .await
            .unwrap();
        assert_eq!(tokio::fs::read(file).await.unwrap(), b"package fixture");
        let link = tmp.path().join("link.forgepkg");
        symlink(&source, &link).unwrap();
        let rejected = cache
            .ingest_local_file(&link, &artifact, false, &CancellationToken::new())
            .await
            .unwrap_err();
        assert!(rejected.to_string().contains("not a regular file"));
    }
}
