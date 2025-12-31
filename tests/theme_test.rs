mod common;

use themectl::theme::Theme;
use common::*;

#[test]
fn test_get_color_required() {
    let theme = create_test_theme();
    
    assert_eq!(theme.get_color("bg"), Some("#282828"));
    assert_eq!(theme.get_color("fg"), Some("#ebdbb2"));
    assert_eq!(theme.get_color("accent"), Some("#fe8019"));
    assert_eq!(theme.get_color("red"), Some("#cc241d"));
    assert_eq!(theme.get_color("green"), Some("#98971a"));
    assert_eq!(theme.get_color("yellow"), Some("#d79921"));
    assert_eq!(theme.get_color("blue"), Some("#458588"));
    assert_eq!(theme.get_color("magenta"), Some("#b16286"));
    assert_eq!(theme.get_color("cyan"), Some("#689d6a"));
}

#[test]
fn test_get_color_optional() {
    let theme = create_full_test_theme();
    
    assert_eq!(theme.get_color("orange"), Some("#d65d0e"));
    assert_eq!(theme.get_color("purple"), Some("#b16286"));
    assert_eq!(theme.get_color("pink"), Some("#d3869b"));
    assert_eq!(theme.get_color("white"), Some("#fbf1c7"));
    assert_eq!(theme.get_color("black"), Some("#1d2021"));
    assert_eq!(theme.get_color("gray"), Some("#928374"));
}

#[test]
fn test_get_color_optional_missing() {
    let theme = create_test_theme();
    
    assert_eq!(theme.get_color("orange"), None);
    assert_eq!(theme.get_color("purple"), None);
    assert_eq!(theme.get_color("pink"), None);
    assert_eq!(theme.get_color("white"), None);
    assert_eq!(theme.get_color("black"), None);
    assert_eq!(theme.get_color("gray"), None);
}

#[test]
fn test_get_color_invalid() {
    let theme = create_test_theme();
    
    assert_eq!(theme.get_color("invalid"), None);
    assert_eq!(theme.get_color(""), None);
    assert_eq!(theme.get_color("background"), None);
    assert_eq!(theme.get_color("foreground"), None);
}

#[test]
fn test_base_name_with_variants() {
    let mut theme = create_test_theme();
    
    theme.name = "gruvbox-dark".to_string();
    assert_eq!(theme.base_name(), "gruvbox");
    
    theme.name = "gruvbox-light".to_string();
    assert_eq!(theme.base_name(), "gruvbox");
    
    theme.name = "gruvbox-darkest".to_string();
    assert_eq!(theme.base_name(), "gruvbox");
    
    theme.name = "gruvbox-lightest".to_string();
    assert_eq!(theme.base_name(), "gruvbox");
}

#[test]
fn test_base_name_without_variant() {
    let mut theme = create_test_theme();
    
    theme.name = "gruvbox".to_string();
    assert_eq!(theme.base_name(), "gruvbox");
    
    theme.name = "test-theme".to_string();
    assert_eq!(theme.base_name(), "test-theme");
}

#[test]
fn test_base_name_edge_cases() {
    let mut theme = create_test_theme();
    
    theme.name = "dark".to_string();
    assert_eq!(theme.base_name(), "dark");
    
    theme.name = "light".to_string();
    assert_eq!(theme.base_name(), "light");
    
    theme.name = "theme-dark-theme".to_string();
    assert_eq!(theme.base_name(), "theme-dark-theme");
}

#[test]
fn test_extract_base_name() {
    assert_eq!(Theme::extract_base_name("gruvbox-dark"), "gruvbox");
    assert_eq!(Theme::extract_base_name("gruvbox-light"), "gruvbox");
    assert_eq!(Theme::extract_base_name("gruvbox-darkest"), "gruvbox");
    assert_eq!(Theme::extract_base_name("gruvbox-lightest"), "gruvbox");
    assert_eq!(Theme::extract_base_name("gruvbox"), "gruvbox");
    assert_eq!(Theme::extract_base_name("test"), "test");
}

#[test]
fn test_detect_variant_from_name() {
    assert_eq!(Theme::detect_variant_from_name("gruvbox-dark"), Some("dark".to_string()));
    assert_eq!(Theme::detect_variant_from_name("gruvbox-light"), Some("light".to_string()));
    assert_eq!(Theme::detect_variant_from_name("gruvbox-darkest"), Some("dark".to_string()));
    assert_eq!(Theme::detect_variant_from_name("gruvbox-lightest"), Some("light".to_string()));
    assert_eq!(Theme::detect_variant_from_name("gruvbox-DARK"), Some("dark".to_string()));
    assert_eq!(Theme::detect_variant_from_name("gruvbox-LIGHT"), Some("light".to_string()));
    assert_eq!(Theme::detect_variant_from_name("gruvbox"), None);
    assert_eq!(Theme::detect_variant_from_name("test-theme"), None);
}

#[test]
fn test_get_variant_from_field() {
    let mut theme = create_test_theme();
    theme.variant = Some("dark".to_string());
    
    assert_eq!(theme.get_variant(), Some("dark".to_string()));
}

#[test]
fn test_get_variant_from_name() {
    let mut theme = create_test_theme();
    theme.variant = None;
    theme.name = "gruvbox-dark".to_string();
    
    assert_eq!(theme.get_variant(), Some("dark".to_string()));
}

#[test]
fn test_get_variant_field_overrides_name() {
    let mut theme = create_test_theme();
    theme.variant = Some("light".to_string());
    theme.name = "gruvbox-dark".to_string();
    
    // Field should take precedence
    assert_eq!(theme.get_variant(), Some("light".to_string()));
}

#[test]
fn test_get_variant_none() {
    let mut theme = create_test_theme();
    theme.variant = None;
    theme.name = "gruvbox".to_string();
    
    assert_eq!(theme.get_variant(), None);
}

#[test]
fn test_full_name_with_variant() {
    let mut theme = create_test_theme();
    theme.variant = Some("dark".to_string());
    theme.name = "gruvbox".to_string();
    
    assert_eq!(theme.full_name(), "gruvbox-dark");
}

#[test]
fn test_full_name_without_variant() {
    let mut theme = create_test_theme();
    theme.variant = None;
    theme.name = "gruvbox".to_string();
    
    assert_eq!(theme.full_name(), "gruvbox");
}

#[test]
fn test_full_name_variant_from_name() {
    let mut theme = create_test_theme();
    theme.variant = None;
    theme.name = "gruvbox-dark".to_string();
    
    // Should use variant detected from name
    assert_eq!(theme.full_name(), "gruvbox-dark");
}
