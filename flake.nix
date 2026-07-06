{
  description = "Rust development environment for Advent of Code 2024";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };

        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          extensions = [ "rust-src" "rust-analyzer" ];
        };
      in
      {
        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            rustToolchain
            cargo-watch
            cargo-edit

            # Additional development tools
            pkg-config
            openssl
          ];

          shellHook = ''
            echo "🦀 Rust development environment loaded!"
            echo "Available tools:"
            echo "  - rustc: $(rustc --version)"
            echo "  - cargo: $(cargo --version)"
            echo "  - rust-analyzer: Available"
            echo "  - cargo-watch: Hot reload for cargo commands"
            echo "  - cargo-edit: Manage Cargo.toml dependencies"
            echo ""
            echo "Quick commands:"
            echo "  - cargo build: Build the project"
            echo "  - cargo test: Run tests"
            echo "  - cargo fmt: Format code"
            echo "  - cargo clippy: Run linter"
            echo "  - cargo watch -x test: Auto-run tests on changes"
          '';
        };
      });
}
