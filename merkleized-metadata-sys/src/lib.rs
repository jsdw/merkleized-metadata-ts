mod utils;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct ExtraInfo(merkleized_metadata::ExtraInfo);

#[wasm_bindgen]
impl ExtraInfo {
    pub fn from_opts(spec_version: u32, spec_name: String, base58_prefix: u16, decimals: u8, token_symbol: String) -> ExtraInfo {
        ExtraInfo(merkleized_metadata::ExtraInfo { spec_version, spec_name, base58_prefix, decimals, token_symbol })
    }
}

#[wasm_bindgen]
pub struct RuntimeMetadata(frame_metadata::RuntimeMetadata);

#[wasm_bindgen]
impl RuntimeMetadata {
    pub fn from_hex(hex: &str) -> Result<RuntimeMetadata, String> {
        use parity_scale_codec::Decode;
        const META_RESERVED: u32 = 0x6174656d;

        let hex = hex.strip_prefix("0x").unwrap_or(hex);
        let scale_bytes = hex::decode(hex)
            .map_err(|e| format!("Could not decode the given string into hex bytes: {e}"))?;

        // Check that all bytes are consumed during decode, and error if this isn't the case.
        fn decode_consuming_all<T: Decode>(cursor: &mut &[u8], type_name: &str) -> Result<T, String> {
            let r = T::decode(cursor);
            if !cursor.is_empty() {
                return Err(format!("Decoding into {} failed to consume all bytes ({} left)", type_name, cursor.len()).into())
            }
            r.map_err(|e| e.to_string())
        }

        // Option<OpaqueMetadata> comes from metadata.metadata_at_version.
        let decode_option_opaque = |bytes: &[u8]| {
            let cursor = &mut &*bytes;
            decode_consuming_all::<Option::<frame_metadata::OpaqueMetadata>>(cursor, "Option<OpaqueMetadata>").and_then(|option_opaque| {
                if let Some(opaque) = option_opaque {
                    decode_consuming_all::<frame_metadata::RuntimeMetadataPrefixed>(&mut &*opaque.0, "RuntimeMetadataPrefixed (from Option<OpaqueMetadata>)")
                } else {
                    Err("Expected Option<OpaqueMetadata> to be Some, but got None".into())
                }
            })
        };

        // OpaqueMetadata comes from legacy state.getMetadata.
        let decode_opaque = |bytes: &[u8]| {
            let cursor = &mut &*bytes;
            decode_consuming_all::<frame_metadata::OpaqueMetadata>(cursor, "OpaqueMetadata").and_then(|opaque| {
                decode_consuming_all::<frame_metadata::RuntimeMetadataPrefixed>(&mut &*opaque.0, "RuntimeMetadataPrefixed (from OpaqueMetadata)")
            })
        };

        // Tools like Subxt hand back RuntimeMetadataPrefixed directly.
        let decode_runtime_metadata_prefixed = |bytes: &[u8]| {
            let cursor = &mut &*bytes;
            decode_consuming_all::<frame_metadata::RuntimeMetadataPrefixed>(cursor, "RuntimeMetadataPrefixed (directly)")
        };

        // Decode in order of runtime API, then legacy RPC, then tools output.
        let runtime_metadata_prefixed = decode_option_opaque(&*scale_bytes)
            .or_else(|_| decode_opaque(&*scale_bytes))
            .or_else(|_| decode_runtime_metadata_prefixed(&*scale_bytes))
            .map_err(|e| format!("Could not decode metadata bytes into Option<OpaqueMetadata>, OpaqueMetadata or RuntimeMetadataPrefixed: {e}"))?;

        if runtime_metadata_prefixed.0 != META_RESERVED {
            return Err(format!("RuntimeMetadataPrefixed should begin with {META_RESERVED:#02x} but got {:#02x}", runtime_metadata_prefixed.0).into())
        }

        Ok(RuntimeMetadata(runtime_metadata_prefixed.1))
    }
}

