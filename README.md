# merkleized-metadata-ts

This is a simple TypeScript wrapper around https://github.com/bkchr/merkleized-metadata.

This consists of two packages:

- `merkleized-metadata-sys`: low level WASM bindings to the rust crate.
- `merkleized-metadata`: a higher level library which wraps a slightly nicer interface over this.

It's expected that people will import `merkleized-metadata` in their TS projects.

Basic example usage of the `merkleized-metadata` package:

```ts
import { init, RuntimeMetadata } from 'merkleized-metadata'

// Initialize the package:
const mm = await init();

// Given some `metadataHex` representing our current metadata,
// we create a `RuntimeMetadata` object:
const runtimeMetadata = RuntimeMetadata.fromHex(metadataHex);

// Now, given some other details, we can create a metadata hash:
const digest = mm.generateMetadataDigest(runtimeMetadata, {
  base58Prefix, // eg 42 for Substrate
  decimals,     // eg 12: decimal places used in token.
  specName,     // eg "rococo"
  specVersion,  // eg 1000000
  tokenSymbol,  // eg "DOT"
});
console.log("Metadata Hash:", digest.hash())

// We can also create and hex encode the proof that devices like ledger
// will use to decode the extrinsic again:
const proof = mm.generateProofForExtrinsic(extrinsicHec, additionalSignedHex, runtimeMetadata);
console.log("Encoded proof:", proof.encode()
```

# Developer Notes

## Publishing a new version

1. Clean things up:
   ```
   (cd merkleized-metadata-sys && rm -rf pkg target)
   (cd merkleized-metadata && rm -rf node_modules dist)
   (cd examples/web && rm -rf node_modules dist)
   (cd examples/node && rm -rf node_modules)
   ```
1. Increment version number in `merkleized-metadata-sys/Cargo.toml`.
2. Build/publish the raw TS interface:
   ```
   cd merkleized-metadata-sys
   wasm-pack build --release
   (cd inline_wasm && cargo run -- ../pkg)
   (cd pkg && npm publish)
   cd ..
   ```
3. Increment version of self and sys crate in `merkleized-metadata/package.json`.
4. Build/publish the high level TS interface:
   ```
   (cd merkleized-metadata && npm i && npm run build && npm publish)
   ```
5. Test the examples as a final sanity check (should be done during dev, below, anyway):
   ```
   (cd examples/node && npm i && npm run start)
   (cd examples/web && npm i && npx webpack serve)
   ```
   Open browser to http://localhost:8080 to confirm web example works.

Bump versions used in `examples/web` and `(cd examples/web && npm i && npx webpack serve)` to verify it still works;

## Testing dev changes

The following will build and link everything such that we'll serve and see any changes made in the `examples/web`.
Without the linking steps, we'd be using published versions and not local versions of things.

```
echo "# Clean things up"
(cd merkleized-metadata-sys && rm -rf pkg target)
(cd merkleized-metadata && rm -rf node_modules dist)
(cd examples/web && rm -rf node_modules dist)
(cd examples/node && rm -rf node_modules)

echo "# Build and link merkleized-metadata-sys"
(cd merkleized-metadata-sys && wasm-pack build --release)
(cd merkleized-metadata-sys/inline_wasm && cargo run -- ../pkg)
(cd merkleized-metadata-sys/pkg && npm link)

echo "# Build and link merkleized-metadata"
(cd merkleized-metadata && npm i && npm link merkleized-metadata-sys)
(cd merkleized-metadata && npm run build)
(cd merkleized-metadata && npm link)

echo "# Install and build node example"
(cd examples/node && npm i && npm link merkleized-metadata-sys && npm link merkleized-metadata)
echo "# Install packages in web example"
(cd examples/web && npm i && npm link merkleized-metadata-sys && npm link merkleized-metadata)
echo "# Serve web example"
(cd examples/web && npx webpack serve)
```
