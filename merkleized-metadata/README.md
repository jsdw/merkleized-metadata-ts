# merkleized-metadata

This is a simple TypeScript wrapper around https://github.com/bkchr/merkleized-metadata.

Basic example usage (where capitalized values need providing):

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