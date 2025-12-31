use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Theme {
    pub name: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub variant: Option<String>,
    pub colors: ColorPalette,
    #[serde(default)]
    pub properties: ThemeProperties,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColorPalette {
    pub bg: String,
    pub fg: String,
    pub accent: String,
    pub red: String,
    pub green: String,
    pub yellow: String,
    pub blue: String,
    pub magenta: String,
    pub cyan: String,
    #[serde(default)]
    pub orange: Option<String>,
    #[serde(default)]
    pub purple: Option<String>,
    #[serde(default)]
    pub pink: Option<String>,
    #[serde(default)]
    pub white: Option<String>,
    #[serde(default)]
    pub black: Option<String>,
    #[serde(default)]
    pub gray: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ThemeProperties {
    #[serde(default)]
    pub border_radius: Option<u32>,
    #[serde(default)]
    pub border_width: Option<u32>,
    #[serde(default)]
    pub shadow_blur: Option<u32>,
    #[serde(default)]
    pub animation_duration: Option<f32>,
    #[serde(default)]
    pub spacing: Option<u32>,
}

impl Theme {
    pub fn get_color(&self, name: &str) -> Option<&str> {
        match name {
            "bg" => Some(&self.colors.bg),
            "fg" => Some(&self.colors.fg),
            "accent" => Some(&self.colors.accent),
            "red" => Some(&self.colors.red),
            "green" => Some(&self.colors.green),
            "yellow" => Some(&self.colors.yellow),
            "blue" => Some(&self.colors.blue),
            "magenta" => Some(&self.colors.magenta),
            "cyan" => Some(&self.colors.cyan),
            "orange" => self.colors.orange.as_deref(),
            "purple" => self.colors.purple.as_deref(),
            "pink" => self.colors.pink.as_deref(),
            "white" => self.colors.white.as_deref(),
            "black" => self.colors.black.as_deref(),
            "gray" => self.colors.gray.as_deref(),
            _ => None,
        }
    }

    /// Extract base name from a theme name that may include variant suffix
    /// e.g., "gruvbox-dark" -> "gruvbox"
    pub fn base_name(&self) -> String {
        Self::extract_base_name(&self.name)
    }

    /// Extract base name from a theme name string
    pub fn extract_base_name(name: &str) -> String {
        // Remove common variant suffixes: -dark, -light, -darkest, -lightest
        let variants = ["-darkest", "-lightest", "-dark", "-light"];
        for variant in &variants {
            if name.ends_with(variant) {
                // Use get() to safely handle potential UTF-8 boundary issues
                if let Some(end_pos) = name.len().checked_sub(variant.len()) {
                    if let Some(slice) = name.get(..end_pos) {
                        return slice.to_string();
                    }
                }
            }
        }
        name.to_string()
    }

    /// Detect variant from theme name
    /// Returns "dark", "light", or None
    pub fn detect_variant_from_name(name: &str) -> Option<String> {
        let name_lower = name.to_lowercase();
        if name_lower.ends_with("-dark") || name_lower.ends_with("-darkest") {
            Some("dark".to_string())
        } else if name_lower.ends_with("-light") || name_lower.ends_with("-lightest") {
            Some("light".to_string())
        } else {
            None
        }
    }

    /// Get the variant name, either from the variant field or detected from name
    pub fn get_variant(&self) -> Option<String> {
        self.variant.clone().or_else(|| Self::detect_variant_from_name(&self.name))
    }

    /// Get the full theme name including variant
    pub fn full_name(&self) -> String {
        if let Some(ref variant) = self.get_variant() {
            format!("{}-{}", self.base_name(), variant)
        } else {
            self.name.clone()
        }
    }
}
