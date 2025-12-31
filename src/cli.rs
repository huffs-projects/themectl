use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use colored::*;
use std::io::{self, Write};
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::config::ThemectlConfig;
use crate::file_manager::FileManager;
use crate::generators;
use crate::parser;
use crate::templates;
use crate::theme::{ColorPalette, Theme, ThemeProperties};
use crate::utils::{hex_to_rgb, normalize_hex, validate_hex_color, generate_variant, calculate_contrast_ratio};

#[derive(Parser)]
#[command(name = "themectl")]
#[command(about = "A unified theming solution for Frog-OS applications")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
    
    #[arg(long, global = true)]
    pub themes_dir: Option<PathBuf>,
    
    #[arg(long, global = true)]
    pub dry_run: bool,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Apply a theme to all applications
    Apply {
        /// Theme name (without .toml extension)
        theme: String,
        #[arg(long)]
        /// Override config directory
        config_dir: Option<PathBuf>,
        #[arg(long)]
        /// Apply to specific applications only (comma-separated)
        apps: Option<String>,
        #[arg(long)]
        /// Apply specific variant (dark/light)
        variant: Option<String>,
    },
    /// List all available themes
    List,
    /// Create a new theme interactively
    Create {
        /// Theme name
        name: String,
    },
    /// Validate a theme file
    Validate {
        /// Path to theme file
        path: PathBuf,
    },
    /// Export theme to a specific format
    Export {
        /// Theme name
        theme: String,
        /// Format (nix, kitty, waybar, neovim, etc.) or "all" for all formats
        format: String,
        #[arg(long)]
        /// Output file path (or directory if --all is used)
        output: Option<PathBuf>,
        #[arg(long)]
        /// Export all formats at once
        all: bool,
    },
    /// Show theme details
    Show {
        /// Theme name
        theme: String,
    },
    /// Edit an existing theme interactively
    Edit {
        /// Theme name (without .toml extension)
        theme: String,
    },
    /// Preview theme with color swatches and generated configs
    Preview {
        /// Theme name (without .toml extension)
        theme: String,
        #[arg(long)]
        /// Show specific format only
        format: Option<String>,
    },
    /// Delete a theme
    Delete {
        /// Theme name (without .toml extension)
        theme: String,
    },
    /// Rename a theme
    Rename {
        /// Old theme name (without .toml extension)
        old: String,
        /// New theme name (without .toml extension)
        new: String,
    },
    /// Duplicate a theme
    Duplicate {
        /// Source theme name (without .toml extension)
        theme: String,
        /// New theme name (without .toml extension)
        new: String,
    },
    /// Search themes by name or description
    Search {
        /// Search query
        query: String,
    },
    /// Manage backup files
    Backups {
        #[command(subcommand)]
        command: BackupCommands,
    },
    /// Initialize theme directory structure
    Init,
    /// Manage theme variants
    Variant {
        #[command(subcommand)]
        command: VariantCommands,
    },
    /// Manage themectl configuration
    Config {
        #[command(subcommand)]
        command: ConfigCommands,
    },
    /// Export all themes
    ExportAll {
        /// Format (nix, kitty, waybar, etc.) or "all" for all formats
        format: String,
        /// Output directory
        output_dir: PathBuf,
    },
    /// Validate all themes
    ValidateAll,
}

#[derive(Subcommand)]
pub enum VariantCommands {
    /// Create a variant of a theme
    Create {
        /// Theme name (without .toml extension)
        theme: String,
        /// Variant name (dark/light)
        variant: String,
        #[arg(long)]
        /// Auto-generate variant instead of prompting
        auto: bool,
    },
    /// Switch to a variant
    Switch {
        /// Theme name (without .toml extension)
        theme: String,
        /// Variant name (dark/light)
        variant: String,
    },
    /// List available variants for a theme
    List {
        /// Theme name (without .toml extension)
        theme: String,
    },
}

#[derive(Subcommand)]
pub enum ConfigCommands {
    /// Set custom path for an application
    SetPath {
        /// Application name
        app: String,
        /// Config file path
        path: PathBuf,
    },
    /// Get current path for an application
    GetPath {
        /// Application name
        app: String,
    },
    /// Enable NixOS mode
    EnableNixos,
    /// Disable NixOS mode
    DisableNixos,
    /// Create a config file template
    CreateTemplate {
        /// Application name
        app: String,
        #[arg(long)]
        /// Output path (default: standard location)
        path: Option<PathBuf>,
    },
    /// Set deployment method (standard or nix)
    SetDeployment {
        /// Deployment method: standard or nix
        method: String,
    },
    /// Get current deployment method
    GetDeployment,
    /// Set Nix output path
    SetNixPath {
        /// Path where Nix modules should be written
        path: PathBuf,
    },
}

#[derive(Subcommand)]
pub enum BackupCommands {
    /// List all backup files
    List {
        #[arg(long)]
        /// Show backups for specific config directory
        config_dir: Option<PathBuf>,
    },
    /// Restore a backup file
    Restore {
        /// Path to backup file
        backup: PathBuf,
    },
    /// Clean old backup files
    Clean {
        #[arg(long)]
        /// Keep backups newer than this many days (default: 30)
        days: Option<u64>,
        #[arg(long)]
        /// Config directory to clean backups from
        config_dir: Option<PathBuf>,
    },
}

impl Cli {
    pub fn execute(&self) -> Result<()> {
        let themes_dir = self.themes_dir.clone().unwrap_or_else(|| {
            PathBuf::from("themes")
        });
        
        match &self.command {
            Commands::Apply { theme, config_dir, apps, variant } => {
                self.apply_theme(theme, &themes_dir, config_dir.as_ref(), apps.as_ref(), variant.as_ref())?;
            }
            Commands::List => {
                self.list_themes(&themes_dir)?;
            }
            Commands::Create { name } => {
                self.create_theme(name, &themes_dir)?;
            }
            Commands::Validate { path } => {
                self.validate_theme(path)?;
            }
            Commands::Export { theme, format, output, all } => {
                if *all || format == "all" {
                    self.export_all_formats(theme, output, &themes_dir)?;
                } else {
                    self.export_theme(theme, format, output, &themes_dir)?;
                }
            }
            Commands::Show { theme } => {
                self.show_theme(theme, &themes_dir)?;
            }
            Commands::Edit { theme } => {
                self.edit_theme(theme, &themes_dir)?;
            }
            Commands::Preview { theme, format } => {
                self.preview_theme(theme, format, &themes_dir)?;
            }
            Commands::Delete { theme } => {
                self.delete_theme(theme, &themes_dir)?;
            }
            Commands::Rename { old, new } => {
                self.rename_theme(old, new, &themes_dir)?;
            }
            Commands::Duplicate { theme, new } => {
                self.duplicate_theme(theme, new, &themes_dir)?;
            }
            Commands::Search { query } => {
                self.search_themes(&query, &themes_dir)?;
            }
            Commands::Backups { command } => {
                match command {
                    BackupCommands::List { config_dir } => {
                        self.list_backups(config_dir.as_ref())?;
                    }
                    BackupCommands::Restore { backup } => {
                        self.restore_backup(backup.clone())?;
                    }
                    BackupCommands::Clean { days, config_dir } => {
                        self.clean_backups(*days, config_dir.as_ref())?;
                    }
                }
            }
            Commands::Init => {
                self.init_themes_dir(&themes_dir)?;
            }
            Commands::Variant { command } => {
                match command {
                    VariantCommands::Create { theme, variant, auto } => {
                        self.create_variant(theme, variant, *auto, &themes_dir)?;
                    }
                    VariantCommands::Switch { theme, variant } => {
                        self.switch_variant(theme, variant, &themes_dir)?;
                    }
                    VariantCommands::List { theme } => {
                        self.list_variants(theme, &themes_dir)?;
                    }
                }
            }
            Commands::Config { command } => {
                match command {
                    ConfigCommands::SetPath { app, path } => {
                        self.config_set_path(app, path)?;
                    }
                    ConfigCommands::GetPath { app } => {
                        self.config_get_path(app)?;
                    }
                    ConfigCommands::EnableNixos => {
                        self.config_enable_nixos()?;
                    }
                    ConfigCommands::DisableNixos => {
                        self.config_disable_nixos()?;
                    }
                    ConfigCommands::CreateTemplate { app, path } => {
                        self.config_create_template(app, path.as_ref())?;
                    }
                    ConfigCommands::SetDeployment { method } => {
                        self.config_set_deployment(&method)?;
                    }
                    ConfigCommands::GetDeployment => {
                        self.config_get_deployment()?;
                    }
                    ConfigCommands::SetNixPath { path } => {
                        self.config_set_nix_path(path.clone())?;
                    }
                }
            }
            Commands::ExportAll { format, output_dir } => {
                self.export_all_themes(format, output_dir, &themes_dir)?;
            }
            Commands::ValidateAll => {
                self.validate_all_themes(&themes_dir)?;
            }
        }
        
        Ok(())
    }
    
