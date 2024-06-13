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
        let scale_bytes = hex::decode(hex)
            .map_err(|e| format!("Could not decode the given string into hex bytes: {e}"))?;
        let metadata = frame_metadata::RuntimeMetadata::decode(&mut &*scale_bytes)
            .map_err(|e| format!("Could not decode the given bytes into metadata: {e}"))?;
        Ok(RuntimeMetadata(metadata))
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

    pub fn type_def(&self) -> TypeDef {
        TypeDef(self.0.type_def.clone())
    }
}

#[wasm_bindgen]
pub struct TypeDef(merkleized_metadata::types::TypeDef);

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