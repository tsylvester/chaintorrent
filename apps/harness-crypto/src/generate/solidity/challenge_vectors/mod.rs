#[cfg(test)]
mod integration_test;
mod interface;
#[cfg(any(test, feature = "mocks"))]
mod mock;
pub mod provides;
#[cfg(test)]
mod test_description;
#[cfg(test)]
mod test_error;

use chain::IChainForms;
use encoding::{
    CanonicalFieldKind, CanonicalFieldReader, CanonicalFieldValue, CanonicalFields, EncodeParams,
    FromFieldsParams, FromFieldsReturn, FromFieldsSuccessReturn, IEncoderAdapter,
    IEncodingContract, ToFieldsParams, ToFieldsReturn, ToFieldsSuccessReturn,
};
use envelope::{EnvelopeComponents, PublicKeysComponents};
use hash_to_scalar::{
    DomainTag, DomainTagConstructorParams, HashToScalarParams, HashToScalarPayload,
};
use pairing::{
    EncodeScalarParams, EncodeScalarPayload, G1GeneratorParams, G1GeneratorPayload,
    G2GeneratorParams, G2GeneratorPayload, IPairingAdapter, ISampleUniformScalar, MulG1Params,
    MulG1Payload, MulG2Params, MulG2Payload, SampleUniformScalarParams, SampleUniformScalarPayload,
};
use proof::{
    DELIVERY_STATEMENT_VERSION_ONE, MintFirstMessages, MintStatement, MintStatementDescription,
    MintStatementDescriptionConstructorParams, TransferFirstMessages, TransferStatement,
    TransferStatementDescription, TransferStatementDescriptionConstructorParams,
};
use random::{FillBytesParams, FillBytesPayload};

use self::interface::{
    CHALLENGE_VECTOR_FIELD_COUNT, CHALLENGE_VECTOR_FIELD_KINDS, CHALLENGE_VECTOR_MEMBER_NAMES,
    ChallengeVector, ChallengeVectorDescription, ChallengeVectorDescriptionConstructorParams,
    ChallengeVectorDescriptionTryNewReturn, ChallengeVectorFromFieldsErrorReturn,
    ChallengeVectorMemberNamesErrorReturn, ChallengeVectorMemberNamesReturn, ChallengeVectorsDeps,
    ChallengeVectorsErrorReturn, ChallengeVectorsParams, ChallengeVectorsPayload,
    ChallengeVectorsReturn, ChallengeVectorsSuccessReturn,
};
use super::render::provides::{SolidityMemberName, SolidityMemberNameConstructorParams};

impl ChallengeVectorDescription {
    pub fn try_new(
        _params: ChallengeVectorDescriptionConstructorParams,
    ) -> ChallengeVectorDescriptionTryNewReturn {
        Ok(ChallengeVectorDescription)
    }

    pub fn member_names(&self) -> ChallengeVectorMemberNamesReturn {
        let mut names = Vec::with_capacity(CHALLENGE_VECTOR_FIELD_COUNT);
        for text in CHALLENGE_VECTOR_MEMBER_NAMES {
            match SolidityMemberName::try_new(SolidityMemberNameConstructorParams {
                text: text.to_string(),
            }) {
                Ok(name) => names.push(name),
                Err(error) => {
                    return Err(ChallengeVectorMemberNamesErrorReturn::MemberName(error));
                }
            }
        }
        Ok(names)
    }
}

impl IEncodingContract for ChallengeVectorDescription {
    type Described = ChallengeVector;
    type FromFieldsErrorReturn = ChallengeVectorFromFieldsErrorReturn;
    const FIELDS: &'static [CanonicalFieldKind] = &CHALLENGE_VECTOR_FIELD_KINDS;

    fn to_fields(&self, _params: ToFieldsParams, payload: &Self::Described) -> ToFieldsReturn {
        Ok(ToFieldsSuccessReturn {
            fields: CanonicalFields {
                values: vec![
                    CanonicalFieldValue::Unsigned16(payload.purpose),
                    CanonicalFieldValue::Bytes(payload.draws.clone()),
                    CanonicalFieldValue::Bytes(payload.encoding.clone()),
                    CanonicalFieldValue::Unsigned256(payload.challenge),
                ],
            },
        })
    }

