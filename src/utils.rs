use regex::Regex;
use crate::theme::Theme;

pub fn hex_to_rgb(hex: &str) -> Option<(u8, u8, u8)> {
    let hex = hex.trim_start_matches('#');
    if hex.len() != 6 {
        return None;
    }
    
    let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
    let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
    let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
    
    Some((r, g, b))
}

pub fn rgb_to_hex(r: u8, g: u8, b: u8) -> String {
    format!("#{:02x}{:02x}{:02x}", r, g, b)
}

pub fn lighten_color(hex: &str, factor: f32) -> Option<String> {
    let (r, g, b) = hex_to_rgb(hex)?;
    let r = ((r as f32) + (255.0 - r as f32) * factor).min(255.0) as u8;
    let g = ((g as f32) + (255.0 - g as f32) * factor).min(255.0) as u8;
    let b = ((b as f32) + (255.0 - b as f32) * factor).min(255.0) as u8;
    Some(rgb_to_hex(r, g, b))
}

pub fn darken_color(hex: &str, factor: f32) -> Option<String> {
    let (r, g, b) = hex_to_rgb(hex)?;
    let r = ((r as f32) * (1.0 - factor)).max(0.0) as u8;
    let g = ((g as f32) * (1.0 - factor)).max(0.0) as u8;
    let b = ((b as f32) * (1.0 - factor)).max(0.0) as u8;
    Some(rgb_to_hex(r, g, b))
}

pub fn dim_color(hex: &str, factor: f32) -> Option<String> {
    let (r, g, b) = hex_to_rgb(hex)?;
    let r = ((r as f32) * factor).min(255.0) as u8;
    let g = ((g as f32) * factor).min(255.0) as u8;
    let b = ((b as f32) * factor).min(255.0) as u8;
    Some(rgb_to_hex(r, g, b))
}

pub fn validate_hex_color(color: &str) -> bool {
    let hex_regex = Regex::new(r"^#?[0-9A-Fa-f]{6}$").unwrap();
    hex_regex.is_match(color)
}

pub fn normalize_hex(hex: &str) -> String {
    if hex.starts_with('#') {
        hex.to_string()
    } else {
        format!("#{}", hex)
    }
}

/// Contrast level requirements (WCAG)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContrastLevel {
    /// WCAG AA standard (4.5:1 for normal text, 3:1 for large text)
    AA,
    /// WCAG AAA standard (7:1 for normal text, 4.5:1 for large text)
    AAA,
    /// WCAG AA for large text (3:1)
    AaLarge,
}

/// Calculate relative luminance of a color (WCAG formula)
/// Returns a value between 0.0 (black) and 1.0 (white)
pub fn calculate_luminance(r: u8, g: u8, b: u8) -> f64 {
    let r = r as f64 / 255.0;
    let g = g as f64 / 255.0;
    let b = b as f64 / 255.0;
    
    let r = if r <= 0.03928 { r / 12.92 } else { ((r + 0.055) / 1.055).powf(2.4) };
    let g = if g <= 0.03928 { g / 12.92 } else { ((g + 0.055) / 1.055).powf(2.4) };
    let b = if b <= 0.03928 { b / 12.92 } else { ((b + 0.055) / 1.055).powf(2.4) };
    
    0.2126 * r + 0.7152 * g + 0.0722 * b
}

/// Calculate contrast ratio between two colors (WCAG formula)
/// Returns a value between 1.0 (same color) and 21.0 (black on white)
pub fn calculate_contrast_ratio(color1: &str, color2: &str) -> Option<f64> {
    let (r1, g1, b1) = hex_to_rgb(color1)?;
    let (r2, g2, b2) = hex_to_rgb(color2)?;
    
    let l1 = calculate_luminance(r1, g1, b1);
    let l2 = calculate_luminance(r2, g2, b2);
    
    let lighter = l1.max(l2);
    let darker = l1.min(l2);
    
    Some((lighter + 0.05) / (darker + 0.05))
}

/// Check if two colors meet the specified contrast level
pub fn check_contrast(color1: &str, color2: &str, level: ContrastLevel) -> bool {
    if let Some(ratio) = calculate_contrast_ratio(color1, color2) {
        match level {
            ContrastLevel::AA => ratio >= 4.5,
            ContrastLevel::AAA => ratio >= 7.0,
            ContrastLevel::AaLarge => ratio >= 3.0,
        }
    } else {
        false
    }
}

/// Calculate Euclidean distance between two colors in RGB space
/// Returns a value between 0.0 (same color) and ~441.67 (max distance)
pub fn color_distance(color1: &str, color2: &str) -> Option<f64> {
    let (r1, g1, b1) = hex_to_rgb(color1)?;
    let (r2, g2, b2) = hex_to_rgb(color2)?;
    
    let dr = (r1 as f64 - r2 as f64).powi(2);
    let dg = (g1 as f64 - g2 as f64).powi(2);
    let db = (b1 as f64 - b2 as f64).powi(2);
    
    Some((dr + dg + db).sqrt())
}