    fn apply_theme(&self, theme_name: &str, themes_dir: &PathBuf, config_dir: Option<&PathBuf>, apps: Option<&String>, variant: Option<&String>) -> Result<()> {
        // Handle variant selection
        let theme_file_name = if let Some(v) = variant {
            format!("{}-{}.toml", theme_name, v)
        } else {
            format!("{}.toml", theme_name)
        };
        
        let theme_path = themes_dir.join(&theme_file_name);
        if !theme_path.exists() {
            // Try base theme name if variant not found
            let base_path = themes_dir.join(format!("{}.toml", theme_name));
            if base_path.exists() {
                let mut theme = parser::parse_theme_file(&base_path)?;
                if let Some(v) = variant {
                    theme = generate_variant(&theme, v)?;
                }
                println!("{} Applying theme: {}", "✓".green(), theme.name.bold());
                
                let file_manager = FileManager::new(config_dir, self.dry_run);
                if let Some(apps_str) = apps {
                    let app_list: Vec<&str> = apps_str.split(',').map(|s| s.trim()).collect();
                    file_manager.apply_theme_filtered(&theme, &app_list)?;
                } else {
                    file_manager.apply_theme(&theme)?;
                }
                return Ok(());
            }
            anyhow::bail!(
                "Theme '{}' not found at {:?}.\n\
                \n\
                Expected theme file: {:?}\n\
                \n\
                Possible causes:\n\
                - Theme name is misspelled\n\
                - Theme file doesn't exist at this location\n\
                - Wrong themes directory specified\n\
                \n\
                To fix:\n\
                1. List available themes: themectl list\n\
                2. Check if the theme exists: ls {:?}\n\
                3. Verify the themes directory is correct",
                theme_file_name, theme_path, theme_path, theme_path.parent().map(|p| p.as_os_str()).unwrap_or_else(|| theme_path.as_os_str())
            );
        }
        
        let theme = parser::parse_theme_file(&theme_path)?;
        println!("{} Applying theme: {}", "✓".green(), theme.name.bold());
        
        let file_manager = FileManager::new(config_dir, self.dry_run);
        if let Some(apps_str) = apps {
            let app_list: Vec<&str> = apps_str.split(',').map(|s| s.trim()).collect();
            file_manager.apply_theme_filtered(&theme, &app_list)?;
        } else {
            file_manager.apply_theme(&theme)?;
        }
        
        Ok(())
    }
    
    fn list_themes(&self, themes_dir: &PathBuf) -> Result<()> {
        let themes = parser::find_theme_files(themes_dir)?;
        
        if themes.is_empty() {
            println!("No themes found in {:?}", themes_dir);
            return Ok(());
        }
        
        println!("{} Available themes:", "📋".cyan());
        for theme_path in themes {
            if let Some(name) = theme_path.file_stem() {
                if let Some(name_str) = name.to_str() {
                    match parser::parse_theme_file(&theme_path) {
                        Ok(theme) => {
                            println!("  {} {} - {}", 
                                "•".green(), 
                                name_str.bold(), 
                                theme.description
                            );
                        }
                        Err(e) => {
                            println!("  {} {} - {}", 
                                "✗".red(), 
                                name_str.bold(), 
                                format!("Error: {}", e)
                            );
                        }
                    }
                }
            }
        }
        
        Ok(())
    }
    
    fn create_theme(&self, name: &str, themes_dir: &PathBuf) -> Result<()> {
        println!("{} Creating new theme: {}", "→".cyan(), name.bold());
        println!();
        
        // Check if theme already exists
        let theme_path = themes_dir.join(format!("{}.toml", name));
        if theme_path.exists() {
            print!("{} Theme '{}' already exists. Overwrite? (y/N): ", "⚠".yellow(), name);
            io::stdout().flush()?;
            let mut response = String::new();
            io::stdin().read_line(&mut response)?;
            if response.trim().to_lowercase() != "y" {
                println!("{} Cancelled.", "✗".red());
                return Ok(());
            }
        }
        
        // Prompt for description
        let description = self.prompt_input("Description", "")?;
        
        println!("\n{} Enter colors (hex format, e.g., #282828):", "🎨".cyan());
        println!();
        
        // Required colors
        let bg = self.prompt_color("Background (bg)", None)?;
        let fg = self.prompt_color("Foreground (fg)", None)?;
        let accent = self.prompt_color("Accent", None)?;
        let red = self.prompt_color("Red", None)?;
        let green = self.prompt_color("Green", None)?;
        let yellow = self.prompt_color("Yellow", None)?;
        let blue = self.prompt_color("Blue", None)?;
        let magenta = self.prompt_color("Magenta", None)?;
        let cyan = self.prompt_color("Cyan", None)?;
        
        // Optional colors
        println!("\n{} Optional colors (press Enter to skip):", "→".cyan());
        let orange = self.prompt_color_optional("Orange")?;
        let purple = self.prompt_color_optional("Purple")?;
        let pink = self.prompt_color_optional("Pink")?;
        let white = self.prompt_color_optional("White")?;
        let black = self.prompt_color_optional("Black")?;
        let gray = self.prompt_color_optional("Gray")?;
        
        // Create theme
        let theme = Theme {
            name: name.to_string(),
            description,
            variant: None,
            colors: ColorPalette {
                bg,
                fg,
                accent,
                red,
                green,
                yellow,
                blue,
                magenta,
                cyan,
                orange,
                purple,
                pink,
                white,
                black,
                gray,
            },
            properties: ThemeProperties::default(),
        };
        
        // Validate theme
        println!("\n{} Validating theme...", "→".cyan());
        parser::validate_theme(&theme)?;
        println!("{} Theme is valid!", "✓".green());
        
        // Save theme
        std::fs::create_dir_all(themes_dir)?;
        let toml_content = toml::to_string(&theme)
            .context("Failed to serialize theme to TOML")?;
        
        std::fs::write(&theme_path, toml_content)?;
        println!("{} Theme saved to: {:?}", "✓".green(), theme_path);
        
        Ok(())
    }
    
    fn prompt_input(&self, prompt: &str, default: &str) -> Result<String> {
        let default_text = if default.is_empty() {
            String::new()
        } else {
            format!(" [{}]", default.cyan())
        };
        print!("{} {}{}: ", "?".cyan(), prompt.bold(), default_text);
        io::stdout().flush()?;
        
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim().to_string();
        
        if input.is_empty() && !default.is_empty() {
            Ok(default.to_string())
        } else {
            Ok(input)
        }
    }
    
