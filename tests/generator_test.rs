mod common;

use themectl::generators;
use common::*;

// Helper function to check if output contains a color
fn output_contains_color(output: &str, color: &str) -> bool {
    output.contains(color)
}

// Helper function to check if output contains theme name
fn output_contains_name(output: &str, name: &str) -> bool {
    output.contains(name)
}

#[test]
fn test_generate_kitty() {
    let theme = create_test_theme();
    let result = generators::generate(&theme, "kitty");
    
    assert!(result.is_ok());
    let output = result.unwrap();
    assert!(!output.is_empty());
    assert!(output_contains_name(&output, &theme.name));
    assert!(output_contains_color(&output, &theme.colors.bg));
    assert!(output_contains_color(&output, &theme.colors.fg));
    assert!(output_contains_color(&output, &theme.colors.accent));
}

#[test]
fn test_generate_kitty_with_optional_colors() {
    let theme = create_full_test_theme();
    let result = generators::generate(&theme, "kitty");
    
    assert!(result.is_ok());
    let output = result.unwrap();
    // Kitty uses black, gray, white if available
    if let Some(black) = &theme.colors.black {
        assert!(output_contains_color(&output, black));
    }
}

#[test]
fn test_generate_waybar() {
    let theme = create_test_theme();
    let result = generators::generate(&theme, "waybar");
    
    assert!(result.is_ok());
    let output = result.unwrap();
    assert!(!output.is_empty());
    assert!(output_contains_name(&output, &theme.name));
    assert!(output.contains("background-color:"));
    assert!(output_contains_color(&output, &theme.colors.bg));
    assert!(output_contains_color(&output, &theme.colors.fg));
}

#[test]
fn test_generate_neovim() {
    let theme = create_test_theme();
    let result = generators::generate(&theme, "neovim");
    
    assert!(result.is_ok());
    let output = result.unwrap();
    assert!(!output.is_empty());
    assert!(output_contains_name(&output, &theme.name));
    assert!(output.contains("local colors = {"));
    assert!(output_contains_color(&output, &theme.colors.bg));
    assert!(output_contains_color(&output, &theme.colors.fg));
}

#[test]
fn test_generate_neovim_with_optional_colors() {
    let theme = create_full_test_theme();
    let result = generators::generate(&theme, "neovim");
    
    assert!(result.is_ok());
    let output = result.unwrap();
    // Neovim uses all optional colors
    if let Some(orange) = &theme.colors.orange {
        assert!(output_contains_color(&output, orange));
    }
}

#[test]
fn test_generate_starship() {
    let theme = create_test_theme();
    let result = generators::generate(&theme, "starship");
    
    assert!(result.is_ok());
    let output = result.unwrap();
    assert!(!output.is_empty());
    assert!(output_contains_name(&output, &theme.name));
    assert!(output.contains("[palette]"));
    assert!(output_contains_color(&output, &theme.colors.yellow));
    assert!(output_contains_color(&output, &theme.colors.blue));
}

#[test]
fn test_generate_starship_with_optional_colors() {
    let theme = create_full_test_theme();
    let result = generators::generate(&theme, "starship");
    
    assert!(result.is_ok());
    let output = result.unwrap();
    // Starship uses orange if available
    if let Some(orange) = &theme.colors.orange {
        assert!(output_contains_color(&output, orange));
    }
}

#[test]
fn test_generate_mako() {
    let theme = create_test_theme();
    let result = generators::generate(&theme, "mako");
    
    assert!(result.is_ok());
    let output = result.unwrap();
    assert!(!output.is_empty());
    assert!(output_contains_name(&output, &theme.name));
    assert!(output_contains_color(&output, &theme.colors.bg));
    assert!(output_contains_color(&output, &theme.colors.fg));
}

#[test]
fn test_generate_hyprland() {
    let theme = create_test_theme();
    let result = generators::generate(&theme, "hyprland");
    
    assert!(result.is_ok());
    let output = result.unwrap();
    assert!(!output.is_empty());
    assert!(output_contains_name(&output, &theme.name));
    assert!(output.contains("col.active_border") || output.contains("rgba"));
    // Hyprland uses RGB format, not hex, so check for structure instead
    assert!(output.contains("col.inactive_border") || output.contains("rgba"));
}

