use anyhow::Result;
use crate::theme::Theme;
use crate::utils::{hex_to_rgb, darken_color, lighten_color};

pub fn generate(theme: &Theme) -> Result<String> {
    // Determine if this is a dark or light theme
    let is_dark = theme.get_variant()
        .map(|v| v == "dark")
        .unwrap_or_else(|| {
            // Heuristic: if background is darker than foreground, it's dark
            if let (Some((bg_r, bg_g, bg_b)), Some((fg_r, fg_g, fg_b))) = 
                (hex_to_rgb(&theme.colors.bg), hex_to_rgb(&theme.colors.fg)) {
                let bg_luma = (bg_r as f32 * 0.299 + bg_g as f32 * 0.587 + bg_b as f32 * 0.114) / 255.0;
                let fg_luma = (fg_r as f32 * 0.299 + fg_g as f32 * 0.587 + fg_b as f32 * 0.114) / 255.0;
                bg_luma < fg_luma
            } else {
                true // Default to dark
            }
        });

    let mut output = String::new();
    
    // Generate settings.ini
    output.push_str("# GTK theme configuration: ");
    output.push_str(&theme.name);
    output.push_str("\n");
    output.push_str("# Place this file at: ~/.config/gtk-4.0/settings.ini\n\n");
    
    output.push_str("[Settings]\n");
    output.push_str(&format!("gtk-application-prefer-dark-theme={}\n", is_dark));
    output.push_str("gtk-theme-name=Adwaita");
    if is_dark {
        output.push_str("-dark");
    }
    output.push_str("\n");
    output.push_str("gtk-icon-theme-name=Adwaita\n");
    output.push_str("gtk-cursor-theme-name=Adwaita\n");
    output.push_str("gtk-cursor-theme-size=24\n");
    
    // Add theme name comment
    output.push_str("\n# Theme: ");
    output.push_str(&theme.name);
    let desc_preview: String = theme.description.chars().take(60).collect();
    if !desc_preview.is_empty() {
        output.push_str(" - ");
        output.push_str(&desc_preview);
    }
    output.push_str("\n");
    
    Ok(output)
}