    fn prompt_color(&self, prompt: &str, default: Option<&str>) -> Result<String> {
        loop {
            let default_text = if let Some(d) = default {
                format!(" [{}]", d.cyan())
            } else {
                String::new()
            };
            print!("{} {}{}: ", "?".cyan(), prompt.bold(), default_text);
            io::stdout().flush()?;
            
            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
            let input = input.trim();
            
            if input.is_empty() {
                if let Some(d) = default {
                    return Ok(normalize_hex(d));
                } else {
                    println!("{} Color is required!", "✗".red());
                    continue;
                }
            }
            
            let normalized = normalize_hex(input);
            if validate_hex_color(&normalized) {
                return Ok(normalized);
            } else {
                println!("{} Invalid color format. Please use hex format (e.g., #282828)", "✗".red());
            }
        }
    }
    
    fn prompt_color_optional(&self, prompt: &str) -> Result<Option<String>> {
        let default_text = format!(" [{}]", "skip".dimmed());
        print!("{} {}{}: ", "?".cyan(), prompt.bold(), default_text);
        io::stdout().flush()?;
        
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim();
        
        if input.is_empty() {
            return Ok(None);
        }
        
        let normalized = normalize_hex(input);
        if validate_hex_color(&normalized) {
            Ok(Some(normalized))
        } else {
            println!("{} Invalid color format. Skipping.", "✗".red());
            Ok(None)
        }
    }
    
    fn validate_theme(&self, path: &PathBuf) -> Result<()> {
        match parser::parse_theme_file(path) {
            Ok(theme) => {
                println!("{} Theme is valid: {}", "✓".green(), theme.name.bold());
                println!("  Description: {}", theme.description);
                println!("  Colors: bg, fg, accent, red, green, yellow, blue, magenta, cyan");
                Ok(())
            }
            Err(e) => {
                eprintln!("{} Theme validation failed: {}", "✗".red(), e);
                Err(e)
            }
        }
    }
    
    fn export_theme(&self, theme_name: &str, format: &str, output: &Option<PathBuf>, themes_dir: &PathBuf) -> Result<()> {
        let theme_path = themes_dir.join(format!("{}.toml", theme_name));
        let theme = parser::parse_theme_file(&theme_path)?;
        
        let content = generators::generate(&theme, format)?;
        
        if let Some(output_path) = output {
            std::fs::write(output_path, content)?;
            println!("{} Exported to {:?}", "✓".green(), output_path);
        } else {
            print!("{}", content);
        }
        
        Ok(())
    }
    
    fn export_all_formats(&self, theme_name: &str, output: &Option<PathBuf>, themes_dir: &PathBuf) -> Result<()> {
        let theme_path = themes_dir.join(format!("{}.toml", theme_name));
        let theme = parser::parse_theme_file(&theme_path)?;
        
        println!("{} Exporting all formats for theme: {}", "→".cyan(), theme.name.bold());
        
        // Use generate_all() function
        let results = generators::generate_all(&theme)?;
        
        if let Some(output_dir) = output {
            // Export to directory
            std::fs::create_dir_all(output_dir)?;
            for (format, content) in results {
                let file_path = output_dir.join(format!("{}.{}", theme_name, format));
                std::fs::write(&file_path, content)?;
                println!("{} Exported {} to {:?}", "✓".green(), format.bold(), file_path);
            }
        } else {
            // Print all formats to stdout
            for (format, content) in results {
                println!("\n{} Format: {}", "─".cyan(), format.bold());
                println!("{}", content);
            }
        }
        
        Ok(())
    }
    
    fn show_theme(&self, theme_name: &str, themes_dir: &PathBuf) -> Result<()> {
        let theme_path = themes_dir.join(format!("{}.toml", theme_name));
        let theme = parser::parse_theme_file(&theme_path)?;
        
        println!("{} Theme: {}", "📋".cyan(), theme.name.bold());
        println!("  Description: {}", theme.description);
        println!("\n{} Colors:", "🎨".cyan());
        
        // Use get_color() for all colors
        let color_names = vec![
            "bg", "fg", "accent", "red", "green", "yellow", 
            "blue", "magenta", "cyan", "orange", "purple", 
            "pink", "white", "black", "gray"
        ];
        
        for color_name in color_names {
            if let Some(color) = theme.get_color(color_name) {
                println!("  {}: {}", color_name, color);
            }
        }
        
        Ok(())
    }
    
    fn init_themes_dir(&self, themes_dir: &PathBuf) -> Result<()> {
        std::fs::create_dir_all(themes_dir)?;
        println!("{} Initialized themes directory: {:?}", "✓".green(), themes_dir);
        Ok(())
    }
    
    fn display_color_swatch(&self, name: &str, color: &str) -> String {
        if let Some((r, g, b)) = hex_to_rgb(color) {
            // ANSI true color escape sequence for background
            let bg_escape = format!("\x1b[48;2;{};{};{}m", r, g, b);
            let reset = "\x1b[0m";
            format!("{}████{} {}: {}", bg_escape, reset, name.bold(), color)
        } else {
            format!("████ {}: {}", name.bold(), color)
        }
    }
    
    fn prompt_property(&self, prompt: &str, default: Option<u32>) -> Result<Option<u32>> {
        let default_text = if let Some(d) = default {
            format!(" [{}]", d.to_string().cyan())
        } else {
            format!(" [{}]", "none".dimmed())
        };
        print!("{} {}{}: ", "?".cyan(), prompt.bold(), default_text);
        io::stdout().flush()?;
        
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim();
        
        if input.is_empty() {
            return Ok(default);
        }
        
        if input.to_lowercase() == "none" || input.to_lowercase() == "clear" {
            return Ok(None);
        }
        
        match input.parse::<u32>() {
            Ok(value) => Ok(Some(value)),
            Err(_) => {
                println!("{} Invalid number. Using default.", "✗".red());
                Ok(default)
            }
        }
    }
    
