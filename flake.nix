{
  description = "themectl - unified theming solution for Frog-OS applications";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
        themectl = pkgs.callPackage ./default.nix {
          src = ./.;
        };
      in
      {
        packages = {
          default = themectl;
          themectl = themectl;
        };

        apps = {
          default = {
            type = "app";
            program = "${themectl}/bin/themectl";
          };
        };

        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            cargo
            rustc
            rustfmt
            clippy
            rust-analyzer
          ];
        };

        formatter = pkgs.nixpkgs-fmt;
      }
    );
}
