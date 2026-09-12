# @attrib: base template from my other projects
{
  description = "tinyos";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-parts.url = "github:hercules-ci/flake-parts";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs =
    inputs@{
      nixpkgs,
      flake-parts,
      rust-overlay,
      ...
    }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      systems = [
        "aarch64-darwin"
        "x86_64-linux"
      ];

      perSystem =
        { pkgs, system, ... }:
        let
          overlays = [ (import rust-overlay) ];
          pkgs = import nixpkgs {
            inherit system overlays;
          };
        in
        {
          formatter = pkgs.nixfmt;

          devShells.default = pkgs.mkShell {
            # @attrib: https://blog.graysonhead.net/posts/nix-flake-rust-bevy/
            nativeBuildInputs = with pkgs; [
              (rust-bin.selectLatestNightlyWith (
                toolchain:
                toolchain.default.override {
                  extensions = [ "clippy" ];
                }
              ))
            ];

            packages = with pkgs; [
              just
            ];
          };
        };
    };
}
