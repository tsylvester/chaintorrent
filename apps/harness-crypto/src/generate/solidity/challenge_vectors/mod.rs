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
    CanonicalFieldKind, CanonicalFieldValue, CanonicalFields, EncodeParams, FromFieldsParams,
    FromFieldsReturn, FromFieldsSuccessReturn, IEncoderAdapter, IEncodingContract, ToFieldsParams,
    ToFieldsReturn, ToFieldsSuccessReturn,
};
use envelope::{EnvelopeComponents, PublicKeysComponents};
use hash_to_scalar::{
    DomainTag, DomainTagConstructorParams, HashToScalarParams, HashToScalarPayload,
};
use hashing::{Root, RootConstructorParams};
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

use super::render::provides::{SolidityMemberName, SolidityMemberNameConstructorParams};
use interface::{
    CHALLENGE_VECTOR_FIELD_COUNT, CHALLENGE_VECTOR_FIELD_KINDS, CHALLENGE_VECTOR_MEMBER_NAMES,
    ChallengeVector, ChallengeVectorDescription, ChallengeVectorDescriptionConstructorParams,
    ChallengeVectorDescriptionTryNewReturn, ChallengeVectorFromFieldsErrorReturn,
    ChallengeVectorMemberNamesErrorReturn, ChallengeVectorMemberNamesReturn, ChallengeVectorsDeps,
    ChallengeVectorsErrorReturn, ChallengeVectorsParams, ChallengeVectorsPayload,
    ChallengeVectorsReturn, ChallengeVectorsSuccessReturn,
};

impl ChallengeVectorDescription {
    pub fn try_new(
        _params: ChallengeVectorDescriptionConstructorParams,
    ) -> ChallengeVectorDescriptionTryNewReturn {
        Ok(ChallengeVectorDescription)
    }

    pub fn member_names(&self) -> ChallengeVectorMemberNamesReturn {
        let mut names = Vec::with_capacity(CHALLENGE_VECTOR_FIELD_COUNT);
        for index in 0..CHALLENGE_VECTOR_FIELD_COUNT {
            names.push(
                SolidityMemberName::try_new(SolidityMemberNameConstructorParams {
                    text: CHALLENGE_VECTOR_MEMBER_NAMES[index].to_string(),
                })
                .map_err(ChallengeVectorMemberNamesErrorReturn::MemberName)?,
            );
        }
        Ok(names)
    }
}

impl IEncodingContract for ChallengeVectorDescription {
    type Described = ChallengeVector;
    type FromFieldsErrorReturn = ChallengeVectorFromFieldsErrorReturn;
    const FIELDS: &[CanonicalFieldKind] = &CHALLENGE_VECTOR_FIELD_KINDS;

