# Generator Documentation

This document describes how `themectl` generates configuration files for each supported application.

## Overview

Each generator takes a `Theme` struct and produces application-specific configuration output. Generators are located in `src/generators/` and are invoked via the `generate()` function.

## Supported Generators

### Kitty

**Format**: `kitty`  
**Output**: Kitty terminal configuration  
**File**: `src/generators/kitty.rs`

Generates a Kitty terminal color scheme configuration.

**Color Usage:**
- `bg` → `background`
- `fg` → `foreground`
- `accent` → `cursor`, `selection_background`, `active_border_color`, `active_tab_background`
- `red`, `green`, `yellow`, `blue`, `magenta`, `cyan` → 16-color palette (color0-color15)
- `black` → `color0`, `color8` (if available)
- `gray` → `color8` (if available, otherwise lightened bg)
- `white` → `color15` (if available, otherwise lightened fg)
- `yellow` → `bell_border_color`
- `cyan` → `url_color`

**Output Location**: `~/.config/kitty/kitty.conf`

---

### Waybar

**Format**: `waybar`  
**Output**: CSS stylesheet  
**File**: `src/generators/waybar.rs`

Generates a Waybar CSS configuration with CSS variables.

**Color Usage:**
- `bg` → `background-color` for window
- `fg` → `color` for text
- `accent` → Border color, focused workspace background
- `red` → Urgent workspace background
- All colors available as CSS variables

**Properties Used:**
- `border_radius` → Applied to all elements
- `spacing` → Padding values

**Output Location**: `~/.config/waybar/style.css`

---

### Neovim

**Format**: `neovim`  
**Output**: Lua colorscheme  
**File**: `src/generators/neovim.rs`

Generates a Neovim Lua colorscheme file.

**Color Usage:**
- All colors (required and optional) are included
- Colors are defined in a `colors` table
- Syntax highlighting groups are mapped to theme colors
- Uses `lighten_color()` and `dim_color()` for variants

**Output Location**: `~/.config/nvim/colors/{theme-name}.lua`

---

### Starship

**Format**: `starship`  
**Output**: TOML configuration  
**File**: `src/generators/starship.rs`

Generates a Starship prompt configuration.

**Color Usage:**
- `orange` or `accent` → `palette.orange`
- `yellow` → `palette.yellow`
- `cyan` → `palette.aqua`
- `blue` → `palette.blue`
- `green` → `palette.green`
- `red` → `palette.red`
- `purple` → `palette.purple` (if available)

**Output Location**: `~/.config/starship.toml`

---

### Mako

**Format**: `mako`  
**Output**: Mako configuration  
**File**: `src/generators/mako.rs`

Generates a Mako notification daemon configuration.

**Color Usage:**
- `bg` → `background-color`
- `fg` → `text-color`
- `accent` → `border-color`
- `purple` → Used for special notification types (if available)
- `orange` → Used for warning notifications (if available)

**Output Location**: `~/.config/mako/config`

---

### Hyprland

**Format**: `hyprland`  
**Output**: Hyprland configuration  
**File**: `src/generators/hyprland.rs`

Generates Hyprland window manager color variables.

**Color Usage:**
- All colors are defined as `$color-*` variables
- Colors can be referenced in other Hyprland config sections

**Properties Used:**
- `border_width` → Window border width
- `shadow_blur` → Window shadow blur

**Output Location**: `~/.config/hypr/hyprland.conf` (appended)

---

### Hyprpaper

**Format**: `hyprpaper`  
**Output**: Hyprpaper configuration  
**File**: `src/generators/hyprpaper.rs`

Generates Hyprpaper wallpaper manager configuration with color references.

**Color Usage:**
- `bg` → Primary background color reference
- Colors are referenced for wallpaper color matching

**Output Location**: `~/.config/hypr/hyprpaper.conf`

---

### Wofi

**Format**: `wofi`  
**Output**: CSS stylesheet  
**File**: `src/generators/wofi.rs`

Generates a Wofi application launcher CSS configuration.

**Color Usage:**
- `bg` → `background-color`
- `fg` → `color`
- `accent` → Hover and selected states
- Uses `darken_color()` for hover effects

**Properties Used:**
- `border_radius` → Applied to window and entries
- `spacing` → Padding values

