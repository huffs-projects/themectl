use anyhow::Result;
use crate::theme::Theme;
use crate::utils::lighten_color;

pub fn generate(theme: &Theme) -> Result<String> {
    let mut output = String::new();
    
    output.push_str("# Kitty theme: ");
    output.push_str(&theme.name);
    output.push_str("\n\n");
    
    // Background and foreground
    output.push_str("# Background\n");
    output.push_str("background ");
    output.push_str(&theme.colors.bg);
    output.push_str("\n");
    output.push_str("foreground ");
    output.push_str(&theme.colors.fg);
    output.push_str("\n\n");
    
    // Color palette (16 colors)
    let colors = generate_color_palette(theme)?;
    output.push_str("# Color palette\n");
    for (i, color) in colors.iter().enumerate() {
        output.push_str(&format!("color{} {}\n", i, color));
    }
    output.push_str("\n");
    
    // Cursor
    output.push_str("# Cursor\n");
    output.push_str("cursor ");
    output.push_str(&theme.colors.accent);
    output.push_str("\n");
    output.push_str("cursor_text_color ");
    output.push_str(&theme.colors.bg);
    output.push_str("\n\n");
    
    // Selection
    output.push_str("# Selection\n");
    output.push_str("selection_background ");
    output.push_str(&theme.colors.accent);
    output.push_str("\n");
    output.push_str("selection_foreground ");
    output.push_str(&theme.colors.bg);
    output.push_str("\n\n");
    
    // Window borders
    output.push_str("# Window borders\n");
    output.push_str("active_border_color ");
    output.push_str(&theme.colors.accent);
    output.push_str("\n");
    output.push_str("inactive_border_color ");
    output.push_str(&theme.colors.bg);
    output.push_str("\n\n");
    
    // Tab bar
    output.push_str("# Tab bar\n");
    output.push_str("tab_bar_background ");
    output.push_str(&theme.colors.bg);
    output.push_str("\n");
    output.push_str("tab_bar_margin_color ");
    output.push_str(&theme.colors.bg);
    output.push_str("\n");
    output.push_str("active_tab_background ");
    output.push_str(&theme.colors.accent);
    output.push_str("\n");
    output.push_str("active_tab_foreground ");
    output.push_str(&theme.colors.bg);
    output.push_str("\n");
    output.push_str("inactive_tab_background ");
    output.push_str(&theme.colors.bg);
    output.push_str("\n");
    output.push_str("inactive_tab_foreground ");
    output.push_str(&theme.colors.fg);
    output.push_str("\n\n");
    
    // Bell and URL
    output.push_str("# Bell and URL\n");
    output.push_str("bell_border_color ");
    output.push_str(&theme.colors.yellow);
    output.push_str("\n");
    output.push_str("url_color ");
    output.push_str(&theme.colors.cyan);
    output.push_str("\n");
    
    Ok(output)
}

fn generate_color_palette(theme: &Theme) -> Result<Vec<String>> {
    let mut colors = Vec::new();
    
    // color0, color8: Background (black/bright black)
    if let Some(black) = theme.get_color("black") {
        colors.push(black.to_string());
    } else {
        colors.push(theme.colors.bg.clone());
    }
    if let Some(gray) = theme.get_color("gray") {
        colors.push(gray.to_string());
    } else if let Some(lightened) = lighten_color(&theme.colors.bg, 0.1) {
        colors.push(lightened);
    } else {
        colors.push(theme.colors.bg.clone());
    }
    
    // color1, color9: Red
    colors.push(theme.colors.red.clone());
    if let Some(lightened) = lighten_color(&theme.colors.red, 0.2) {
        colors.push(lightened);
    } else {
        colors.push(theme.colors.red.clone());
    }
    
    // color2, color10: Green
    colors.push(theme.colors.green.clone());
    if let Some(lightened) = lighten_color(&theme.colors.green, 0.2) {
        colors.push(lightened);
    } else {
        colors.push(theme.colors.green.clone());
    }
    
    // color3, color11: Yellow
    colors.push(theme.colors.yellow.clone());
    if let Some(lightened) = lighten_color(&theme.colors.yellow, 0.2) {
        colors.push(lightened);
    } else {
        colors.push(theme.colors.yellow.clone());
    }
    
    // color4, color12: Blue
    colors.push(theme.colors.blue.clone());
    if let Some(lightened) = lighten_color(&theme.colors.blue, 0.2) {
        colors.push(lightened);
    } else {
        colors.push(theme.colors.blue.clone());
    }
    
    // color5, color13: Magenta
    colors.push(theme.colors.magenta.clone());
    if let Some(lightened) = lighten_color(&theme.colors.magenta, 0.2) {
        colors.push(lightened);
    } else {
        colors.push(theme.colors.magenta.clone());
    }
    
    // color6, color14: Cyan
    colors.push(theme.colors.cyan.clone());
    if let Some(lightened) = lighten_color(&theme.colors.cyan, 0.2) {
        colors.push(lightened);
    } else {
        colors.push(theme.colors.cyan.clone());
    }
    
    // color7, color15: Foreground (white/bright white)
    colors.push(theme.colors.fg.clone());
    if let Some(white) = theme.get_color("white") {
        colors.push(white.to_string());
    } else if let Some(lightened) = lighten_color(&theme.colors.fg, 0.2) {
        colors.push(lightened);
    } else {
        colors.push(theme.colors.fg.clone());
    }
    
    Ok(colors)
}
