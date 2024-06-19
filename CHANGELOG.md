# Changelog

## merkleized-metadata-sys 0.0.8 + merkleized-metadata 0.0.4

- Add `Proof.encode()` to encode the proof to hex so that it can be sent to a ledger device or similar to decode the extrinsic.
- Rejig things a little more to make the packages work in NodeJS as well as browser envs.

## merkleized-metadata-sys 0.0.7 + merkleized-metadata 0.0.3

- Inline WASM and re-generate the interface so that it can be used in browsers without any special bundling capabilities/WASM knowhow.
- Adjust the `RuntimeMetadata.from_hex` call to cater for all of the likely metadata hex formats.
- As a consequence of the above, the higher level interface now exposes an `init()` method which initializes the WASM and returns the methods.