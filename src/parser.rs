use anyhow::{Context, Result};
use std::fs;
use std::path::Path;

use crate::cache;
use crate::theme::Theme;
use crate::utils::{validate_hex_color, check_contrast, ContrastLevel, find_similar_colors, calculate_contrast_ratio};

pub fn parse_theme_file<P: AsRef<Path>>(path: P) -> Result<Theme> {
    let content = fs::read_to_string(&path)
        .with_context(|| format!(
            "Failed to read theme file at {:?}.\n\
            \n\
            Possible causes:\n\
            - File does not exist at this path\n\
            - Insufficient read permissions\n\
            - File is corrupted or locked by another process\n\
            - Disk I/O error\n\
            \n\
            To fix: Check that the file exists and you have read permissions. \
            Use 'themectl list' to see available themes.",
            path.as_ref()
        ))?;
    
    parse_theme(&content)
        .with_context(|| format!(
            "Failed to parse theme file at {:?}.\n\
            The file was read successfully but could not be parsed.",
            path.as_ref()
        ))
}

/// Parse a theme file using the global cache
/// This is the preferred method for batch operations where the same theme
/// might be parsed multiple times
pub fn parse_theme_file_cached<P: AsRef<Path>>(path: P) -> Result<Theme> {
    cache::global_cache().get_or_parse(path)
}

pub fn parse_theme(content: &str) -> Result<Theme> {
    let theme: Theme = toml::from_str(content)
        .with_context(|| format!(
            "Failed to parse TOML theme file.\n\
            \n\
            The file exists but contains invalid TOML syntax.\n\
            \n\
            Common TOML syntax errors:\n\
            - Missing quotes around string values\n\
            - Invalid table syntax (e.g., missing brackets)\n\
            - Invalid color format (must be #RRGGBB)\n\
            - Missing required fields (name, colors.bg, colors.fg, etc.)\n\
            - Invalid property values (numbers, strings, etc.)\n\
            \n\
            To fix: Check the TOML syntax in your theme file. \
            See docs/THEME_FORMAT.md for the correct format."
        ))?;
    
    validate_theme(&theme)?;
    
    Ok(theme)
}

#[derive(Debug, Clone)]
pub struct ValidationWarning {
    pub level: ValidationLevel,
    pub message: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValidationLevel {
    Error,
    Warning,
    Info,
}

pub fn validate_theme(theme: &Theme) -> Result<()> {
    if theme.name.is_empty() {
        anyhow::bail!(
            "Theme validation failed: theme name cannot be empty.\n\
            \n\
            Every theme must have a non-empty 'name' field in the TOML file.\n\
            \n\
            To fix: Add a 'name' field to your theme file:\n\
            name = \"your-theme-name\""
        );
    }
    
    validate_color(&theme.colors.bg, "bg")?;
    validate_color(&theme.colors.fg, "fg")?;
    validate_color(&theme.colors.accent, "accent")?;
    validate_color(&theme.colors.red, "red")?;
    validate_color(&theme.colors.green, "green")?;
    validate_color(&theme.colors.yellow, "yellow")?;
    validate_color(&theme.colors.blue, "blue")?;
    validate_color(&theme.colors.magenta, "magenta")?;
    validate_color(&theme.colors.cyan, "cyan")?;
    
    if let Some(ref orange) = theme.colors.orange {
        validate_color(orange, "orange")?;
    }
    if let Some(ref purple) = theme.colors.purple {
        validate_color(purple, "purple")?;
    }
    if let Some(ref pink) = theme.colors.pink {
        validate_color(pink, "pink")?;
    }
    if let Some(ref white) = theme.colors.white {
        validate_color(white, "white")?;
    }
    if let Some(ref black) = theme.colors.black {
        validate_color(black, "black")?;
    }
    if let Some(ref gray) = theme.colors.gray {
        validate_color(gray, "gray")?;
    }
    
    Ok(())
}

/// Validate theme accessibility and return warnings
pub fn validate_accessibility(theme: &Theme) -> Vec<ValidationWarning> {
    let mut warnings = Vec::new();
    
    // Check bg/fg contrast (minimum AA level)
    if !check_contrast(&theme.colors.bg, &theme.colors.fg, ContrastLevel::AA) {
        if let Some(ratio) = calculate_contrast_ratio(&theme.colors.bg, &theme.colors.fg) {
            warnings.push(ValidationWarning {
                level: ValidationLevel::Error,
                message: format!(
                    "Background/foreground contrast ratio ({:.2}:1) does not meet WCAG AA standard (4.5:1 required)",
                    ratio
                ),
            });
        }
    } else if let Some(ratio) = calculate_contrast_ratio(&theme.colors.bg, &theme.colors.fg) {
        if ratio < 7.0 {
            warnings.push(ValidationWarning {
                level: ValidationLevel::Info,
                message: format!(
                    "Background/foreground contrast ratio ({:.2}:1) meets AA but not AAA standard (7:1 for AAA)",
                    ratio
                ),
            });
        }
    }
    
    // Check for similar colors
    let similar = find_similar_colors(theme, 30.0);
    for (color1, color2, distance) in similar {
        warnings.push(ValidationWarning {
            level: ValidationLevel::Warning,
            message: format!(
                "Colors '{}' and '{}' are very similar (distance: {:.1})",
                color1, color2, distance
            ),
        });
    }
    
    warnings
}

fn validate_color(color: &str, name: &str) -> Result<()> {
    if !validate_hex_color(color) {
        anyhow::bail!(
            "Invalid color format for '{}': '{}'.\n\
            \n\
            Expected format: #RRGGBB (hexadecimal color code)\n\
            \n\
            Examples of valid colors:\n\
            - #282828 (dark gray)\n\
            - #ebdbb2 (light beige)\n\
            - #458588 (blue)\n\
            \n\
            Your value: '{}'\n\
            \n\
            To fix: Use a valid hex color code starting with '#' followed by 6 hexadecimal digits (0-9, A-F). \
            You can use online color pickers or tools like 'gpick' to get hex codes.",
            name, color, color
        );
    }
    Ok(())
}

pub fn find_theme_files<P: AsRef<Path>>(dir: P) -> Result<Vec<std::path::PathBuf>> {
    let mut themes = Vec::new();
    
    if !dir.as_ref().exists() {
        return Ok(themes);
    }
    
    for entry in walkdir::WalkDir::new(dir) {
        let entry = entry?;
        if entry.file_type().is_file() {
            if let Some(ext) = entry.path().extension() {
                if ext == "toml" {
                    themes.push(entry.path().to_path_buf());
                }
            }
        }
    }
    
    Ok(themes)
}
