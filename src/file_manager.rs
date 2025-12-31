use anyhow::{Context, Result};
use colored::*;
use rayon::prelude::*;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::config::ThemectlConfig;
use crate::generators;
use crate::incremental::IncrementalManager;
use crate::theme::Theme;

pub struct FileManager {
    config_dir: Option<PathBuf>,
    dry_run: bool,
    themectl_config: Option<crate::config::ThemectlConfig>,
    incremental: Option<IncrementalManager>,
}

impl FileManager {
    pub fn new(config_dir: Option<&PathBuf>, dry_run: bool) -> Self {
        let themectl_config = crate::config::ThemectlConfig::load().ok().flatten();
        let incremental = IncrementalManager::new().ok();
        Self {
            config_dir: config_dir.cloned(),
            dry_run,
            themectl_config,
            incremental,
        }
    }

    pub fn with_config(config_dir: Option<&PathBuf>, dry_run: bool, themectl_config: Option<crate::config::ThemectlConfig>) -> Self {
        let incremental = IncrementalManager::new().ok();
        Self {
            config_dir: config_dir.cloned(),
            dry_run,
            themectl_config,
            incremental,
        }
    }
    
    pub fn apply_theme(&self, theme: &Theme) -> Result<()> {
        let configs = self.detect_config_files(theme)?;
        
        println!("\n{} Applying theme to {} config files...\n", "→".cyan(), configs.len());
        
        // Use parallel processing for better performance
        let theme = Arc::new(theme.clone());
        let results: Vec<_> = configs
            .par_iter()
            .map(|(app, path)| {
                let result = self.apply_to_file(&theme, app, path);
                (app.clone(), path.clone(), result)
            })
            .collect();
        
        // Process results and maintain output ordering
        for (app, _path, result) in results {
            match result {
                Ok(_) => {
                    if self.dry_run {
                        println!("  {} {} (dry-run)", "✓".yellow(), app);
                    } else {
                        println!("  {} {}", "✓".green(), app);
                    }
                }
                Err(e) => {
                    eprintln!("  {} {} - Error: {}", "✗".red(), app, e);
                }
            }
        }
        
        Ok(())
    }

    pub fn apply_theme_filtered(&self, theme: &Theme, apps: &[&str]) -> Result<()> {
        let configs = self.detect_config_files(theme)?;
        let app_set: std::collections::HashSet<&str> = apps.iter().cloned().collect();
        
        let filtered: Vec<_> = configs.into_iter()
            .filter(|(app, _)| app_set.contains(app.as_str()))
            .collect();
        
        println!("\n{} Applying theme to {} config files...\n", "→".cyan(), filtered.len());
        
        // Use parallel processing for better performance
        let theme = Arc::new(theme.clone());
        let results: Vec<_> = filtered
            .par_iter()
            .map(|(app, path)| {
                let result = self.apply_to_file(&theme, app, path);
                (app.clone(), path.clone(), result)
            })
            .collect();
        
        // Process results and maintain output ordering
        for (app, _path, result) in results {
            match result {
                Ok(_) => {
                    if self.dry_run {
                        println!("  {} {} (dry-run)", "✓".yellow(), app);
                    } else {
                        println!("  {} {}", "✓".green(), app);
                    }
                }
                Err(e) => {
                    eprintln!("  {} {} - Error: {}", "✗".red(), app, e);
                }
            }
        }
        
        Ok(())
    }
    
    fn apply_to_file(&self, theme: &Theme, app: &str, path: &Path) -> Result<()> {
        let deployment_method = self.themectl_config.as_ref()
            .map(|c| c.get_deployment_method())
            .unwrap_or("nix");
        
        match deployment_method {
            "nix" => self.apply_nix(theme, app)
                .with_context(|| format!(
                    "Failed to apply theme '{}' to application '{}' using Nix deployment method.\n\
                    \n\
                    Deployment method: nix\n\
                    Application: {}\n\
                    \n\
                    This will generate a Home Manager module. Ensure your nix.output_path is correctly configured.",
                    theme.name, app, app
                ))?,
            _ => self.apply_standard(theme, app, path)
                .with_context(|| format!(
                    "Failed to apply theme '{}' to application '{}' using standard deployment method.\n\
                    \n\
                    Deployment method: standard\n\
                    Application: {}\n\
                    Target path: {:?}\n\
                    \n\
                    This will write directly to the configuration file location.",
                    theme.name, app, app, path
                ))?,
        }
        
        Ok(())
    }
    
