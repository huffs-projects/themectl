# Theme Format Specification

This document describes the TOML-based theme format used by `themectl`.

## Overview

Themes in `themectl` are defined using TOML (Tom's Obvious Minimal Language) format. Each theme file must have a `.toml` extension and contain a valid theme definition.

## File Structure

A theme file consists of three main sections:

1. **Metadata** - Theme name, description, and variant information
2. **Colors** - Color palette definitions (required and optional)
3. **Properties** - Visual properties like border radius, spacing, etc.

## Complete Example

```toml
name = "gruvbox-dark"
description = "Gruvbox dark theme"
variant = "dark"

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

[properties]
border_radius = 0
border_width = 2
shadow_blur = 10
animation_duration = 0.2
spacing = 4
```

## Metadata Fields

### `name` (required)
- **Type**: String
- **Description**: The theme name. Must be non-empty.
- **Example**: `name = "gruvbox-dark"`

### `description` (optional)
- **Type**: String
- **Default**: Empty string
- **Description**: A human-readable description of the theme.
- **Example**: `description = "Gruvbox dark theme"`

### `variant` (optional)
- **Type**: String
- **Default**: None
- **Description**: Theme variant type. Typically "dark" or "light".
- **Example**: `variant = "dark"`

## Color Palette

All colors must be specified in hexadecimal format with a `#` prefix (e.g., `#RRGGBB`).

### Required Colors

The following colors are mandatory and must be present in every theme:

- **`bg`** - Background color. Used as the primary background throughout applications.
- **`fg`** - Foreground/text color. Used for primary text content.
- **`accent`** - Accent color. Used for highlights, active states, and emphasis.
- **`red`** - Red color in the palette.
- **`green`** - Green color in the palette.
- **`yellow`** - Yellow color in the palette.
- **`blue`** - Blue color in the palette.
- **`magenta`** - Magenta color in the palette.
- **`cyan`** - Cyan color in the palette.

### Optional Colors

The following colors are optional but recommended for full theme support:

- **`orange`** - Orange color. Used by Starship, Neovim, Fastfetch, and others.
- **`purple`** - Purple color. Used by Neovim, Starship, Mako, and others.
- **`pink`** - Pink color. Used by Neovim and Nix generators.
- **`white`** - White color. Used by Kitty, Neovim, Yazi, and others.
- **`black`** - Black color. Used by Kitty, Neovim, Yazi, and others.
- **`gray`** - Gray color. Used by Fastfetch, Yazi, and others.

### Color Format

Colors must be valid hexadecimal color codes:
- Format: `#RRGGBB` where RR, GG, BB are hexadecimal values (00-FF)
- Case-insensitive: `#282828` and `#282828` are equivalent
- The `#` prefix is required

**Valid examples:**
- `#282828`
- `#ebdbb2`
- `#FF0000`
- `#00ff00`

**Invalid examples:**
- `282828` (missing #)
- `#28` (too short)
- `#28282828` (too long)
- `#gggggg` (invalid hex characters)

## Theme Properties

The `[properties]` section is optional and contains visual properties that some generators may use:

### `border_radius` (optional)
- **Type**: Unsigned integer (u32)
- **Description**: Border radius in pixels. Used by Waybar, Wofi, and Wlogout.
- **Example**: `border_radius = 8`

### `border_width` (optional)
- **Type**: Unsigned integer (u32)
- **Description**: Border width in pixels. Used by Hyprland.
- **Example**: `border_width = 2`

### `shadow_blur` (optional)
- **Type**: Unsigned integer (u32)
- **Description**: Shadow blur radius in pixels. Used by Hyprland.
- **Example**: `shadow_blur = 10`

### `animation_duration` (optional)
- **Type**: Floating-point number (f32)
- **Description**: Animation duration in seconds.
- **Example**: `animation_duration = 0.2`

### `spacing` (optional)
- **Type**: Unsigned integer (u32)
- **Description**: Spacing value in pixels. Used by Waybar, Wofi, and Wlogout.
- **Example**: `spacing = 4`

## Validation Rules

### Required Fields
- `name` must be present and non-empty
- All required colors (`bg`, `fg`, `accent`, `red`, `green`, `yellow`, `blue`, `magenta`, `cyan`) must be present

### Color Validation
- All colors (required and optional) must be valid hexadecimal color codes
- Colors must match the pattern `^#?[0-9A-Fa-f]{6}$`

### Accessibility Validation
When validating a theme, `themectl` performs accessibility checks:

- **Contrast Ratio**: Background and foreground colors must meet WCAG AA standards (4.5:1 minimum)
- **Similar Colors**: Warnings are issued if colors are too similar (within 30 RGB distance units)

## Variant Naming Convention

Theme variants can be specified in two ways:

1. **Explicit variant field**: `variant = "dark"` or `variant = "light"`
2. **Implicit from name**: Theme names ending with `-dark`, `-light`, `-darkest`, or `-lightest` are automatically detected

Examples:
- `gruvbox-dark.toml` → variant detected as "dark"
- `gruvbox-light.toml` → variant detected as "light"
- `theme-darkest.toml` → variant detected as "dark"
- `theme-lightest.toml` → variant detected as "light"

## Minimal Valid Theme

A minimal valid theme requires only the metadata and required colors:

```toml
name = "minimal-theme"
description = "A minimal theme"

[colors]
bg = "#000000"
fg = "#ffffff"
accent = "#ff0000"
red = "#ff0000"
green = "#00ff00"
yellow = "#ffff00"
blue = "#0000ff"
magenta = "#ff00ff"
cyan = "#00ffff"
```

## Best Practices

1. **Use descriptive names**: Choose theme names that clearly indicate the color scheme
2. **Provide descriptions**: Help users understand what the theme looks like
3. **Include optional colors**: While optional, including all colors ensures better generator support
4. **Test contrast**: Ensure background and foreground colors have sufficient contrast (WCAG AA: 4.5:1)
5. **Use consistent naming**: Follow naming conventions like `theme-name-variant.toml`
6. **Validate before sharing**: Use `themectl validate` to check your theme before committing

## File Naming

Theme files should be named using the theme name with a `.toml` extension:
- `gruvbox-dark.toml`
- `nord-light.toml`
- `dracula.toml`

The file name should match the theme name (without extension) for best compatibility.

## See Also

- [Generator Documentation](GENERATORS.md) - How each generator uses theme colors
- [CLI Documentation](../README.md) - Command-line usage
- [Examples](../examples/) - Sample theme files
