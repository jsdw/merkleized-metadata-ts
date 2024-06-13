# merkleized-metadata-ts

This is a simple TypeScript wrapper around https://github.com/bkchr/merkleized-metadata.

This consists of two packages.

- `merkleized-metadata-sys`: low level WASM bindings to the rust crate.
- `merkleized-metadata`: a higher level library which wraps a slightly nicer interface over this.

It's expected that people will import `merkleized-metadata` in their TS projects.

# Publshing a new version

```
cd merkleized-metadata-sys
wasm-pack build
(cd pkg && npm publish)

cd ..

cd merkleized-metadata
npm run build
npm publish
```