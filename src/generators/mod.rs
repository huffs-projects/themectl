mod btop;
mod fastfetch;
mod git;
mod gtk;
mod hyprland;
mod hyprpaper;
mod kitty;
mod mako;
mod neovim;
mod nix;
mod starship;
mod waybar;
mod wofi;
mod wlogout;
mod yazi;

use anyhow::Result;
use rayon::prelude::*;
use crate::theme::Theme;

pub fn generate(theme: &Theme, format: &str) -> Result<String> {
    match format.to_lowercase().as_str() {
        "nix" => nix::generate(theme),
        "kitty" => kitty::generate(theme),
        "waybar" => waybar::generate(theme),
        "neovim" => neovim::generate(theme),
        "starship" => starship::generate(theme),
        "mako" => mako::generate(theme),
        "hyprland" => hyprland::generate(theme),
        "hyprpaper" => hyprpaper::generate(theme),
        "wofi" => wofi::generate(theme),
        "wlogout" => wlogout::generate(theme),
        "fastfetch" => fastfetch::generate(theme),
        "yazi" => yazi::generate(theme),
        "gtk" => gtk::generate(theme),
        "gtk-css" => gtk::generate_css(theme),
        "btop" => btop::generate(theme),
        "git" => git::generate(theme),
        "git-nix" => git::generate_nix(theme),
        _ => anyhow::bail!(
            "Unknown format: '{}'.\n\
            \n\
            Supported formats are:\n\
            - kitty: Terminal emulator configuration\n\
            - waybar: Status bar CSS\n\
            - neovim: Lua color scheme\n\
            - starship: Shell prompt configuration\n\
            - mako: Notification daemon colors\n\
            - hyprland: Window manager colors\n\
            - hyprpaper: Wallpaper manager configuration\n\
            - wofi: Application launcher colors\n\
            - wlogout: Logout menu colors\n\
            - fastfetch: System info display colors\n\
            - yazi: File manager TOML configuration\n\
            - gtk: GTK4 theme configuration\n\
            - btop: System monitor theme\n\
            - git: Git color configuration\n\
            - nix: Nix color attribute set\n\
            \n\
            You requested: '{}'\n\
            \n\
            To fix: Use one of the supported formats listed above.",
            format, format
        ),
    }
}

/// Generate a Home Manager module for a specific application
pub fn generate_home_manager_module(theme: &Theme, app: &str) -> Result<String> {
    nix::generate_home_manager_module(theme, app)
}

pub fn generate_all(theme: &Theme) -> Result<Vec<(String, String)>> {
    let formats = vec![
        "nix", "kitty", "waybar", "neovim", "starship", 
        "mako", "hyprland", "hyprpaper", "wofi", "wlogout", "fastfetch", "yazi", "gtk", "btop", "git"
    ];
    
    let mut results = Vec::new();
    for format in formats {
        match generate(theme, format) {
            Ok(content) => results.push((format.to_string(), content)),
            Err(e) => eprintln!("Warning: Failed to generate {}: {}", format, e),
        }
    }
    
    Ok(results)
}

/// Generate all formats in parallel
pub fn generate_all_parallel(theme: &Theme) -> Vec<(String, Result<String>)> {
    let formats = vec![
        "nix", "kitty", "waybar", "neovim", "starship", 
        "mako", "hyprland", "hyprpaper", "wofi", "wlogout", "fastfetch", "yazi", "gtk", "btop", "git"
    ];
    
    formats
        .into_par_iter()
        .map(|format| {
            let result = generate(theme, format);
            (format.to_string(), result)
        })
        .collect()
}