    fn edit_theme(&self, theme_name: &str, themes_dir: &PathBuf) -> Result<()> {
        let theme_path = themes_dir.join(format!("{}.toml", theme_name));
        if !theme_path.exists() {
            anyhow::bail!(
                "Theme '{}' not found at {:?}.\n\
                \n\
                Expected theme file: {:?}\n\
                \n\
                Possible causes:\n\
                - Theme name is misspelled\n\
                - Theme file doesn't exist\n\
                - Wrong themes directory\n\
                \n\
                To fix:\n\
                1. List available themes: themectl list\n\
                2. Check if the theme exists: ls {:?}\n\
                3. Create the theme if it doesn't exist: themectl create {}",
                theme_name, theme_path, theme_path, theme_path.parent().map(|p| p.as_os_str()).unwrap_or_else(|| theme_path.as_os_str()), theme_name
            );
        }
        
        let mut theme = parser::parse_theme_file(&theme_path)?;
        println!("{} Editing theme: {}", "→".cyan(), theme.name.bold());
        println!();
        
        // Edit name
        let new_name = self.prompt_input("Theme name", &theme.name)?;
        if new_name != theme.name {
            // Check if new name already exists
            let new_theme_path = themes_dir.join(format!("{}.toml", new_name));
            if new_theme_path.exists() && new_theme_path != theme_path {
                anyhow::bail!("Theme '{}' already exists", new_name);
            }
            theme.name = new_name;
        }
        
        // Edit description
        theme.description = self.prompt_input("Description", &theme.description)?;
        
        println!("\n{} Enter colors (hex format, e.g., #282828, press Enter to keep current):", "🎨".cyan());
        println!();
        
        // Edit required colors
        theme.colors.bg = self.prompt_color("Background (bg)", Some(&theme.colors.bg))?;
        theme.colors.fg = self.prompt_color("Foreground (fg)", Some(&theme.colors.fg))?;
        theme.colors.accent = self.prompt_color("Accent", Some(&theme.colors.accent))?;
        theme.colors.red = self.prompt_color("Red", Some(&theme.colors.red))?;
        theme.colors.green = self.prompt_color("Green", Some(&theme.colors.green))?;
        theme.colors.yellow = self.prompt_color("Yellow", Some(&theme.colors.yellow))?;
        theme.colors.blue = self.prompt_color("Blue", Some(&theme.colors.blue))?;
        theme.colors.magenta = self.prompt_color("Magenta", Some(&theme.colors.magenta))?;
        theme.colors.cyan = self.prompt_color("Cyan", Some(&theme.colors.cyan))?;
        
        // Edit optional colors
        println!("\n{} Optional colors (press Enter to keep current, 'none' to clear):", "→".cyan());
        if let Some(current) = &theme.colors.orange {
            let new = self.prompt_color_optional_with_default("Orange", Some(current))?;
            theme.colors.orange = new;
        } else {
            theme.colors.orange = self.prompt_color_optional("Orange")?;
        }
        
        if let Some(current) = &theme.colors.purple {
            let new = self.prompt_color_optional_with_default("Purple", Some(current))?;
            theme.colors.purple = new;
        } else {
            theme.colors.purple = self.prompt_color_optional("Purple")?;
        }
        
        if let Some(current) = &theme.colors.pink {
            let new = self.prompt_color_optional_with_default("Pink", Some(current))?;
            theme.colors.pink = new;
        } else {
            theme.colors.pink = self.prompt_color_optional("Pink")?;
        }
        
        if let Some(current) = &theme.colors.white {
            let new = self.prompt_color_optional_with_default("White", Some(current))?;
            theme.colors.white = new;
        } else {
            theme.colors.white = self.prompt_color_optional("White")?;
        }
        
        if let Some(current) = &theme.colors.black {
            let new = self.prompt_color_optional_with_default("Black", Some(current))?;
            theme.colors.black = new;
        } else {
            theme.colors.black = self.prompt_color_optional("Black")?;
        }
        
        if let Some(current) = &theme.colors.gray {
            let new = self.prompt_color_optional_with_default("Gray", Some(current))?;
            theme.colors.gray = new;
        } else {
            theme.colors.gray = self.prompt_color_optional("Gray")?;
        }
        
        // Edit properties
        println!("\n{} Theme properties (press Enter to keep current, 'none' to clear):", "→".cyan());
        theme.properties.border_radius = self.prompt_property("Border radius", theme.properties.border_radius)?;
        theme.properties.border_width = self.prompt_property("Border width", theme.properties.border_width)?;
        theme.properties.shadow_blur = self.prompt_property("Shadow blur", theme.properties.shadow_blur)?;
        
        if let Some(current) = theme.properties.animation_duration {
            let default_text = format!(" [{}]", current.to_string().cyan());
            print!("{} {}{}: ", "?".cyan(), "Animation duration".bold(), default_text);
            io::stdout().flush()?;
            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
            let input = input.trim();
            if input.is_empty() {
                // Keep current
            } else if input.to_lowercase() == "none" || input.to_lowercase() == "clear" {
                theme.properties.animation_duration = None;
            } else {
                match input.parse::<f32>() {
                    Ok(value) => theme.properties.animation_duration = Some(value),
                    Err(_) => {
                        println!("{} Invalid number. Keeping current value.", "✗".red());
                    }
                }
            }
        } else {
            let default_text = format!(" [{}]", "none".dimmed());
            print!("{} {}{}: ", "?".cyan(), "Animation duration".bold(), default_text);
            io::stdout().flush()?;
            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
            let input = input.trim();
            if !input.is_empty() && input.to_lowercase() != "none" {
                if let Ok(value) = input.parse::<f32>() {
                    theme.properties.animation_duration = Some(value);
                }
            }
        }
        
        theme.properties.spacing = self.prompt_property("Spacing", theme.properties.spacing)?;
        
        // Validate theme
        println!("\n{} Validating theme...", "→".cyan());
        parser::validate_theme(&theme)?;
        println!("{} Theme is valid!", "✓".green());
        
        // Determine save path (may have changed if name changed)
        let save_path = if theme.name != theme_name {
            themes_dir.join(format!("{}.toml", theme.name))
        } else {
            theme_path.clone()
        };
        
        // Save theme
        let toml_content = toml::to_string(&theme)
            .context("Failed to serialize theme to TOML")?;
        
        std::fs::write(&save_path, toml_content)?;
        println!("{} Theme saved to: {:?}", "✓".green(), save_path);
        
        // If name changed, remove old file
        if theme.name != theme_name {
            std::fs::remove_file(&theme_path)?;
            println!("{} Removed old theme file: {:?}", "✓".green(), theme_path);
        }
        
        Ok(())
    }
    
    fn prompt_color_optional_with_default(&self, prompt: &str, default: Option<&str>) -> Result<Option<String>> {
        let default_text = if let Some(d) = default {
            format!(" [{}]", d.cyan())
        } else {
            format!(" [{}]", "skip".dimmed())
        };
        print!("{} {}{}: ", "?".cyan(), prompt.bold(), default_text);
        io::stdout().flush()?;
        
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim();
        
        if input.is_empty() {
            return Ok(default.map(|s| s.to_string()));
        }
        
        if input.to_lowercase() == "none" || input.to_lowercase() == "clear" {
            return Ok(None);
        }
        
        let normalized = normalize_hex(input);
        if validate_hex_color(&normalized) {
            Ok(Some(normalized))
        } else {
            println!("{} Invalid color format. Keeping current value.", "✗".red());
            Ok(default.map(|s| s.to_string()))
        }
    }
    
    fn preview_theme(&self, theme_name: &str, format: &Option<String>, themes_dir: &PathBuf) -> Result<()> {
        let theme_path = themes_dir.join(format!("{}.toml", theme_name));
        if !theme_path.exists() {
            anyhow::bail!(
                "Theme '{}' not found at {:?}.\n\
                \n\
                Expected theme file: {:?}\n\
                \n\
                Possible causes:\n\
                - Theme name is misspelled\n\
                - Theme file doesn't exist\n\
                - Wrong themes directory\n\
                \n\
                To fix:\n\
                1. List available themes: themectl list\n\
                2. Check if the theme exists: ls {:?}\n\
                3. Create the theme if it doesn't exist: themectl create {}",
                theme_name, theme_path, theme_path, theme_path.parent().map(|p| p.as_os_str()).unwrap_or_else(|| theme_path.as_os_str()), theme_name
            );
        }
        
        let theme = parser::parse_theme_file(&theme_path)?;
        
        println!("{} Theme Preview: {}", "📋".cyan(), theme.name.bold());
        println!("  Description: {}", theme.description);
        
        // Display color swatches
        println!("\n{} Color Palette:", "🎨".cyan());
        println!();
        
        // Required colors
        println!("  {} Required Colors:", "→".cyan());
        println!("    {}", self.display_color_swatch("bg", &theme.colors.bg));
        println!("    {}", self.display_color_swatch("fg", &theme.colors.fg));
        println!("    {}", self.display_color_swatch("accent", &theme.colors.accent));
        println!("    {}", self.display_color_swatch("red", &theme.colors.red));
        println!("    {}", self.display_color_swatch("green", &theme.colors.green));
        println!("    {}", self.display_color_swatch("yellow", &theme.colors.yellow));
        println!("    {}", self.display_color_swatch("blue", &theme.colors.blue));
        println!("    {}", self.display_color_swatch("magenta", &theme.colors.magenta));
        println!("    {}", self.display_color_swatch("cyan", &theme.colors.cyan));
        
        // Optional colors
        let optional_colors = vec![
            ("orange", &theme.colors.orange),
            ("purple", &theme.colors.purple),
            ("pink", &theme.colors.pink),
            ("white", &theme.colors.white),
            ("black", &theme.colors.black),
            ("gray", &theme.colors.gray),
        ];
        
        let has_optional = optional_colors.iter().any(|(_, color)| color.is_some());
        if has_optional {
            println!("\n  {} Optional Colors:", "→".cyan());
            for (name, color_opt) in optional_colors {
                if let Some(color) = color_opt {
                    println!("    {}", self.display_color_swatch(name, color));
                }
            }
        }
        
        // Display properties
        let has_properties = theme.properties.border_radius.is_some()
            || theme.properties.border_width.is_some()
            || theme.properties.shadow_blur.is_some()
            || theme.properties.animation_duration.is_some()
            || theme.properties.spacing.is_some();
        
        if has_properties {
            println!("\n  {} Properties:", "→".cyan());
            if let Some(v) = theme.properties.border_radius {
                println!("    border_radius: {}", v);
            }
            if let Some(v) = theme.properties.border_width {
                println!("    border_width: {}", v);
            }
            if let Some(v) = theme.properties.shadow_blur {
                println!("    shadow_blur: {}", v);
            }
            if let Some(v) = theme.properties.animation_duration {
                println!("    animation_duration: {}", v);
            }
            if let Some(v) = theme.properties.spacing {
                println!("    spacing: {}", v);
            }
        }

        // Display accessibility information
        println!("\n{} Accessibility:", "♿".cyan());
        let warnings = parser::validate_accessibility(&theme);
        
        if let Some(ratio) = calculate_contrast_ratio(&theme.colors.bg, &theme.colors.fg) {
            let status = if ratio >= 7.0 {
                "AAA".green()
            } else if ratio >= 4.5 {
                "AA".yellow()
            } else {
                "FAIL".red()
            };
            println!("  Contrast ratio (bg/fg): {:.2}:1 ({})", ratio, status);
        }

        if warnings.is_empty() {
            println!("  {} No accessibility issues found", "✓".green());
        } else {
            for warning in &warnings {
                let symbol = match warning.level {
                    parser::ValidationLevel::Error => "✗".red(),
                    parser::ValidationLevel::Warning => "⚠".yellow(),
                    parser::ValidationLevel::Info => "ℹ".cyan(),
                };
                println!("  {} {}", symbol, warning.message);
            }
        }
        
        // Display generated configs
        println!("\n{} Generated Configurations:", "→".cyan());
        println!();
        
        if let Some(format) = format {
            // Show specific format
            match generators::generate(&theme, format) {
                Ok(content) => {
                    println!("{} Format: {}", "─".cyan(), format.bold());
                    println!("{}", content);
                }
                Err(e) => {
                    eprintln!("{} Failed to generate {}: {}", "✗".red(), format, e);
                }
            }
        } else {
            // Show all formats
            let results = generators::generate_all(&theme)?;
            for (format, content) in results {
                println!("{} Format: {}", "─".cyan(), format.bold());
                println!("{}", content);
                println!();
            }
        }
        
        Ok(())
    }
    
