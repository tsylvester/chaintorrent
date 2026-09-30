mod interface;
pub(crate) mod provides;
#[cfg(test)]
mod test;

use crate::factory::provides::{
    HASH_TO_SCALAR_INTERFACE_VERSION, HashToScalarDeclaration, HashToScalarErrorReturn,
    HashToScalarIdentifier, HashToScalarParams, HashToScalarPayload, HashToScalarReturn,
    HashToScalarSuccessReturn, IHashToScalarAdapter,
};
use domain::{Secret, SecretConstructorParams};
use interface::{
    KECCAK256_DIGEST_LENGTH, Keccak256HashToScalar, Keccak256HashToScalarConstructorParams,
    Keccak256HashToScalarErrorReturn, Keccak256HashToScalarTryNewReturn,
};
use pairing::{ISampleUniformScalar, SampleUniformScalarParams, SampleUniformScalarPayload};
use sha3::{Digest, Keccak256};

impl Keccak256HashToScalar {
    pub const DECLARATION: HashToScalarDeclaration = HashToScalarDeclaration {
        identifier: HashToScalarIdentifier::Keccak256V1,
        adapter_version: 1,
        interface_version: HASH_TO_SCALAR_INTERFACE_VERSION,
    };

    pub fn try_new(
        _params: Keccak256HashToScalarConstructorParams,
    ) -> Keccak256HashToScalarTryNewReturn {
        Ok(Keccak256HashToScalar)
    }
}

impl<S: ISampleUniformScalar + Clone> IHashToScalarAdapter<S> for Keccak256HashToScalar {
    fn hash_to_scalar(
        &self,
        params: HashToScalarParams<'_>,
        payload: HashToScalarPayload<'_>,
    ) -> HashToScalarReturn<S> {
        let length = params.tag.as_bytes().len();
        let Ok(prefix) = u8::try_from(length) else {
            return Err(HashToScalarErrorReturn::Keccak256(
                Keccak256HashToScalarErrorReturn::TagLengthExceedsPrefix { length },
            ));
        };
        let Some(offset) = S::UNIFORM_BYTES_LENGTH.checked_sub(KECCAK256_DIGEST_LENGTH) else {
            return Err(HashToScalarErrorReturn::Keccak256(
                Keccak256HashToScalarErrorReturn::UniformLengthBelowDigest {
                    uniform_length: S::UNIFORM_BYTES_LENGTH,
                    digest_length: KECCAK256_DIGEST_LENGTH,
                },
            ));
        };
        let mut hasher = Keccak256::new();
        hasher.update([prefix]);
        hasher.update(params.tag.as_bytes());
        hasher.update(payload.message);
        let digest = hasher.finalize();
        let mut buffer = vec![0u8; S::UNIFORM_BYTES_LENGTH];
        buffer[offset..].copy_from_slice(&digest);
        let Ok(uniform) = Secret::try_new(SecretConstructorParams { value: buffer });
        let success = match S::sample_from_uniform_bytes(
            SampleUniformScalarParams,
            SampleUniformScalarPayload { uniform },
        ) {
            Ok(success) => success,
            Err(error) => {
                return Err(HashToScalarErrorReturn::Keccak256(
                    Keccak256HashToScalarErrorReturn::Sampling(error),
                ));
            }
        };
        Ok(HashToScalarSuccessReturn {
            scalar: success.scalar.expose().clone(),
        })
    }
}