**Output Location**: `~/.config/wofi/style.css`

---

### Wlogout

**Format**: `wlogout`  
**Output**: CSS stylesheet  
**File**: `src/generators/wlogout.rs`

Generates a Wlogout logout menu CSS configuration.

**Color Usage:**
- `bg` → `background-color`
- `fg` → `color`
- `accent` → Button hover states
- Uses `darken_color()` for hover effects

**Properties Used:**
- `border_radius` → Applied to buttons
- `spacing` → Button padding

**Output Location**: `~/.config/wlogout/style.css`

---

### Fastfetch

**Format**: `fastfetch`  
**Output**: JSON configuration  
**File**: `src/generators/fastfetch.rs`

Generates a Fastfetch system info display configuration.

**Color Usage:**
- `orange` → Used for accent colors (if available)
- `purple` → Used for special sections (if available)
- `gray` → Used for secondary text (if available)
- All colors available in color array

**Output Location**: `~/.config/fastfetch/config.jsonc`

---

### Nix

**Format**: `nix`  
**Output**: Nix expression  
**File**: `src/generators/nix.rs`

Generates a Nix Home Manager module with theme colors.

**Color Usage:**
- All colors (required and optional) are included
- Colors are defined in a `colors` attribute set
- Can be used in NixOS/Home Manager configurations

**Output Location**: Custom (typically in Home Manager config)

---

### Yazi

**Format**: `yazi`  
**Output**: TOML configuration  
**File**: `src/generators/yazi.rs`

Generates a Yazi file manager TOML configuration.

**Color Usage:**
- `bg` → Background colors
- `fg` → Foreground colors
- `accent` → Selection and active states
- `orange`, `purple`, `gray` → Used for various UI elements (if available)
- `white`, `black` → Used for contrast (if available)
- Uses `darken_color()` for hover states

**Output Location**: `~/.config/yazi/yazi.toml`

---

## Generator Function Signature

All generators follow this signature:

```rust
pub fn generate(theme: &Theme) -> Result<String>
```

The function:
1. Takes a reference to a `Theme` struct
2. Returns a `Result<String>` containing the generated configuration
3. May return an error if generation fails

## Using Generators

### Via CLI

```bash
# Generate for a specific format
themectl export <theme> <format>

# Generate all formats
themectl export <theme> all
```

### Programmatically

```rust
use themectl::generators;
use themectl::parser;

let theme = parser::parse_theme_file("themes/my-theme.toml")?;
let kitty_config = generators::generate(&theme, "kitty")?;
```

## Generator Selection

Generators are selected by format name (case-insensitive):

- `kitty` → Kitty generator
- `waybar` → Waybar generator
- `neovim` → Neovim generator
- `starship` → Starship generator
- `mako` → Mako generator
- `hyprland` → Hyprland generator
- `hyprpaper` → Hyprpaper generator
- `wofi` → Wofi generator
- `wlogout` → Wlogout generator
- `fastfetch` → Fastfetch generator
- `nix` → Nix generator
- `yazi` → Yazi generator

## Batch Generation

Use `generate_all()` to generate all formats at once:

```rust
let all_configs = generators::generate_all(&theme)?;
// Returns Vec<(String, String)> where each tuple is (format_name, content)
```

## Error Handling

If a generator fails, `generate_all()` will:
- Continue processing other generators
- Print a warning to stderr
- Include successful generations in the result

## Adding New Generators

To add a new generator:

1. Create a new file in `src/generators/` (e.g., `myapp.rs`)
2. Implement the `generate()` function
3. Add the generator to `src/generators/mod.rs`
4. Add the format to the `generate()` match statement
5. Add the format to the `generate_all()` formats list

Example:

```rust
// src/generators/myapp.rs
use anyhow::Result;
use crate::theme::Theme;

pub fn generate(theme: &Theme) -> Result<String> {
    let mut output = String::new();
    output.push_str(&format!("background: {}\n", theme.colors.bg));
    output.push_str(&format!("foreground: {}\n", theme.colors.fg));
    Ok(output)
}
```

## See Also

- [Theme Format Specification](THEME_FORMAT.md) - Theme file structure
- [CLI Documentation](../README.md) - Command-line usage
