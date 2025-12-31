{ lib
, rustPlatform
, fetchFromGitHub
, pname ? "themectl"
, version ? "0.1.0"
, src ? null
, ...
} @ args:

let
  # If src is not provided, use fetchFromGitHub (for nixpkgs)
  # Otherwise use the provided src (for local development)
  source = if src != null then src else fetchFromGitHub {
    owner = "huffs-projects";
    repo = "themectl";
    rev = "v${version}";
    sha256 = lib.fakeSha256;  # Will be replaced during nixpkgs submission
  };
in

rustPlatform.buildRustPackage {
  inherit pname version;
  src = source;

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