/// Find similar colors in a theme
/// Returns a vector of tuples: (color1_name, color2_name, distance)
pub fn find_similar_colors(theme: &Theme, threshold: f64) -> Vec<(String, String, f64)> {
    let mut similar = Vec::new();
    
    // Collect all colors with their names
    let colors: Vec<(&str, &str)> = vec![
        ("bg", &theme.colors.bg),
        ("fg", &theme.colors.fg),
        ("accent", &theme.colors.accent),
        ("red", &theme.colors.red),
        ("green", &theme.colors.green),
        ("yellow", &theme.colors.yellow),
        ("blue", &theme.colors.blue),
        ("magenta", &theme.colors.magenta),
        ("cyan", &theme.colors.cyan),
    ];
    
    let mut optional_colors = Vec::new();
    if let Some(ref c) = theme.colors.orange {
        optional_colors.push(("orange", c.as_str()));
    }
    if let Some(ref c) = theme.colors.purple {
        optional_colors.push(("purple", c.as_str()));
    }
    if let Some(ref c) = theme.colors.pink {
        optional_colors.push(("pink", c.as_str()));
    }
    if let Some(ref c) = theme.colors.white {
        optional_colors.push(("white", c.as_str()));
    }
    if let Some(ref c) = theme.colors.black {
        optional_colors.push(("black", c.as_str()));
    }
    if let Some(ref c) = theme.colors.gray {
        optional_colors.push(("gray", c.as_str()));
    }
    
    // Compare all pairs
    let all_colors: Vec<(&str, &str)> = colors.into_iter()
        .chain(optional_colors.into_iter())
        .collect();
    
    for i in 0..all_colors.len() {
        for j in (i + 1)..all_colors.len() {
            if let Some(distance) = color_distance(all_colors[i].1, all_colors[j].1) {
                if distance < threshold {
                    similar.push((
                        all_colors[i].0.to_string(),
                        all_colors[j].0.to_string(),
                        distance,
                    ));
                }
            }
        }
    }
    
    similar.sort_by(|a, b| a.2.partial_cmp(&b.2).unwrap_or(std::cmp::Ordering::Equal));
    similar
}

/// Check if a color is light (luminance > 0.5)
pub fn is_light_color(hex: &str) -> bool {
    if let Some((r, g, b)) = hex_to_rgb(hex) {
        calculate_luminance(r, g, b) > 0.5
    } else {
        false
    }
}

/// Generate a variant theme by intelligently inverting colors
/// For dark→light: lightens bg, darkens fg, adjusts colors for readability
/// For light→dark: darkens bg, lightens fg, adjusts colors for readability
pub fn generate_variant(theme: &Theme, variant: &str) -> anyhow::Result<Theme> {
    let is_dark = variant == "dark";
    let is_light = variant == "light";
    
    if !is_dark && !is_light {
        anyhow::bail!(
            "Invalid variant: '{}'.\n\
            \n\
            Valid variants are:\n\
            - 'dark': Dark theme variant\n\
            - 'light': Light theme variant\n\
            \n\
            You provided: '{}'\n\
            \n\
            To fix: Use 'dark' or 'light' as the variant name.",
            variant, variant
        );
    }
    
    // Determine if source theme is dark or light
    let source_is_dark = !is_light_color(&theme.colors.bg);
    
    // If generating same variant type, return a copy
    if (is_dark && source_is_dark) || (is_light && !source_is_dark) {
        let mut new_theme = theme.clone();
        new_theme.variant = Some(variant.to_string());
        new_theme.name = format!("{}-{}", theme.base_name(), variant);
        return Ok(new_theme);
    }
    
    // Generate opposite variant
    let bg_factor = if is_dark { 0.85 } else { 0.75 };
    let fg_factor = if is_dark { 0.15 } else { 0.25 };
    let color_factor = if is_dark { 0.2 } else { 0.3 };
    
    let new_bg = if is_dark {
        darken_color(&theme.colors.bg, bg_factor)
            .unwrap_or_else(|| theme.colors.bg.clone())
    } else {
        lighten_color(&theme.colors.bg, bg_factor)
            .unwrap_or_else(|| theme.colors.bg.clone())
    };
    
    let new_fg = if is_dark {
        lighten_color(&theme.colors.fg, fg_factor)
            .unwrap_or_else(|| theme.colors.fg.clone())
    } else {
        darken_color(&theme.colors.fg, fg_factor)
            .unwrap_or_else(|| theme.colors.fg.clone())
    };
    
    // Adjust colors - for dark themes, make colors brighter; for light, make them darker
    let adjust_color = |color: &str| -> String {
        if is_dark {
            lighten_color(color, color_factor)
                .unwrap_or_else(|| color.to_string())
        } else {
            darken_color(color, color_factor)
                .unwrap_or_else(|| color.to_string())
        }
    };
    
    let new_theme = Theme {
        name: format!("{}-{}", theme.base_name(), variant),
        description: format!("{} ({})", theme.description, variant),
        variant: Some(variant.to_string()),
        colors: crate::theme::ColorPalette {
            bg: new_bg,
            fg: new_fg,
            accent: adjust_color(&theme.colors.accent),
            red: adjust_color(&theme.colors.red),
            green: adjust_color(&theme.colors.green),
            yellow: adjust_color(&theme.colors.yellow),
            blue: adjust_color(&theme.colors.blue),
            magenta: adjust_color(&theme.colors.magenta),
            cyan: adjust_color(&theme.colors.cyan),
            orange: theme.colors.orange.as_ref().map(|c| adjust_color(c)),
            purple: theme.colors.purple.as_ref().map(|c| adjust_color(c)),
            pink: theme.colors.pink.as_ref().map(|c| adjust_color(c)),
            white: theme.colors.white.as_ref().map(|c| adjust_color(c)),
            black: theme.colors.black.as_ref().map(|c| adjust_color(c)),
            gray: theme.colors.gray.as_ref().map(|c| adjust_color(c)),
        },
        properties: theme.properties.clone(),
    };
    
    Ok(new_theme)
}

/// Invert theme colors (simple inversion - generates opposite variant)
pub fn invert_theme_colors(theme: &Theme) -> anyhow::Result<Theme> {
    let current_variant = theme.get_variant();
    let target_variant = if current_variant.as_deref() == Some("dark") {
        "light"
    } else {
        "dark"
    };
    
    generate_variant(theme, target_variant)
}