#[test]
fn test_generate_hyprpaper() {
    let theme = create_test_theme();
    let result = generators::generate(&theme, "hyprpaper");
    
    assert!(result.is_ok());
    let output = result.unwrap();
    assert!(!output.is_empty());
    assert!(output_contains_name(&output, &theme.name));
}

#[test]
fn test_generate_wofi() {
    let theme = create_test_theme();
    let result = generators::generate(&theme, "wofi");
    
    assert!(result.is_ok());
    let output = result.unwrap();
    assert!(!output.is_empty());
    assert!(output_contains_name(&output, &theme.name));
    assert!(output.contains("background-color:"));
    assert!(output_contains_color(&output, &theme.colors.bg));
}

#[test]
fn test_generate_wlogout() {
    let theme = create_test_theme();
    let result = generators::generate(&theme, "wlogout");
    
    assert!(result.is_ok());
    let output = result.unwrap();
    assert!(!output.is_empty());
    assert!(output_contains_name(&output, &theme.name));
    assert!(output.contains("background-color:"));
}

#[test]
fn test_generate_fastfetch() {
    let theme = create_test_theme();
    let result = generators::generate(&theme, "fastfetch");
    
    assert!(result.is_ok());
    let output = result.unwrap();
    assert!(!output.is_empty());
    assert!(output_contains_name(&output, &theme.name));
    // Fastfetch uses JSON format
    assert!(output.contains("\"") || output.contains("{"));
}

#[test]
fn test_generate_fastfetch_with_optional_colors() {
    let theme = create_full_test_theme();
    let result = generators::generate(&theme, "fastfetch");
    
    assert!(result.is_ok());
    let output = result.unwrap();
    // Fastfetch uses orange, purple, gray if available
    if let Some(orange) = &theme.colors.orange {
        assert!(output_contains_color(&output, orange));
    }
}

#[test]
fn test_generate_nix() {
    let theme = create_test_theme();
    let result = generators::generate(&theme, "nix");
    
    assert!(result.is_ok());
    let output = result.unwrap();
    assert!(!output.is_empty());
    assert!(output_contains_name(&output, &theme.name));
    assert!(output.contains("bg =") || output.contains("{"));
    assert!(output_contains_color(&output, &theme.colors.bg));
}

#[test]
fn test_generate_nix_with_optional_colors() {
    let theme = create_full_test_theme();
    let result = generators::generate(&theme, "nix");
    
    assert!(result.is_ok());
    let output = result.unwrap();
    // Nix uses all optional colors
    if let Some(orange) = &theme.colors.orange {
        assert!(output_contains_color(&output, orange));
    }
}

#[test]
fn test_generate_yazi() {
    let theme = create_test_theme();
    let result = generators::generate(&theme, "yazi");
    
    assert!(result.is_ok());
    let output = result.unwrap();
    assert!(!output.is_empty());
    assert!(output_contains_name(&output, &theme.name));
    // Yazi uses TOML format
    assert!(output.contains("["));
    assert!(output_contains_color(&output, &theme.colors.bg));
}

#[test]
fn test_generate_yazi_with_optional_colors() {
    let theme = create_full_test_theme();
    let result = generators::generate(&theme, "yazi");
    
    assert!(result.is_ok());
    let output = result.unwrap();
    // Yazi uses orange, purple, gray, white, black if available
    if let Some(orange) = &theme.colors.orange {
        assert!(output_contains_color(&output, orange));
    }
}

#[test]
fn test_generate_unknown_format() {
    let theme = create_test_theme();
    let result = generators::generate(&theme, "unknown-format");
    
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("Unknown format"));
}

