{
  inputs = {
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    nixpkgs.url = "nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, fenix, flake-utils, nixpkgs }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        packages = nixpkgs.legacyPackages.${system};
        rust-toolchain = with fenix.packages.${system}; fromToolchainFile {
          file = ./rust-toolchain.toml;
          sha256 = "sha256-+9FmLhAOezBZCOziO0Qct1NOrfpjNsXxc/8I0c7BdKE=";
        };
      in {
        devShell = packages.mkShell rec {
          buildInputs = with packages; [
            lld
            rust-toolchain
            rust-analyzer
            wasm-pack
            wasm-bindgen-cli_0_2_100
            firefox
          ];
          LD_LIBRARY_PATH = packages.lib.makeLibraryPath buildInputs;
        };
      }
    );
}