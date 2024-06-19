import { init, RuntimeMetadata } from 'merkleized-metadata'
import * as defaults from './defaults.js'

const mm = await init();

console.log("Initialized")

const runtimeMetadata = RuntimeMetadata.fromHex(defaults.METADATA);
const digest = mm.generateMetadataDigest(runtimeMetadata, {
  base58Prefix: defaults.BASE58_PREFIX,
  decimals: defaults.DECIMALS,
  specName: defaults.SPEC_NAME,
  specVersion: defaults.SPEC_VERSION,
  tokenSymbol: defaults.TOKEN_SYMBOL
});

console.log("Metadata Hash:", digest.hash())