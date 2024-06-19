import {
    ExtraInfo as ExtraInfoSys,
    RuntimeMetadata as RuntimeMetadataSys,
    Type as TypeSys,
    Proof as ProofSys,
    MetadataDigest as MetadataDigestSys,
    SignedExtrinsicData as SignedExtrinsicDataSys,
    generate_metadata_digest as generate_metadata_digest_sys,
    generate_proof_for_extrinsic as generate_proof_for_extrinsic_sys,
    generate_proof_for_extrinsic_parts as generate_proof_for_extrinsic_parts_sys,
    init as init_sys,
} from 'merkleized-metadata-sys'

/**
 * Extra information that is required to generate the MetadataDigest.
 */
export type ExtraInfo = {
    /** The spec version of the runtime. */
    specVersion: number,
    /** The spec name of the runtime. */
    specName: string,
    /** The base58 prefix for addresses. */
    base58Prefix: number,
    /** The number of decimals of the primary token. */
    decimals: number,
    /** The token symbol of the primary token. */
    tokenSymbol: string
}

/**
 * The metadata that we'll use.
 */
export class RuntimeMetadata {
    #metadata: RuntimeMetadataSys

    /**
     * Convert a hex encoded SCALE encoded string into a `RuntimeMetadata`
     * object.
     */
    static fromHex(metadataHex: string): RuntimeMetadata {
        if (metadataHex.startsWith('0x') || metadataHex.startsWith('0X')) {
            metadataHex = metadataHex.slice(2);
        }
        return new RuntimeMetadata(metadataHex)
    }

    // Private so that we can have explicit static methods to convert
    // from hex or JSON or whatever.
    private constructor(metadataHex: string) {
        this.#metadata = RuntimeMetadataSys.from_hex(metadataHex);
    }

    __getRuntimeMetadataSys(): RuntimeMetadataSys {
        return this.#metadata
    }
}

/**
 * The metadata digest.
 */
export class MetadataDigest {
    #digest: MetadataDigestSys

    constructor(metadataDigestSys: MetadataDigestSys) {
        this.#digest = metadataDigestSys
    }

    /**
     * Return the hash of this digest.
     */
    hash(): string {
        return this.#digest.hash()
    }
}

/**
 * A proof containing all the nodes to decode a specific extrinsic.
 */
export class Proof {
    #proof: ProofSys

    constructor(proofSys: ProofSys) {
        this.#proof = proofSys
    }

    /**
     * SCALE encode this proof to a hex string.
     */
    encode(): String {
        return this.#proof.encode()
    }

    /**
     * The leaves of the tree.
     *
     * They are sorted that the left most leaves are first.
     */
    *leaves(): Generator<Type, void, unknown> {
        for (const leaf of this.#proof.leaves()) {
            yield new Type(leaf)
        }
    }

    /**
     * The indices of the leaves in the tree, in the same order as leaves.
     */
    *leafIndices(): Generator<number, void, unknown> {
        for (const index of this.#proof.leaf_indices()) {
            yield index
        }
    }

    /**
    * All the node hashes that can not be calculated out of the leaves.
    *
    * These are all the nodes that are required to proof that all the leaves are part of the same merkle tree.
    *
    * They are sorted from left to right, from the root to the leaf.
    */
    *nodes(): Generator<string, void, unknown> {
        for (const node of this.#proof.nodes()) {
            yield node
        }
    }
}

export class Type {
    #type: TypeSys

    constructor(typeSys: TypeSys) {
        this.#type = typeSys
    }

    /**
     * Returns the hash of this type.
     */
    hash(): string {
        return this.#type.hash()
    }

    /**
     * The unique id of this type.
     */
    typeId(): number {
        return this.#type.type_id()
    }
}

export type SignedExtrinsicData = {
    includedInExtrinsic: string,
    includedInSignedData: string
}

export type Methods = Awaited<ReturnType<typeof init>>;

/**
 * Initialize the merkleized-metadata WASM, returning the available methods.
 *
 * This can safely be called multiple times and will only initialize things once.
 */
export function init() {
    return init_sys().then(() => {
        return {
            /**
             * This generates the MetadataDigest for the given metadata. The hash of this digest is what is called the
             * “metadata hash” in [RFC-78](https://polkadot-fellows.github.io/RFCs/approved/0078-merkleized-metadata.html).
             */
            generateMetadataDigest(metadata: RuntimeMetadata, extraInfo: ExtraInfo): MetadataDigest {
                const metadataSys = metadata.__getRuntimeMetadataSys();
                const extraInfoSys = ExtraInfoSys.from_opts(
                    extraInfo.specVersion,
                    extraInfo.specName,
                    extraInfo.base58Prefix,
                    extraInfo.decimals,
                    extraInfo.tokenSymbol
                );

                const digest = generate_metadata_digest_sys(metadataSys, extraInfoSys);
                return new MetadataDigest(digest)
            },

            /**
             * Generate a proof for the given extrinsic using the given metadata.
             *
             * If `additional_data` is provided, it will be decoded as well and the required type information are included in the proof.
             *
             * If the full extrinsic is not available, `generateProofForExtrinsicParts` is maybe the better option as it only requires the call and the additional data.
             */
            generateProofForExtrinsic(extrinsicHex: string, additionalSignedHex: string | undefined, metadata: RuntimeMetadata): Proof {
                const proof = generate_proof_for_extrinsic_sys(extrinsicHex, additionalSignedHex, metadata.__getRuntimeMetadataSys());
                return new Proof(proof)
            },

            /**
             * Generate a proof for the given extrinsic parts using the given metadata.
             *
             * This generates a proof that contains all the types required to decode an extrinsic that is build using the given `callHex` and `signedExtData`. When `signedExtData` is not
             * undefined, it is assumed that the extrinsic is signed and thus all the signed extension types are included in the proof as well. The same applies for the signature and
             * address types which are only included when `signedExtData` is not undefined.
             */
            generateProofForExtrinsicParts(callHex: string, signedExtData: SignedExtrinsicData | undefined, metadata: RuntimeMetadata): Proof {
                const signedExtDataSys = typeof signedExtData !== "undefined"
                    ? SignedExtrinsicDataSys.from_bytes(signedExtData.includedInExtrinsic, signedExtData.includedInSignedData)
                    : undefined;

                const proof = generate_proof_for_extrinsic_parts_sys(callHex, signedExtDataSys, metadata.__getRuntimeMetadataSys());
                return new Proof(proof)
            }
        }
    })
}
