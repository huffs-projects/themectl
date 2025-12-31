use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemectlConfig {
    #[serde(default = "default_deployment_method")]
    pub deployment_method: String,
    #[serde(default)]
    pub app_paths: HashMap<String, PathBuf>,
    #[serde(default)]
    pub nixos_mode: bool,
    #[serde(default)]
    pub search_paths: Vec<PathBuf>,
    #[serde(default)]
    pub nix: NixConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NixConfig {
    #[serde(default)]
    pub output_path: Option<PathBuf>,
}

fn default_deployment_method() -> String {
    "nix".to_string()
}

impl Default for NixConfig {
    fn default() -> Self {
        Self {
            output_path: None,
        }
    }
}

impl Default for ThemectlConfig {
    fn default() -> Self {
        Self {
            deployment_method: "nix".to_string(),
            app_paths: HashMap::new(),
            nixos_mode: false,
            search_paths: Vec::new(),
            nix: NixConfig::default(),
        }
    }
}

impl ThemectlConfig {
    /// Load configuration from standard locations
    pub fn load() -> Result<Option<ThemectlConfig>> {
        let config_path = Self::config_path()?;
        
        if !config_path.exists() {
            return Ok(None);
        }
        
        let content = fs::read_to_string(&config_path)
            .with_context(|| format!(
                "Failed to read configuration file at {:?}.\n\
                Possible causes:\n\
                - File permissions issue (check if you have read access)\n\
                - File is corrupted or locked by another process\n\
                - Disk I/O error\n\
                \n\
                To fix: Check file permissions and ensure the file is not locked.",
                config_path
            ))?;
        
        let config: ThemectlConfig = toml::from_str(&content)
            .with_context(|| format!(
                "Failed to parse configuration file at {:?} as TOML.\n\
                The file exists but contains invalid TOML syntax.\n\
                \n\
                Common issues:\n\
                - Missing quotes around string values\n\
                - Invalid table syntax (e.g., missing brackets)\n\
                - Invalid path format\n\
                - Invalid deployment_method value\n\
                \n\
                To fix: Check the TOML syntax in the config file. You can regenerate it by deleting \
                the file and running themectl config commands again.",
                config_path
            ))?;
        
        Ok(Some(config))
    }

    /// Save configuration to standard location
    pub fn save(&self) -> Result<()> {
        let config_path = Self::config_path()?;
        
        // Create parent directory if needed
        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent)
                .with_context(|| format!(
                    "Failed to create configuration directory at {:?}.\n\
                    Possible causes:\n\
                    - Insufficient permissions to create directories\n\
                    - Disk is full\n\
                    - Path contains invalid characters\n\
                    - Parent directory is read-only\n\
                    \n\
                    To fix: Ensure you have write permissions in the parent directory. \
                    You may need to create the directory manually: mkdir -p {:?}",
                    parent, parent
                ))?;
        }
        
        let content = toml::to_string_pretty(self)
            .with_context(|| format!(
                "Failed to serialize configuration to TOML format.\n\
                This is an internal error - the configuration structure cannot be converted to TOML.\n\
                \n\
                Possible causes:\n\
                - Invalid path values (non-UTF8 characters)\n\
                - Invalid deployment_method value\n\
                \n\
                To fix: Check your configuration values and try again. If the problem persists, \
                this may be a bug - please report it.",
            ))?;
        
        fs::write(&config_path, content)
            .with_context(|| format!(
                "Failed to write configuration file to {:?}.\n\
                Possible causes:\n\
                - Insufficient write permissions\n\
                - Disk is full\n\
                - File is locked by another process\n\
                - Parent directory doesn't exist\n\
                \n\
                To fix: Ensure you have write permissions and sufficient disk space. \
                Check if another process is using the file.",
                config_path
            ))?;
        
        Ok(())
    }

    /// Get the standard config file path
    pub fn config_path() -> Result<PathBuf> {
        let config_dir = if let Ok(xdg_config) = std::env::var("XDG_CONFIG_HOME") {
            PathBuf::from(xdg_config)
        } else {
            dirs::home_dir()
                .ok_or_else(|| anyhow::anyhow!(
                    "Could not determine home directory.\n\
                    \n\
                    Possible causes:\n\
                    - HOME environment variable is not set\n\
                    - User account has no home directory\n\
                    - Running in an unusual environment (container, chroot, etc.)\n\
                    \n\
                    To fix: Set the HOME environment variable or use XDG_CONFIG_HOME to specify \
                    the configuration directory location."
                ))?
                .join(".config")
        };
        
        Ok(config_dir.join("themectl").join("config.toml"))
    }

    /// Set a custom path for an application
    pub fn set_app_path(&mut self, app: &str, path: PathBuf) {
        self.app_paths.insert(app.to_string(), path);
    }

    /// Get a custom path for an application
    pub fn get_app_path(&self, app: &str) -> Option<&PathBuf> {
        self.app_paths.get(app)
    }

    /// Remove a custom path for an application
    pub fn remove_app_path(&mut self, app: &str) {
        self.app_paths.remove(app);
    }

    /// Get the deployment method
    pub fn get_deployment_method(&self) -> &str {
        &self.deployment_method
    }

    /// Set the deployment method
    pub fn set_deployment_method(&mut self, method: &str) -> Result<()> {
        match method {
            "standard" | "nix" => {
                self.deployment_method = method.to_string();
                Ok(())
            }
            _ => anyhow::bail!(
                "Invalid deployment method: '{}'.\n\
                \n\
                Valid deployment methods are:\n\
                - 'standard': Write config files directly to ~/.config/ locations (default)\n\
                - 'nix': Generate Nix Home Manager modules instead of config files\n\
                \n\
                You provided: '{}'\n\
                \n\
                To fix: Use one of the valid deployment methods listed above.\n\
                Example: themectl config set-deployment standard",
                method, method
            ),
        }
    }

    /// Get the nix output path, with default fallback
    pub fn get_nix_output_path(&self) -> PathBuf {
        self.nix.output_path.clone().unwrap_or_else(|| {
            dirs::home_dir()
                .unwrap_or_else(|| PathBuf::from("~"))
                .join(".config")
                .join("nixpkgs")
                .join("modules")
                .join("themectl")
        })
    }

    /// Set the nix output path
    pub fn set_nix_output_path(&mut self, path: PathBuf) {
        self.nix.output_path = Some(path);
    }
}