    fn apply_standard(&self, theme: &Theme, app: &str, path: &Path) -> Result<()> {
        let content = generators::generate(theme, app)
            .with_context(|| format!(
                "Failed to generate configuration for application '{}'.\n\
                \n\
                Theme: {}\n\
                Application: {}\n\
                Target path: {:?}\n\
                \n\
                Possible causes:\n\
                - Application '{}' is not supported\n\
                - Theme data is invalid or incomplete\n\
                - Generator encountered an internal error\n\
                \n\
                To fix: Check if the application is supported. Use 'themectl list' to see available themes.",
                app, theme.name, app, path, app
            ))?;
        
        if self.dry_run {
            println!("    Would write to: {:?}", path);
            return Ok(());
        }
        
        // Check if incremental update should skip this file
        if let Some(ref incremental) = self.incremental {
            if let Ok(false) = incremental.should_update(path, &theme.name, &content) {
                // File is up to date, skip writing
                return Ok(());
            }
        }
        
        // Backup existing file
        if path.exists() {
            self.backup_file(path)?;
        }
        
        // Create parent directory if needed
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)
                .with_context(|| format!(
                    "Failed to create directory for application '{}' at {:?}.\n\
                    \n\
                    Possible causes:\n\
                    - Insufficient permissions to create directories\n\
                    - Disk is full\n\
                    - Path contains invalid characters\n\
                    - Parent directory is read-only\n\
                    \n\
                    To fix: Ensure you have write permissions. You may need to create the directory manually:\n\
                    mkdir -p {:?}",
                    app, parent, parent
                ))?;
        }
        
        // Write new content
        fs::write(path, &content)
            .with_context(|| format!(
                "Failed to write configuration for application '{}' to {:?}.\n\
                \n\
                Possible causes:\n\
                - Insufficient write permissions\n\
                - Disk is full\n\
                - File is locked by another process\n\
                - Path is read-only\n\
                \n\
                To fix: Check file permissions and ensure sufficient disk space. \
                If the file is locked, close any applications using it.",
                app, path
            ))?;
        
        // Update incremental metadata
        if let Some(ref incremental) = self.incremental {
            incremental.update_metadata(path, &theme.name, &content)?;
        }
        
        Ok(())
    }
    
    fn apply_nix(&self, theme: &Theme, app: &str) -> Result<()> {
        let nix_path = if let Some(config) = &self.themectl_config {
            config.get_nix_output_path()
        } else {
            dirs::home_dir()
                .unwrap_or_else(|| PathBuf::from("~"))
                .join(".config")
                .join("nixpkgs")
                .join("modules")
                .join("themectl")
        };
        
        // Generate Nix module for this application
        let nix_content = crate::generators::generate_home_manager_module(theme, app)
            .with_context(|| format!(
                "Failed to generate Nix Home Manager module for application '{}'.\n\
                \n\
                Theme: {}\n\
                Application: {}\n\
                \n\
                Possible causes:\n\
                - Application '{}' is not supported for Nix module generation\n\
                - Theme data is invalid or incomplete\n\
                - Generator encountered an internal error\n\
                \n\
                To fix: Check if the application is supported. Supported applications include: \
                kitty, waybar, neovim, starship, mako, hyprland, wofi, wlogout, fastfetch, yazi, \
                hyprpaper, gtk, btop, git",
                app, theme.name, app, app
            ))?;
        let module_path = nix_path.join(format!("{}.nix", app));
        
        if self.dry_run {
            println!("    Would write Nix module to: {:?}", module_path);
            return Ok(());
        }
        
        // Create parent directory if needed
        if let Some(parent) = module_path.parent() {
            fs::create_dir_all(parent)
                .with_context(|| format!(
                    "Failed to create Nix module directory for application '{}' at {:?}.\n\
                    \n\
                    Deployment method: nix\n\
                    Nix output path: {:?}\n\
                    \n\
                    Possible causes:\n\
                    - Insufficient permissions to create directories\n\
                    - Disk is full\n\
                    - Invalid path in nix.output_path configuration\n\
                    \n\
                    To fix: Ensure you have write permissions. Check your nix.output_path setting:\n\
                    themectl config get-nix-path\n\
                    \n\
                    You can set a different path with:\n\
                    themectl config set-nix-path <path>",
                    app, parent, nix_path
                ))?;
        }
        
        // Write Nix module
        fs::write(&module_path, &nix_content)
            .with_context(|| format!(
                "Failed to write Nix Home Manager module for application '{}' to {:?}.\n\
                \n\
                Deployment method: nix\n\
                Theme: {}\n\
                \n\
                Possible causes:\n\
                - Insufficient write permissions\n\
                - Disk is full\n\
                - File is locked by another process\n\
                \n\
                To fix: Check file permissions and ensure sufficient disk space. \
                After writing, you'll need to import this module in your Home Manager configuration.",
                app, module_path, theme.name
            ))?;
        
        Ok(())
    }
    
    pub fn backup_file(&self, path: &Path) -> Result<()> {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        let backup_path = path.with_extension(format!("{}.bak", timestamp));
        fs::copy(path, &backup_path)
            .with_context(|| format!(
                "Failed to create backup of configuration file at {:?}.\n\
                \n\
                Backup path: {:?}\n\
                \n\
                Possible causes:\n\
                - Insufficient permissions to read the original file\n\
                - Insufficient permissions to write the backup file\n\
                - Disk is full\n\
                - Source file is locked by another process\n\
                \n\
                To fix: Check file permissions and ensure sufficient disk space. \
                The backup is created before modifying the original file for safety.",
                path, backup_path
            ))?;
        
        Ok(())
    }
    
    fn detect_config_files(&self, theme: &Theme) -> Result<Vec<(String, PathBuf)>> {
        let deployment_method = self.themectl_config.as_ref()
            .map(|c| c.get_deployment_method())
            .unwrap_or("nix");
        
        match deployment_method {
            "nix" => self.detect_nix_files(theme),
            _ => self.detect_standard_files(theme),
        }
    }
    
    fn detect_standard_files(&self, theme: &Theme) -> Result<Vec<(String, PathBuf)>> {
        let mut configs = Vec::new();
        
        let base_dir = self.config_dir.clone().unwrap_or_else(|| {
            dirs::home_dir()
                .unwrap_or_else(|| PathBuf::from("~"))
                .join(".config")
        });
        
        let apps = vec![
            "kitty", "waybar", "neovim", "starship", "mako", 
            "hyprland", "wofi", "wlogout", "fastfetch", "yazi", "hyprpaper", "gtk", "btop", "git"
        ];
        
        for app in apps {
            // Try to discover config file using enhanced detection
            if let Some(path) = Self::discover_config_file(app, self.themectl_config.as_ref(), theme) {
                if path.exists() || !self.dry_run {
                    configs.push((app.to_string(), path));
                    continue;
                }
            }
            
            // Fallback to standard paths if discovery didn't find anything
            let standard_path = match app {
                "kitty" => base_dir.join("kitty").join("kitty.conf"),
                "waybar" => base_dir.join("waybar").join("style.css"),
                "neovim" => base_dir.join("nvim").join("colors").join(format!("{}.lua", theme.name)),
                "starship" => base_dir.join("starship.toml"),
                "mako" => base_dir.join("mako").join("config"),
                "hyprland" => base_dir.join("hypr").join("hyprland.conf"),
                "wofi" => base_dir.join("wofi").join("style.css"),
                "wlogout" => base_dir.join("wlogout").join("style.css"),
                "fastfetch" => base_dir.join("fastfetch").join("config.jsonc"),
                "yazi" => base_dir.join("yazi").join("yazi.toml"),
                "hyprpaper" => base_dir.join("hypr").join("hyprpaper.conf"),
                "gtk" => base_dir.join("gtk-4.0").join("settings.ini"),
                "btop" => {
                    // Btop themes are typically in ~/.config/btop/themes/ or alongside btop executable
                    // For now, use a standard location
                    if let Some(home) = dirs::home_dir() {
                        home.join(".config").join("btop").join("themes").join(format!("{}.theme", theme.name))
                    } else {
                        base_dir.join("btop").join("themes").join(format!("{}.theme", theme.name))
                    }
                },
                "git" => {
                    // Git config can be included via include.path in ~/.gitconfig
                    // Store in ~/.config/git/themes/
                    if let Some(home) = dirs::home_dir() {
                        home.join(".config").join("git").join("themes").join(format!("{}.conf", theme.name))
                    } else {
                        base_dir.join("git").join("themes").join(format!("{}.conf", theme.name))
                    }
                },
                _ => continue,
            };
            
            if standard_path.exists() || standard_path.parent().map(|p| p.exists()).unwrap_or(false) || !self.dry_run {
                configs.push((app.to_string(), standard_path));
            }
        }
        
        Ok(configs)
    }
    
    fn detect_nix_files(&self, _theme: &Theme) -> Result<Vec<(String, PathBuf)>> {
        let mut configs = Vec::new();
        
        let nix_path = if let Some(config) = &self.themectl_config {
            config.get_nix_output_path()
        } else {
            dirs::home_dir()
                .unwrap_or_else(|| PathBuf::from("~"))
                .join(".config")
                .join("nixpkgs")
                .join("modules")
                .join("themectl")
        };
        
        let apps = vec![
            "kitty", "waybar", "neovim", "starship", "mako", 
            "hyprland", "wofi", "wlogout", "fastfetch", "yazi", "hyprpaper", "gtk", "btop", "git"
        ];
        
        for app in apps {
            let module_path = nix_path.join(format!("{}.nix", app));
            // Always include Nix modules (they'll be created if they don't exist)
            configs.push((app.to_string(), module_path));
        }
        
        Ok(configs)
    }

    /// Detect NixOS-specific paths
    pub fn detect_nixos_paths() -> Vec<PathBuf> {
        let mut paths = Vec::new();
        
        // Check NIXOS environment variable
        if std::env::var("NIXOS").is_ok() {
            // Check /etc/nixos for system config
            if Path::new("/etc/nixos").exists() {
                paths.push(PathBuf::from("/etc/nixos"));
            }
            
            // Check ~/.config/nixpkgs for Home Manager
            if let Some(home) = dirs::home_dir() {
                let nixpkgs = home.join(".config").join("nixpkgs");
                if nixpkgs.exists() {
                    paths.push(nixpkgs);
                }
            }
        }
        
        // Also check /etc/nixos even without env var (some setups don't set it)
        if Path::new("/etc/nixos").exists() {
            paths.push(PathBuf::from("/etc/nixos"));
        }
        
        paths
    }

    /// Find NixOS-specific config path for an application
    pub fn find_nixos_config_path(_app: &str) -> Option<PathBuf> {
        if let Some(home) = dirs::home_dir() {
            let nixpkgs = home.join(".config").join("nixpkgs");
            if nixpkgs.exists() {
                // Home Manager typically stores configs in home.nix or home-manager.nix
                // For now, return the nixpkgs directory
                // Applications would be configured in the Nix file itself
                return Some(nixpkgs);
            }
        }
        None
    }

    /// Discover config file for an application using multiple search strategies
    pub fn discover_config_file(app: &str, config: Option<&ThemectlConfig>, theme: &Theme) -> Option<PathBuf> {
        // 1. Check custom path from config first
        if let Some(cfg) = config {
            if let Some(custom_path) = cfg.get_app_path(app) {
                if custom_path.exists() {
                    return Some(custom_path.clone());
                }
            }
        }
        
        // 2. Check standard locations
        let base_dir = dirs::home_dir()?.join(".config");
        let standard_path = match app {
            "kitty" => Some(base_dir.join("kitty").join("kitty.conf")),
            "waybar" => Some(base_dir.join("waybar").join("style.css")),
            "neovim" => Some(base_dir.join("nvim").join("colors").join(format!("{}.lua", theme.name))),
            "starship" => Some(base_dir.join("starship.toml")),
            "mako" => Some(base_dir.join("mako").join("config")),
            "hyprland" => Some(base_dir.join("hypr").join("hyprland.conf")),
            "wofi" => Some(base_dir.join("wofi").join("style.css")),
            "wlogout" => Some(base_dir.join("wlogout").join("style.css")),
            "fastfetch" => Some(base_dir.join("fastfetch").join("config.jsonc")),
            "yazi" => Some(base_dir.join("yazi").join("yazi.toml")),
            "hyprpaper" => Some(base_dir.join("hypr").join("hyprpaper.conf")),
            "gtk" => Some(base_dir.join("gtk-4.0").join("settings.ini")),
            "btop" => {
                // Btop themes are in ~/.config/btop/themes/
                if let Some(home) = dirs::home_dir() {
                    Some(home.join(".config").join("btop").join("themes").join(format!("{}.theme", theme.name)))
                } else {
                    Some(base_dir.join("btop").join("themes").join(format!("{}.theme", theme.name)))
                }
            },
            "git" => {
                // Git config can be included via include.path in ~/.gitconfig
                if let Some(home) = dirs::home_dir() {
                    Some(home.join(".config").join("git").join("themes").join(format!("{}.conf", theme.name)))
                } else {
                    Some(base_dir.join("git").join("themes").join(format!("{}.conf", theme.name)))
                }
            },
            _ => None,
        };
        
        if let Some(path) = standard_path {
            if path.exists() || path.parent().map(|p| p.exists()).unwrap_or(false) {
                return Some(path);
            }
        }
        
        // 3. Check NixOS paths if enabled
        if let Some(cfg) = config {
            if cfg.nixos_mode {
                if let Some(nix_path) = Self::find_nixos_config_path(app) {
                    return Some(nix_path);
                }
            }
        }
        
        // 4. Check additional search paths from config
        if let Some(cfg) = config {
            for search_path in &cfg.search_paths {
                let potential_path = search_path.join(app);
                if potential_path.exists() {
                    return Some(potential_path);
                }
            }
        }
        
        None
    }
}
