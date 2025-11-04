{ pkgs ? import <nixpkgs> { } }:

pkgs.mkShell {
  buildInputs = with pkgs; [
    rustup # Rust toolchain manager
    just # Just runner
    nixfmt-classic # Nix formatter
    trunk # Rust web bundler
    wasm-pack # Rust to WebAssembly compiler
  ];

  # Shell hook to set up environment
  shellHook = ''
    rustup toolchain install nightly
    # Add wasm32-unknown-unknown target for web builds
    rustup target add wasm32-unknown-unknown

    echo "Rust WASM development environment ready!"
    echo "Available commands:"
    echo "  just web   - Build for web"
    echo "  trunk serve - Serve the app locally"
  '';
}