#[wasm_bindgen]
pub struct MetadataDigest(merkleized_metadata::types::MetadataDigest);

#[wasm_bindgen]
impl MetadataDigest {
    pub fn hash(&self) -> String {
        let metadata_hash = self.0.hash();
        hex::encode(metadata_hash)
    }
}

#[wasm_bindgen]
pub struct Proof(merkleized_metadata::Proof);

#[wasm_bindgen]
impl Proof {
    pub fn leaves(&self) -> Vec<Type> {
        self.0.leaves.iter().map(|leaf| Type(leaf.clone())).collect()
    }

    pub fn leaf_indices(&self) -> Vec<u32> {
        self.0.leaf_indices.clone()
    }

    pub fn nodes(&self) -> Vec<String> {
        self.0.nodes.iter().map(|hash| hex::encode(hash)).collect()
    }
}

#[wasm_bindgen]
pub struct Type(merkleized_metadata::types::Type);

#[wasm_bindgen]
impl Type {
    pub fn hash(&self) -> String {
        let type_hash = self.0.hash();
        hex::encode(type_hash)
    }

    pub fn type_id(&self) -> u32 {
        self.0.type_id.0
    }
}

#[wasm_bindgen]
pub struct SignedExtrinsicData {
    in_extrinsic_hex: String,
    in_signed_data_hex: String
}

#[wasm_bindgen]
impl SignedExtrinsicData {
    pub fn from_bytes(in_extrinsic_hex: String, in_signed_data_hex: String) -> SignedExtrinsicData {
        SignedExtrinsicData {
            in_extrinsic_hex,
            in_signed_data_hex,
        }
    }
}

#[wasm_bindgen]
pub fn generate_metadata_digest(metadata: &RuntimeMetadata, extra_info: ExtraInfo) -> Result<MetadataDigest, String> {
    let digest = merkleized_metadata::generate_metadata_digest(&metadata.0, extra_info.0)?;
    Ok(MetadataDigest(digest))
}

#[wasm_bindgen]
pub fn generate_proof_for_extrinsic(extrinsic_hex: String, additional_signed_hex: Option<String>, metadata: &RuntimeMetadata) -> Result<Proof, String> {
    let extrinsic = hex::decode(extrinsic_hex)
        .map_err(|e| format!("Could not decode extrinsic bytes from hex: {e}"))?;
    let additional_signed = additional_signed_hex
        .map(|s| hex::decode(s))
        .transpose()
        .map_err(|e| format!("Could not decode additional signed bytes from hex: {e}"))?;

    let proof = merkleized_metadata::generate_proof_for_extrinsic(&extrinsic, additional_signed.as_deref(), &metadata.0)?;
    Ok(Proof(proof))
}

#[wasm_bindgen]
pub fn generate_proof_for_extrinsic_parts(call_hex: String, signed_ext_data: Option<SignedExtrinsicData>, metadata: &RuntimeMetadata) -> Result<Proof, String> {
    let call = hex::decode(call_hex)
        .map_err(|e| format!("Could not decode call data bytes from hex: {e}"))?;

    let signed_ext_data_parts = signed_ext_data.map(|d| {
        let in_extrinsic = hex::decode(d.in_extrinsic_hex)
            .map_err(|e| format!("Could not decode 'in extrinsic payload' bytes from hex: {e}"))?;
        let in_signed_data = hex::decode(d.in_signed_data_hex)
            .map_err(|e| format!("Could not decode 'in signed data' bytes from hex: {e}"))?;
        Ok::<_,String>((in_extrinsic, in_signed_data))
    }).transpose()?;

    let signed_ext_data = signed_ext_data_parts.as_ref().map(|(in_extrinsic, in_signed_data)| {
        merkleized_metadata::SignedExtrinsicData {
            included_in_extrinsic: &in_extrinsic,
            included_in_signed_data: &in_signed_data,
        }
    });

    let proof = merkleized_metadata::generate_proof_for_extrinsic_parts(&call, signed_ext_data, &metadata.0)?;
    Ok(Proof(proof))
}