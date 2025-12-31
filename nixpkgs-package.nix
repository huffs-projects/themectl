# This is the package expression for nixpkgs submission
# Place this file in: pkgs/tools/misc/themectl/default.nix
# 
# Usage in nixpkgs:
# 1. Copy this file to nixpkgs/pkgs/tools/misc/themectl/default.nix
# 2. Update the fetchFromGitHub section with actual values
# 3. Add yourself to maintainers list
# 4. Submit a PR to nixpkgs

{ lib
, rustPlatform
, fetchFromGitHub
}:

rustPlatform.buildRustPackage rec {
  pname = "themectl";
  version = "0.1.0";

  src = fetchFromGitHub {
    owner = "huffs-projects";
    repo = "themectl";
    rev = "v${version}";
    sha256 = "sha256-AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA=";  # UPDATE: Run nix-prefetch-github to get the hash
  };

  cargoLock = {
    lockFile = ./Cargo.lock;
  };

  meta = with lib; {
    description = "A unified theming solution for Frog-OS applications";
    longDescription = ''
      themectl reads TOML theme definitions and generates configuration files
      for 13+ applications including Kitty, Waybar, Hyprland, Neovim, Starship,
      Mako, and more. It provides a unified theming solution for Frog-OS
      applications with support for auto-apply, backup safety, validation,
      and Nix/Home Manager integration.
    '';
    homepage = "https://github.com/huffs-projects/themectl";
    license = licenses.gpl2Only;
    maintainers = with maintainers; [ Huff Mullen ];
    platforms = platforms.unix;
    mainProgram = "themectl";
  };
}
