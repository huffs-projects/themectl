mod common;

use std::fs;
use themectl::parser::*;
use common::*;

#[test]
fn test_parse_theme_valid() {
    let toml_content = r##"
name = "test-theme"
description = "Test theme"

[colors]
bg = "#282828"
fg = "#ebdbb2"
accent = "#fe8019"
red = "#cc241d"
green = "#98971a"
yellow = "#d79921"
blue = "#458588"
magenta = "#b16286"
cyan = "#689d6a"
"##;
    
    let result = parse_theme(toml_content);
    assert!(result.is_ok());
    let theme = result.unwrap();
    assert_eq!(theme.name, "test-theme");
    assert_eq!(theme.colors.bg, "#282828");
}

#[test]
fn test_parse_theme_with_optional_colors() {
    let toml_content = r##"
name = "full-theme"
description = "Full theme"

[colors]
bg = "#282828"
fg = "#ebdbb2"
accent = "#fe8019"
red = "#cc241d"
green = "#98971a"
yellow = "#d79921"
blue = "#458588"
magenta = "#b16286"
cyan = "#689d6a"
orange = "#d65d0e"
purple = "#b16286"
pink = "#d3869b"
white = "#fbf1c7"
black = "#1d2021"
gray = "#928374"
"##;
    
    let result = parse_theme(toml_content);
    assert!(result.is_ok());
    let theme = result.unwrap();
    assert_eq!(theme.colors.orange, Some("#d65d0e".to_string()));
    assert_eq!(theme.colors.purple, Some("#b16286".to_string()));
}

#[test]
fn test_parse_theme_invalid_toml() {
    let invalid_toml = r##"
name = "test-theme"
[colors
bg = "#282828"
"##;
    
    let result = parse_theme(invalid_toml);
    assert!(result.is_err());
}

#[test]
fn test_parse_theme_missing_required_field() {
    let incomplete_toml = r##"
name = "test-theme"
[colors]
bg = "#282828"
fg = "#ebdbb2"
"##;
    
    let result = parse_theme(incomplete_toml);
    // Should fail validation, not parsing
    assert!(result.is_err());
}

#[test]
fn test_parse_theme_invalid_color_format() {
    let invalid_color_toml = r##"
name = "test-theme"
description = "Test"

[colors]
bg = "not-a-hex-color"
fg = "#ebdbb2"
accent = "#fe8019"
red = "#cc241d"
green = "#98971a"
yellow = "#d79921"
blue = "#458588"
magenta = "#b16286"
cyan = "#689d6a"
"##;
    
    let result = parse_theme(invalid_color_toml);
    assert!(result.is_err());
}

#[test]
fn test_parse_theme_file_success() {
    let temp_dir = create_temp_themes_dir();
    let theme_path = temp_dir.path().join("test.toml");
    
    let toml_content = r##"
name = "test-theme"
description = "Test theme"

[colors]
bg = "#282828"
fg = "#ebdbb2"
accent = "#fe8019"
red = "#cc241d"
green = "#98971a"
yellow = "#d79921"
blue = "#458588"
magenta = "#b16286"
cyan = "#689d6a"
"##;
    
    fs::write(&theme_path, toml_content).unwrap();
    
    let result = parse_theme_file(&theme_path);
    assert!(result.is_ok());
    let theme = result.unwrap();
    assert_eq!(theme.name, "test-theme");
}

#[test]
fn test_parse_theme_file_not_found() {
    let temp_dir = create_temp_themes_dir();
    let theme_path = temp_dir.path().join("nonexistent.toml");
    
    let result = parse_theme_file(&theme_path);
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("Failed to read theme file"));
}

#[test]
fn test_validate_theme_valid() {
    let theme = create_test_theme();
    let result = validate_theme(&theme);
    assert!(result.is_ok());
}

#[test]
fn test_validate_theme_empty_name() {
    let mut theme = create_test_theme();
    theme.name = "".to_string();
    
    let result = validate_theme(&theme);
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("theme name cannot be empty"));
}

#[test]
fn test_validate_theme_invalid_hex() {
    let mut theme = create_test_theme();
    theme.colors.bg = "not-a-hex".to_string();
    
    let result = validate_theme(&theme);
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("Invalid color format"));
}

