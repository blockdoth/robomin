{
  inputs = {
    flake-parts.url = "github:hercules-ci/flake-parts";
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs =
    inputs@{ self, flake-parts, ... }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      systems = [
        "x86_64-linux"
        "aarch64-linux"
        "x86_64-darwin"
        "aarch64-darwin"
      ];
      perSystem =
        {
          inputs',
          pkgs,
          system,
          ...
        }:
        let
          pkgs = import inputs.nixpkgs {
            inherit system;
            overlays = [ inputs.rust-overlay.overlays.default ];
          };
          toolchain = pkgs.rust-bin.fromRustupToolchainFile ./toolchain.toml;
        in
        {
          devShells.default = pkgs.mkShell {
            packages = with pkgs; [
              toolchain
              rust-analyzer-unwrapped
              cmake
              glfw
              wayland
              clang
              SDL2
            ];
            
            LD_LIBRARY_PATH = with pkgs; lib.makeLibraryPath [
              libGL
              xorg.libXrandr
              xorg.libXinerama
              xorg.libXcursor
              xorg.libXi
            ];
            LIBCLANG_PATH = "${pkgs.libclang.lib}/lib";

            RUST_SRC_PATH = "${toolchain}/lib/rustlib/src/rust/library";
          };

          packages.default = pkgs.rustPlatform.buildRustPackage {
            pname = "pathfinding";
            version = "0.0.1";
            src = ./.;
            cargoLock.lockFile = ./Cargo.lock;
            cargoToml = ./Cargo.toml;
            release = true;
            nativeBuildInputs = with pkgs; [ 
              cmake
              toolchain 
            ];
          };
        };
    };
}