    fn delete_theme(&self, theme_name: &str, themes_dir: &PathBuf) -> Result<()> {
        let theme_path = themes_dir.join(format!("{}.toml", theme_name));
        if !theme_path.exists() {
            anyhow::bail!(
                "Theme '{}' not found at {:?}.\n\
                \n\
                Expected theme file: {:?}\n\
                \n\
                Possible causes:\n\
                - Theme name is misspelled\n\
                - Theme file doesn't exist\n\
                - Wrong themes directory\n\
                \n\
                To fix:\n\
                1. List available themes: themectl list\n\
                2. Check if the theme exists: ls {:?}\n\
                3. Create the theme if it doesn't exist: themectl create {}",
                theme_name, theme_path, theme_path, theme_path.parent().map(|p| p.as_os_str()).unwrap_or_else(|| theme_path.as_os_str()), theme_name
            );
        }
        
        // Confirm deletion
        print!("{} Are you sure you want to delete theme '{}'? (y/N): ", "⚠".yellow(), theme_name.bold());
        io::stdout().flush()?;
        let mut response = String::new();
        io::stdin().read_line(&mut response)?;
        if response.trim().to_lowercase() != "y" {
            println!("{} Cancelled.", "✗".red());
            return Ok(());
        }
        
        std::fs::remove_file(&theme_path)
            .with_context(|| format!("Failed to delete theme: {:?}", theme_path))?;
        println!("{} Theme '{}' deleted successfully.", "✓".green(), theme_name.bold());
        
        Ok(())
    }
    
    fn rename_theme(&self, old_name: &str, new_name: &str, themes_dir: &PathBuf) -> Result<()> {
        let old_path = themes_dir.join(format!("{}.toml", old_name));
        let new_path = themes_dir.join(format!("{}.toml", new_name));
        
        if !old_path.exists() {
            anyhow::bail!("Theme '{}' not found at {:?}", old_name, old_path);
        }
        
        if new_path.exists() {
            anyhow::bail!(
                "Cannot rename theme: '{}' already exists at {:?}.\n\
                \n\
                You tried to rename '{}' to '{}', but a theme with the name '{}' already exists.\n\
                \n\
                To fix:\n\
                1. Delete the existing theme first: themectl delete {}\n\
                2. Or choose a different name for the rename operation\n\
                3. Or use 'duplicate' instead: themectl duplicate {} <new-name>",
                new_name, new_path, old_name, new_name, new_name, new_name, old_name
            );
        }
        
        // Load and update theme name
        let mut theme = parser::parse_theme_file(&old_path)?;
        theme.name = new_name.to_string();
        
        // Validate updated theme
        parser::validate_theme(&theme)?;
        
        // Save with new name
        let toml_content = toml::to_string(&theme)
            .context("Failed to serialize theme to TOML")?;
        std::fs::write(&new_path, toml_content)?;
        
        // Remove old file
        std::fs::remove_file(&old_path)?;
        
        println!("{} Theme '{}' renamed to '{}'.", "✓".green(), old_name.bold(), new_name.bold());
        
        Ok(())
    }
    
    fn duplicate_theme(&self, theme_name: &str, new_name: &str, themes_dir: &PathBuf) -> Result<()> {
        let source_path = themes_dir.join(format!("{}.toml", theme_name));
        let dest_path = themes_dir.join(format!("{}.toml", new_name));
        
        if !source_path.exists() {
            anyhow::bail!("Theme '{}' not found at {:?}", theme_name, source_path);
        }
        
        if dest_path.exists() {
            anyhow::bail!("Theme '{}' already exists at {:?}", new_name, dest_path);
        }
        
        // Load and update theme
        let mut theme = parser::parse_theme_file(&source_path)?;
        theme.name = new_name.to_string();
        
        // Validate updated theme
        parser::validate_theme(&theme)?;
        
        // Save as new theme
        let toml_content = toml::to_string(&theme)
            .context("Failed to serialize theme to TOML")?;
        std::fs::write(&dest_path, toml_content)?;
        
        println!("{} Theme '{}' duplicated as '{}'.", "✓".green(), theme_name.bold(), new_name.bold());
        
        Ok(())
    }
    
    fn search_themes(&self, query: &str, themes_dir: &PathBuf) -> Result<()> {
        let themes = parser::find_theme_files(themes_dir)?;
        let query_lower = query.to_lowercase();
        
        let mut matches = Vec::new();
        
        for theme_path in themes {
            if let Some(name) = theme_path.file_stem() {
                if let Some(name_str) = name.to_str() {
                    match parser::parse_theme_file(&theme_path) {
                        Ok(theme) => {
                            let name_matches = name_str.to_lowercase().contains(&query_lower);
                            let desc_matches = theme.description.to_lowercase().contains(&query_lower);
                            
                            if name_matches || desc_matches {
                                matches.push((name_str.to_string(), theme));
                            }
                        }
                        Err(_) => continue,
                    }
                }
            }
        }
        
        if matches.is_empty() {
            println!("{} No themes found matching '{}'", "✗".red(), query.bold());
            return Ok(());
        }
        
        println!("{} Found {} theme(s) matching '{}':", "✓".green(), matches.len(), query.bold());
        for (name, theme) in matches {
            println!("  {} {} - {}", "•".green(), name.bold(), theme.description);
        }
        
        Ok(())
    }
    
