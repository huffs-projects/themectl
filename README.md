# themectl

A unified theming solution for Frog-OS applications. `themectl` reads TOML theme definitions and generates configuration files for 13+ applications including Kitty, Waybar, Hyprland, Neovim, Starship, Mako, and more.

## Features

- **Unified Theme Format**: Define themes once in TOML format
- **Multi-Application Support**: Generates configs for Kitty, Waybar, Neovim, Starship, Mako, Hyprland, Wofi, Wlogout, Fastfetch, and more
- **Auto-Apply**: Automatically detects and updates configuration files
- **Backup Safety**: Creates backups before modifying existing configs
- **Validation**: Validates theme files before applying
- **Export**: Export themes to specific formats

## Installation

```bash
cargo build --release
```

The binary will be available at `target/release/themectl`.

## Usage

### Apply a Theme

Apply a theme to all detected applications:

```bash
themectl apply gruvbox-dark
```

Use `--dry-run` to preview changes without applying:

```bash
themectl apply gruvbox-dark --dry-run
```

### List Available Themes

```bash
themectl list
```

### Show Theme Details

```bash
themectl show gruvbox-dark
```

### Validate a Theme File

```bash
themectl validate themes/my-theme.toml
```

### Export to Specific Format

Export a theme to a specific application format:

```bash
themectl export gruvbox-dark kitty --output kitty.conf
themectl export gruvbox-dark waybar --output waybar.css
themectl export gruvbox-dark neovim --output colors/frogos.lua
```

### Initialize Theme Directory

```bash
themectl init
```

## Theme File Format

Themes are defined in TOML format. Here's an example:

```toml
name = "gruvbox-dark"
description = "Gruvbox dark theme"

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

[properties]
border_radius = 0
border_width = 2
shadow_blur = 10
animation_duration = 0.2
spacing = 4
```

### Required Colors

- `bg` - Background color
- `fg` - Foreground/text color
- `accent` - Accent color
- `red`, `green`, `yellow`, `blue`, `magenta`, `cyan` - Color palette

### Optional Colors

- `orange`, `purple`, `pink`, `white`, `black`, `gray`

### Properties

- `border_radius` - Border radius in pixels
- `border_width` - Border width in pixels
- `shadow_blur` - Shadow blur radius
- `animation_duration` - Animation duration in seconds
- `spacing` - Spacing value in pixels

## Supported Applications

- **Kitty** - Terminal emulator
- **Waybar** - Status bar
- **Neovim** - Text editor
- **Starship** - Shell prompt
- **Mako** - Notification daemon
- **Hyprland** - Window manager
- **Wofi** - Application launcher
- **Wlogout** - Logout menu
- **Fastfetch** - System info display
- **Nix** - Home Manager module generation

## Nix Flakes and Home Manager Integration

For Nix users, `themectl` can generate Home Manager modules for declarative theme management. See the [Nix Integration Guide](docs/NIX_INTEGRATION.md) for detailed instructions on:

- Installing `themectl` via Nix flakes
- Generating Home Manager modules
- Integrating themes into your NixOS/Home Manager configuration
- Best practices and examples

## Deployment Methods

By default, `themectl` uses **Home Manager (nix)** deployment, which generates Nix modules for declarative theme management. You can also use **standard** deployment to write configuration files directly.

### Home Manager (Default)

The default deployment method generates Home Manager modules:

```bash
# Apply theme (generates Home Manager modules by default)
themectl apply gruvbox-dark
```

This generates Nix modules in `~/.config/nixpkgs/modules/themectl/` that you can import into your Home Manager configuration. See the [Nix Integration Guide](docs/NIX_INTEGRATION.md) for details.

### Standard Deployment

To write configuration files directly instead:

```bash
# Set deployment method to standard
themectl config set-deployment standard

# Apply theme (writes directly to ~/.config/)
themectl apply gruvbox-dark
```

You can override the config directory with the `--config-dir` flag:

```bash
themectl apply gruvbox-dark --config-dir /custom/path
```

## Documentation

- [Main README](README.md) - This file
- [Nix Integration Guide](docs/NIX_INTEGRATION.md) - Using themectl with Nix flakes and Home Manager
- [Theme Format](docs/THEME_FORMAT.md) - Theme file format specification
- [Generators](docs/GENERATORS.md) - Generator documentation

## Project Structure

```
themectl/
├── Cargo.toml
├── src/
│   ├── main.rs              # CLI entry point
│   ├── cli.rs               # Command definitions
│   ├── theme.rs             # Theme data structures
│   ├── parser.rs            # TOML parsing and validation
│   ├── file_manager.rs      # Config file management
│   ├── utils.rs             # Color utilities
│   └── generators/          # Application-specific generators
│       ├── kitty.rs
│       ├── waybar.rs
│       ├── neovim.rs
│       └── ...
├── themes/                  # Theme files
│   └── default.toml
├── docs/                    # Documentation
│   ├── NIX_INTEGRATION.md   # Nix flakes and Home Manager guide
│   ├── THEME_FORMAT.md      # Theme format specification
│   └── GENERATORS.md        # Generator documentation
└── README.md
```

## License

GPL-2.0
