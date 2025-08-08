# Setup

TODO

## Using nix

If you use nix in the root of the project, run: `nix develop --impure`

### Limitations while using the `nix shell`

1. The `mdbook` command is not available, since the [`mdbook-tabs`](https://crates.io/crates/mdbook-tabs) is not included
   in the nixpkgs
2. Testing in `chrome` does not work, beacause the `wasm-pack test --headless --release --chrome` searches for driver under
   wrong path