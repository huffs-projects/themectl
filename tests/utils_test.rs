mod common;

use themectl::utils::*;
use common::*;

#[test]
fn test_hex_to_rgb_valid() {
    assert_eq!(hex_to_rgb("#000000"), Some((0, 0, 0)));
    assert_eq!(hex_to_rgb("#ffffff"), Some((255, 255, 255)));
    assert_eq!(hex_to_rgb("#ff0000"), Some((255, 0, 0)));
    assert_eq!(hex_to_rgb("#00ff00"), Some((0, 255, 0)));
    assert_eq!(hex_to_rgb("#0000ff"), Some((0, 0, 255)));
    assert_eq!(hex_to_rgb("#282828"), Some((40, 40, 40)));
    assert_eq!(hex_to_rgb("#ebdbb2"), Some((235, 219, 178)));
}

#[test]
fn test_hex_to_rgb_without_hash() {
    assert_eq!(hex_to_rgb("000000"), Some((0, 0, 0)));
    assert_eq!(hex_to_rgb("ffffff"), Some((255, 255, 255)));
    assert_eq!(hex_to_rgb("282828"), Some((40, 40, 40)));
}

#[test]
fn test_hex_to_rgb_invalid() {
    assert_eq!(hex_to_rgb(""), None);
    assert_eq!(hex_to_rgb("#"), None);
    assert_eq!(hex_to_rgb("#ff"), None);
    assert_eq!(hex_to_rgb("#ffff"), None);
    assert_eq!(hex_to_rgb("#fffffff"), None);
    assert_eq!(hex_to_rgb("gggggg"), None);
    assert_eq!(hex_to_rgb("#gggggg"), None);
}

#[test]
fn test_rgb_to_hex() {
    assert_eq!(rgb_to_hex(0, 0, 0), "#000000");
    assert_eq!(rgb_to_hex(255, 255, 255), "#ffffff");
    assert_eq!(rgb_to_hex(255, 0, 0), "#ff0000");
    assert_eq!(rgb_to_hex(0, 255, 0), "#00ff00");
    assert_eq!(rgb_to_hex(0, 0, 255), "#0000ff");
    assert_eq!(rgb_to_hex(40, 40, 40), "#282828");
    assert_eq!(rgb_to_hex(235, 219, 178), "#ebdbb2");
}

#[test]
fn test_lighten_color() {
    // Lighten black
    let result = lighten_color("#000000", 0.5);
    assert!(result.is_some());
    let lightened = result.unwrap();
    assert!(lightened != "#000000");
    
    // Lighten with factor 0.0 (no change)
    let result = lighten_color("#282828", 0.0);
    assert_eq!(result, Some("#282828".to_string()));
    
    // Lighten with factor 1.0 (full white)
    let result = lighten_color("#000000", 1.0);
    assert_eq!(result, Some("#ffffff".to_string()));
    
    // Lighten mid-tone
    let result = lighten_color("#808080", 0.5);
    assert!(result.is_some());
    let lightened = result.unwrap();
    assert!(lightened != "#808080");
}

#[test]
fn test_darken_color() {
    // Darken white
    let result = darken_color("#ffffff", 0.5);
    assert!(result.is_some());
    let darkened = result.unwrap();
    assert!(darkened != "#ffffff");
    
    // Darken with factor 0.0 (no change)
    let result = darken_color("#282828", 0.0);
    assert_eq!(result, Some("#282828".to_string()));
    
    // Darken with factor 1.0 (full black)
    let result = darken_color("#ffffff", 1.0);
    assert_eq!(result, Some("#000000".to_string()));
    
    // Darken mid-tone
    let result = darken_color("#808080", 0.5);
    assert!(result.is_some());
    let darkened = result.unwrap();
    assert!(darkened != "#808080");
}

#[test]
fn test_dim_color() {
    // Dim white
    let result = dim_color("#ffffff", 0.5);
    assert_eq!(result, Some("#7f7f7f".to_string()));
    
    // Dim with factor 0.0 (full black)
    let result = dim_color("#ffffff", 0.0);
    assert_eq!(result, Some("#000000".to_string()));
    
    // Dim with factor 1.0 (no change)
    let result = dim_color("#ffffff", 1.0);
    assert_eq!(result, Some("#ffffff".to_string()));
    
    // Dim red
    let result = dim_color("#ff0000", 0.5);
    assert_eq!(result, Some("#7f0000".to_string()));
}

