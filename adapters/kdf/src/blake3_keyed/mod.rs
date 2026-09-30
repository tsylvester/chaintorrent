mod interface;
pub(crate) mod provides;
#[cfg(test)]
mod test;

use crate::factory::provides::{
    DerivationPurpose, DeriveKeyErrorReturn, DeriveKeyParams, DeriveKeyPayload, DeriveKeyReturn,
    DeriveKeySuccessReturn, IKeyDerivationAdapter, KDF_INTERFACE_VERSION, KdfDeclaration,
    KdfIdentifier,
};
use blake3::Hasher;
use domain::{Secret, SecretConstructorParams};
use interface::{
    BLAKE3_KEYED_ASSET_ROOT_CONTEXT, BLAKE3_KEYED_CAPSULE_RANDOMNESS_CONTEXT,
    BLAKE3_KEYED_IDENTITY_BASES_CONTEXT, BLAKE3_KEYED_MASTER_SCALAR_CONTEXT,
    BLAKE3_KEYED_PIECE_GROUP_KEY_CONTEXT, BLAKE3_KEYED_PLAINTEXT_ROOT_KEY_CONTEXT,
    BLAKE3_KEYED_PUBLISHER_ROOT_CONTEXT, BLAKE3_KEYED_WRAPPING_KEY_CONTEXT, Blake3KeyedKdf,
    Blake3KeyedKdfConstructorParams, Blake3KeyedKdfDeriveKeyErrorReturn,
    Blake3KeyedKdfTryNewReturn,
};
use zeroize::Zeroize;

impl Blake3KeyedKdf {
    pub const DECLARATION: KdfDeclaration = KdfDeclaration {
        identifier: KdfIdentifier::Blake3KeyedV1,
        adapter_version: 1,
        interface_version: KDF_INTERFACE_VERSION,
    };

    pub fn try_new(_params: Blake3KeyedKdfConstructorParams) -> Blake3KeyedKdfTryNewReturn {
        Ok(Blake3KeyedKdf)
    }
}

impl IKeyDerivationAdapter for Blake3KeyedKdf {
    fn derive_key(
        &self,
        params: DeriveKeyParams,
        payload: DeriveKeyPayload<'_>,
    ) -> DeriveKeyReturn {
        let material = payload.key_material.expose();
        let Ok(length) = u64::try_from(material.len()) else {
            return Err(DeriveKeyErrorReturn::Blake3Keyed(
                Blake3KeyedKdfDeriveKeyErrorReturn::KeyMaterialLengthExceedsPrefix {
                    length: material.len(),
                },
            ));
        };
        let context = match params.purpose {
            DerivationPurpose::WrappingKey => BLAKE3_KEYED_WRAPPING_KEY_CONTEXT,
            DerivationPurpose::PublisherRoot => BLAKE3_KEYED_PUBLISHER_ROOT_CONTEXT,
            DerivationPurpose::AssetRoot => BLAKE3_KEYED_ASSET_ROOT_CONTEXT,
            DerivationPurpose::MasterScalar => BLAKE3_KEYED_MASTER_SCALAR_CONTEXT,
            DerivationPurpose::IdentityBases => BLAKE3_KEYED_IDENTITY_BASES_CONTEXT,
            DerivationPurpose::CapsuleRandomness => BLAKE3_KEYED_CAPSULE_RANDOMNESS_CONTEXT,
            DerivationPurpose::PieceGroupKey => BLAKE3_KEYED_PIECE_GROUP_KEY_CONTEXT,
            DerivationPurpose::PlaintextRootKey => BLAKE3_KEYED_PLAINTEXT_ROOT_KEY_CONTEXT,
        };
        let mut hasher = Hasher::new_derive_key(context);
        hasher.update(&length.to_be_bytes());
        hasher.update(material);
        hasher.update(payload.context);
        let mut reader = hasher.finalize_xof();
        let mut buffer = vec![0u8; params.length];
        reader.fill(&mut buffer);
        reader.zeroize();
        hasher.zeroize();
        let Ok(key) = Secret::try_new(SecretConstructorParams { value: buffer });
        Ok(DeriveKeySuccessReturn { key })
    }
}
