#[cfg(test)]
mod integration_test;
mod interface;
#[cfg(any(test, feature = "mocks"))]
mod mock;
pub mod provides;
#[cfg(test)]
mod test_basic;
#[cfg(test)]
mod test_description;
#[cfg(test)]
mod test_error;

use chain::IChainForms;
use core::marker::PhantomData;
use encoding::{
    CanonicalFieldKind, CanonicalFieldReader, CanonicalFieldValue, CanonicalFields,
    FromFieldsParams, FromFieldsReturn, FromFieldsSuccessReturn, ICanonicalField,
    IEncodingContract, ToFieldParams, ToFieldsParams, ToFieldsReturn, ToFieldsSuccessReturn,
};
use envelope::{EnvelopeComponents, IKeyAgreementAdapter, PublicKeysComponents};
use kem::ICredentialKemAdapter;
use pairing::{
    EncodeG1Params, EncodeG1Payload, EncodeG2Params, EncodeG2Payload, EncodeScalarParams,
    EncodeScalarPayload, IPairingAdapter,
};
use proof::{
    AlgebraicMintStatement, AlgebraicTransferStatement, DeliveryContext, DeliveryProofComponents,
    DeliveryRelation, IDeliveryProofAdapter, MintPurpose, ProofComponentsParams,
    ProofComponentsPayload, TransferPurpose,
};

use self::interface::{
    MINT_PROOF_VECTOR_FIELD_COUNT, MINT_PROOF_VECTOR_MEMBER_NAMES, MintProofVector,
    MintProofVectorDescription, MintProofVectorDescriptionConstructorParams,
    MintProofVectorDescriptionTryNewReturn, MintProofVectorFromFieldsErrorReturn,
    ProofVectorContext, ProofVectorMemberNamesErrorReturn, ProofVectorMemberNamesReturn,
    ProofVectorsDeps, ProofVectorsErrorReturn, ProofVectorsParams, ProofVectorsPayload,
    ProofVectorsReturn, ProofVectorsSuccessReturn, TRANSFER_PROOF_VECTOR_FIELD_COUNT,
    TRANSFER_PROOF_VECTOR_MEMBER_NAMES, TransferProofVector, TransferProofVectorDescription,
    TransferProofVectorDescriptionConstructorParams, TransferProofVectorDescriptionTryNewReturn,
    TransferProofVectorFromFieldsErrorReturn,
};
use super::render::provides::{SolidityMemberName, SolidityMemberNameConstructorParams};
use crate::delivery::provides::{
    MintDeps, MintParams, MintPayload, TransferDeps, TransferParams, TransferPayload, mint,
    transfer,
};

impl<F: IChainForms> MintProofVectorDescription<F> {
    pub fn try_new(
        _params: MintProofVectorDescriptionConstructorParams,
    ) -> MintProofVectorDescriptionTryNewReturn<F> {
        Ok(MintProofVectorDescription { forms: PhantomData })
    }

    pub fn member_names(&self) -> ProofVectorMemberNamesReturn {
        let mut names = Vec::with_capacity(MINT_PROOF_VECTOR_FIELD_COUNT);
        for text in MINT_PROOF_VECTOR_MEMBER_NAMES {
            match SolidityMemberName::try_new(SolidityMemberNameConstructorParams {
                text: text.to_string(),
            }) {
                Ok(name) => names.push(name),
                Err(error) => {
                    return Err(ProofVectorMemberNamesErrorReturn::MemberName(error));
                }
            }
        }
        Ok(names)
    }
}

