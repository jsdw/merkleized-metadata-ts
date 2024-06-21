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

console.log("Initialized")

// Build our metadata object from the hex bytes obtained from
// state.getMetadata or the runtime API metadata.metadata_at_version(15):
const runtimeMetadata = RuntimeMetadata.fromHex(METADATA);

// Calculate the metadata digest and then hash it to get the metadata hash
// that we'd add to the signer payload for the CheckMetadataHash extension:
const digest = mm.generateMetadataDigest(runtimeMetadata, {
  base58Prefix: BASE58_PREFIX, // Eg 0 for Polkadot, 42 for Substrate
  decimals: DECIMALS,          // Eg 10 for Polkadot
  specName: SPEC_NAME,         // Eg "polkadot"
  specVersion: SPEC_VERSION,   // Eg 1_002_004 for Polkadot 1.2.4
  tokenSymbol: TOKEN_SYMBOL    // Eg "DOT"
});

console.log("Metadata Hash:", digest.hash())

// We can also build a proof which contains the information needed to
// decode a given extrinsic. This would be sent to devices like ledgers along
// with the above hash so that they could decode and use it to display an extrinsic.
const proof = mm.generateProofForExtrinsic(
    TX,                   // Hex for the transaction bytes
    TX_ADDITIONAL_SIGNED, // The bytes that extensions add to the signer payload (optional)
    runtimeMetadata
);

console.log("Extrinsic proof:", proof.encode())
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
(cd examples/node && npm i && npm link merkleized-metadata-sys && npm link merkleized-metadata && npm run start)
echo "# Install packages in web example"
(cd examples/web && npm i && npm link merkleized-metadata-sys && npm link merkleized-metadata)
echo "# Serve web example"
(cd examples/web && npx webpack serve)
```
