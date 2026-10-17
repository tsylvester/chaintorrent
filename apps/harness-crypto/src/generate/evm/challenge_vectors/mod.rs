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
    CanonicalFieldKind, CanonicalFieldValue, CanonicalFields, EncodeParams, FromFieldParams,
    FromFieldsParams, FromFieldsReturn, FromFieldsSuccessReturn, ICanonicalField, IEncoderAdapter,
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
    DELIVERY_STATEMENT_VERSION_ONE, DeliveryRelation, MintFirstMessages, MintStatement,
    MintStatementDescription, MintStatementDescriptionConstructorParams, TransferFirstMessages,
    TransferStatement, TransferStatementDescription, TransferStatementDescriptionConstructorParams,
};
use random::{FillBytesParams, FillBytesPayload};

use super::render::provides::{
    SolidityMemberName, SolidityMemberNameConstructorParams, SolidityRecordMember,
    SolidityRecordMembers, SolidityRecordMembersConstructorParams,
};
use interface::{
    CHALLENGE_VECTOR_FIELD_COUNT, CHALLENGE_VECTOR_FIELD_KINDS, CHALLENGE_VECTOR_MEMBER_NAMES,
    CanonicalDeliveryContext, ChallengeVector, ChallengeVectorDescription,
    ChallengeVectorDescriptionConstructorParams, ChallengeVectorDescriptionTryNewReturn,
    ChallengeVectorFromFieldsErrorReturn, ChallengeVectorMembersErrorReturn,
    ChallengeVectorMembersReturn, ChallengeVectorsDeps, ChallengeVectorsErrorReturn,
    ChallengeVectorsParams, ChallengeVectorsPayload, ChallengeVectorsReturn,
    ChallengeVectorsSuccessReturn, DeliveryContextField,
};

impl ChallengeVectorDescription {
    pub fn try_new(
        _params: ChallengeVectorDescriptionConstructorParams,
    ) -> ChallengeVectorDescriptionTryNewReturn {
        Ok(ChallengeVectorDescription)
    }