    fn list_backups(&self, config_dir: Option<&PathBuf>) -> Result<()> {
        let base_dir = config_dir.cloned().unwrap_or_else(|| {
            dirs::home_dir()
                .unwrap_or_else(|| PathBuf::from("~"))
                .join(".config")
        });
        
        let mut backups = Vec::new();
        
        // Walk through config directory to find all .bak files
        if base_dir.exists() {
            for entry in walkdir::WalkDir::new(&base_dir) {
                let entry = entry?;
                if entry.file_type().is_file() {
                    let path = entry.path();
                    if let Some(ext) = path.extension() {
                        if ext == "bak" {
                            if let Some(file_name) = path.file_name() {
                                if let Some(name_str) = file_name.to_str() {
                                    // Parse timestamp from filename (format: original.{timestamp}.bak)
                                    if let Some(timestamp) = self.parse_backup_timestamp(name_str) {
                                        let metadata = std::fs::metadata(path)?;
                                        let modified = metadata.modified()?;
                                        backups.push((path.to_path_buf(), timestamp, modified));
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        
        if backups.is_empty() {
            println!("{} No backup files found in {:?}", "→".cyan(), base_dir);
            return Ok(());
        }
        
        // Sort by timestamp (newest first)
        backups.sort_by(|a, b| b.1.cmp(&a.1));
        
        println!("{} Found {} backup file(s):", "📋".cyan(), backups.len());
        println!();
        
        for (path, timestamp, _) in backups {
            let original_path = self.get_original_path_from_backup(&path)?;
            let relative = path.strip_prefix(&base_dir).unwrap_or(&path);
            println!("  {} {:?}", "•".green(), relative);
            println!("    Original: {:?}", original_path);
            println!("    Created: {}", self.format_timestamp(timestamp));
            println!();
        }
        
        Ok(())
    }
    
    fn restore_backup(&self, backup_path: PathBuf) -> Result<()> {
        if !backup_path.exists() {
            anyhow::bail!(
                "Backup file not found: {:?}.\n\
                \n\
                Possible causes:\n\
                - Backup file was deleted\n\
                - Incorrect backup path specified\n\
                - Backup was cleaned up (older than retention period)\n\
                \n\
                To fix:\n\
                1. List available backups: themectl backups list\n\
                2. Check if the backup still exists: ls {:?}\n\
                3. If backups were cleaned, the original file may have been modified",
                backup_path, backup_path.parent().map(|p| p.as_os_str()).unwrap_or_else(|| backup_path.as_os_str())
            );
        }
        
        if backup_path.extension() != Some(std::ffi::OsStr::new("bak")) {
            anyhow::bail!(
                "File is not a backup file: {:?}.\n\
                \n\
                Backup files must have a .bak extension with a timestamp.\n\
                Expected format: <filename>.<timestamp>.bak\n\
                \n\
                Your file: {:?}\n\
                \n\
                To fix: Use a valid backup file. List backups with: themectl backups list",
                backup_path, backup_path
            );
        }
        
        let original_path = self.get_original_path_from_backup(&backup_path)?;
        
        // Confirm restoration
        print!("{} Restore backup {:?} to {:?}? (y/N): ", 
            "⚠".yellow(), 
            backup_path.file_name().unwrap_or_default(),
            original_path
        );
        io::stdout().flush()?;
        let mut response = String::new();
        io::stdin().read_line(&mut response)?;
        if response.trim().to_lowercase() != "y" {
            println!("{} Cancelled.", "✗".red());
            return Ok(());
        }
        
        // Create backup of current file if it exists
        if original_path.exists() {
            let file_manager = FileManager::new(None, false);
            file_manager.backup_file(&original_path)?;
        }
        
        // Restore backup
        std::fs::copy(&backup_path, &original_path)
            .with_context(|| format!("Failed to restore backup to {:?}", original_path))?;
        
        println!("{} Backup restored to {:?}", "✓".green(), original_path);
        
        Ok(())
    }
    
    fn clean_backups(&self, days: Option<u64>, config_dir: Option<&PathBuf>) -> Result<()> {
        let base_dir = config_dir.cloned().unwrap_or_else(|| {
            dirs::home_dir()
                .unwrap_or_else(|| PathBuf::from("~"))
                .join(".config")
        });
        
        let days_to_keep = days.unwrap_or(30);
        let cutoff_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
            - (days_to_keep * 24 * 60 * 60);
        
        let mut backups_to_remove = Vec::new();
        
        // Find all backup files
        if base_dir.exists() {
            for entry in walkdir::WalkDir::new(&base_dir) {
                let entry = entry?;
                if entry.file_type().is_file() {
                    let path = entry.path();
                    if let Some(ext) = path.extension() {
                        if ext == "bak" {
                            if let Some(file_name) = path.file_name() {
                                if let Some(name_str) = file_name.to_str() {
                                    if let Some(timestamp) = self.parse_backup_timestamp(name_str) {
                                        if timestamp < cutoff_time {
                                            backups_to_remove.push(path.to_path_buf());
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        
        if backups_to_remove.is_empty() {
            println!("{} No backup files older than {} days found.", "→".cyan(), days_to_keep);
            return Ok(());
        }
        
        println!("{} Found {} backup file(s) older than {} days:", 
            "⚠".yellow(), 
            backups_to_remove.len(), 
            days_to_keep
        );
        for backup in &backups_to_remove {
            println!("  {}", backup.display());
        }
        
        print!("{} Delete these backup files? (y/N): ", "⚠".yellow());
        io::stdout().flush()?;
        let mut response = String::new();
        io::stdin().read_line(&mut response)?;
        if response.trim().to_lowercase() != "y" {
            println!("{} Cancelled.", "✗".red());
            return Ok(());
        }
        
        let mut deleted = 0;
        for backup in &backups_to_remove {
            match std::fs::remove_file(backup) {
                Ok(_) => deleted += 1,
                Err(e) => eprintln!("  {} Failed to delete {:?}: {}", "✗".red(), backup, e),
            }
        }
        
        println!("{} Deleted {} backup file(s).", "✓".green(), deleted);
        
        Ok(())
    }
    
    fn parse_backup_timestamp(&self, filename: &str) -> Option<u64> {
        // Backup filename format: original.{timestamp}.bak
        // Extract timestamp from between the dots
        let parts: Vec<&str> = filename.rsplitn(3, '.').collect();
        if parts.len() >= 3 && parts[0] == "bak" {
            parts[1].parse::<u64>().ok()
        } else {
            None
        }
    }
    
    fn get_original_path_from_backup(&self, backup_path: &PathBuf) -> Result<PathBuf> {
        // Backup filename format: original.{timestamp}.bak
        // Example: kitty.conf -> kitty.{timestamp}.bak
        // The backup_file method uses: path.with_extension(format!("{}.bak", timestamp))
        // This means: kitty.conf becomes kitty.{timestamp}.bak
        
        if let Some(file_name) = backup_path.file_name() {
            if let Some(name_str) = file_name.to_str() {
                // Remove .{timestamp}.bak to get base name
                // Split: "kitty.{timestamp}.bak" -> ["bak", "{timestamp}", "kitty"]
                let parts: Vec<&str> = name_str.rsplitn(3, '.').collect();
                if parts.len() == 3 && parts[0] == "bak" {
                    let base_name = parts[2];
                    
                    if let Some(parent) = backup_path.parent() {
                        // Try to find existing file with common extensions
                        // This handles the case where the original file still exists
                        for ext in &["conf", "css", "toml", "lua", "jsonc", "json"] {
                            let candidate = parent.join(format!("{}.{}", base_name, ext));
                            if candidate.exists() {
                                return Ok(candidate);
                            }
                        }
                        
                        // If original doesn't exist, we need to guess the extension
                        // Look for other backups in the same directory to infer the pattern
                        // For now, default to .conf as it's most common
                        return Ok(parent.join(format!("{}.conf", base_name)));
                    }
                }
            }
        }
        
        anyhow::bail!(
            "Could not determine original path from backup file: {:?}.\n\
            \n\
            Backup files should follow the format: <original-filename>.<timestamp>.bak\n\
            \n\
            Possible causes:\n\
            - Backup file has an unexpected naming format\n\
            - Backup file is corrupted\n\
            - Backup was created by a different version of themectl\n\
            \n\
            To fix:\n\
            1. Check the backup file name format\n\
            2. Try listing backups: themectl backups list\n\
            3. If the backup is valid, you may need to restore manually",
            backup_path
        );
    }
    
    fn format_timestamp(&self, timestamp: u64) -> String {
        use std::time::{SystemTime, UNIX_EPOCH};
        match SystemTime::now().duration_since(UNIX_EPOCH) {
            Ok(now) => {
                let age_secs = now.as_secs().saturating_sub(timestamp);
                if age_secs < 60 {
                    format!("{} seconds ago", age_secs)
                } else if age_secs < 3600 {
                    format!("{} minutes ago", age_secs / 60)
                } else if age_secs < 86400 {
                    format!("{} hours ago", age_secs / 3600)
                } else {
                    format!("{} days ago", age_secs / 86400)
                }
            }
            Err(_) => format!("timestamp: {}", timestamp),
        }
    }

    // Variant commands
    fn create_variant(&self, theme_name: &str, variant: &str, auto: bool, themes_dir: &PathBuf) -> Result<()> {
        if variant != "dark" && variant != "light" {
            anyhow::bail!(
                "Invalid variant: '{}'.\n\
                \n\
                Valid variants are:\n\
                - 'dark': Dark theme variant\n\
                - 'light': Light theme variant\n\
                \n\
                You provided: '{}'\n\
                \n\
                To fix: Use 'dark' or 'light' as the variant name.\n\
                Example: themectl variant create my-theme dark",
                variant, variant
            );
        }

        let theme_path = themes_dir.join(format!("{}.toml", theme_name));
        if !theme_path.exists() {
            anyhow::bail!(
                "Theme '{}' not found at {:?}.\n\
                \n\
                Expected theme file: {:?}\n\
                \n\
                Possible causes:\n\
                - Theme name is misspelled\n\
                - Theme file doesn't exist\n\
                - Wrong themes directory\n\
                \n\
                To fix:\n\
                1. List available themes: themectl list\n\
                2. Check if the theme exists: ls {:?}\n\
                3. Create the theme if it doesn't exist: themectl create {}",
                theme_name, theme_path, theme_path, theme_path.parent().map(|p| p.as_os_str()).unwrap_or_else(|| theme_path.as_os_str()), theme_name
            );
        }

        let base_theme = parser::parse_theme_file(&theme_path)?;
        let new_variant = if auto {
            generate_variant(&base_theme, variant)?
        } else {
            // Interactive creation - for now, just auto-generate
            // Could be enhanced to allow manual color editing
            generate_variant(&base_theme, variant)?
        };

        let variant_path = themes_dir.join(format!("{}-{}.toml", theme_name, variant));
        if variant_path.exists() {
            print!("{} Variant '{}' already exists. Overwrite? (y/N): ", "⚠".yellow(), variant);
            io::stdout().flush()?;
            let mut response = String::new();
            io::stdin().read_line(&mut response)?;
            if response.trim().to_lowercase() != "y" {
                println!("{} Cancelled.", "✗".red());
                return Ok(());
            }
        }

        let toml_content = toml::to_string_pretty(&new_variant)
            .context("Failed to serialize variant to TOML")?;
        std::fs::write(&variant_path, toml_content)?;
        println!("{} Created variant: {}", "✓".green(), variant_path.display());
        Ok(())
    }

    fn switch_variant(&self, theme_name: &str, variant: &str, themes_dir: &PathBuf) -> Result<()> {
        let variant_path = themes_dir.join(format!("{}-{}.toml", theme_name, variant));
        if !variant_path.exists() {
            anyhow::bail!(
                "Variant '{}' not found at {:?}.\n\
                \n\
                Expected variant file: {:?}\n\
                \n\
                Possible causes:\n\
                - Variant name is misspelled\n\
                - Variant file doesn't exist\n\
                - Base theme doesn't exist\n\
                \n\
                To fix:\n\
                1. List variants: themectl variant list <theme-name>\n\
                2. Create the variant: themectl variant create <theme-name> {}\n\
                3. Check if base theme exists: themectl list",
                variant, variant_path, variant_path, variant
            );
        }

        let theme = parser::parse_theme_file(&variant_path)?;
        println!("{} Switching to variant: {}", "✓".green(), variant.bold());
        
        let file_manager = FileManager::new(None, self.dry_run);
        file_manager.apply_theme(&theme)?;
        Ok(())
    }

    fn list_variants(&self, theme_name: &str, themes_dir: &PathBuf) -> Result<()> {
        let base_name = Theme::extract_base_name(theme_name);
        let themes = parser::find_theme_files(themes_dir)?;
        
        let mut variants = Vec::new();
        for theme_path in themes {
            if let Some(name) = theme_path.file_stem() {
                if let Some(name_str) = name.to_str() {
                    if name_str.starts_with(&base_name) {
                        if let Ok(theme) = parser::parse_theme_file(&theme_path) {
                            let variant = theme.get_variant();
                            variants.push((name_str.to_string(), variant));
                        }
                    }
                }
            }
        }

        if variants.is_empty() {
            println!("No variants found for theme '{}'", base_name);
            return Ok(());
        }

        println!("{} Variants for '{}':", "📋".cyan(), base_name);
        for (name, variant) in variants {
            if let Some(v) = variant {
                println!("  {} {} ({})", "•".green(), name.bold(), v);
            } else {
                println!("  {} {} (base)", "•".green(), name.bold());
            }
        }
        Ok(())
    }

    // Config commands
    fn config_set_path(&self, app: &str, path: &PathBuf) -> Result<()> {
        let mut config = ThemectlConfig::load()?.unwrap_or_default();
        config.set_app_path(app, path.clone());
        config.save()?;
        println!("{} Set path for '{}' to: {}", "✓".green(), app, path.display());
        Ok(())
    }

    fn config_get_path(&self, app: &str) -> Result<()> {
        if let Some(config) = ThemectlConfig::load()? {
            if let Some(path) = config.get_app_path(app) {
                println!("{}", path.display());
            } else {
                println!("No custom path set for '{}'", app);
            }
        } else {
            println!("No custom path set for '{}'", app);
        }
        Ok(())
    }

    fn config_enable_nixos(&self) -> Result<()> {
        let mut config = ThemectlConfig::load()?.unwrap_or_default();
        config.nixos_mode = true;
        config.save()?;
        println!("{} NixOS mode enabled", "✓".green());
        Ok(())
    }

    fn config_disable_nixos(&self) -> Result<()> {
        let mut config = ThemectlConfig::load()?.unwrap_or_default();
        config.nixos_mode = false;
        config.save()?;
        println!("{} NixOS mode disabled", "✓".green());
        Ok(())
    }

    fn config_create_template(&self, app: &str, path: Option<&PathBuf>) -> Result<()> {
        // Create a default theme for template generation
        let default_theme = Theme {
            name: "default".to_string(),
            description: "Default theme".to_string(),
            variant: None,
            colors: ColorPalette {
                bg: "#282828".to_string(),
                fg: "#ebdbb2".to_string(),
                accent: "#458588".to_string(),
                red: "#cc241d".to_string(),
                green: "#98971a".to_string(),
                yellow: "#d79921".to_string(),
                blue: "#458588".to_string(),
                magenta: "#b16286".to_string(),
                cyan: "#689d6a".to_string(),
                orange: None,
                purple: None,
                pink: None,
                white: None,
                black: None,
                gray: None,
            },
            properties: ThemeProperties::default(),
        };

        let output_path = if let Some(p) = path {
            p.clone()
        } else {
            templates::get_standard_config_path(app, &default_theme)
                .ok_or_else(|| anyhow::anyhow!("Could not determine standard path for '{}'", app))?
        };

        templates::create_missing_config(app, &output_path, &default_theme)?;
        println!("{} Created template for '{}' at: {}", "✓".green(), app, output_path.display());
        Ok(())
    }

    fn config_set_deployment(&self, method: &str) -> Result<()> {
        let mut config = ThemectlConfig::load()?.unwrap_or_default();
        config.set_deployment_method(method)?;
        config.save()?;
        println!("{} Deployment method set to: {}", "✓".green(), method);
        Ok(())
    }

    fn config_get_deployment(&self) -> Result<()> {
        let config = ThemectlConfig::load()?.unwrap_or_default();
        println!("{}", config.get_deployment_method());
        Ok(())
    }

    fn config_set_nix_path(&self, path: PathBuf) -> Result<()> {
        let mut config = ThemectlConfig::load()?.unwrap_or_default();
        config.set_nix_output_path(path.clone());
        config.save()?;
        println!("{} Nix output path set to: {}", "✓".green(), path.display());
        Ok(())
    }

    // Batch operations
    fn export_all_themes(&self, format: &str, output_dir: &PathBuf, themes_dir: &PathBuf) -> Result<()> {
        let themes = parser::find_theme_files(themes_dir)?;
        
        if themes.is_empty() {
            println!("No themes found in {:?}", themes_dir);
            return Ok(());
        }

        std::fs::create_dir_all(output_dir)?;
        let all_formats = format == "all";
        let formats = if all_formats {
            vec!["kitty", "waybar", "neovim", "starship", "mako", "hyprland", "wofi", "wlogout", "fastfetch", "yazi", "hyprpaper", "nix", "gtk", "btop", "git"]
        } else {
            vec![format]
        };

        let mut success_count = 0;
        let mut error_count = 0;

        // Use parallel processing for theme parsing and generation
        use rayon::prelude::*;
        let results: Vec<_> = themes
            .par_iter()
            .map(|theme_path| {
                if let Some(name) = theme_path.file_stem() {
                    if let Some(name_str) = name.to_str() {
                        // Use cached parsing for better performance
                        match parser::parse_theme_file_cached(theme_path) {
                            Ok(theme) => {
                                let theme_dir = output_dir.join(name_str);
                                if let Err(e) = std::fs::create_dir_all(&theme_dir) {
                                    return Err(anyhow::anyhow!("Failed to create directory for {}: {}", name_str, e));
                                }

                                let mut errors = Vec::new();
                                
                                if all_formats {
                                    // Use parallel generation for all formats
                                    let gen_results = generators::generate_all_parallel(&theme);
                                    for (fmt, result) in gen_results {
                                        match result {
                                            Ok(content) => {
                                                let ext = match fmt.as_str() {
                                                    "neovim" => "lua",
                                                    "starship" => "toml",
                                                    "fastfetch" => "jsonc",
                                                    "yazi" => "toml",
                                                    "nix" => "nix",
                                                    "btop" => "theme",
                                                    _ => "conf",
                                                };
                                                let file_path = theme_dir.join(format!("{}.{}", fmt, ext));
                                                if let Err(e) = std::fs::write(&file_path, content) {
                                                    errors.push(format!("Failed to write {}: {}", fmt, e));
                                                }
                                            }
                                            Err(e) => {
                                                errors.push(format!("Failed to generate {}: {}", fmt, e));
                                            }
                                        }
                                    }
                                } else {
                                    // Single format
                                    for fmt in &formats {
                                        match generators::generate(&theme, fmt) {
                                            Ok(content) => {
                                                let ext = match *fmt {
                                                    "neovim" => "lua",
                                                    "starship" => "toml",
                                                    "fastfetch" => "jsonc",
                                                    "yazi" => "toml",
                                                    "nix" => "nix",
                                                    "btop" => "theme",
                                                    _ => "conf",
                                                };
                                                let file_path = theme_dir.join(format!("{}.{}", fmt, ext));
                                                if let Err(e) = std::fs::write(&file_path, content) {
                                                    errors.push(format!("Failed to write {}: {}", fmt, e));
                                                }
                                            }
                                            Err(e) => {
                                                errors.push(format!("Failed to generate {}: {}", fmt, e));
                                            }
                                        }
                                    }
                                }
                                
                                if errors.is_empty() {
                                    Ok(name_str.to_string())
                                } else {
                                    Err(anyhow::anyhow!("Errors for {}: {}", name_str, errors.join("; ")))
                                }
                            }
                            Err(e) => Err(anyhow::anyhow!("Failed to parse: {}", e)),
                        }
                    } else {
                        Err(anyhow::anyhow!("Invalid theme name"))
                    }
                } else {
                    Err(anyhow::anyhow!("Could not extract theme name"))
                }
            })
            .collect();

        // Process results
        for result in results {
            match result {
                Ok(_name) => {
                    success_count += 1;
                }
                Err(e) => {
                    error_count += 1;
                    eprintln!("  {} Error: {}", "✗".red(), e);
                }
            }
        }

        println!("{} Exported {} theme(s) successfully", "✓".green(), success_count);
        if error_count > 0 {
            eprintln!("{} {} error(s) occurred", "✗".red(), error_count);
        }
        Ok(())
    }

    fn validate_all_themes(&self, themes_dir: &PathBuf) -> Result<()> {
        let themes = parser::find_theme_files(themes_dir)?;
        
        if themes.is_empty() {
            println!("No themes found in {:?}", themes_dir);
            return Ok(());
        }

        // Use parallel processing with cached parsing
        use rayon::prelude::*;
        let results: Vec<_> = themes
            .par_iter()
            .map(|theme_path| {
                if let Some(name) = theme_path.file_stem() {
                    if let Some(name_str) = name.to_str() {
                        // Use cached parsing for better performance
                        match parser::parse_theme_file_cached(theme_path) {
                            Ok(theme) => {
                                let warnings = parser::validate_accessibility(&theme);
                                (name_str.to_string(), Ok(warnings))
                            }
                            Err(e) => {
                                let error_warning = vec![parser::ValidationWarning {
                                    level: parser::ValidationLevel::Error,
                                    message: format!("Parse error: {}", e),
                                }];
                                (name_str.to_string(), Err(error_warning))
                            }
                        }
                    } else {
                        ("unknown".to_string(), Err(vec![]))
                    }
                } else {
                    ("unknown".to_string(), Err(vec![]))
                }
            })
            .collect();

        let mut valid_count = 0;
        let mut invalid_count = 0;
        let mut warning_count = 0;
        let mut themes_with_issues = Vec::new();

        for (name, result) in results {
            match result {
                Ok(warnings) => {
                    if warnings.is_empty() {
                        valid_count += 1;
                    } else {
                        warning_count += warnings.len();
                        themes_with_issues.push((name, warnings));
                    }
                }
                Err(warnings) => {
                    invalid_count += 1;
                    themes_with_issues.push((name, warnings));
                }
            }
        }

        println!("\n{} Validation Summary:", "→".cyan());
        println!("  {} Valid: {}", "✓".green(), valid_count);
        println!("  {} Invalid: {}", "✗".red(), invalid_count);
        println!("  {} Warnings: {}", "⚠".yellow(), warning_count);

        if !themes_with_issues.is_empty() {
            println!("\n{} Themes with issues:", "⚠".yellow());
            for (name, warnings) in themes_with_issues {
                println!("  {} {}", "•".yellow(), name.bold());
                for warning in warnings {
                    let symbol = match warning.level {
                        parser::ValidationLevel::Error => "✗".red(),
                        parser::ValidationLevel::Warning => "⚠".yellow(),
                        parser::ValidationLevel::Info => "ℹ".cyan(),
                    };
                    println!("    {} {}", symbol, warning.message);
                }
            }
        }

        Ok(())
    }
}