impl<F: IChainForms> IEncodingContract for MintProofVectorDescription<F> {
    type Described = MintProofVector;
    type FromFieldsErrorReturn = MintProofVectorFromFieldsErrorReturn;
    const FIELDS: &'static [CanonicalFieldKind] = &[
        CanonicalFieldKind::Bytes,
        CanonicalFieldKind::FixedBytes32,
        CanonicalFieldKind::Unsigned16,
        <F::ChainIdentifier as ICanonicalField>::KIND,
        <F::Identity as ICanonicalField>::KIND,
        CanonicalFieldKind::FixedBytes32,
        CanonicalFieldKind::FixedBytes32,
        <F::Entitlement as ICanonicalField>::KIND,
        <F::Entitlement as ICanonicalField>::KIND,
        <F::Interval as ICanonicalField>::KIND,
        <F::Interval as ICanonicalField>::KIND,
        CanonicalFieldKind::Unsigned16,
        <F::Identity as ICanonicalField>::KIND,
        <F::Identity as ICanonicalField>::KIND,
        CanonicalFieldKind::Unsigned64,
        CanonicalFieldKind::Bytes,
        CanonicalFieldKind::Bytes,
        CanonicalFieldKind::Bytes,
        CanonicalFieldKind::Bytes,
        CanonicalFieldKind::Bytes,
        CanonicalFieldKind::Bytes,
        CanonicalFieldKind::Bytes,
        CanonicalFieldKind::Bytes,
        CanonicalFieldKind::Unsigned256,
        CanonicalFieldKind::Unsigned256,
        CanonicalFieldKind::Unsigned256,
        CanonicalFieldKind::Unsigned256,
        CanonicalFieldKind::Unsigned256,
        CanonicalFieldKind::Bytes,
    ];

    fn to_fields(&self, _params: ToFieldsParams, payload: &Self::Described) -> ToFieldsReturn {
        Ok(ToFieldsSuccessReturn {
            fields: CanonicalFields {
                values: vec![
                    CanonicalFieldValue::Bytes(payload.draws.clone()),
                    CanonicalFieldValue::FixedBytes32(payload.context.suite_identifier),
                    CanonicalFieldValue::Unsigned16(payload.context.suite_version),
                    payload.context.chain.clone(),
                    payload.context.entitlement_contract.clone(),
                    CanonicalFieldValue::FixedBytes32(payload.context.asset_root),
                    CanonicalFieldValue::FixedBytes32(payload.context.parameter_set),
                    payload.context.source_entitlement.clone(),
                    payload.context.target_entitlement.clone(),
                    payload.context.old_interval.clone(),
                    payload.context.new_interval.clone(),
                    CanonicalFieldValue::Unsigned16(payload.context.purpose),
                    payload.context.seller.clone(),
                    payload.context.buyer.clone(),
                    CanonicalFieldValue::Unsigned64(payload.context.expiry),
                    CanonicalFieldValue::Bytes(payload.hpub.clone()),
                    CanonicalFieldValue::Bytes(payload.identity_element.clone()),
                    CanonicalFieldValue::Bytes(payload.buyer_pk1.clone()),
                    CanonicalFieldValue::Bytes(payload.buyer_pk2.clone()),
                    CanonicalFieldValue::Bytes(payload.envelope_c1.clone()),
                    CanonicalFieldValue::Bytes(payload.envelope_c2.clone()),
                    CanonicalFieldValue::Bytes(payload.envelope_d1.clone()),
                    CanonicalFieldValue::Bytes(payload.envelope_d2.clone()),
                    CanonicalFieldValue::Unsigned256(payload.challenge),
                    CanonicalFieldValue::Unsigned256(payload.alpha),
                    CanonicalFieldValue::Unsigned256(payload.r),
                    CanonicalFieldValue::Unsigned256(payload.rho),
                    CanonicalFieldValue::Unsigned256(payload.sigma),
                    CanonicalFieldValue::Bytes(payload.second_group_first_messages.clone()),
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
            .map_err(MintProofVectorFromFieldsErrorReturn::Fields)?;
        let draws = reader
            .read_bytes()
            .map_err(MintProofVectorFromFieldsErrorReturn::Fields)?;
        let suite_identifier = reader
            .read_fixed_bytes32()
            .map_err(MintProofVectorFromFieldsErrorReturn::Fields)?;
        let suite_version = reader
            .read_unsigned16()
            .map_err(MintProofVectorFromFieldsErrorReturn::Fields)?;
        let chain_form = reader
            .read_form::<F::ChainIdentifier>()
            .map_err(MintProofVectorFromFieldsErrorReturn::Fields)?;
        let Ok(chain) = F::ChainIdentifier::to_field(ToFieldParams, &chain_form);
        let entitlement_contract_form = reader
            .read_form::<F::Identity>()
            .map_err(MintProofVectorFromFieldsErrorReturn::Fields)?;
        let Ok(entitlement_contract) =
            F::Identity::to_field(ToFieldParams, &entitlement_contract_form);
        let asset_root = reader
            .read_fixed_bytes32()
            .map_err(MintProofVectorFromFieldsErrorReturn::Fields)?;
        let parameter_set = reader
            .read_fixed_bytes32()
            .map_err(MintProofVectorFromFieldsErrorReturn::Fields)?;
        let source_entitlement_form = reader
            .read_form::<F::Entitlement>()
            .map_err(MintProofVectorFromFieldsErrorReturn::Fields)?;
        let Ok(source_entitlement) =
            F::Entitlement::to_field(ToFieldParams, &source_entitlement_form);
        let target_entitlement_form = reader
            .read_form::<F::Entitlement>()
            .map_err(MintProofVectorFromFieldsErrorReturn::Fields)?;
        let Ok(target_entitlement) =
            F::Entitlement::to_field(ToFieldParams, &target_entitlement_form);
        let old_interval_form = reader
            .read_form::<F::Interval>()
            .map_err(MintProofVectorFromFieldsErrorReturn::Fields)?;
        let Ok(old_interval) = F::Interval::to_field(ToFieldParams, &old_interval_form);
        let new_interval_form = reader
            .read_form::<F::Interval>()
            .map_err(MintProofVectorFromFieldsErrorReturn::Fields)?;
        let Ok(new_interval) = F::Interval::to_field(ToFieldParams, &new_interval_form);
        let purpose = reader
            .read_unsigned16()
            .map_err(MintProofVectorFromFieldsErrorReturn::Fields)?;
        let seller_form = reader
            .read_form::<F::Identity>()
            .map_err(MintProofVectorFromFieldsErrorReturn::Fields)?;
        let Ok(seller) = F::Identity::to_field(ToFieldParams, &seller_form);
        let buyer_form = reader
            .read_form::<F::Identity>()
            .map_err(MintProofVectorFromFieldsErrorReturn::Fields)?;
        let Ok(buyer) = F::Identity::to_field(ToFieldParams, &buyer_form);
        let expiry = reader
            .read_unsigned64()
            .map_err(MintProofVectorFromFieldsErrorReturn::Fields)?;
        let hpub = reader
            .read_bytes()
            .map_err(MintProofVectorFromFieldsErrorReturn::Fields)?;
        let identity_element = reader
            .read_bytes()
            .map_err(MintProofVectorFromFieldsErrorReturn::Fields)?;
        let buyer_pk1 = reader
            .read_bytes()
            .map_err(MintProofVectorFromFieldsErrorReturn::Fields)?;
        let buyer_pk2 = reader
            .read_bytes()
            .map_err(MintProofVectorFromFieldsErrorReturn::Fields)?;
        let envelope_c1 = reader
            .read_bytes()
            .map_err(MintProofVectorFromFieldsErrorReturn::Fields)?;
        let envelope_c2 = reader
            .read_bytes()
            .map_err(MintProofVectorFromFieldsErrorReturn::Fields)?;
        let envelope_d1 = reader
            .read_bytes()
            .map_err(MintProofVectorFromFieldsErrorReturn::Fields)?;
        let envelope_d2 = reader
            .read_bytes()
            .map_err(MintProofVectorFromFieldsErrorReturn::Fields)?;
        let challenge = reader
            .read_unsigned256()
            .map_err(MintProofVectorFromFieldsErrorReturn::Fields)?;
        let alpha = reader
            .read_unsigned256()
            .map_err(MintProofVectorFromFieldsErrorReturn::Fields)?;
        let r = reader
            .read_unsigned256()
            .map_err(MintProofVectorFromFieldsErrorReturn::Fields)?;
        let rho = reader
            .read_unsigned256()
            .map_err(MintProofVectorFromFieldsErrorReturn::Fields)?;
        let sigma = reader
            .read_unsigned256()
            .map_err(MintProofVectorFromFieldsErrorReturn::Fields)?;
        let second_group_first_messages = reader
            .read_bytes()
            .map_err(MintProofVectorFromFieldsErrorReturn::Fields)?;
        Ok(FromFieldsSuccessReturn {
            described: MintProofVector {
                draws,
                context: ProofVectorContext {
                    suite_identifier,
                    suite_version,
                    chain: chain.field,
                    entitlement_contract: entitlement_contract.field,
                    asset_root,
                    parameter_set,
                    source_entitlement: source_entitlement.field,
                    target_entitlement: target_entitlement.field,
                    old_interval: old_interval.field,
                    new_interval: new_interval.field,
                    purpose,
                    seller: seller.field,
                    buyer: buyer.field,
                    expiry,
                },
                hpub,
                identity_element,
                buyer_pk1,
                buyer_pk2,
                envelope_c1,
                envelope_c2,
                envelope_d1,
                envelope_d2,
                challenge,
                alpha,
                r,
                rho,
                sigma,
                second_group_first_messages,
            },
        })
    }
}

impl<F: IChainForms> TransferProofVectorDescription<F> {
    pub fn try_new(
        _params: TransferProofVectorDescriptionConstructorParams,
    ) -> TransferProofVectorDescriptionTryNewReturn<F> {
        Ok(TransferProofVectorDescription { forms: PhantomData })
    }

    pub fn member_names(&self) -> ProofVectorMemberNamesReturn {
        let mut names = Vec::with_capacity(TRANSFER_PROOF_VECTOR_FIELD_COUNT);
        for text in TRANSFER_PROOF_VECTOR_MEMBER_NAMES {
            match SolidityMemberName::try_new(SolidityMemberNameConstructorParams {
                text: text.to_string(),
            }) {
                Ok(name) => names.push(name),
                Err(error) => {
                    return Err(ProofVectorMemberNamesErrorReturn::MemberName(error));
                }
            }
        }
        Ok(names)
    }
}

impl<F: IChainForms> IEncodingContract for TransferProofVectorDescription<F> {
    type Described = TransferProofVector;
    type FromFieldsErrorReturn = TransferProofVectorFromFieldsErrorReturn;
    const FIELDS: &'static [CanonicalFieldKind] = &[
        CanonicalFieldKind::Bytes,
        CanonicalFieldKind::FixedBytes32,
        CanonicalFieldKind::Unsigned16,
        <F::ChainIdentifier as ICanonicalField>::KIND,
        <F::Identity as ICanonicalField>::KIND,
        CanonicalFieldKind::FixedBytes32,
        CanonicalFieldKind::FixedBytes32,
        <F::Entitlement as ICanonicalField>::KIND,
        <F::Entitlement as ICanonicalField>::KIND,
        <F::Interval as ICanonicalField>::KIND,
        <F::Interval as ICanonicalField>::KIND,
        CanonicalFieldKind::Unsigned16,
        <F::Identity as ICanonicalField>::KIND,
        <F::Identity as ICanonicalField>::KIND,
        CanonicalFieldKind::Unsigned64,
        CanonicalFieldKind::Bytes,
        CanonicalFieldKind::Bytes,
        CanonicalFieldKind::Bytes,
        CanonicalFieldKind::Bytes,
        CanonicalFieldKind::Bytes,
        CanonicalFieldKind::Bytes,
        CanonicalFieldKind::Bytes,
        CanonicalFieldKind::Bytes,
        CanonicalFieldKind::Bytes,
        CanonicalFieldKind::Bytes,
        CanonicalFieldKind::Bytes,
        CanonicalFieldKind::Bytes,
        CanonicalFieldKind::Bytes,
        CanonicalFieldKind::Unsigned256,
        CanonicalFieldKind::Unsigned256,
        CanonicalFieldKind::Unsigned256,
        CanonicalFieldKind::Unsigned256,
        CanonicalFieldKind::Unsigned256,
        CanonicalFieldKind::Unsigned256,
        CanonicalFieldKind::Bytes,
    ];

    fn to_fields(&self, _params: ToFieldsParams, payload: &Self::Described) -> ToFieldsReturn {
        Ok(ToFieldsSuccessReturn {
            fields: CanonicalFields {
                values: vec![
                    CanonicalFieldValue::Bytes(payload.draws.clone()),
                    CanonicalFieldValue::FixedBytes32(payload.context.suite_identifier),
                    CanonicalFieldValue::Unsigned16(payload.context.suite_version),
                    payload.context.chain.clone(),
                    payload.context.entitlement_contract.clone(),
                    CanonicalFieldValue::FixedBytes32(payload.context.asset_root),
                    CanonicalFieldValue::FixedBytes32(payload.context.parameter_set),
                    payload.context.source_entitlement.clone(),
                    payload.context.target_entitlement.clone(),
                    payload.context.old_interval.clone(),
                    payload.context.new_interval.clone(),
                    CanonicalFieldValue::Unsigned16(payload.context.purpose),
                    payload.context.seller.clone(),
                    payload.context.buyer.clone(),
                    CanonicalFieldValue::Unsigned64(payload.context.expiry),
                    CanonicalFieldValue::Bytes(payload.identity_element.clone()),
                    CanonicalFieldValue::Bytes(payload.seller_pk1.clone()),
                    CanonicalFieldValue::Bytes(payload.seller_pk2.clone()),
                    CanonicalFieldValue::Bytes(payload.buyer_pk1.clone()),
                    CanonicalFieldValue::Bytes(payload.buyer_pk2.clone()),
                    CanonicalFieldValue::Bytes(payload.old_envelope_c1.clone()),
                    CanonicalFieldValue::Bytes(payload.old_envelope_c2.clone()),
                    CanonicalFieldValue::Bytes(payload.old_envelope_d1.clone()),
                    CanonicalFieldValue::Bytes(payload.old_envelope_d2.clone()),
                    CanonicalFieldValue::Bytes(payload.new_envelope_c1.clone()),
                    CanonicalFieldValue::Bytes(payload.new_envelope_c2.clone()),
                    CanonicalFieldValue::Bytes(payload.new_envelope_d1.clone()),
                    CanonicalFieldValue::Bytes(payload.new_envelope_d2.clone()),
                    CanonicalFieldValue::Unsigned256(payload.challenge),
                    CanonicalFieldValue::Unsigned256(payload.x),
                    CanonicalFieldValue::Unsigned256(payload.y),
                    CanonicalFieldValue::Unsigned256(payload.s),
                    CanonicalFieldValue::Unsigned256(payload.rho),
                    CanonicalFieldValue::Unsigned256(payload.sigma),
                    CanonicalFieldValue::Bytes(payload.second_group_first_messages.clone()),
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
            .map_err(TransferProofVectorFromFieldsErrorReturn::Fields)?;
        let draws = reader
            .read_bytes()
            .map_err(TransferProofVectorFromFieldsErrorReturn::Fields)?;
        let suite_identifier = reader
            .read_fixed_bytes32()
            .map_err(TransferProofVectorFromFieldsErrorReturn::Fields)?;
        let suite_version = reader
            .read_unsigned16()
            .map_err(TransferProofVectorFromFieldsErrorReturn::Fields)?;
        let chain_form = reader
            .read_form::<F::ChainIdentifier>()
            .map_err(TransferProofVectorFromFieldsErrorReturn::Fields)?;
        let Ok(chain) = F::ChainIdentifier::to_field(ToFieldParams, &chain_form);
        let entitlement_contract_form = reader
            .read_form::<F::Identity>()
            .map_err(TransferProofVectorFromFieldsErrorReturn::Fields)?;
        let Ok(entitlement_contract) =
            F::Identity::to_field(ToFieldParams, &entitlement_contract_form);
        let asset_root = reader
            .read_fixed_bytes32()
            .map_err(TransferProofVectorFromFieldsErrorReturn::Fields)?;
        let parameter_set = reader
            .read_fixed_bytes32()
            .map_err(TransferProofVectorFromFieldsErrorReturn::Fields)?;
        let source_entitlement_form = reader
            .read_form::<F::Entitlement>()
            .map_err(TransferProofVectorFromFieldsErrorReturn::Fields)?;
        let Ok(source_entitlement) =
            F::Entitlement::to_field(ToFieldParams, &source_entitlement_form);
        let target_entitlement_form = reader
            .read_form::<F::Entitlement>()
            .map_err(TransferProofVectorFromFieldsErrorReturn::Fields)?;
        let Ok(target_entitlement) =
            F::Entitlement::to_field(ToFieldParams, &target_entitlement_form);
        let old_interval_form = reader
            .read_form::<F::Interval>()
            .map_err(TransferProofVectorFromFieldsErrorReturn::Fields)?;
        let Ok(old_interval) = F::Interval::to_field(ToFieldParams, &old_interval_form);
        let new_interval_form = reader
            .read_form::<F::Interval>()
            .map_err(TransferProofVectorFromFieldsErrorReturn::Fields)?;
        let Ok(new_interval) = F::Interval::to_field(ToFieldParams, &new_interval_form);
        let purpose = reader
            .read_unsigned16()
            .map_err(TransferProofVectorFromFieldsErrorReturn::Fields)?;
        let seller_form = reader
            .read_form::<F::Identity>()
            .map_err(TransferProofVectorFromFieldsErrorReturn::Fields)?;
        let Ok(seller) = F::Identity::to_field(ToFieldParams, &seller_form);
        let buyer_form = reader
            .read_form::<F::Identity>()
            .map_err(TransferProofVectorFromFieldsErrorReturn::Fields)?;
        let Ok(buyer) = F::Identity::to_field(ToFieldParams, &buyer_form);
        let expiry = reader
            .read_unsigned64()
            .map_err(TransferProofVectorFromFieldsErrorReturn::Fields)?;
        let identity_element = reader
            .read_bytes()
            .map_err(TransferProofVectorFromFieldsErrorReturn::Fields)?;
        let seller_pk1 = reader
            .read_bytes()
            .map_err(TransferProofVectorFromFieldsErrorReturn::Fields)?;
        let seller_pk2 = reader
            .read_bytes()
            .map_err(TransferProofVectorFromFieldsErrorReturn::Fields)?;
        let buyer_pk1 = reader
            .read_bytes()
            .map_err(TransferProofVectorFromFieldsErrorReturn::Fields)?;
        let buyer_pk2 = reader
            .read_bytes()
            .map_err(TransferProofVectorFromFieldsErrorReturn::Fields)?;
        let old_envelope_c1 = reader
            .read_bytes()
            .map_err(TransferProofVectorFromFieldsErrorReturn::Fields)?;
        let old_envelope_c2 = reader
            .read_bytes()
            .map_err(TransferProofVectorFromFieldsErrorReturn::Fields)?;
        let old_envelope_d1 = reader
            .read_bytes()
            .map_err(TransferProofVectorFromFieldsErrorReturn::Fields)?;
        let old_envelope_d2 = reader
            .read_bytes()
            .map_err(TransferProofVectorFromFieldsErrorReturn::Fields)?;
        let new_envelope_c1 = reader
            .read_bytes()
            .map_err(TransferProofVectorFromFieldsErrorReturn::Fields)?;
        let new_envelope_c2 = reader
            .read_bytes()
            .map_err(TransferProofVectorFromFieldsErrorReturn::Fields)?;
        let new_envelope_d1 = reader
            .read_bytes()
            .map_err(TransferProofVectorFromFieldsErrorReturn::Fields)?;
        let new_envelope_d2 = reader
            .read_bytes()
            .map_err(TransferProofVectorFromFieldsErrorReturn::Fields)?;
        let challenge = reader
            .read_unsigned256()
            .map_err(TransferProofVectorFromFieldsErrorReturn::Fields)?;
        let x = reader
            .read_unsigned256()
            .map_err(TransferProofVectorFromFieldsErrorReturn::Fields)?;
        let y = reader
            .read_unsigned256()
            .map_err(TransferProofVectorFromFieldsErrorReturn::Fields)?;
        let s = reader
            .read_unsigned256()
            .map_err(TransferProofVectorFromFieldsErrorReturn::Fields)?;
        let rho = reader
            .read_unsigned256()
            .map_err(TransferProofVectorFromFieldsErrorReturn::Fields)?;
        let sigma = reader
            .read_unsigned256()
            .map_err(TransferProofVectorFromFieldsErrorReturn::Fields)?;
        let second_group_first_messages = reader
            .read_bytes()
            .map_err(TransferProofVectorFromFieldsErrorReturn::Fields)?;
        Ok(FromFieldsSuccessReturn {
            described: TransferProofVector {
                draws,
                context: ProofVectorContext {
                    suite_identifier,
                    suite_version,
                    chain: chain.field,
                    entitlement_contract: entitlement_contract.field,
                    asset_root,
                    parameter_set,
                    source_entitlement: source_entitlement.field,
                    target_entitlement: target_entitlement.field,
                    old_interval: old_interval.field,
                    new_interval: new_interval.field,
                    purpose,
                    seller: seller.field,
                    buyer: buyer.field,
                    expiry,
                },
                identity_element,
                seller_pk1,
                seller_pk2,
                buyer_pk1,
                buyer_pk2,
                old_envelope_c1,
                old_envelope_c2,
                old_envelope_d1,
                old_envelope_d2,
                new_envelope_c1,
                new_envelope_c2,
                new_envelope_d1,
                new_envelope_d2,
                challenge,
                x,
                y,
                s,
                rho,
                sigma,
                second_group_first_messages,
            },
        })
    }
}

pub fn proof_vectors<
    'a,
    P: IPairingAdapter,
    K: ICredentialKemAdapter<Pairing = P>,
    A: IKeyAgreementAdapter<Pairing = P>,
    D: IDeliveryProofAdapter<Pairing = P>,
>(
    deps: &ProofVectorsDeps<'a, P, K, A, D>,
    params: ProofVectorsParams,
    payload: ProofVectorsPayload<'_, D::Forms>,
) -> ProofVectorsReturn {
    let scalar_bytes = |scalar: &P::Scalar| -> Result<[u8; 32], ProofVectorsErrorReturn> {
        let Ok(encoded) = deps.pairing.encode_scalar(
            EncodeScalarParams,
            EncodeScalarPayload {
                scalar: scalar.clone(),
            },
        );
        let bytes: &[u8] = encoded.bytes.expose().as_ref();
        match <[u8; 32]>::try_from(bytes) {
            Ok(array) => Ok(array),
            Err(_error) => Err(ProofVectorsErrorReturn::ScalarLength {
                actual: bytes.len(),
            }),
        }
    };
    let g1_bytes = |point: &P::G1| -> Vec<u8> {
        let Ok(encoded) = deps.pairing.encode_g1(
            EncodeG1Params,
            EncodeG1Payload {
                point: point.clone(),
            },
        );
        encoded.bytes.as_ref().to_vec()
    };
    let g2_bytes = |point: &P::G2| -> Vec<u8> {
        let Ok(encoded) = deps.pairing.encode_g2(
            EncodeG2Params,
            EncodeG2Payload {
                point: point.clone(),
            },
        );
        encoded.bytes.as_ref().to_vec()
    };
    let record_mint_context =
        |context: &DeliveryContext<D::Forms, MintPurpose>| -> ProofVectorContext {
            let Ok(chain) =
                <D::Forms as IChainForms>::ChainIdentifier::to_field(ToFieldParams, &context.chain);
            let Ok(entitlement_contract) = <D::Forms as IChainForms>::Identity::to_field(
                ToFieldParams,
                &context.entitlement_contract,
            );
            let Ok(source_entitlement) = <D::Forms as IChainForms>::Entitlement::to_field(
                ToFieldParams,
                &context.source_entitlement,
            );
            let Ok(target_entitlement) = <D::Forms as IChainForms>::Entitlement::to_field(
                ToFieldParams,
                &context.target_entitlement,
            );
            let Ok(old_interval) =
                <D::Forms as IChainForms>::Interval::to_field(ToFieldParams, &context.old_interval);
            let Ok(new_interval) =
                <D::Forms as IChainForms>::Interval::to_field(ToFieldParams, &context.new_interval);
            let Ok(seller) =
                <D::Forms as IChainForms>::Identity::to_field(ToFieldParams, &context.seller);
            let Ok(buyer) =
                <D::Forms as IChainForms>::Identity::to_field(ToFieldParams, &context.buyer);
            ProofVectorContext {
                suite_identifier: *context.suite_identifier.identifier(),
                suite_version: context.suite_identifier.version(),
                chain: chain.field,
                entitlement_contract: entitlement_contract.field,
                asset_root: *context.asset_root.as_bytes(),
                parameter_set: *context.parameter_set_identifier.as_bytes(),
                source_entitlement: source_entitlement.field,
                target_entitlement: target_entitlement.field,
                old_interval: old_interval.field,
                new_interval: new_interval.field,
                purpose: context.purpose.code(),
                seller: seller.field,
                buyer: buyer.field,
                expiry: context.expiry,
            }
        };
    let record_transfer_context =
        |context: &DeliveryContext<D::Forms, TransferPurpose>| -> ProofVectorContext {
            let Ok(chain) =
                <D::Forms as IChainForms>::ChainIdentifier::to_field(ToFieldParams, &context.chain);
            let Ok(entitlement_contract) = <D::Forms as IChainForms>::Identity::to_field(
                ToFieldParams,
                &context.entitlement_contract,
            );
            let Ok(source_entitlement) = <D::Forms as IChainForms>::Entitlement::to_field(
                ToFieldParams,
                &context.source_entitlement,
            );
            let Ok(target_entitlement) = <D::Forms as IChainForms>::Entitlement::to_field(
                ToFieldParams,
                &context.target_entitlement,
            );
            let Ok(old_interval) =
                <D::Forms as IChainForms>::Interval::to_field(ToFieldParams, &context.old_interval);
            let Ok(new_interval) =
                <D::Forms as IChainForms>::Interval::to_field(ToFieldParams, &context.new_interval);
            let Ok(seller) =
                <D::Forms as IChainForms>::Identity::to_field(ToFieldParams, &context.seller);
            let Ok(buyer) =
                <D::Forms as IChainForms>::Identity::to_field(ToFieldParams, &context.buyer);
            ProofVectorContext {
                suite_identifier: *context.suite_identifier.identifier(),
                suite_version: context.suite_identifier.version(),
                chain: chain.field,
                entitlement_contract: entitlement_contract.field,
                asset_root: *context.asset_root.as_bytes(),
                parameter_set: *context.parameter_set_identifier.as_bytes(),
                source_entitlement: source_entitlement.field,
                target_entitlement: target_entitlement.field,
                old_interval: old_interval.field,
                new_interval: new_interval.field,
                purpose: context.purpose.code(),
                seller: seller.field,
                buyer: buyer.field,
                expiry: context.expiry,
            }
        };

    let mut mint_records = Vec::new();
    for _ in 0..params.mint_count.get() {
        let minted = mint::<P, K, A, D>(
            &MintDeps {
                kem: deps.kem,
                key_agreement: deps.key_agreement,
                delivery_proof: deps.delivery_proof,
                random: deps.random,
            },
            MintParams,
            MintPayload {
                context: payload.mint,
                identity: payload.identity,
            },
        )
        .map_err(ProofVectorsErrorReturn::Mint)?
        .delivery;
        let Ok(components) = deps.delivery_proof.proof_components(
            ProofComponentsParams,
            ProofComponentsPayload {
                proof: &minted.proof,
            },
        );
        let AlgebraicMintStatement {
            context,
            hpub,
            identity_element,
            buyer_keys,
            envelope,
        } = minted.statement;
        let PublicKeysComponents { pk1, pk2 } = buyer_keys;
        let EnvelopeComponents { c1, c2, d1, d2 } = envelope;
        let context = record_mint_context(&context);
        let draws = minted.draws;
        match components.components {
            DeliveryProofComponents::MintBothGroups {
                challenge,
                responses,
            } => mint_records.push(MintProofVector {
                draws,
                context,
                hpub: g2_bytes(&hpub),
                identity_element: g1_bytes(&identity_element),
                buyer_pk1: g1_bytes(&pk1),
                buyer_pk2: g2_bytes(&pk2),
                envelope_c1: g1_bytes(&c1),
                envelope_c2: g1_bytes(&c2),
                envelope_d1: g2_bytes(&d1),
                envelope_d2: g2_bytes(&d2),
                challenge: scalar_bytes(&challenge)?,
                alpha: scalar_bytes(&responses.alpha)?,
                r: scalar_bytes(&responses.r)?,
                rho: scalar_bytes(&responses.rho)?,
                sigma: scalar_bytes(&responses.sigma)?,
                second_group_first_messages: Vec::new(),
            }),
            DeliveryProofComponents::MintFirstGroupOnly {
                challenge,
                responses,
                first_messages,
            } => {
                let mut second_group_first_messages = g2_bytes(&first_messages.hpub);
                second_group_first_messages.extend_from_slice(&g2_bytes(&first_messages.d1));
                second_group_first_messages.extend_from_slice(&g2_bytes(&first_messages.d2));
                mint_records.push(MintProofVector {
                    draws,
                    context,
                    hpub: g2_bytes(&hpub),
                    identity_element: g1_bytes(&identity_element),
                    buyer_pk1: g1_bytes(&pk1),
                    buyer_pk2: g2_bytes(&pk2),
                    envelope_c1: g1_bytes(&c1),
                    envelope_c2: g1_bytes(&c2),
                    envelope_d1: g2_bytes(&d1),
                    envelope_d2: g2_bytes(&d2),
                    challenge: scalar_bytes(&challenge)?,
                    alpha: scalar_bytes(&responses.alpha)?,
                    r: scalar_bytes(&responses.r)?,
                    rho: scalar_bytes(&responses.rho)?,
                    sigma: scalar_bytes(&responses.sigma)?,
                    second_group_first_messages,
                });
            }
            DeliveryProofComponents::TransferBothGroups { .. }
            | DeliveryProofComponents::TransferFirstGroupOnly { .. } => {
                return Err(ProofVectorsErrorReturn::ProofComponents {
                    relation: DeliveryRelation::Transfer,
                });
            }
        }
    }

    let mut transfer_records = Vec::new();
    for _ in 0..params.transfer_count.get() {
        let minted = mint::<P, K, A, D>(
            &MintDeps {
                kem: deps.kem,
                key_agreement: deps.key_agreement,
                delivery_proof: deps.delivery_proof,
                random: deps.random,
            },
            MintParams,
            MintPayload {
                context: payload.mint,
                identity: payload.identity,
            },
        )
        .map_err(ProofVectorsErrorReturn::Mint)?
        .delivery;
        let transferred = transfer::<P, K, A, D>(
            &TransferDeps {
                kem: deps.kem,
                key_agreement: deps.key_agreement,
                delivery_proof: deps.delivery_proof,
                random: deps.random,
            },
            TransferParams,
            TransferPayload {
                context: payload.transfer,
                parameter_set: &minted.parameter_set,
                identity_element: &minted.identity_element,
                holder: &minted.recipient,
            },
        )
        .map_err(ProofVectorsErrorReturn::Transfer)?
        .delivery;
        let Ok(components) = deps.delivery_proof.proof_components(
            ProofComponentsParams,
            ProofComponentsPayload {
                proof: &transferred.proof,
            },
        );
        let mut draws = minted.draws;
        draws.extend_from_slice(&transferred.draws);
        let AlgebraicTransferStatement {
            context,
            identity_element,
            seller_keys,
            buyer_keys,
            old_envelope,
            new_envelope,
        } = transferred.statement;
        let PublicKeysComponents {
            pk1: seller_pk1,
            pk2: seller_pk2,
        } = seller_keys;
        let PublicKeysComponents {
            pk1: buyer_pk1,
            pk2: buyer_pk2,
        } = buyer_keys;
        let EnvelopeComponents {
            c1: old_envelope_c1,
            c2: old_envelope_c2,
            d1: old_envelope_d1,
            d2: old_envelope_d2,
        } = old_envelope;
        let EnvelopeComponents {
            c1: new_envelope_c1,
            c2: new_envelope_c2,
            d1: new_envelope_d1,
            d2: new_envelope_d2,
        } = new_envelope;
        let context = record_transfer_context(&context);
        match components.components {
            DeliveryProofComponents::TransferBothGroups {
                challenge,
                responses,
            } => transfer_records.push(TransferProofVector {
                draws,
                context,
                identity_element: g1_bytes(&identity_element),
                seller_pk1: g1_bytes(&seller_pk1),
                seller_pk2: g2_bytes(&seller_pk2),
                buyer_pk1: g1_bytes(&buyer_pk1),
                buyer_pk2: g2_bytes(&buyer_pk2),
                old_envelope_c1: g1_bytes(&old_envelope_c1),
                old_envelope_c2: g1_bytes(&old_envelope_c2),
                old_envelope_d1: g2_bytes(&old_envelope_d1),
                old_envelope_d2: g2_bytes(&old_envelope_d2),
                new_envelope_c1: g1_bytes(&new_envelope_c1),
                new_envelope_c2: g1_bytes(&new_envelope_c2),
                new_envelope_d1: g2_bytes(&new_envelope_d1),
                new_envelope_d2: g2_bytes(&new_envelope_d2),
                challenge: scalar_bytes(&challenge)?,
                x: scalar_bytes(&responses.x)?,
                y: scalar_bytes(&responses.y)?,
                s: scalar_bytes(&responses.s)?,
                rho: scalar_bytes(&responses.rho)?,
                sigma: scalar_bytes(&responses.sigma)?,
                second_group_first_messages: Vec::new(),
            }),
            DeliveryProofComponents::TransferFirstGroupOnly {
                challenge,
                responses,
                first_messages,
            } => {
                let mut second_group_first_messages = g2_bytes(&first_messages.pk2);
                second_group_first_messages.extend_from_slice(&g2_bytes(&first_messages.d1));
                second_group_first_messages.extend_from_slice(&g2_bytes(&first_messages.d2));
                transfer_records.push(TransferProofVector {
                    draws,
                    context,
                    identity_element: g1_bytes(&identity_element),
                    seller_pk1: g1_bytes(&seller_pk1),
                    seller_pk2: g2_bytes(&seller_pk2),
                    buyer_pk1: g1_bytes(&buyer_pk1),
                    buyer_pk2: g2_bytes(&buyer_pk2),
                    old_envelope_c1: g1_bytes(&old_envelope_c1),
                    old_envelope_c2: g1_bytes(&old_envelope_c2),
                    old_envelope_d1: g2_bytes(&old_envelope_d1),
                    old_envelope_d2: g2_bytes(&old_envelope_d2),
                    new_envelope_c1: g1_bytes(&new_envelope_c1),
                    new_envelope_c2: g1_bytes(&new_envelope_c2),
                    new_envelope_d1: g2_bytes(&new_envelope_d1),
                    new_envelope_d2: g2_bytes(&new_envelope_d2),
                    challenge: scalar_bytes(&challenge)?,
                    x: scalar_bytes(&responses.x)?,
                    y: scalar_bytes(&responses.y)?,
                    s: scalar_bytes(&responses.s)?,
                    rho: scalar_bytes(&responses.rho)?,
                    sigma: scalar_bytes(&responses.sigma)?,
                    second_group_first_messages,
                });
            }
            DeliveryProofComponents::MintBothGroups { .. }
            | DeliveryProofComponents::MintFirstGroupOnly { .. } => {
                return Err(ProofVectorsErrorReturn::ProofComponents {
                    relation: DeliveryRelation::Mint,
                });
            }
        }
    }
    Ok(ProofVectorsSuccessReturn {
        mint: mint_records,
        transfer: transfer_records,
    })
}
