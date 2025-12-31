use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

/// Metadata about a config file's last applied state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigMetadata {
    pub theme: String,
    pub hash: String,
    pub last_updated: u64,
}

/// Manager for incremental update metadata
pub struct IncrementalManager {
    cache_dir: PathBuf,
}

impl IncrementalManager {
    /// Create a new incremental manager
    pub fn new() -> Result<Self> {
        let cache_dir = Self::cache_dir()?;
        fs::create_dir_all(&cache_dir)
            .with_context(|| format!("Failed to create cache directory: {:?}", cache_dir))?;
        
        Ok(Self { cache_dir })
    }

    /// Get the cache directory path
    fn cache_dir() -> Result<PathBuf> {
        let config_dir = if let Ok(xdg_config) = std::env::var("XDG_CONFIG_HOME") {
            PathBuf::from(xdg_config)
        } else {
            dirs::home_dir()
                .ok_or_else(|| anyhow::anyhow!("Could not determine home directory"))?
                .join(".config")
        };
        
        Ok(config_dir.join("themectl").join(".cache"))
    }

    /// Get the metadata file path
    fn metadata_path(&self) -> PathBuf {
        self.cache_dir.join("metadata.toml")
    }

    /// Load all metadata from disk
    fn load_metadata(&self) -> Result<HashMap<PathBuf, ConfigMetadata>> {
        let metadata_path = self.metadata_path();
        
        if !metadata_path.exists() {
            return Ok(HashMap::new());
        }
        
        let content = fs::read_to_string(&metadata_path)
            .with_context(|| format!("Failed to read metadata file: {:?}", metadata_path))?;
        
        let metadata: HashMap<String, ConfigMetadata> = toml::from_str(&content)
            .context("Failed to parse metadata file")?;
        
        // Convert string keys to PathBuf
        Ok(metadata
            .into_iter()
            .map(|(k, v)| (PathBuf::from(k), v))
            .collect())
    }

    /// Save all metadata to disk
    fn save_metadata(&self, metadata: &HashMap<PathBuf, ConfigMetadata>) -> Result<()> {
        let metadata_path = self.metadata_path();
        
        // Convert PathBuf keys to strings
        let serializable: HashMap<String, ConfigMetadata> = metadata
            .iter()
            .map(|(k, v)| (k.display().to_string(), v.clone()))
            .collect();
        
        let content = toml::to_string_pretty(&serializable)
            .context("Failed to serialize metadata")?;
        
        fs::write(&metadata_path, content)
            .with_context(|| format!("Failed to write metadata file: {:?}", metadata_path))?;
        
        Ok(())
    }

    /// Compute SHA256 hash of content
    pub fn compute_hash(content: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(content.as_bytes());
        format!("{:x}", hasher.finalize())
    }

    /// Check if a config file needs to be updated
    pub fn should_update(&self, config_path: &Path, theme_name: &str, content: &str) -> Result<bool> {
        let content_hash = Self::compute_hash(content);
        
        // Load metadata
        let metadata = self.load_metadata()?;
        
        // Check if we have cached metadata for this file
        if let Some(cached) = metadata.get(config_path) {
            // Check if theme and content hash match
            if cached.theme == theme_name && cached.hash == content_hash {
                // Also verify the file on disk matches (in case it was modified externally)
                if config_path.exists() {
                    let existing_content = fs::read_to_string(config_path)
                        .with_context(|| format!("Failed to read existing config: {:?}", config_path))?;
                    let existing_hash = Self::compute_hash(&existing_content);
                    if existing_hash == content_hash {
                        // File is up to date, no need to write
                        return Ok(false);
                    }
                }
            }
        }
        
        // Need to update
        Ok(true)
    }

    /// Update metadata after writing a config file
    pub fn update_metadata(&self, config_path: &Path, theme_name: &str, content: &str) -> Result<()> {
        let content_hash = Self::compute_hash(content);
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        let mut metadata = self.load_metadata()?;
        
        metadata.insert(
            config_path.to_path_buf(),
            ConfigMetadata {
                theme: theme_name.to_string(),
                hash: content_hash,
                last_updated: timestamp,
            },
        );
        
        self.save_metadata(&metadata)?;
        
        Ok(())
    }

    /// Clear metadata for a specific config file
    pub fn clear_metadata(&self, config_path: &Path) -> Result<()> {
        let mut metadata = self.load_metadata()?;
        metadata.remove(config_path);
        self.save_metadata(&metadata)?;
        Ok(())
    }

    /// Clear all metadata
    pub fn clear_all(&self) -> Result<()> {
        let metadata = HashMap::new();
        self.save_metadata(&metadata)?;
        Ok(())
    }
}