    fn fields_to_value(
        &self,
        _params: FromFieldsParams,
        payload: CanonicalFields,
    ) -> FromFieldsReturn<Self::Described, Self::FromFieldsErrorReturn> {
        let mut reader = CanonicalFieldReader::try_new(payload, Self::FIELDS)
            .map_err(ChallengeVectorFromFieldsErrorReturn::Fields)?;
        let purpose = reader
            .read_unsigned16()
            .map_err(ChallengeVectorFromFieldsErrorReturn::Fields)?;
        let draws = reader
            .read_bytes()
            .map_err(ChallengeVectorFromFieldsErrorReturn::Fields)?;
        let encoding = reader
            .read_bytes()
            .map_err(ChallengeVectorFromFieldsErrorReturn::Fields)?;
        let challenge = reader
            .read_unsigned256()
            .map_err(ChallengeVectorFromFieldsErrorReturn::Fields)?;
        Ok(FromFieldsSuccessReturn {
            described: ChallengeVector {
                purpose,
                draws,
                encoding,
                challenge,
            },
        })
    }
}

pub fn challenge_vectors<'a, P: IPairingAdapter, E: IEncoderAdapter, F: IChainForms>(
    deps: &ChallengeVectorsDeps<'a, P, E>,
    params: ChallengeVectorsParams,
    payload: ChallengeVectorsPayload<'_, F>,
) -> ChallengeVectorsReturn {
    if params.statement_version != DELIVERY_STATEMENT_VERSION_ONE {
        return Err(ChallengeVectorsErrorReturn::UnsupportedStatementVersion {
            version: params.statement_version,
        });
    }
    let tag = match DomainTag::try_new(DomainTagConstructorParams {
        bytes: payload.delivery_proof.challenge_tag.to_vec(),
    }) {
        Ok(tag) => tag,
        Err(error) => return Err(ChallengeVectorsErrorReturn::DomainTag(error)),
    };
    let Ok(g1) = deps
        .pairing
        .g1_generator(G1GeneratorParams, G1GeneratorPayload);
    let Ok(g2) = deps
        .pairing
        .g2_generator(G2GeneratorParams, G2GeneratorPayload);

    let scalar_bytes = |scalar: &P::Scalar| -> Result<[u8; 32], ChallengeVectorsErrorReturn> {
        let Ok(encoded) = deps.pairing.encode_scalar(
            EncodeScalarParams,
            EncodeScalarPayload {
                scalar: scalar.clone(),
            },
        );
        let bytes: &[u8] = encoded.bytes.expose().as_ref();
        match <[u8; 32]>::try_from(bytes) {
            Ok(array) => Ok(array),
            Err(_error) => Err(ChallengeVectorsErrorReturn::ScalarLength {
                actual: bytes.len(),
            }),
        }
    };
    let draw_scalar = || -> Result<P::Scalar, ChallengeVectorsErrorReturn> {
        let uniform = match deps.random.fill_bytes(
            FillBytesParams,
            FillBytesPayload {
                length: <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH,
            },
        ) {
            Ok(uniform) => uniform,
            Err(error) => return Err(ChallengeVectorsErrorReturn::FillBytes(error)),
        };
        let sampled = match <P::Scalar as ISampleUniformScalar>::sample_from_uniform_bytes(
            SampleUniformScalarParams,
            SampleUniformScalarPayload {
                uniform: uniform.bytes,
            },
        ) {
            Ok(sampled) => sampled,
            Err(error) => return Err(ChallengeVectorsErrorReturn::SampleScalar(error)),
        };
        Ok(sampled.scalar.expose().clone())
    };
    let draw_g1 = |draws: &mut Vec<u8>| -> Result<P::G1, ChallengeVectorsErrorReturn> {
        let scalar = draw_scalar()?;
        let Ok(product) = deps.pairing.mul_g1(
            MulG1Params,
            MulG1Payload {
                point: g1.point.clone(),
                scalar: scalar.clone(),
            },
        );
        draws.extend_from_slice(&scalar_bytes(&scalar)?);
        Ok(product.product)
    };
    let draw_g2 = |draws: &mut Vec<u8>| -> Result<P::G2, ChallengeVectorsErrorReturn> {
        let scalar = draw_scalar()?;
        let Ok(product) = deps.pairing.mul_g2(
            MulG2Params,
            MulG2Payload {
                point: g2.point.clone(),
                scalar: scalar.clone(),
            },
        );
        draws.extend_from_slice(&scalar_bytes(&scalar)?);
        Ok(product.product)
    };

    let mut records = Vec::new();
    for _ in 0..params.count.get() {
        let mut draws = Vec::new();
        let buyer_keys = PublicKeysComponents {
            pk1: draw_g1(&mut draws)?,
            pk2: draw_g2(&mut draws)?,
        };
        let envelope = EnvelopeComponents {
            c1: draw_g1(&mut draws)?,
            c2: draw_g1(&mut draws)?,
            d1: draw_g2(&mut draws)?,
            d2: draw_g2(&mut draws)?,
        };
        let first_messages = MintFirstMessages {
            hpub: draw_g2(&mut draws)?,
            c1: draw_g1(&mut draws)?,
            c2: draw_g1(&mut draws)?,
            d1: draw_g2(&mut draws)?,
            d2: draw_g2(&mut draws)?,
        };
        let context = payload.mint.clone();
        let statement = MintStatement {
            suite_identifier: context.suite_identifier,
            chain: context.chain,
            entitlement_contract: context.entitlement_contract,
            asset_root: context.asset_root,
            parameter_set_identifier: context.parameter_set_identifier,
            source_entitlement: context.source_entitlement,
            target_entitlement: context.target_entitlement,
            old_interval: context.old_interval,
            new_interval: context.new_interval,
            purpose: context.purpose,
            seller: context.seller,
            buyer: context.buyer,
            buyer_keys,
            envelope,
            expiry: context.expiry,
            first_messages,
        };
        let Ok(description) =
            MintStatementDescription::<P, F>::try_new(MintStatementDescriptionConstructorParams {
                pairing: deps.pairing,
            });
        let bytes = match deps.encoder.encode(
            EncodeParams {
                description: &description,
            },
            &statement,
        ) {
            Ok(success) => success.bytes,
            Err(error) => return Err(ChallengeVectorsErrorReturn::Encoding(error)),
        };
        let mapped = match deps.hash_to_scalar.hash_to_scalar(
            HashToScalarParams { tag: &tag },
            HashToScalarPayload { message: &bytes },
        ) {
            Ok(mapped) => mapped,
            Err(error) => return Err(ChallengeVectorsErrorReturn::HashToScalar(error)),
        };
        records.push(ChallengeVector {
            purpose: statement.purpose.code(),
            draws,
            encoding: bytes,
            challenge: scalar_bytes(&mapped.scalar)?,
        });

        let mut draws = Vec::new();
        let seller_keys = PublicKeysComponents {
            pk1: draw_g1(&mut draws)?,
            pk2: draw_g2(&mut draws)?,
        };
        let buyer_keys = PublicKeysComponents {
            pk1: draw_g1(&mut draws)?,
            pk2: draw_g2(&mut draws)?,
        };
        let old_envelope = EnvelopeComponents {
            c1: draw_g1(&mut draws)?,
            c2: draw_g1(&mut draws)?,
            d1: draw_g2(&mut draws)?,
            d2: draw_g2(&mut draws)?,
        };
        let new_envelope = EnvelopeComponents {
            c1: draw_g1(&mut draws)?,
            c2: draw_g1(&mut draws)?,
            d1: draw_g2(&mut draws)?,
            d2: draw_g2(&mut draws)?,
        };
        let first_messages = TransferFirstMessages {
            pk1: draw_g1(&mut draws)?,
            c1: draw_g1(&mut draws)?,
            c2: draw_g1(&mut draws)?,
            pk2: draw_g2(&mut draws)?,
            d1: draw_g2(&mut draws)?,
            d2: draw_g2(&mut draws)?,
        };
        let context = payload.transfer.clone();
        let statement = TransferStatement {
            suite_identifier: context.suite_identifier,
            chain: context.chain,
            entitlement_contract: context.entitlement_contract,
            asset_root: context.asset_root,
            parameter_set_identifier: context.parameter_set_identifier,
            source_entitlement: context.source_entitlement,
            target_entitlement: context.target_entitlement,
            old_interval: context.old_interval,
            new_interval: context.new_interval,
            purpose: context.purpose,
            seller: context.seller,
            buyer: context.buyer,
            seller_keys,
            buyer_keys,
            old_envelope,
            new_envelope,
            expiry: context.expiry,
            first_messages,
        };
        let Ok(description) = TransferStatementDescription::<P, F>::try_new(
            TransferStatementDescriptionConstructorParams {
                pairing: deps.pairing,
            },
        );
        let bytes = match deps.encoder.encode(
            EncodeParams {
                description: &description,
            },
            &statement,
        ) {
            Ok(success) => success.bytes,
            Err(error) => return Err(ChallengeVectorsErrorReturn::Encoding(error)),
        };
        let mapped = match deps.hash_to_scalar.hash_to_scalar(
            HashToScalarParams { tag: &tag },
            HashToScalarPayload { message: &bytes },
        ) {
            Ok(mapped) => mapped,
            Err(error) => return Err(ChallengeVectorsErrorReturn::HashToScalar(error)),
        };
        records.push(ChallengeVector {
            purpose: statement.purpose.code(),
            draws,
            encoding: bytes,
            challenge: scalar_bytes(&mapped.scalar)?,
        });
    }
    Ok(ChallengeVectorsSuccessReturn { records })
}