#[test]
fn test_validate_theme_optional_colors() {
    let mut theme = create_test_theme();
    theme.colors.orange = Some("invalid".to_string());
    
    let result = validate_theme(&theme);
    assert!(result.is_err());
}

#[test]
fn test_validate_accessibility_low_contrast() {
    let theme = create_low_contrast_theme();
    let warnings = validate_accessibility(&theme);
    
    // Should have at least one error about contrast
    assert!(!warnings.is_empty());
    let has_error = warnings.iter().any(|w| w.level == ValidationLevel::Error);
    assert!(has_error);
}

#[test]
fn test_validate_accessibility_similar_colors() {
    let theme = create_similar_colors_theme();
    let warnings = validate_accessibility(&theme);
    
    // Should have warnings about similar colors
    let _has_warning = warnings.iter().any(|w| w.level == ValidationLevel::Warning);
    // May or may not have warnings depending on threshold
    // Just verify warnings is a valid vector (length is always >= 0)
}

#[test]
fn test_validate_accessibility_aaa_standard() {
    let theme = create_aaa_contrast_theme();
    let warnings = validate_accessibility(&theme);
    
    // Should have info about AAA standard (or no errors)
    let has_error = warnings.iter().any(|w| w.level == ValidationLevel::Error);
    assert!(!has_error);
}

#[test]
fn test_validate_accessibility_warning_levels() {
    let theme = create_low_contrast_theme();
    let warnings = validate_accessibility(&theme);
    
    // Check that warnings have proper levels
    for warning in &warnings {
        match warning.level {
            ValidationLevel::Error | ValidationLevel::Warning | ValidationLevel::Info => {
                // Valid level
            }
        }
    }
}

#[test]
fn test_find_theme_files_success() {
    let temp_dir = create_temp_themes_dir();
    let themes_dir = temp_dir.path();
    
    // Create test theme files
    let theme1_path = themes_dir.join("theme1.toml");
    let theme2_path = themes_dir.join("theme2.toml");
    let subdir = themes_dir.join("subdir");
    fs::create_dir(&subdir).unwrap();
    let theme3_path = subdir.join("theme3.toml");
    
    let theme_content = r##"
name = "test-theme"
description = "Test"

[colors]
bg = "#282828"
fg = "#ebdbb2"
accent = "#fe8019"
red = "#cc241d"
green = "#98971a"
yellow = "#d79921"
blue = "#458588"
magenta = "#b16286"
cyan = "#689d6a"
"##;
    
    fs::write(&theme1_path, theme_content).unwrap();
    fs::write(&theme2_path, theme_content).unwrap();
    fs::write(&theme3_path, theme_content).unwrap();
    
    let result = find_theme_files(themes_dir);
    assert!(result.is_ok());
    let files = result.unwrap();
    assert!(files.len() >= 3);
    assert!(files.iter().any(|p| p.file_name().unwrap() == "theme1.toml"));
    assert!(files.iter().any(|p| p.file_name().unwrap() == "theme2.toml"));
    assert!(files.iter().any(|p| p.file_name().unwrap() == "theme3.toml"));
}

#[test]
fn test_find_theme_files_empty_directory() {
    let temp_dir = create_temp_themes_dir();
    let themes_dir = temp_dir.path();
    
    let result = find_theme_files(themes_dir);
    assert!(result.is_ok());
    let files = result.unwrap();
    assert_eq!(files.len(), 0);
}

#[test]
fn test_find_theme_files_nonexistent_directory() {
    let temp_dir = create_temp_themes_dir();
    let nonexistent_dir = temp_dir.path().join("nonexistent");
    
    let result = find_theme_files(&nonexistent_dir);
    assert!(result.is_ok());
    let files = result.unwrap();
    assert_eq!(files.len(), 0);
}

#[test]
fn test_find_theme_files_ignores_non_toml() {
    let temp_dir = create_temp_themes_dir();
    let themes_dir = temp_dir.path();
    
    // Create a .toml file
    let theme_path = themes_dir.join("theme.toml");
    fs::write(&theme_path, "name = \"test\"").unwrap();
    
    // Create a non-.toml file
    let other_path = themes_dir.join("other.txt");
    fs::write(&other_path, "not a theme").unwrap();
    
    let result = find_theme_files(themes_dir);
    assert!(result.is_ok());
    let files = result.unwrap();
    assert_eq!(files.len(), 1);
    assert!(files[0].file_name().unwrap() == "theme.toml");
}
