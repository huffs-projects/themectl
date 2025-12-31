use anyhow::Result;

use crate::generators;
use crate::theme::Theme;
use std::fs;

/// Generate a config file template for an application
pub fn generate_template(app: &str, theme: &Theme) -> Result<String> {
    // Use the existing generator to create the template
    // This ensures templates match the actual generated format
    generators::generate(theme, app)
}

/// Create a missing config file with template content
pub fn create_missing_config(app: &str, path: &std::path::Path, theme: &Theme) -> Result<()> {
    let content = generate_template(app, theme)?;
    
    // Create parent directory if needed
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    
    // Write template content
    fs::write(path, content)?;
    
    Ok(())
}

/// Get the standard config file path for an application
pub fn get_standard_config_path(app: &str, theme: &Theme) -> Option<std::path::PathBuf> {
    let base_dir = dirs::home_dir()?.join(".config");
    
    Some(match app {
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
        _ => return None,
    })
}