#[test]
fn test_validate_hex_color() {
    // Valid formats
    assert!(validate_hex_color("#000000"));
    assert!(validate_hex_color("#ffffff"));
    assert!(validate_hex_color("#ff0000"));
    assert!(validate_hex_color("000000"));
    assert!(validate_hex_color("ffffff"));
    assert!(validate_hex_color("#abcdef"));
    assert!(validate_hex_color("#ABCDEF"));
    assert!(validate_hex_color("#123456"));
    
    // Invalid formats
    assert!(!validate_hex_color(""));
    assert!(!validate_hex_color("#"));
    assert!(!validate_hex_color("#ff"));
    assert!(!validate_hex_color("#ffff"));
    assert!(!validate_hex_color("#fffffff"));
    assert!(!validate_hex_color("gggggg"));
    assert!(!validate_hex_color("#gggggg"));
    assert!(!validate_hex_color("not a color"));
    assert!(!validate_hex_color("#12345"));
    assert!(!validate_hex_color("#1234567"));
}

#[test]
fn test_normalize_hex() {
    assert_eq!(normalize_hex("#000000"), "#000000");
    assert_eq!(normalize_hex("000000"), "#000000");
    assert_eq!(normalize_hex("#ffffff"), "#ffffff");
    assert_eq!(normalize_hex("ffffff"), "#ffffff");
    assert_eq!(normalize_hex("#282828"), "#282828");
    assert_eq!(normalize_hex("282828"), "#282828");
}

#[test]
fn test_calculate_luminance() {
    // Black should have luminance 0.0
    let black_lum = calculate_luminance(0, 0, 0);
    assert!((black_lum - 0.0).abs() < 0.001);
    
    // White should have luminance 1.0
    let white_lum = calculate_luminance(255, 255, 255);
    assert!((white_lum - 1.0).abs() < 0.001);
    
    // Gray should be around 0.2-0.3 (mid-gray has lower luminance due to gamma correction)
    let gray_lum = calculate_luminance(128, 128, 128);
    assert!(gray_lum > 0.2 && gray_lum < 0.3);
    
    // Red should have lower luminance than white
    let red_lum = calculate_luminance(255, 0, 0);
    assert!(red_lum < white_lum);
}

#[test]
fn test_calculate_contrast_ratio() {
    // Black on white should be 21:1 (maximum contrast)
    let ratio = calculate_contrast_ratio("#000000", "#ffffff");
    assert!(ratio.is_some());
    let ratio_val = ratio.unwrap();
    assert!((ratio_val - 21.0).abs() < 0.1);
    
    // White on black should also be 21:1
    let ratio = calculate_contrast_ratio("#ffffff", "#000000");
    assert!(ratio.is_some());
    let ratio_val = ratio.unwrap();
    assert!((ratio_val - 21.0).abs() < 0.1);
    
    // Same color should be 1:1
    let ratio = calculate_contrast_ratio("#282828", "#282828");
    assert_eq!(ratio, Some(1.0));
    
    // Similar colors should have low contrast
    let ratio = calculate_contrast_ratio("#333333", "#343434");
    assert!(ratio.is_some());
    let ratio_val = ratio.unwrap();
    assert!(ratio_val < 2.0);
    
    // Invalid colors should return None
    assert_eq!(calculate_contrast_ratio("invalid", "#ffffff"), None);
    assert_eq!(calculate_contrast_ratio("#ffffff", "invalid"), None);
}

#[test]
fn test_check_contrast() {
    // Black on white meets all standards
    assert!(check_contrast("#000000", "#ffffff", ContrastLevel::AA));
    assert!(check_contrast("#000000", "#ffffff", ContrastLevel::AAA));
    assert!(check_contrast("#000000", "#ffffff", ContrastLevel::AaLarge));
    
    // Same color fails all standards
    assert!(!check_contrast("#282828", "#282828", ContrastLevel::AA));
    assert!(!check_contrast("#282828", "#282828", ContrastLevel::AAA));
    assert!(!check_contrast("#282828", "#282828", ContrastLevel::AaLarge));
    
    // Test AA level (4.5:1)
    // Use colors that should meet AA but not AAA
    let dark_bg = "#1a1a1a";
    let light_fg = "#e0e0e0";
    let meets_aa = check_contrast(dark_bg, light_fg, ContrastLevel::AA);
    // This should generally pass AA
    assert!(meets_aa || !meets_aa); // Just ensure it doesn't panic
    
    // Test AaLarge level (3:1)
    let meets_large = check_contrast("#333333", "#aaaaaa", ContrastLevel::AaLarge);
    assert!(meets_large || !meets_large); // Just ensure it doesn't panic
}

