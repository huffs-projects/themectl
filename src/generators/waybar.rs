use anyhow::Result;
use crate::theme::Theme;
use crate::utils::darken_color;

pub fn generate(theme: &Theme) -> Result<String> {
    let mut output = String::new();
    
    let border_radius = theme.properties.border_radius.unwrap_or(0);
    let spacing = theme.properties.spacing.unwrap_or(8);
    
    output.push_str("/* Waybar theme: ");
    output.push_str(&theme.name);
    output.push_str(" */\n\n");
    
    output.push_str("* {\n");
    output.push_str("  border: none;\n");
    output.push_str(&format!("  border-radius: {}px;\n", border_radius));
    output.push_str("  font-family: monospace;\n");
    output.push_str("  font-size: 12px;\n");
    output.push_str("  min-height: 0;\n");
    output.push_str("}\n\n");
    
    output.push_str("window#waybar {\n");
    output.push_str("  background-color: ");
    output.push_str(&theme.colors.bg);
    output.push_str(";\n");
    output.push_str("  color: ");
    output.push_str(&theme.colors.fg);
    output.push_str(";\n");
    output.push_str("  border-bottom: 2px solid ");
    output.push_str(&theme.colors.accent);
    output.push_str(";\n");
    output.push_str("}\n\n");
    
    // Workspace buttons
    output.push_str("#workspaces button {\n");
    output.push_str("  color: ");
    if let Some(dimmed) = darken_color(&theme.colors.fg, 0.3) {
        output.push_str(&dimmed);
    } else {
        output.push_str(&theme.colors.fg);
    }
    output.push_str(";\n");
    output.push_str(&format!("  padding: 0 {}px;\n", spacing));
    output.push_str("}\n\n");
    
    output.push_str("#workspaces button:hover {\n");
    output.push_str("  background-color: rgba(");
    output.push_str(&hex_to_rgba(&theme.colors.accent, 0.2));
    output.push_str(");\n");
    output.push_str("  color: ");
    output.push_str(&theme.colors.accent);
    output.push_str(";\n");
    output.push_str("}\n\n");
    
    output.push_str("#workspaces button.focused {\n");
    output.push_str("  background-color: ");
    output.push_str(&theme.colors.accent);
    output.push_str(";\n");
    output.push_str("  color: ");
    output.push_str(&theme.colors.bg);
    output.push_str(";\n");
    output.push_str("}\n\n");
    
    output.push_str("#workspaces button.urgent {\n");
    output.push_str("  background-color: ");
    output.push_str(&theme.colors.red);
    output.push_str(";\n");
    output.push_str("  color: ");
    output.push_str(&theme.colors.bg);
    output.push_str(";\n");
    output.push_str("}\n\n");
    
    // Clock
    output.push_str("#clock {\n");
    output.push_str("  background-color: ");
    output.push_str(&theme.colors.accent);
    output.push_str(";\n");
    output.push_str("  color: ");
    output.push_str(&theme.colors.bg);
    output.push_str(";\n");
    output.push_str(&format!("  padding: 0 {}px;\n", spacing + 4));
    output.push_str("}\n\n");
    
    // Music player
    output.push_str("#custom-music {\n");
    output.push_str("  color: ");
    output.push_str(&theme.colors.fg);
    output.push_str(";\n");
    output.push_str(&format!("  padding: 0 {}px;\n", spacing));
    output.push_str("}\n\n");
    
    output.push_str("#custom-music.disconnected { color: ");
    output.push_str(&theme.colors.red);
    output.push_str("; }\n");
    output.push_str("#custom-music.stopped { color: ");
    output.push_str(&theme.colors.yellow);
    output.push_str("; }\n");
    output.push_str("#custom-music.playing { color: ");
    output.push_str(&theme.colors.green);
    output.push_str("; }\n");
    output.push_str("#custom-music.paused { color: ");
    output.push_str(&theme.colors.cyan);
    output.push_str("; }\n\n");
    
    // System modules
    output.push_str("#pulseaudio, #network, #battery {\n");
    output.push_str("  color: ");
    output.push_str(&theme.colors.fg);
    output.push_str(";\n");
    output.push_str(&format!("  padding: 0 {}px;\n", spacing));
    output.push_str("  border-left: 2px solid rgba(");
    output.push_str(&hex_to_rgba(&theme.colors.accent, 0.2));
    output.push_str(");\n");
    output.push_str("}\n\n");
    
    output.push_str("#pulseaudio { color: ");
    output.push_str(&theme.colors.blue);
    output.push_str("; }\n");
    output.push_str("#pulseaudio.muted { color: ");
    output.push_str(&theme.colors.red);
    output.push_str("; }\n\n");
    
    output.push_str("#network { color: ");
    output.push_str(&theme.colors.cyan);
    output.push_str("; }\n");
    output.push_str("#network.disconnected { color: ");
    output.push_str(&theme.colors.red);
    output.push_str("; }\n\n");
    
    output.push_str("#battery { color: ");
    output.push_str(&theme.colors.green);
    output.push_str("; }\n");
    output.push_str("#battery.warning { color: ");
    output.push_str(&theme.colors.yellow);
    output.push_str("; }\n");
    output.push_str("#battery.critical { color: ");
    output.push_str(&theme.colors.red);
    output.push_str("; }\n\n");
    
    // Tooltip
    output.push_str("tooltip {\n");
    output.push_str("  background-color: ");
    output.push_str(&theme.colors.bg);
    output.push_str(";\n");
    output.push_str("  color: ");
    output.push_str(&theme.colors.fg);
    output.push_str(";\n");
    output.push_str("  border: 1px solid ");
    output.push_str(&theme.colors.accent);
    output.push_str(";\n");
    output.push_str("}\n");
    
    Ok(output)
}

fn hex_to_rgba(hex: &str, alpha: f32) -> String {
    if let Some((r, g, b)) = crate::utils::hex_to_rgb(hex) {
        format!("{}, {}, {}, {}", r, g, b, alpha)
    } else {
        format!("0, 0, 0, {}", alpha)
    }
}