/// Generate GTK CSS customization file
pub fn generate_css(theme: &Theme) -> Result<String> {
    let mut output = String::new();
    
    let border_radius = theme.properties.border_radius.unwrap_or(6);
    let border_width = theme.properties.border_width.unwrap_or(1);
    let shadow_blur = theme.properties.shadow_blur.unwrap_or(4);
    
    output.push_str("/* GTK CSS theme: ");
    output.push_str(&theme.name);
    output.push_str(" */\n");
    output.push_str("/* Place this file at: ~/.config/gtk-4.0/gtk.css */\n\n");
    
    // Define CSS variables for theme colors
    output.push_str("@define-color theme_bg_color ");
    output.push_str(&theme.colors.bg);
    output.push_str(";\n");
    
    output.push_str("@define-color theme_fg_color ");
    output.push_str(&theme.colors.fg);
    output.push_str(";\n");
    
    output.push_str("@define-color theme_selected_bg_color ");
    output.push_str(&theme.colors.accent);
    output.push_str(";\n");
    
    output.push_str("@define-color theme_selected_fg_color ");
    output.push_str(&theme.colors.bg);
    output.push_str(";\n");
    
    // Generate hover/active colors
    if let Some(hover_bg) = lighten_color(&theme.colors.bg, 0.1) {
        output.push_str("@define-color theme_hover_bg_color ");
        output.push_str(&hover_bg);
        output.push_str(";\n");
    }
    
    if let Some(active_bg) = darken_color(&theme.colors.bg, 0.1) {
        output.push_str("@define-color theme_active_bg_color ");
        output.push_str(&active_bg);
        output.push_str(";\n");
    }
    
    // Accent colors
    output.push_str("@define-color accent_color ");
    output.push_str(&theme.colors.accent);
    output.push_str(";\n");
    
    if let Some(accent_hover) = lighten_color(&theme.colors.accent, 0.1) {
        output.push_str("@define-color accent_hover_color ");
        output.push_str(&accent_hover);
        output.push_str(";\n");
    }
    
    // Error, warning, success colors
    output.push_str("@define-color error_color ");
    output.push_str(&theme.colors.red);
    output.push_str(";\n");
    
    output.push_str("@define-color warning_color ");
    output.push_str(&theme.colors.yellow);
    output.push_str(";\n");
    
    output.push_str("@define-color success_color ");
    output.push_str(&theme.colors.green);
    output.push_str(";\n");
    
    output.push_str("\n");
    
    // Window styling
    output.push_str("window {\n");
    output.push_str("  background-color: @theme_bg_color;\n");
    output.push_str("  color: @theme_fg_color;\n");
    output.push_str("}\n\n");
    
    // Button styling
    output.push_str("button {\n");
    output.push_str(&format!("  border-radius: {}px;\n", border_radius));
    output.push_str(&format!("  border-width: {}px;\n", border_width));
    output.push_str("  border-color: alpha(@theme_fg_color, 0.2);\n");
    output.push_str("  background-color: @theme_bg_color;\n");
    output.push_str("  color: @theme_fg_color;\n");
    if let Some(duration) = theme.properties.animation_duration {
        output.push_str(&format!("  transition: all {}ms ease-in-out;\n", (duration * 1000.0) as u32));
    }
    output.push_str("}\n\n");
    
    output.push_str("button:hover {\n");
    output.push_str("  background-color: @theme_hover_bg_color;\n");
    output.push_str("  border-color: @accent_color;\n");
    output.push_str("}\n\n");
    
    output.push_str("button:active {\n");
    output.push_str("  background-color: @theme_active_bg_color;\n");
    output.push_str("}\n\n");
    
    output.push_str("button:checked {\n");
    output.push_str("  background-color: @accent_color;\n");
    output.push_str("  color: @theme_selected_fg_color;\n");
    output.push_str("}\n\n");
    
    // Entry (text input) styling
    output.push_str("entry {\n");
    output.push_str(&format!("  border-radius: {}px;\n", border_radius));
    output.push_str(&format!("  border-width: {}px;\n", border_width));
    output.push_str("  border-color: alpha(@theme_fg_color, 0.3);\n");
    output.push_str("  background-color: @theme_bg_color;\n");
    output.push_str("  color: @theme_fg_color;\n");
    output.push_str("  padding: 6px 12px;\n");
    output.push_str("}\n\n");
    
    output.push_str("entry:focus {\n");
    output.push_str("  border-color: @accent_color;\n");
    output.push_str(&format!("  box-shadow: 0 0 0 {}px alpha(@accent_color, 0.2);\n", shadow_blur / 2));
    output.push_str("}\n\n");
    
    // Notebook (tabs) styling
    output.push_str("notebook > header > tabs > tab {\n");
    output.push_str(&format!("  border-radius: {}px {}px 0 0;\n", border_radius, border_radius));
    output.push_str("  background-color: alpha(@theme_fg_color, 0.1);\n");
    output.push_str("  color: @theme_fg_color;\n");
    output.push_str("  padding: 6px 12px;\n");
    output.push_str("}\n\n");
    
    output.push_str("notebook > header > tabs > tab:checked {\n");
    output.push_str("  background-color: @accent_color;\n");
    output.push_str("  color: @theme_selected_fg_color;\n");
    output.push_str("}\n\n");
    
    // Progress bar styling
    output.push_str("progressbar > trough {\n");
    output.push_str(&format!("  border-radius: {}px;\n", border_radius));
    output.push_str("  background-color: alpha(@theme_fg_color, 0.1);\n");
    output.push_str("  min-height: 4px;\n");
    output.push_str("}\n\n");
    
    output.push_str("progressbar > trough > progress {\n");
    output.push_str(&format!("  border-radius: {}px;\n", border_radius));
    output.push_str("  background-color: @accent_color;\n");
    output.push_str("}\n\n");
    
    // Scrollbar styling
    output.push_str("scrollbar > trough {\n");
    output.push_str("  background-color: alpha(@theme_fg_color, 0.05);\n");
    output.push_str("  min-width: 12px;\n");
    output.push_str("  min-height: 12px;\n");
    output.push_str("}\n\n");
    
    output.push_str("scrollbar > trough > slider {\n");
    output.push_str(&format!("  border-radius: {}px;\n", border_radius / 2));
    output.push_str("  background-color: alpha(@theme_fg_color, 0.3);\n");
    output.push_str("  min-width: 8px;\n");
    output.push_str("  min-height: 8px;\n");
    output.push_str("}\n\n");
    
    output.push_str("scrollbar > trough > slider:hover {\n");
    output.push_str("  background-color: alpha(@theme_fg_color, 0.5);\n");
    output.push_str("}\n\n");
    
    // Menu styling
    output.push_str("menu {\n");
    output.push_str("  background-color: @theme_bg_color;\n");
    output.push_str("  color: @theme_fg_color;\n");
    output.push_str(&format!("  border-radius: {}px;\n", border_radius));
    if shadow_blur > 0 {
        output.push_str(&format!("  box-shadow: 0 2px {}px alpha(@theme_fg_color, 0.2);\n", shadow_blur));
    }
    output.push_str("}\n\n");
    
    output.push_str("menuitem {\n");
    output.push_str("  padding: 6px 12px;\n");
    output.push_str(&format!("  border-radius: {}px;\n", border_radius / 2));
    output.push_str("}\n\n");
    
    output.push_str("menuitem:hover {\n");
    output.push_str("  background-color: @accent_color;\n");
    output.push_str("  color: @theme_selected_fg_color;\n");
    output.push_str("}\n\n");
    
    // Tooltip styling
    output.push_str("tooltip {\n");
    output.push_str("  background-color: @theme_bg_color;\n");
    output.push_str("  color: @theme_fg_color;\n");
    output.push_str(&format!("  border-radius: {}px;\n", border_radius));
    output.push_str("  border: 1px solid alpha(@theme_fg_color, 0.2);\n");
    if shadow_blur > 0 {
        output.push_str(&format!("  box-shadow: 0 2px {}px alpha(@theme_fg_color, 0.3);\n", shadow_blur));
    }
    output.push_str("  padding: 6px 12px;\n");
    output.push_str("}\n\n");
    
    // Error/warning/success message styling
    output.push_str(".error {\n");
    output.push_str("  background-color: alpha(@error_color, 0.1);\n");
    output.push_str("  color: @error_color;\n");
    output.push_str("  border: 1px solid @error_color;\n");
    output.push_str("}\n\n");
    
    output.push_str(".warning {\n");
    output.push_str("  background-color: alpha(@warning_color, 0.1);\n");
    output.push_str("  color: @warning_color;\n");
    output.push_str("  border: 1px solid @warning_color;\n");
    output.push_str("}\n\n");
    
    output.push_str(".success {\n");
    output.push_str("  background-color: alpha(@success_color, 0.1);\n");
    output.push_str("  color: @success_color;\n");
    output.push_str("  border: 1px solid @success_color;\n");
    output.push_str("}\n");
    
    Ok(output)
}