    fn to_fields(&self, _params: ToFieldsParams, payload: &ChallengeVector) -> ToFieldsReturn {
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
    ) -> FromFieldsReturn<ChallengeVector, ChallengeVectorFromFieldsErrorReturn> {
        let actual = payload.values.len();
        let Ok([purpose_value, draws_value, encoding_value, challenge_value]) =
            <[CanonicalFieldValue; CHALLENGE_VECTOR_FIELD_COUNT]>::try_from(payload.values)
        else {
            return Err(ChallengeVectorFromFieldsErrorReturn::FieldCount {
                expected: CHALLENGE_VECTOR_FIELD_COUNT,
                actual,
            });
        };
        let CanonicalFieldValue::Unsigned16(purpose) = purpose_value else {
            return Err(ChallengeVectorFromFieldsErrorReturn::FieldKind {
                index: 0,
                expected: CanonicalFieldKind::Unsigned16,
            });
        };
        let CanonicalFieldValue::Bytes(draws) = draws_value else {
            return Err(ChallengeVectorFromFieldsErrorReturn::FieldKind {
                index: 1,
                expected: CanonicalFieldKind::Bytes,
            });
        };
        let CanonicalFieldValue::Bytes(encoding) = encoding_value else {
            return Err(ChallengeVectorFromFieldsErrorReturn::FieldKind {
                index: 2,
                expected: CanonicalFieldKind::Bytes,
            });
        };
        let CanonicalFieldValue::Unsigned256(challenge) = challenge_value else {
            return Err(ChallengeVectorFromFieldsErrorReturn::FieldKind {
                index: 3,
                expected: CanonicalFieldKind::Unsigned256,
            });
        };
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

fn scalar_bytes<P: IPairingAdapter, E: IEncoderAdapter>(
    deps: &ChallengeVectorsDeps<'_, P, E>,
    scalar: P::Scalar,
) -> Result<[u8; 32], ChallengeVectorsErrorReturn> {
    let Ok(encoded) = deps
        .pairing
        .encode_scalar(EncodeScalarParams, EncodeScalarPayload { scalar });
    let bytes = encoded.bytes.expose();
    let actual = bytes.as_ref().len();
    <[u8; 32]>::try_from(bytes.as_ref())
        .map_err(|_| ChallengeVectorsErrorReturn::ScalarLength { actual })
}

fn draw_scalar<P: IPairingAdapter, E: IEncoderAdapter>(
    deps: &ChallengeVectorsDeps<'_, P, E>,
) -> Result<P::Scalar, ChallengeVectorsErrorReturn> {
    let uniform = deps
        .random
        .fill_bytes(
            FillBytesParams,
            FillBytesPayload {
                length: <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH,
            },
        )
        .map_err(ChallengeVectorsErrorReturn::FillBytes)?;
    let sampled = <P::Scalar as ISampleUniformScalar>::sample_from_uniform_bytes(
        SampleUniformScalarParams,
        SampleUniformScalarPayload {
            uniform: uniform.bytes,
        },
    )
    .map_err(ChallengeVectorsErrorReturn::SampleScalar)?;
    Ok(sampled.scalar.expose().clone())
}

fn draw_g1<P: IPairingAdapter, E: IEncoderAdapter>(
    deps: &ChallengeVectorsDeps<'_, P, E>,
    generator: &P::G1,
    draws: &mut Vec<u8>,
) -> Result<P::G1, ChallengeVectorsErrorReturn> {
    let scalar = draw_scalar::<P, E>(deps)?;
    let Ok(product) = deps.pairing.mul_g1(
        MulG1Params,
        MulG1Payload {
            point: generator.clone(),
            scalar: scalar.clone(),
        },
    );
    draws.extend_from_slice(&scalar_bytes::<P, E>(deps, scalar)?);
    Ok(product.product)
}

fn draw_g2<P: IPairingAdapter, E: IEncoderAdapter>(
    deps: &ChallengeVectorsDeps<'_, P, E>,
    generator: &P::G2,
    draws: &mut Vec<u8>,
) -> Result<P::G2, ChallengeVectorsErrorReturn> {
    let scalar = draw_scalar::<P, E>(deps)?;
    let Ok(product) = deps.pairing.mul_g2(
        MulG2Params,
        MulG2Payload {
            point: generator.clone(),
            scalar: scalar.clone(),
        },
    );
    draws.extend_from_slice(&scalar_bytes::<P, E>(deps, scalar)?);
    Ok(product.product)
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
    let tag = DomainTag::try_new(DomainTagConstructorParams {
        bytes: payload.delivery_proof.challenge_tag.to_vec(),
    })
    .map_err(ChallengeVectorsErrorReturn::DomainTag)?;
    let Ok(g1_generator) = deps
        .pairing
        .g1_generator(G1GeneratorParams, G1GeneratorPayload);
    let Ok(g2_generator) = deps
        .pairing
        .g2_generator(G2GeneratorParams, G2GeneratorPayload);
    let mut records = Vec::new();
    for _index in 0..params.count.get() {
        let mut mint_draws = Vec::new();
        let mint = payload.mint.clone();
        let mint_asset_root = match Root::try_new(RootConstructorParams {
            bytes: *mint.asset_identity_hash.as_bytes(),
        }) {
            Ok(root) => root,
            Err(_) => unreachable!("an asset identity hash is never all zero"),
        };
        let mint_pk1 = draw_g1::<P, E>(deps, &g1_generator.point, &mut mint_draws)?;
        let mint_pk2 = draw_g2::<P, E>(deps, &g2_generator.point, &mut mint_draws)?;
        let mint_envelope_c1 = draw_g1::<P, E>(deps, &g1_generator.point, &mut mint_draws)?;
        let mint_envelope_c2 = draw_g1::<P, E>(deps, &g1_generator.point, &mut mint_draws)?;
        let mint_envelope_d1 = draw_g2::<P, E>(deps, &g2_generator.point, &mut mint_draws)?;
        let mint_envelope_d2 = draw_g2::<P, E>(deps, &g2_generator.point, &mut mint_draws)?;
        let mint_hpub = draw_g2::<P, E>(deps, &g2_generator.point, &mut mint_draws)?;
        let mint_first_c1 = draw_g1::<P, E>(deps, &g1_generator.point, &mut mint_draws)?;
        let mint_first_c2 = draw_g1::<P, E>(deps, &g1_generator.point, &mut mint_draws)?;
        let mint_first_d1 = draw_g2::<P, E>(deps, &g2_generator.point, &mut mint_draws)?;
        let mint_first_d2 = draw_g2::<P, E>(deps, &g2_generator.point, &mut mint_draws)?;
        let mint_statement = MintStatement {
            suite_identifier: mint.suite_identifier,
            chain: mint.chain,
            entitlement_contract: mint.entitlement_contract,
            asset_root: mint_asset_root,
            parameter_set_identifier: mint.parameter_set_identifier,
            source_entitlement: mint.source_entitlement,
            target_entitlement: mint.target_entitlement,
            old_interval: mint.old_interval,
            new_interval: mint.new_interval,
            purpose: mint.purpose,
            seller: mint.seller,
            buyer: mint.buyer,
            buyer_keys: PublicKeysComponents {
                pk1: mint_pk1,
                pk2: mint_pk2,
            },
            envelope: EnvelopeComponents {
                c1: mint_envelope_c1,
                c2: mint_envelope_c2,
                d1: mint_envelope_d1,
                d2: mint_envelope_d2,
            },
            expiry: mint.expiry,
            first_messages: MintFirstMessages {
                hpub: mint_hpub,
                c1: mint_first_c1,
                c2: mint_first_c2,
                d1: mint_first_d1,
                d2: mint_first_d2,
            },
        };
        let Ok(mint_description) =
            MintStatementDescription::<P, F>::try_new(MintStatementDescriptionConstructorParams {
                pairing: deps.pairing,
            });
        let mint_encoding = deps
            .encoder
            .encode(
                EncodeParams {
                    description: &mint_description,
                },
                &mint_statement,
            )
            .map_err(ChallengeVectorsErrorReturn::Encoding)?
            .bytes;
        let mint_challenge_scalar = deps
            .hash_to_scalar
            .hash_to_scalar(
                HashToScalarParams { tag: &tag },
                HashToScalarPayload {
                    message: &mint_encoding,
                },
            )
            .map_err(ChallengeVectorsErrorReturn::HashToScalar)?
            .scalar;
        records.push(ChallengeVector {
            purpose: mint_statement.purpose.code(),
            draws: mint_draws,
            encoding: mint_encoding,
            challenge: scalar_bytes::<P, E>(deps, mint_challenge_scalar)?,
        });
        let mut transfer_draws = Vec::new();
        let transfer = payload.transfer.clone();
        let transfer_asset_root = match Root::try_new(RootConstructorParams {
            bytes: *transfer.asset_identity_hash.as_bytes(),
        }) {
            Ok(root) => root,
            Err(_) => unreachable!("an asset identity hash is never all zero"),
        };
        let transfer_seller_pk1 = draw_g1::<P, E>(deps, &g1_generator.point, &mut transfer_draws)?;
        let transfer_seller_pk2 = draw_g2::<P, E>(deps, &g2_generator.point, &mut transfer_draws)?;
        let transfer_buyer_pk1 = draw_g1::<P, E>(deps, &g1_generator.point, &mut transfer_draws)?;
        let transfer_buyer_pk2 = draw_g2::<P, E>(deps, &g2_generator.point, &mut transfer_draws)?;
        let transfer_old_c1 = draw_g1::<P, E>(deps, &g1_generator.point, &mut transfer_draws)?;
        let transfer_old_c2 = draw_g1::<P, E>(deps, &g1_generator.point, &mut transfer_draws)?;
        let transfer_old_d1 = draw_g2::<P, E>(deps, &g2_generator.point, &mut transfer_draws)?;
        let transfer_old_d2 = draw_g2::<P, E>(deps, &g2_generator.point, &mut transfer_draws)?;
        let transfer_new_c1 = draw_g1::<P, E>(deps, &g1_generator.point, &mut transfer_draws)?;
        let transfer_new_c2 = draw_g1::<P, E>(deps, &g1_generator.point, &mut transfer_draws)?;
        let transfer_new_d1 = draw_g2::<P, E>(deps, &g2_generator.point, &mut transfer_draws)?;
        let transfer_new_d2 = draw_g2::<P, E>(deps, &g2_generator.point, &mut transfer_draws)?;
        let transfer_first_pk1 = draw_g1::<P, E>(deps, &g1_generator.point, &mut transfer_draws)?;
        let transfer_first_c1 = draw_g1::<P, E>(deps, &g1_generator.point, &mut transfer_draws)?;
        let transfer_first_c2 = draw_g1::<P, E>(deps, &g1_generator.point, &mut transfer_draws)?;
        let transfer_first_pk2 = draw_g2::<P, E>(deps, &g2_generator.point, &mut transfer_draws)?;
        let transfer_first_d1 = draw_g2::<P, E>(deps, &g2_generator.point, &mut transfer_draws)?;
        let transfer_first_d2 = draw_g2::<P, E>(deps, &g2_generator.point, &mut transfer_draws)?;
        let transfer_statement = TransferStatement {
            suite_identifier: transfer.suite_identifier,
            chain: transfer.chain,
            entitlement_contract: transfer.entitlement_contract,
            asset_root: transfer_asset_root,
            parameter_set_identifier: transfer.parameter_set_identifier,
            source_entitlement: transfer.source_entitlement,
            target_entitlement: transfer.target_entitlement,
            old_interval: transfer.old_interval,
            new_interval: transfer.new_interval,
            purpose: transfer.purpose,
            seller: transfer.seller,
            buyer: transfer.buyer,
            seller_keys: PublicKeysComponents {
                pk1: transfer_seller_pk1,
                pk2: transfer_seller_pk2,
            },
            buyer_keys: PublicKeysComponents {
                pk1: transfer_buyer_pk1,
                pk2: transfer_buyer_pk2,
            },
            old_envelope: EnvelopeComponents {
                c1: transfer_old_c1,
                c2: transfer_old_c2,
                d1: transfer_old_d1,
                d2: transfer_old_d2,
            },
            new_envelope: EnvelopeComponents {
                c1: transfer_new_c1,
                c2: transfer_new_c2,
                d1: transfer_new_d1,
                d2: transfer_new_d2,
            },
            expiry: transfer.expiry,
            first_messages: TransferFirstMessages {
                pk1: transfer_first_pk1,
                c1: transfer_first_c1,
                c2: transfer_first_c2,
                pk2: transfer_first_pk2,
                d1: transfer_first_d1,
                d2: transfer_first_d2,
            },
        };
        let Ok(transfer_description) = TransferStatementDescription::<P, F>::try_new(
            TransferStatementDescriptionConstructorParams {
                pairing: deps.pairing,
            },
        );
        let transfer_encoding = deps
            .encoder
            .encode(
                EncodeParams {
                    description: &transfer_description,
                },
                &transfer_statement,
            )
            .map_err(ChallengeVectorsErrorReturn::Encoding)?
            .bytes;
        let transfer_challenge_scalar = deps
            .hash_to_scalar
            .hash_to_scalar(
                HashToScalarParams { tag: &tag },
                HashToScalarPayload {
                    message: &transfer_encoding,
                },
            )
            .map_err(ChallengeVectorsErrorReturn::HashToScalar)?
            .scalar;
        records.push(ChallengeVector {
            purpose: transfer_statement.purpose.code(),
            draws: transfer_draws,
            encoding: transfer_encoding,
            challenge: scalar_bytes::<P, E>(deps, transfer_challenge_scalar)?,
        });
    }
    Ok(ChallengeVectorsSuccessReturn { records })
}
