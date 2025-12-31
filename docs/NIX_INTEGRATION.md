# Nix Flakes and Home Manager Integration

This guide explains how to use `themectl` with Nix flakes and Home Manager for declarative theme management.

## Table of Contents

- [Installation via Nix Flakes](#installation-via-nix-flakes)
- [Home Manager Integration](#home-manager-integration)
- [Generating Home Manager Modules](#generating-home-manager-modules)
- [Using Generated Modules](#using-generated-modules)
- [Complete Example](#complete-example)
- [Best Practices](#best-practices)

## Installation via Nix Flakes

### Adding themectl to your flake

Add `themectl` to your `flake.nix`:

```nix
{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    themectl.url = "path:./path/to/themectl";  # Or use a git URL
    # themectl.url = "github:huffs-projects/themectl";
  };

  outputs = { self, nixpkgs, themectl, ... }@inputs: {
    nixosConfigurations.your-host = nixpkgs.lib.nixosSystem {
      system = "x86_64-linux";
      modules = [
        # Your other modules
      ];
    };
  };
}
```

### Using themectl in a development shell

Create a `flake.nix` in your themectl directory:

```nix
{
  description = "themectl - unified theming solution";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
      in
      {
        packages.default = pkgs.rustPlatform.buildRustPackage {
          pname = "themectl";
          version = "0.1.0";
          src = ./.;
          cargoLock = {
            lockFile = ./Cargo.lock;
          };
        };

        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            cargo
            rustc
            rustfmt
            clippy
          ];
        };
      }
    );
}
```

### Installing via nix profile

```bash
nix profile install github:huffs-projects/themectl
```

Or from a local path:

```bash
nix profile install path:./themectl
```

## Home Manager Integration

### Setting up themectl with Home Manager

`themectl` defaults to **Home Manager (nix)** deployment mode, so no configuration is required to get started. However, you can customize the Nix output path if needed.

1. **Configure Nix output path** (optional, defaults to `~/.config/nixpkgs/modules/themectl`):

```bash
# Edit ~/.config/themectl/config.toml
[nix]
output_path = "/path/to/your/nixpkgs/modules/themectl"
```

### Configuration file structure

Your `~/.config/themectl/config.toml` will look like this (nix is the default):

```toml
deployment_method = "nix"

[nix]
output_path = "/home/username/.config/nixpkgs/modules/themectl"
```

Note: If you want to use standard file deployment instead, you can set `deployment_method = "standard"`.

## Generating Home Manager Modules

### Generate modules for all applications

```bash
# Apply a theme and generate Home Manager modules
themectl apply gruvbox-dark
```

This will generate Nix modules in your configured output path for all detected applications.

### Generate module for a specific application

```bash
# Export theme to Nix format for a specific app
themectl export gruvbox-dark nix --output ~/.config/nixpkgs/modules/themectl/kitty.nix
```

### Generate Home Manager module directly

```bash
# Generate Home Manager module for kitty
themectl export gruvbox-dark kitty --format nix > ~/.config/nixpkgs/modules/themectl/kitty.nix
```

## Using Generated Modules

### Importing in Home Manager

In your `home.nix` or `home-manager.nix`:

```nix
{ config, pkgs, ... }:

{
  imports = [
    # Import themectl-generated modules
    ./modules/themectl/kitty.nix
    ./modules/themectl/waybar.nix
    ./modules/themectl/neovim.nix
    ./modules/themectl/starship.nix
    # ... other applications
  ];

  # Your other Home Manager configuration
}
```

### Using with Flakes

In your `flake.nix`:

```nix
{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    home-manager = {
      url = "github:nix-community/home-manager";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, home-manager, ... }@inputs: {
    homeConfigurations."username" = home-manager.lib.homeManagerConfiguration {
      pkgs = nixpkgs.legacyPackages.x86_64-linux;
      modules = [
        ./home.nix
        # Import themectl modules
        ./modules/themectl/kitty.nix
        ./modules/themectl/waybar.nix
        ./modules/themectl/neovim.nix
      ];
    };
  };
}
```

## Complete Example

### Step 1: Create a theme

```bash
# Create or use an existing theme
themectl create my-theme
# Or use an existing theme from themes/ directory
```

### Step 2: Generate Home Manager modules

```bash
# Apply theme (generates modules for all apps)
# Note: nix is the default deployment method, so no configuration needed
themectl apply my-theme
```

### Step 3: Integrate into Home Manager

Create `~/.config/nixpkgs/home.nix`:

```nix
{ config, pkgs, lib, ... }:

{
  imports = [
    # Import themectl-generated theme modules
    ./modules/themectl/kitty.nix
    ./modules/themectl/waybar.nix
    ./modules/themectl/neovim.nix
    ./modules/themectl/starship.nix
    ./modules/themectl/mako.nix
    ./modules/themectl/hyprland.nix
  ];

  # Your other Home Manager configuration
  home.username = "username";
  home.homeDirectory = "/home/username";
  
  # Additional configuration
  programs.git.enable = true;
  # ... etc
}
```

### Step 4: Build and switch

```bash
# With Home Manager standalone
home-manager switch

# Or with flakes
nixos-rebuild switch --flake .#your-host
# Or
home-manager switch --flake .#username
```

## Generated Module Examples

### Kitty Module

Generated `kitty.nix`:

```nix
# Home Manager module for kitty theme configuration
# Generated by themectl
# Theme: gruvbox-dark

{ config, lib, pkgs, ... }:

{
  programs.kitty = {
    enable = true;
    settings = {
      extraConfig = ''
        # Kitty theme: gruvbox-dark
        # Generated by themectl
        
        foreground #ebdbb2
        background #282828
        # ... more colors
      '';
    };
  };
}
```

### Waybar Module

Generated `waybar.nix`:

```nix
# Home Manager module for waybar theme configuration
# Generated by themectl
# Theme: gruvbox-dark

{ config, lib, pkgs, ... }:

{
  programs.waybar = {
    enable = true;
    style = ''
      * {
        background-color: #282828;
        color: #ebdbb2;
      }
      /* ... more styles */
    '';
  };
}
```

### Neovim Module

Generated `neovim.nix`:

```nix
# Home Manager module for neovim theme configuration
# Generated by themectl
# Theme: gruvbox-dark

{ config, lib, pkgs, ... }:

{
  programs.neovim = {
    enable = true;
    extraLuaConfig = ''
      local colors = {
        bg = "#282828",
        fg = "#ebdbb2",
        -- ... more colors
      }
      -- ... color scheme setup
    '';
  };
}
```

## Best Practices

### 1. Version Control

Commit your generated modules to version control:

```bash
git add modules/themectl/
git commit -m "Update theme modules"
```

### 2. Theme Management

Keep themes in a separate directory and version control them:

```bash
# Store themes in a dedicated location
mkdir -p ~/.config/themectl/themes
themectl init --themes-dir ~/.config/themectl/themes
```

### 3. Automated Updates

Create a script to regenerate modules when themes change:

```bash
#!/bin/bash
# update-themes.sh

THEME="gruvbox-dark"
THEMES_DIR="$HOME/.config/themectl/themes"
OUTPUT_DIR="$HOME/.config/nixpkgs/modules/themectl"

themectl apply "$THEME" --themes-dir "$THEMES_DIR"
```

### 4. Multiple Themes

Use variants for light/dark themes:

```bash
# Create variants
themectl variant create gruvbox-dark light
themectl variant create gruvbox-dark dark

# Generate modules for specific variant
themectl apply gruvbox-dark-light
```

### 5. Conditional Theme Application

In your Home Manager config, conditionally apply themes:

```nix
{ config, pkgs, ... }:

let
  # Select theme based on system or user preference
  theme = "gruvbox-dark";
in
{
  imports = if theme == "gruvbox-dark" then [
    ./modules/themectl/gruvbox-dark/kitty.nix
    ./modules/themectl/gruvbox-dark/waybar.nix
  ] else [
    ./modules/themectl/other-theme/kitty.nix
    ./modules/themectl/other-theme/waybar.nix
  ];
}
```

### 6. Flake-based Theme Management

Create a flake for theme management:

```nix
# themes/flake.nix
{
  description = "Theme configurations";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    themectl.url = "path:../themectl";
  };

  outputs = { self, nixpkgs, themectl }:
    let
      system = "x86_64-linux";
      pkgs = import nixpkgs { inherit system; };
    in
    {
      packages.${system}.default = pkgs.writeShellScriptBin "apply-theme" ''
        ${themectl.packages.${system}.default}/bin/themectl apply "$@"
      '';
    };
}
```

## Troubleshooting

### Modules not found

If Home Manager can't find your modules:

1. Check the output path in `~/.config/themectl/config.toml`
2. Ensure modules are in the correct location
3. Verify import paths in your `home.nix`

### Theme not applying

1. Verify modules are generated:
   ```bash
   ls ~/.config/nixpkgs/modules/themectl/
   ```

3. Check Home Manager logs:
   ```bash
   home-manager switch --show-trace
   ```

### Regenerating modules

To regenerate all modules for a theme:

```bash
# Remove old modules (optional)
rm -rf ~/.config/nixpkgs/modules/themectl/*.nix

# Regenerate
themectl apply your-theme
```

## Advanced Usage

### Custom Module Templates

You can customize the generated modules by modifying the generator in `src/generators/nix.rs`, or create wrapper modules:

```nix
# modules/themectl/custom-kitty.nix
{ config, pkgs, ... }:

let
  # Import generated module
  baseConfig = import ./kitty.nix { inherit config pkgs; };
in
{
  # Extend with custom settings
  programs.kitty = baseConfig.programs.kitty // {
    settings = baseConfig.programs.kitty.settings // {
      font_size = 12;
      # ... your custom settings
    };
  };
}
```

### Theme Switching Script

Create a script to switch themes:

```bash
#!/bin/bash
# switch-theme.sh

THEME="$1"
if [ -z "$THEME" ]; then
  echo "Usage: $0 <theme-name>"
  exit 1
fi

# Generate modules
themectl apply "$THEME"

# Rebuild Home Manager
home-manager switch

echo "Theme switched to $THEME"
```

## See Also

- [Theme Format Documentation](./THEME_FORMAT.md)
- [Main README](../README.md)
- [Home Manager Documentation](https://nix-community.github.io/home-manager/)
- [Nix Flakes Documentation](https://nixos.wiki/wiki/Flakes)