    pub fn members(&self) -> ChallengeVectorMembersReturn {
        let mut members = Vec::new();
        for (index, text) in CHALLENGE_VECTOR_MEMBER_NAMES.iter().enumerate() {
            let name = SolidityMemberName::try_new(SolidityMemberNameConstructorParams {
                text: text.to_string(),
            })
            .map_err(ChallengeVectorMembersErrorReturn::MemberName)?;
            members.push(SolidityRecordMember {
                name,
                kind: CHALLENGE_VECTOR_FIELD_KINDS[index],
            });
        }
        SolidityRecordMembers::try_new(SolidityRecordMembersConstructorParams { members })
            .map_err(ChallengeVectorMembersErrorReturn::RecordMembers)
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
        let actual = payload.values.len();
        let Ok(fields) =
            <[CanonicalFieldValue; CHALLENGE_VECTOR_FIELD_COUNT]>::try_from(payload.values)
        else {
            return Err(ChallengeVectorFromFieldsErrorReturn::FieldCount {
                expected: CHALLENGE_VECTOR_FIELD_COUNT,
                actual,
            });
        };
        let [purpose, draws, encoding, challenge] = fields;
        let CanonicalFieldValue::Unsigned16(purpose) = purpose else {
            return Err(ChallengeVectorFromFieldsErrorReturn::FieldKind {
                index: 0,
                expected: CHALLENGE_VECTOR_FIELD_KINDS[0],
            });
        };
        let CanonicalFieldValue::Bytes(draws) = draws else {
            return Err(ChallengeVectorFromFieldsErrorReturn::FieldKind {
                index: 1,
                expected: CHALLENGE_VECTOR_FIELD_KINDS[1],
            });
        };
        let CanonicalFieldValue::Bytes(encoding) = encoding else {
            return Err(ChallengeVectorFromFieldsErrorReturn::FieldKind {
                index: 2,
                expected: CHALLENGE_VECTOR_FIELD_KINDS[2],
            });
        };
        let CanonicalFieldValue::Unsigned256(challenge) = challenge else {
            return Err(ChallengeVectorFromFieldsErrorReturn::FieldKind {
                index: 3,
                expected: CHALLENGE_VECTOR_FIELD_KINDS[3],
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

pub fn challenge_vectors<'a, P: IPairingAdapter, E: IEncoderAdapter, F: IChainForms>(
    deps: &ChallengeVectorsDeps<'a, P, E>,
    params: ChallengeVectorsParams,
    payload: ChallengeVectorsPayload<'_>,
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

    struct AdmittedForms<F: IChainForms> {
        chain: F::ChainIdentifier,
        entitlement_contract: F::Identity,
        source_entitlement: F::Entitlement,
        target_entitlement: F::Entitlement,
        old_interval: F::Interval,
        new_interval: F::Interval,
        seller: F::Identity,
        buyer: F::Identity,
    }

    let rebuild_forms = |context: &CanonicalDeliveryContext,
                         relation: DeliveryRelation|
     -> Result<AdmittedForms<F>, ChallengeVectorsErrorReturn> {
        let chain = <F::ChainIdentifier as ICanonicalField>::from_field(
            FromFieldParams,
            context.chain.clone(),
        )
        .map_err(|error| ChallengeVectorsErrorReturn::ChainIdentifier { relation, error })?
        .value;
        let entitlement_contract = <F::Identity as ICanonicalField>::from_field(
            FromFieldParams,
            context.entitlement_contract.clone(),
        )
        .map_err(|error| ChallengeVectorsErrorReturn::Identity {
            relation,
            field: DeliveryContextField::EntitlementContract,
            error,
        })?
        .value;
        let source_entitlement = <F::Entitlement as ICanonicalField>::from_field(
            FromFieldParams,
            context.source_entitlement.clone(),
        )
        .map_err(|error| ChallengeVectorsErrorReturn::Entitlement {
            relation,
            field: DeliveryContextField::SourceEntitlement,
            error,
        })?
        .value;
        let target_entitlement = <F::Entitlement as ICanonicalField>::from_field(
            FromFieldParams,
            context.target_entitlement.clone(),
        )
        .map_err(|error| ChallengeVectorsErrorReturn::Entitlement {
            relation,
            field: DeliveryContextField::TargetEntitlement,
            error,
        })?
        .value;
        let old_interval = <F::Interval as ICanonicalField>::from_field(
            FromFieldParams,
            context.old_interval.clone(),
        )
        .map_err(|error| ChallengeVectorsErrorReturn::Interval {
            relation,
            field: DeliveryContextField::OldInterval,
            error,
        })?
        .value;
        let new_interval = <F::Interval as ICanonicalField>::from_field(
            FromFieldParams,
            context.new_interval.clone(),
        )
        .map_err(|error| ChallengeVectorsErrorReturn::Interval {
            relation,
            field: DeliveryContextField::NewInterval,
            error,
        })?
        .value;
        let seller =
            <F::Identity as ICanonicalField>::from_field(FromFieldParams, context.seller.clone())
                .map_err(|error| ChallengeVectorsErrorReturn::Identity {
                    relation,
                    field: DeliveryContextField::Seller,
                    error,
                })?
                .value;
        let buyer =
            <F::Identity as ICanonicalField>::from_field(FromFieldParams, context.buyer.clone())
                .map_err(|error| ChallengeVectorsErrorReturn::Identity {
                    relation,
                    field: DeliveryContextField::Buyer,
                    error,
                })?
                .value;
        Ok(AdmittedForms {
            chain,
            entitlement_contract,
            source_entitlement,
            target_entitlement,
            old_interval,
            new_interval,
            seller,
            buyer,
        })
    };

    let mint_forms = rebuild_forms(payload.mint, DeliveryRelation::Mint)?;
    let transfer_forms = rebuild_forms(payload.transfer, DeliveryRelation::Transfer)?;

    let Ok(g1) = deps
        .pairing
        .g1_generator(G1GeneratorParams, G1GeneratorPayload);
    let Ok(g2) = deps
        .pairing
        .g2_generator(G2GeneratorParams, G2GeneratorPayload);

    let scalar_bytes = |scalar: P::Scalar| -> Result<[u8; 32], ChallengeVectorsErrorReturn> {
        let Ok(encoded) = deps
            .pairing
            .encode_scalar(EncodeScalarParams, EncodeScalarPayload { scalar });
        let actual = encoded.bytes.expose().as_ref().len();
        let Ok(bytes) = <[u8; 32]>::try_from(encoded.bytes.expose().as_ref()) else {
            return Err(ChallengeVectorsErrorReturn::ScalarLength { actual });
        };
        Ok(bytes)
    };

    let draw_scalar = |draws: &mut Vec<u8>| -> Result<P::Scalar, ChallengeVectorsErrorReturn> {
        let uniform = deps
            .random
            .fill_bytes(
                FillBytesParams,
                FillBytesPayload {
                    length: <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH,
                },
            )
            .map_err(ChallengeVectorsErrorReturn::FillBytes)?
            .bytes;
        let sampled = <P::Scalar as ISampleUniformScalar>::sample_from_uniform_bytes(
            SampleUniformScalarParams,
            SampleUniformScalarPayload { uniform },
        )
        .map_err(ChallengeVectorsErrorReturn::SampleScalar)?;
        let scalar = sampled.scalar.expose().clone();
        draws.extend_from_slice(&scalar_bytes(scalar.clone())?);
        Ok(scalar)
    };

    let draw_g1 = |draws: &mut Vec<u8>| -> Result<P::G1, ChallengeVectorsErrorReturn> {
        let scalar = draw_scalar(draws)?;
        let Ok(product) = deps.pairing.mul_g1(
            MulG1Params,
            MulG1Payload {
                point: g1.point.clone(),
                scalar,
            },
        );
        Ok(product.product)
    };

    let draw_g2 = |draws: &mut Vec<u8>| -> Result<P::G2, ChallengeVectorsErrorReturn> {
        let scalar = draw_scalar(draws)?;
        let Ok(product) = deps.pairing.mul_g2(
            MulG2Params,
            MulG2Payload {
                point: g2.point.clone(),
                scalar,
            },
        );
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
        let statement = MintStatement {
            suite_identifier: payload.mint.suite_identifier.clone(),
            chain: mint_forms.chain.clone(),
            entitlement_contract: mint_forms.entitlement_contract.clone(),
            asset_identity_hash: payload.mint.asset_identity_hash.clone(),
            parameter_set_identifier: payload.mint.parameter_set_identifier.clone(),
            source_entitlement: mint_forms.source_entitlement.clone(),
            target_entitlement: mint_forms.target_entitlement.clone(),
            old_interval: mint_forms.old_interval.clone(),
            new_interval: mint_forms.new_interval.clone(),
            purpose: payload.mint_purpose,
            seller: mint_forms.seller.clone(),
            buyer: mint_forms.buyer.clone(),
            buyer_keys,
            envelope,
            expiry: payload.mint.expiry,
            first_messages,
        };
        let Ok(description) =
            MintStatementDescription::<P, F>::try_new(MintStatementDescriptionConstructorParams {
                pairing: deps.pairing,
            });
        let encoding = deps
            .encoder
            .encode(
                EncodeParams {
                    description: &description,
                },
                &statement,
            )
            .map_err(ChallengeVectorsErrorReturn::Encoding)?
            .bytes;
        let challenge = deps
            .hash_to_scalar
            .hash_to_scalar(
                HashToScalarParams { tag: &tag },
                HashToScalarPayload { message: &encoding },
            )
            .map_err(ChallengeVectorsErrorReturn::HashToScalar)?
            .scalar;
        records.push(ChallengeVector {
            purpose: payload.mint_purpose.code(),
            draws,
            encoding,
            challenge: scalar_bytes(challenge)?,
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
        let statement = TransferStatement {
            suite_identifier: payload.transfer.suite_identifier.clone(),
            chain: transfer_forms.chain.clone(),
            entitlement_contract: transfer_forms.entitlement_contract.clone(),
            asset_identity_hash: payload.transfer.asset_identity_hash.clone(),
            parameter_set_identifier: payload.transfer.parameter_set_identifier.clone(),
            source_entitlement: transfer_forms.source_entitlement.clone(),
            target_entitlement: transfer_forms.target_entitlement.clone(),
            old_interval: transfer_forms.old_interval.clone(),
            new_interval: transfer_forms.new_interval.clone(),
            purpose: payload.transfer_purpose,
            seller: transfer_forms.seller.clone(),
            buyer: transfer_forms.buyer.clone(),
            seller_keys,
            buyer_keys,
            old_envelope,
            new_envelope,
            expiry: payload.transfer.expiry,
            first_messages,
        };
        let Ok(description) = TransferStatementDescription::<P, F>::try_new(
            TransferStatementDescriptionConstructorParams {
                pairing: deps.pairing,
            },
        );
        let encoding = deps
            .encoder
            .encode(
                EncodeParams {
                    description: &description,
                },
                &statement,
            )
            .map_err(ChallengeVectorsErrorReturn::Encoding)?
            .bytes;
        let challenge = deps
            .hash_to_scalar
            .hash_to_scalar(
                HashToScalarParams { tag: &tag },
                HashToScalarPayload { message: &encoding },
            )
            .map_err(ChallengeVectorsErrorReturn::HashToScalar)?
            .scalar;
        records.push(ChallengeVector {
            purpose: payload.transfer_purpose.code(),
            draws,
            encoding,
            challenge: scalar_bytes(challenge)?,
        });
    }

    Ok(ChallengeVectorsSuccessReturn { records })
}
