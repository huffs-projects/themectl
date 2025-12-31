use std::path::PathBuf;
use tempfile::TempDir;
use themectl::theme::{ColorPalette, Theme, ThemeProperties};

/// Create a minimal valid theme with only required colors
#[allow(dead_code)]
pub fn create_test_theme() -> Theme {
    Theme {
        name: "test-theme".to_string(),
        description: "Test theme".to_string(),
        variant: None,
        colors: ColorPalette {
            bg: "#282828".to_string(),
            fg: "#ebdbb2".to_string(),
            accent: "#fe8019".to_string(),
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
    }
}

/// Create a full theme with all optional colors
#[allow(dead_code)]
pub fn create_full_test_theme() -> Theme {
    Theme {
        name: "full-test-theme".to_string(),
        description: "Full test theme with all colors".to_string(),
        variant: Some("dark".to_string()),
        colors: ColorPalette {
            bg: "#282828".to_string(),
            fg: "#ebdbb2".to_string(),
            accent: "#fe8019".to_string(),
            red: "#cc241d".to_string(),
            green: "#98971a".to_string(),
            yellow: "#d79921".to_string(),
            blue: "#458588".to_string(),
            magenta: "#b16286".to_string(),
            cyan: "#689d6a".to_string(),
            orange: Some("#d65d0e".to_string()),
            purple: Some("#b16286".to_string()),
            pink: Some("#d3869b".to_string()),
            white: Some("#fbf1c7".to_string()),
            black: Some("#1d2021".to_string()),
            gray: Some("#928374".to_string()),
        },
        properties: ThemeProperties {
            border_radius: Some(8),
            border_width: Some(2),
            shadow_blur: Some(10),
            animation_duration: Some(0.2),
            spacing: Some(4),
        },
    }
}

/// Create a temporary themes directory for testing
#[allow(dead_code)]
pub fn create_temp_themes_dir() -> TempDir {
    tempfile::tempdir().expect("Failed to create temporary directory")
}

/// Get the themes directory path from a TempDir
#[allow(dead_code)]
pub fn get_themes_path(temp_dir: &TempDir) -> PathBuf {
    temp_dir.path().to_path_buf()
}

/// Assert that a theme has valid structure
#[allow(dead_code)]
pub fn assert_theme_valid(theme: &Theme) {
    assert!(!theme.name.is_empty(), "Theme name should not be empty");
    assert!(!theme.colors.bg.is_empty(), "Background color should not be empty");
    assert!(!theme.colors.fg.is_empty(), "Foreground color should not be empty");
    assert!(!theme.colors.accent.is_empty(), "Accent color should not be empty");
    assert!(!theme.colors.red.is_empty(), "Red color should not be empty");
    assert!(!theme.colors.green.is_empty(), "Green color should not be empty");
    assert!(!theme.colors.yellow.is_empty(), "Yellow color should not be empty");
    assert!(!theme.colors.blue.is_empty(), "Blue color should not be empty");
    assert!(!theme.colors.magenta.is_empty(), "Magenta color should not be empty");
    assert!(!theme.colors.cyan.is_empty(), "Cyan color should not be empty");
}

/// Create a theme with low contrast (for accessibility testing)
#[allow(dead_code)]
pub fn create_low_contrast_theme() -> Theme {
    Theme {
        name: "low-contrast-theme".to_string(),
        description: "Low contrast theme for testing".to_string(),
        variant: None,
        colors: ColorPalette {
            bg: "#333333".to_string(),
            fg: "#343434".to_string(), // Very similar to bg
            accent: "#fe8019".to_string(),
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
    }
}

/// Create a theme with similar colors (for similarity testing)
#[allow(dead_code)]
pub fn create_similar_colors_theme() -> Theme {
    Theme {
        name: "similar-colors-theme".to_string(),
        description: "Theme with similar colors".to_string(),
        variant: None,
        colors: ColorPalette {
            bg: "#282828".to_string(),
            fg: "#ebdbb2".to_string(),
            accent: "#fe8019".to_string(),
            red: "#cc241d".to_string(),
            green: "#cc241e".to_string(), // Very similar to red
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
    }
}

/// Create a theme with high contrast (AAA standard)
#[allow(dead_code)]
pub fn create_aaa_contrast_theme() -> Theme {
    Theme {
        name: "aaa-contrast-theme".to_string(),
        description: "High contrast theme meeting AAA standard".to_string(),
        variant: None,
        colors: ColorPalette {
            bg: "#000000".to_string(),
            fg: "#ffffff".to_string(), // Black on white = 21:1 contrast
            accent: "#fe8019".to_string(),
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
    }
}
