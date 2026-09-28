use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use thiserror::Error;
use url::{Host, Url};

pub const MAX_CATALOG_BYTES: usize = 512 * 1024;
pub const MAX_ARTIFACT_BYTES: u64 = 1024 * 1024 * 1024;
pub const MAX_FORGEPKG_BYTES: u64 = 64 * 1024 * 1024;

#[derive(Debug, Error)]
pub enum CatalogError {
    #[error("catalogue is too large")]
    TooLarge,
    #[error("invalid catalogue JSON: {0}")]
    Json(#[from] serde_json::Error),
    #[error("invalid catalogue: {0}")]
    Invalid(&'static str),
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Catalog {
    pub schema_version: u32,
    pub entries: Vec<AppEntry>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct LocalizedText {
    #[serde(rename = "zhCN")]
    pub zh_cn: String,
    pub en: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Compatibility {
    pub status: CompatibilityStatus,
    pub evidence: Option<String>,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum CompatibilityStatus {
    Unknown,
    Tested,
    Limited,
    Failed,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct AppEntry {
    pub id: String,
    pub name: LocalizedText,
    pub summary: LocalizedText,
    pub publisher: String,
    pub license: String,
    pub version: String,
    pub origin: String,
    pub permissions: Vec<String>,
    pub compatibility: Compatibility,
    pub delivery: Delivery,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "backend", rename_all = "kebab-case", deny_unknown_fields)]
pub enum Delivery {
    Compatforge {
        #[serde(rename = "reviewedApplicationId")]
        reviewed_application_id: String,
        artifact: Artifact,
    },
    Flatpak {
        remote: String,
        reference: String,
    },
    ForgePackage {
        artifact: Artifact,
    },
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Artifact {
    pub target: String,
    pub url: String,
    pub sha256: String,
    pub size: u64,
}

impl Catalog {
    pub fn parse(bytes: &[u8]) -> Result<Self, CatalogError> {
        if bytes.len() > MAX_CATALOG_BYTES {
            return Err(CatalogError::TooLarge);
        }
        let catalog: Self = serde_json::from_slice(bytes)?;
        if catalog.schema_version != 1 || catalog.entries.len() > 1024 {
            return Err(CatalogError::Invalid("unsupported version or entry count"));
        }
        let mut ids = HashSet::new();
        for entry in &catalog.entries {
            if !valid_id(&entry.id) || !ids.insert(&entry.id) {
                return Err(CatalogError::Invalid("invalid or duplicate application ID"));
            }
            for field in [
                &entry.name.zh_cn,
                &entry.name.en,
                &entry.summary.zh_cn,
                &entry.summary.en,
                &entry.publisher,
                &entry.license,
                &entry.version,
            ] {
                if field.is_empty() || field.len() > 512 || field.chars().any(char::is_control) {
                    return Err(CatalogError::Invalid("invalid text field"));
                }
            }
            safe_https_url(&entry.origin)?;
            if entry.permissions.len() > 32
                || entry
                    .permissions
                    .iter()
                    .any(|p| p.len() > 128 || p.is_empty())
            {
                return Err(CatalogError::Invalid("invalid permission list"));
            }
            match &entry.delivery {
                Delivery::Compatforge {
                    reviewed_application_id,
                    artifact,
                } => {
                    if reviewed_application_id != &entry.id {
                        return Err(CatalogError::Invalid("reviewed application ID differs"));
                    }
                    artifact.validate_remote(MAX_ARTIFACT_BYTES)?;
                }
                Delivery::Flatpak { remote, reference } => {
                    if !matches!(remote.as_str(), "flathub" | "forge-store-fixture")
                        || !valid_flatpak_ref(reference)
                    {
                        return Err(CatalogError::Invalid(
                            "unreviewed Flatpak remote or invalid ref",
                        ));
                    }
                }
                Delivery::ForgePackage { artifact } => {
                    artifact.validate(MAX_FORGEPKG_BYTES)?;
                }
            }
        }
        if serde_json::to_vec(&catalog)?.len() > MAX_CATALOG_BYTES {
            return Err(CatalogError::TooLarge);
        }
        Ok(catalog)
    }
}

impl Artifact {
    pub fn validate(&self, max: u64) -> Result<Url, CatalogError> {
        if self.size == 0
            || self.size > max
            || self.sha256.len() != 64
            || !self.sha256.bytes().all(|byte| byte.is_ascii_hexdigit())
            || !valid_target_name(&self.target)
        {
            return Err(CatalogError::Invalid("invalid artifact size or SHA-256"));
        }
        let parsed = Url::parse(&self.url).map_err(|_| CatalogError::Invalid("invalid URL"))?;
        if parsed.scheme() == "file" {
            let expected = format!("/usr/share/forge-store/fixtures/{}.forgepkg", self.sha256);
            if parsed.host_str().is_some()
                || parsed.query().is_some()
                || parsed.fragment().is_some()
                || parsed.path() != expected
            {
                return Err(CatalogError::Invalid(
                    "local package must use the fixed image fixture path",
                ));
            }
            Ok(parsed)
        } else {
            safe_https_url(&self.url)
        }
    }

    pub fn validate_remote(&self, max: u64) -> Result<Url, CatalogError> {
        self.validate(max)?;
        safe_https_url(&self.url)
    }
}

fn valid_target_name(name: &str) -> bool {
    !name.is_empty()
        && name.len() <= 256
        && !name.contains("..")
        && !name.starts_with('/')
        && name
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || b"-._/".contains(&b))
}

pub fn safe_https_url(input: &str) -> Result<Url, CatalogError> {
    let url = Url::parse(input).map_err(|_| CatalogError::Invalid("invalid URL"))?;
    let host = match url.host() {
        Some(Host::Domain(host)) => host,
        _ => return Err(CatalogError::Invalid("URL requires a public DNS hostname")),
    };
    if url.scheme() != "https"
        || url.username() != ""
        || url.password().is_some()
        || url.fragment().is_some()
        || url.query().is_some()
        || url.port().is_some_and(|p| p != 443)
        || !host.contains('.')
        || host.ends_with(".local")
        || host.ends_with(".localhost")
        || host == "localhost"
    {
        return Err(CatalogError::Invalid("URL is not an allowed HTTPS source"));
    }
    Ok(url)
}

fn valid_id(id: &str) -> bool {
    !id.is_empty()
        && id.len() <= 64
        && id
            .bytes()
            .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'-' || b == b'.')
        && !id.starts_with(['-', '.'])
        && !id.ends_with(['-', '.'])
}

fn valid_flatpak_ref(reference: &str) -> bool {
    reference.len() <= 256
        && reference.starts_with("app/")
        && reference.split('/').count() == 4
        && reference
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || b"./_-".contains(&b))
}