#[test]
fn test_generate_case_insensitive() {
    let theme = create_test_theme();
    
    // Test uppercase
    let result_upper = generators::generate(&theme, "KITTY");
    assert!(result_upper.is_ok());
    let output_upper = result_upper.unwrap();
    
    // Test mixed case
    let result_mixed = generators::generate(&theme, "KiTtY");
    assert!(result_mixed.is_ok());
    let output_mixed = result_mixed.unwrap();
    
    // Test lowercase (should work)
    let result_lower = generators::generate(&theme, "kitty");
    assert!(result_lower.is_ok());
    let output_lower = result_lower.unwrap();
    
    // All should produce same output
    assert_eq!(output_upper, output_mixed);
    assert_eq!(output_mixed, output_lower);
}

#[test]
fn test_generate_all() {
    let theme = create_test_theme();
    let result = generators::generate_all(&theme);
    
    assert!(result.is_ok());
    let all_formats = result.unwrap();
    
    // Should have all 15 formats
    assert_eq!(all_formats.len(), 15);
    
    // Check that all expected formats are present
    let format_names: Vec<String> = all_formats.iter().map(|(name, _)| name.clone()).collect();
    assert!(format_names.contains(&"kitty".to_string()));
    assert!(format_names.contains(&"waybar".to_string()));
    assert!(format_names.contains(&"neovim".to_string()));
    assert!(format_names.contains(&"starship".to_string()));
    assert!(format_names.contains(&"mako".to_string()));
    assert!(format_names.contains(&"hyprland".to_string()));
    assert!(format_names.contains(&"hyprpaper".to_string()));
    assert!(format_names.contains(&"wofi".to_string()));
    assert!(format_names.contains(&"wlogout".to_string()));
    assert!(format_names.contains(&"fastfetch".to_string()));
    assert!(format_names.contains(&"nix".to_string()));
    assert!(format_names.contains(&"yazi".to_string()));
    assert!(format_names.contains(&"gtk".to_string()));
    assert!(format_names.contains(&"btop".to_string()));
    assert!(format_names.contains(&"git".to_string()));
}

#[test]
fn test_generate_all_non_empty() {
    let theme = create_test_theme();
    let result = generators::generate_all(&theme);
    
    assert!(result.is_ok());
    let all_formats = result.unwrap();
    
    // All formats should have non-empty content
    for (format_name, content) in &all_formats {
        assert!(!content.is_empty(), "Format {} produced empty content", format_name);
    }
}

#[test]
fn test_generate_all_contains_theme_name() {
    let theme = create_test_theme();
    let result = generators::generate_all(&theme);
    
    assert!(result.is_ok());
    let all_formats = result.unwrap();
    
    // All formats should contain theme name
    for (format_name, content) in &all_formats {
        assert!(
            output_contains_name(content, &theme.name),
            "Format {} does not contain theme name",
            format_name
        );
    }
}

#[test]
fn test_generate_all_required_colors() {
    let theme = create_test_theme();
    let result = generators::generate_all(&theme);
    
    assert!(result.is_ok());
    let all_formats = result.unwrap();
    
    // Most formats should contain at least bg color
    let formats_with_bg: Vec<&String> = all_formats
        .iter()
        .filter(|(_, content)| output_contains_color(content, &theme.colors.bg))
        .map(|(name, _)| name)
        .collect();
    
    // At least some formats should contain bg
    assert!(!formats_with_bg.is_empty());
}

#[test]
fn test_generate_all_with_full_theme() {
    let theme = create_full_test_theme();
    let result = generators::generate_all(&theme);
    
    assert!(result.is_ok());
    let all_formats = result.unwrap();
    
    // Should still have all 15 formats
    assert_eq!(all_formats.len(), 15);
    
    // Formats that use optional colors should contain them
    if let Some(orange) = &theme.colors.orange {
        // Neovim, Starship, Fastfetch, Nix, Yazi use orange
        let formats_with_orange: Vec<&String> = all_formats
            .iter()
            .filter(|(name, content)| {
                output_contains_color(content, orange) && 
                (name == "neovim" || name == "starship" || name == "fastfetch" || name == "nix" || name == "yazi")
            })
            .map(|(name, _)| name)
            .collect();
        
        // At least some should use orange
        assert!(!formats_with_orange.is_empty());
    }
}
