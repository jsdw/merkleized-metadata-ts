# merkleized-metadata-ts

This is a simple TypeScript wrapper around https://github.com/bkchr/merkleized-metadata.

This consists of two packages.

- `merkleized-metadata-sys`: low level WASM bindings to the rust crate.
- `merkleized-metadata`: a higher level library which wraps a slightly nicer interface over this.

It's expected that people will import `merkleized-metadata` in their TS projects.

# Developer Notes

## Publishing a new version

1. Increment version number in `merkleized-metadata-sys/Cargo.toml`.
2. Build/publish the raw TS interface:
   ```
   cd merkleized-metadata-sys
   wasm-pack build --release
   (cd inline_wasm && cargo run -- ../pkg)
   (cd pkg && npm publish)
   cd ..
   ```
3. Increment version of sys crate used in `merkleized-metadata/package.json`.
4. `(cd merkleized-metadata && npm i && npm run build && npm publish)` to build/publish the high level TS interface.
5. Bump versions used in `examples/web` and `(cd examples/web && npm i && npx webpack serve)` to verify it still works;

## Testing updates

The following will build and link everything such that we'll serve and see any changes made in the `examples/web`.

```
echo "# Build merkleized-metadata-sys"
(cd merkleized-metadata-sys && wasm-pack build)
echo "# Rewrite entrypoint for merkleized-metadata-sys"
(cd merkleized-metadata-sys/inline_wasm && cargo run -- ../pkg)
echo "# Link merkleized-metadata-sys"
(cd merkleized-metadata-sys/pkg && npm link)
echo "# Install packages in merkleized-metadata"
(cd merkleized-metadata && npm i && npm link merkleized-metadata-sys)
echo "# Build merkleized-metadata"
(cd merkleized-metadata && npm run build)
echo "# Link merkleized-metadata"
(cd merkleized-metadata && npm link)
echo "# Install and build node example"
(cd examples/node && npm i && npm link merkleized-metadata-sys && npm link merkleized-metadata)
echo "# Install packages in web example"
(cd examples/web && npm i && npm link merkleized-metadata-sys && npm link merkleized-metadata)
echo "# Serve web example"
(cd examples/web && npx webpack serve)
```
