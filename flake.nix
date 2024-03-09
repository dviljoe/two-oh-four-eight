{
  inputs = {
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.follows = "rust-overlay/flake-utils";
    nixpkgs.follows = "rust-overlay/nixpkgs";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };
        rust-stable = pkgs.rust-bin.stable.latest.default.override {
          extensions = [ "rust-src" ];
        };
      in with pkgs; {
        devShells.default = mkShell rec {
          nativeBuildInputs = [
            semgrep
            pkg-config
            rust-bin.stable.latest.clippy
            rust-bin.stable.latest.rust-analyzer
            rust-stable
            clang
            mold
            gnumake

          ];
          buildInputs = [
            udev
            alsa-lib
            vulkan-loader
            xorg.libX11
            xorg.libXcursor
            xorg.libXi
            libxkbcommon
            wayland # To use the wayland feature
          ];
          LD_LIBRARY_PATH = lib.makeLibraryPath buildInputs;
        };
      });
}
