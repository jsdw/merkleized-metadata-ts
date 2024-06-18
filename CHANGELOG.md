# Changelog

## 0.0.7

- Inline WASM and re-generate the interface so that it can be used in browsers without any special bundling capabilities/WASM knowhow.
- Adjust the `RuntimeMetadata.from_hex` call to cater for all of the likely metadata hex formats.
- As a consequence of the above, the higher level interface now exposes an `init()` method which initializes the WASM and returns the methods.