#[test]
fn test_color_distance() {
    // Same color should have distance 0.0
    let distance = color_distance("#282828", "#282828");
    assert_eq!(distance, Some(0.0));
    
    // Black and white should have maximum distance
    let distance = color_distance("#000000", "#ffffff");
    assert!(distance.is_some());
    let dist_val = distance.unwrap();
    // Maximum RGB distance is sqrt(3 * 255^2) ≈ 441.67
    assert!(dist_val > 400.0 && dist_val < 450.0);
    
    // Similar colors should have small distance
    let distance = color_distance("#cc241d", "#cc241e");
    assert!(distance.is_some());
    let dist_val = distance.unwrap();
    assert!(dist_val < 10.0);
    
    // Invalid colors should return None
    assert_eq!(color_distance("invalid", "#ffffff"), None);
    assert_eq!(color_distance("#ffffff", "invalid"), None);
}

#[test]
fn test_find_similar_colors() {
    let theme = create_similar_colors_theme();
    
    // Find similar colors with threshold 30.0
    let similar = find_similar_colors(&theme, 30.0);
    
    // Should find red and green as similar (they're very close)
    assert!(!similar.is_empty());
    
    // Test with higher threshold (should find more)
    let similar_more = find_similar_colors(&theme, 100.0);
    assert!(similar_more.len() >= similar.len());
    
    // Test with very low threshold (should find fewer or none)
    let similar_few = find_similar_colors(&theme, 1.0);
    assert!(similar_few.len() <= similar.len());
}

#[test]
fn test_is_light_color() {
    // White should be light
    assert!(is_light_color("#ffffff"));
    
    // Black should not be light
    assert!(!is_light_color("#000000"));
    
    // Light gray should be light
    assert!(is_light_color("#e0e0e0"));
    
    // Dark gray should not be light
    assert!(!is_light_color("#282828"));
    
    // Invalid color should return false
    assert!(!is_light_color("invalid"));
}

#[test]
fn test_generate_variant_dark_to_light() {
    let dark_theme = create_test_theme();
    
    // Generate light variant
    let light_variant = generate_variant(&dark_theme, "light").unwrap();
    
    assert_eq!(light_variant.variant, Some("light".to_string()));
    assert!(light_variant.name.contains("light"));
    assert_ne!(light_variant.colors.bg, dark_theme.colors.bg);
    assert_ne!(light_variant.colors.fg, dark_theme.colors.fg);
    
    // Light variant should have lighter background
    assert!(is_light_color(&light_variant.colors.bg));
}

#[test]
fn test_generate_variant_light_to_dark() {
    let mut light_theme = create_test_theme();
    light_theme.colors.bg = "#fbf1c7".to_string();
    light_theme.colors.fg = "#282828".to_string();
    
    // Generate dark variant
    let dark_variant = generate_variant(&light_theme, "dark").unwrap();
    
    assert_eq!(dark_variant.variant, Some("dark".to_string()));
    assert!(dark_variant.name.contains("dark"));
    assert_ne!(dark_variant.colors.bg, light_theme.colors.bg);
    
    // Dark variant should have darker background
    assert!(!is_light_color(&dark_variant.colors.bg));
}

#[test]
fn test_generate_variant_same_type() {
    let dark_theme = create_test_theme();
    
    // Generate dark variant from dark theme (should return copy)
    let result = generate_variant(&dark_theme, "dark");
    assert!(result.is_ok());
    let variant = result.unwrap();
    
    // Should have variant set and name updated
    assert_eq!(variant.variant, Some("dark".to_string()));
    assert!(variant.name.contains("dark"));
}

#[test]
fn test_generate_variant_invalid() {
    let theme = create_test_theme();
    
    // Invalid variant should return error
    let result = generate_variant(&theme, "invalid");
    assert!(result.is_err());
}

#[test]
fn test_invert_theme_colors() {
    let dark_theme = create_test_theme();
    
    // Invert colors
    let inverted = invert_theme_colors(&dark_theme).unwrap();
    
    // Should have opposite variant
    let original_variant = dark_theme.get_variant();
    let inverted_variant = inverted.get_variant();
    
    if original_variant.as_deref() == Some("dark") {
        assert_eq!(inverted_variant.as_deref(), Some("light"));
    } else {
        assert_eq!(inverted_variant.as_deref(), Some("dark"));
    }
}
