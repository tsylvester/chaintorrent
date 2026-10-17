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
#[cfg(test)]
mod test_error_2;

use chain::IChainForms;
use core::marker::PhantomData;
use domain::{ASSET_IDENTITY_HASH_LENGTH, AssetIdentityHash, AssetIdentityHashConstructorParams};
use encoding::{
    CanonicalFieldKind, CanonicalFieldValue, CanonicalFields, FromFieldParams, FromFieldsParams,
    FromFieldsReturn, FromFieldsSuccessReturn, ICanonicalField, IEncodingContract, ToFieldParams,
    ToFieldsParams, ToFieldsReturn, ToFieldsSuccessReturn,
};
use envelope::{
    EnvelopeComponentsParams, EnvelopeComponentsPayload, GenerateKeysParams, GenerateKeysPayload,
    IKeyAgreementAdapter, KeyPairComponentsParams, KeyPairComponentsPayload,
    KeyPairPublicKeysParams, KeyPairPublicKeysPayload, PublicKeysComponents,
    PublicKeysFromComponentsParams, PublicKeysFromComponentsPayload, UnwrapParams, UnwrapPayload,
    WrapToParams, WrapToPayload,
};
use kem::{
    CredentialComponentsParams, CredentialComponentsPayload, CredentialFromComponentsParams,
    CredentialFromComponentsPayload, DeriveIdentityParams, DeriveIdentityPayload,
    ICredentialKemAdapter, IdentityElementComponentsParams, IdentityElementComponentsPayload,
    IdentityScope, IssueParams, IssuePayload, KemIdentity, MasterScalarComponentsParams,
    MasterScalarComponentsPayload, ParameterSetComponentsParams, ParameterSetComponentsPayload,
    RerandomizeParams, RerandomizePayload, SetupParams, SetupPayload, SetupScope,
};
use pairing::{
    EncodeG1Params, EncodeG1Payload, EncodeG2Params, EncodeG2Payload, EncodeScalarParams,
    EncodeScalarPayload, IPairingAdapter, ISampleUniformScalar,
};
use proof::{
    AlgebraicMintStatement, AlgebraicTransferStatement, DeliveryContext, DeliveryProofComponents,
    DeliveryRelation, IDeliveryProofAdapter, ProofComponentsParams, ProofComponentsPayload,
    ProveMintParams, ProveMintPayload, ProveTransferParams, ProveTransferPayload,
};
use random::{FillBytesParams, FillBytesPayload};

use super::challenge_vectors::provides::{CanonicalDeliveryContext, DeliveryContextField};
use super::render::provides::{
    SolidityMemberName, SolidityMemberNameConstructorParams, SolidityRecordMember,
    SolidityRecordMembers, SolidityRecordMembersConstructorParams,
};
use interface::{
    MINT_PROOF_VECTOR_FIELD_COUNT, MINT_PROOF_VECTOR_MEMBER_NAMES, MintProofVector,
    MintProofVectorDescription, MintProofVectorDescriptionConstructorParams,
    MintProofVectorDescriptionTryNewReturn, MintProofVectorFromFieldsErrorReturn,
    ProofVectorContext, ProofVectorMembersErrorReturn, ProofVectorMembersReturn, ProofVectorsDeps,
    ProofVectorsErrorReturn, ProofVectorsParams, ProofVectorsPayload, ProofVectorsReturn,
    ProofVectorsSuccessReturn, TRANSFER_PROOF_VECTOR_FIELD_COUNT,
    TRANSFER_PROOF_VECTOR_MEMBER_NAMES, TransferProofVector, TransferProofVectorDescription,
    TransferProofVectorDescriptionConstructorParams, TransferProofVectorDescriptionTryNewReturn,
    TransferProofVectorFromFieldsErrorReturn,
};

impl<F: IChainForms> MintProofVectorDescription<F> {
    pub fn try_new(
        _params: MintProofVectorDescriptionConstructorParams,
    ) -> MintProofVectorDescriptionTryNewReturn<F> {
        Ok(MintProofVectorDescription { forms: PhantomData })
    }

    pub fn members(&self) -> ProofVectorMembersReturn {
        let mut members = Vec::new();
        for (index, text) in MINT_PROOF_VECTOR_MEMBER_NAMES.iter().enumerate() {
            let name = SolidityMemberName::try_new(SolidityMemberNameConstructorParams {
                text: text.to_string(),
            })
            .map_err(ProofVectorMembersErrorReturn::MemberName)?;
            members.push(SolidityRecordMember {
                name,
                kind: <Self as IEncodingContract>::FIELDS[index],
            });
        }
        SolidityRecordMembers::try_new(SolidityRecordMembersConstructorParams { members })
            .map_err(ProofVectorMembersErrorReturn::RecordMembers)
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
                    CanonicalFieldValue::FixedBytes32(payload.context.asset_identity_hash),
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
        let actual = payload.values.len();
        let Ok(fields) =
            <[CanonicalFieldValue; MINT_PROOF_VECTOR_FIELD_COUNT]>::try_from(payload.values)
        else {
            return Err(MintProofVectorFromFieldsErrorReturn::FieldCount {
                expected: MINT_PROOF_VECTOR_FIELD_COUNT,
                actual,
            });
        };
        let kind_of = |value: &CanonicalFieldValue| match value {
            CanonicalFieldValue::FixedBytes32(_) => CanonicalFieldKind::FixedBytes32,
            CanonicalFieldValue::Unsigned16(_) => CanonicalFieldKind::Unsigned16,
            CanonicalFieldValue::Unsigned32(_) => CanonicalFieldKind::Unsigned32,
            CanonicalFieldValue::Unsigned64(_) => CanonicalFieldKind::Unsigned64,
            CanonicalFieldValue::Text(_) => CanonicalFieldKind::Text,
            CanonicalFieldValue::Bytes(_) => CanonicalFieldKind::Bytes,
            CanonicalFieldValue::FixedBytes20(_) => CanonicalFieldKind::FixedBytes20,
            CanonicalFieldValue::Unsigned256(_) => CanonicalFieldKind::Unsigned256,
        };
        for (index, value) in fields.iter().enumerate() {
            if kind_of(value) != Self::FIELDS[index] {
                return Err(MintProofVectorFromFieldsErrorReturn::FieldKind {
                    index,
                    expected: Self::FIELDS[index],
                });
            }
        }
        let [
            draws,
            suite_identifier,
            suite_version,
            chain,
            entitlement_contract,
            asset_identity_hash,
            parameter_set,
            source_entitlement,
            target_entitlement,
            old_interval,
            new_interval,
            purpose,
            seller,
            buyer,
            expiry,
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
        ] = fields;
        let CanonicalFieldValue::Bytes(draws) = draws else {
            return Err(MintProofVectorFromFieldsErrorReturn::FieldKind {
                index: 0,
                expected: Self::FIELDS[0],
            });
        };
        let CanonicalFieldValue::FixedBytes32(suite_identifier) = suite_identifier else {
            return Err(MintProofVectorFromFieldsErrorReturn::FieldKind {
                index: 1,
                expected: Self::FIELDS[1],
            });
        };
        let CanonicalFieldValue::Unsigned16(suite_version) = suite_version else {
            return Err(MintProofVectorFromFieldsErrorReturn::FieldKind {
                index: 2,
                expected: Self::FIELDS[2],
            });
        };
        let CanonicalFieldValue::FixedBytes32(asset_identity_hash) = asset_identity_hash else {
            return Err(MintProofVectorFromFieldsErrorReturn::FieldKind {
                index: 5,
                expected: Self::FIELDS[5],
            });
        };
        let CanonicalFieldValue::FixedBytes32(parameter_set) = parameter_set else {
            return Err(MintProofVectorFromFieldsErrorReturn::FieldKind {
                index: 6,
                expected: Self::FIELDS[6],
            });
        };
        let CanonicalFieldValue::Unsigned16(purpose) = purpose else {
            return Err(MintProofVectorFromFieldsErrorReturn::FieldKind {
                index: 11,
                expected: Self::FIELDS[11],
            });
        };
        let CanonicalFieldValue::Unsigned64(expiry) = expiry else {
            return Err(MintProofVectorFromFieldsErrorReturn::FieldKind {
                index: 14,
                expected: Self::FIELDS[14],
            });
        };
        let CanonicalFieldValue::Bytes(hpub) = hpub else {
            return Err(MintProofVectorFromFieldsErrorReturn::FieldKind {
                index: 15,
                expected: Self::FIELDS[15],
            });
        };
        let CanonicalFieldValue::Bytes(identity_element) = identity_element else {
            return Err(MintProofVectorFromFieldsErrorReturn::FieldKind {
                index: 16,
                expected: Self::FIELDS[16],
            });
        };
        let CanonicalFieldValue::Bytes(buyer_pk1) = buyer_pk1 else {
            return Err(MintProofVectorFromFieldsErrorReturn::FieldKind {
                index: 17,
                expected: Self::FIELDS[17],
            });
        };
        let CanonicalFieldValue::Bytes(buyer_pk2) = buyer_pk2 else {
            return Err(MintProofVectorFromFieldsErrorReturn::FieldKind {
                index: 18,
                expected: Self::FIELDS[18],
            });
        };
        let CanonicalFieldValue::Bytes(envelope_c1) = envelope_c1 else {
            return Err(MintProofVectorFromFieldsErrorReturn::FieldKind {
                index: 19,
                expected: Self::FIELDS[19],
            });
        };
        let CanonicalFieldValue::Bytes(envelope_c2) = envelope_c2 else {
            return Err(MintProofVectorFromFieldsErrorReturn::FieldKind {
                index: 20,
                expected: Self::FIELDS[20],
            });
        };
        let CanonicalFieldValue::Bytes(envelope_d1) = envelope_d1 else {
            return Err(MintProofVectorFromFieldsErrorReturn::FieldKind {
                index: 21,
                expected: Self::FIELDS[21],
            });
        };
        let CanonicalFieldValue::Bytes(envelope_d2) = envelope_d2 else {
            return Err(MintProofVectorFromFieldsErrorReturn::FieldKind {
                index: 22,
                expected: Self::FIELDS[22],
            });
        };
        let CanonicalFieldValue::Unsigned256(challenge) = challenge else {
            return Err(MintProofVectorFromFieldsErrorReturn::FieldKind {
                index: 23,
                expected: Self::FIELDS[23],
            });
        };
        let CanonicalFieldValue::Unsigned256(alpha) = alpha else {
            return Err(MintProofVectorFromFieldsErrorReturn::FieldKind {
                index: 24,
                expected: Self::FIELDS[24],
            });
        };
        let CanonicalFieldValue::Unsigned256(r) = r else {
            return Err(MintProofVectorFromFieldsErrorReturn::FieldKind {
                index: 25,
                expected: Self::FIELDS[25],
            });
        };
        let CanonicalFieldValue::Unsigned256(rho) = rho else {
            return Err(MintProofVectorFromFieldsErrorReturn::FieldKind {
                index: 26,
                expected: Self::FIELDS[26],
            });
        };
        let CanonicalFieldValue::Unsigned256(sigma) = sigma else {
            return Err(MintProofVectorFromFieldsErrorReturn::FieldKind {
                index: 27,
                expected: Self::FIELDS[27],
            });
        };
        let CanonicalFieldValue::Bytes(second_group_first_messages) = second_group_first_messages
        else {
            return Err(MintProofVectorFromFieldsErrorReturn::FieldKind {
                index: 28,
                expected: Self::FIELDS[28],
            });
        };
        Ok(FromFieldsSuccessReturn {
            described: MintProofVector {
                draws,
                context: ProofVectorContext {
                    suite_identifier,
                    suite_version,
                    chain,
                    entitlement_contract,
                    asset_identity_hash,
                    parameter_set,
                    source_entitlement,
                    target_entitlement,
                    old_interval,
                    new_interval,
                    purpose,
                    seller,
                    buyer,
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

    pub fn members(&self) -> ProofVectorMembersReturn {
        let mut members = Vec::new();
        for (index, text) in TRANSFER_PROOF_VECTOR_MEMBER_NAMES.iter().enumerate() {
            let name = SolidityMemberName::try_new(SolidityMemberNameConstructorParams {
                text: text.to_string(),
            })
            .map_err(ProofVectorMembersErrorReturn::MemberName)?;
            members.push(SolidityRecordMember {
                name,
                kind: <Self as IEncodingContract>::FIELDS[index],
            });
        }
        SolidityRecordMembers::try_new(SolidityRecordMembersConstructorParams { members })
            .map_err(ProofVectorMembersErrorReturn::RecordMembers)
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
                    CanonicalFieldValue::FixedBytes32(payload.context.asset_identity_hash),
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
        let actual = payload.values.len();
        let Ok(fields) =
            <[CanonicalFieldValue; TRANSFER_PROOF_VECTOR_FIELD_COUNT]>::try_from(payload.values)
        else {
            return Err(TransferProofVectorFromFieldsErrorReturn::FieldCount {
                expected: TRANSFER_PROOF_VECTOR_FIELD_COUNT,
                actual,
            });
        };
        let kind_of = |value: &CanonicalFieldValue| match value {
            CanonicalFieldValue::FixedBytes32(_) => CanonicalFieldKind::FixedBytes32,
            CanonicalFieldValue::Unsigned16(_) => CanonicalFieldKind::Unsigned16,
            CanonicalFieldValue::Unsigned32(_) => CanonicalFieldKind::Unsigned32,
            CanonicalFieldValue::Unsigned64(_) => CanonicalFieldKind::Unsigned64,
            CanonicalFieldValue::Text(_) => CanonicalFieldKind::Text,
            CanonicalFieldValue::Bytes(_) => CanonicalFieldKind::Bytes,
            CanonicalFieldValue::FixedBytes20(_) => CanonicalFieldKind::FixedBytes20,
            CanonicalFieldValue::Unsigned256(_) => CanonicalFieldKind::Unsigned256,
        };
        for (index, value) in fields.iter().enumerate() {
            if kind_of(value) != Self::FIELDS[index] {
                return Err(TransferProofVectorFromFieldsErrorReturn::FieldKind {
                    index,
                    expected: Self::FIELDS[index],
                });
            }
        }
        let [
            draws,
            suite_identifier,
            suite_version,
            chain,
            entitlement_contract,
            asset_identity_hash,
            parameter_set,
            source_entitlement,
            target_entitlement,
            old_interval,
            new_interval,
            purpose,
            seller,
            buyer,
            expiry,
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
        ] = fields;
        let CanonicalFieldValue::Bytes(draws) = draws else {
            return Err(TransferProofVectorFromFieldsErrorReturn::FieldKind {
                index: 0,
                expected: Self::FIELDS[0],
            });
        };
        let CanonicalFieldValue::FixedBytes32(suite_identifier) = suite_identifier else {
            return Err(TransferProofVectorFromFieldsErrorReturn::FieldKind {
                index: 1,
                expected: Self::FIELDS[1],
            });
        };
        let CanonicalFieldValue::Unsigned16(suite_version) = suite_version else {
            return Err(TransferProofVectorFromFieldsErrorReturn::FieldKind {
                index: 2,
                expected: Self::FIELDS[2],
            });
        };
        let CanonicalFieldValue::FixedBytes32(asset_identity_hash) = asset_identity_hash else {
            return Err(TransferProofVectorFromFieldsErrorReturn::FieldKind {
                index: 5,
                expected: Self::FIELDS[5],
            });
        };
        let CanonicalFieldValue::FixedBytes32(parameter_set) = parameter_set else {
            return Err(TransferProofVectorFromFieldsErrorReturn::FieldKind {
                index: 6,
                expected: Self::FIELDS[6],
            });
        };
        let CanonicalFieldValue::Unsigned16(purpose) = purpose else {
            return Err(TransferProofVectorFromFieldsErrorReturn::FieldKind {
                index: 11,
                expected: Self::FIELDS[11],
            });
        };
        let CanonicalFieldValue::Unsigned64(expiry) = expiry else {
            return Err(TransferProofVectorFromFieldsErrorReturn::FieldKind {
                index: 14,
                expected: Self::FIELDS[14],
            });
        };
        let CanonicalFieldValue::Bytes(identity_element) = identity_element else {
            return Err(TransferProofVectorFromFieldsErrorReturn::FieldKind {
                index: 15,
                expected: Self::FIELDS[15],
            });
        };
        let CanonicalFieldValue::Bytes(seller_pk1) = seller_pk1 else {
            return Err(TransferProofVectorFromFieldsErrorReturn::FieldKind {
                index: 16,
                expected: Self::FIELDS[16],
            });
        };
        let CanonicalFieldValue::Bytes(seller_pk2) = seller_pk2 else {
            return Err(TransferProofVectorFromFieldsErrorReturn::FieldKind {
                index: 17,
                expected: Self::FIELDS[17],
            });
        };
        let CanonicalFieldValue::Bytes(buyer_pk1) = buyer_pk1 else {
            return Err(TransferProofVectorFromFieldsErrorReturn::FieldKind {
                index: 18,
                expected: Self::FIELDS[18],
            });
        };
        let CanonicalFieldValue::Bytes(buyer_pk2) = buyer_pk2 else {
            return Err(TransferProofVectorFromFieldsErrorReturn::FieldKind {
                index: 19,
                expected: Self::FIELDS[19],
            });
        };
        let CanonicalFieldValue::Bytes(old_envelope_c1) = old_envelope_c1 else {
            return Err(TransferProofVectorFromFieldsErrorReturn::FieldKind {
                index: 20,
                expected: Self::FIELDS[20],
            });
        };
        let CanonicalFieldValue::Bytes(old_envelope_c2) = old_envelope_c2 else {
            return Err(TransferProofVectorFromFieldsErrorReturn::FieldKind {
                index: 21,
                expected: Self::FIELDS[21],
            });
        };
        let CanonicalFieldValue::Bytes(old_envelope_d1) = old_envelope_d1 else {
            return Err(TransferProofVectorFromFieldsErrorReturn::FieldKind {
                index: 22,
                expected: Self::FIELDS[22],
            });
        };
        let CanonicalFieldValue::Bytes(old_envelope_d2) = old_envelope_d2 else {
            return Err(TransferProofVectorFromFieldsErrorReturn::FieldKind {
                index: 23,
                expected: Self::FIELDS[23],
            });
        };
        let CanonicalFieldValue::Bytes(new_envelope_c1) = new_envelope_c1 else {
            return Err(TransferProofVectorFromFieldsErrorReturn::FieldKind {
                index: 24,
                expected: Self::FIELDS[24],
            });
        };
        let CanonicalFieldValue::Bytes(new_envelope_c2) = new_envelope_c2 else {
            return Err(TransferProofVectorFromFieldsErrorReturn::FieldKind {
                index: 25,
                expected: Self::FIELDS[25],
            });
        };
        let CanonicalFieldValue::Bytes(new_envelope_d1) = new_envelope_d1 else {
            return Err(TransferProofVectorFromFieldsErrorReturn::FieldKind {
                index: 26,
                expected: Self::FIELDS[26],
            });
        };
        let CanonicalFieldValue::Bytes(new_envelope_d2) = new_envelope_d2 else {
            return Err(TransferProofVectorFromFieldsErrorReturn::FieldKind {
                index: 27,
                expected: Self::FIELDS[27],
            });
        };
        let CanonicalFieldValue::Unsigned256(challenge) = challenge else {
            return Err(TransferProofVectorFromFieldsErrorReturn::FieldKind {
                index: 28,
                expected: Self::FIELDS[28],
            });
        };
        let CanonicalFieldValue::Unsigned256(x) = x else {
            return Err(TransferProofVectorFromFieldsErrorReturn::FieldKind {
                index: 29,
                expected: Self::FIELDS[29],
            });
        };
        let CanonicalFieldValue::Unsigned256(y) = y else {
            return Err(TransferProofVectorFromFieldsErrorReturn::FieldKind {
                index: 30,
                expected: Self::FIELDS[30],
            });
        };
        let CanonicalFieldValue::Unsigned256(s) = s else {
            return Err(TransferProofVectorFromFieldsErrorReturn::FieldKind {
                index: 31,
                expected: Self::FIELDS[31],
            });
        };
        let CanonicalFieldValue::Unsigned256(rho) = rho else {
            return Err(TransferProofVectorFromFieldsErrorReturn::FieldKind {
                index: 32,
                expected: Self::FIELDS[32],
            });
        };
        let CanonicalFieldValue::Unsigned256(sigma) = sigma else {
            return Err(TransferProofVectorFromFieldsErrorReturn::FieldKind {
                index: 33,
                expected: Self::FIELDS[33],
            });
        };
        let CanonicalFieldValue::Bytes(second_group_first_messages) = second_group_first_messages
        else {
            return Err(TransferProofVectorFromFieldsErrorReturn::FieldKind {
                index: 34,
                expected: Self::FIELDS[34],
            });
        };
        Ok(FromFieldsSuccessReturn {
            described: TransferProofVector {
                draws,
                context: ProofVectorContext {
                    suite_identifier,
                    suite_version,
                    chain,
                    entitlement_contract,
                    asset_identity_hash,
                    parameter_set,
                    source_entitlement,
                    target_entitlement,
                    old_interval,
                    new_interval,
                    purpose,
                    seller,
                    buyer,
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
    payload: ProofVectorsPayload<'_>,
) -> ProofVectorsReturn {
    let asset_hash = match params.scope {
        IdentityScope::Entitlement => None,
        IdentityScope::Asset => {
            let bytes = match <[u8; ASSET_IDENTITY_HASH_LENGTH]>::try_from(payload.identity) {
                Ok(bytes) => bytes,
                Err(_) => {
                    return Err(ProofVectorsErrorReturn::AssetIdentityLength {
                        actual: payload.identity.len(),
                    });
                }
            };
            match AssetIdentityHash::try_new(AssetIdentityHashConstructorParams { bytes }) {
                Ok(hash) => Some(hash),
                Err(error) => return Err(ProofVectorsErrorReturn::AssetIdentityHash(error)),
            }
        }
    };

    let setup_scope = || match &asset_hash {
        Some(hash) => SetupScope::Asset {
            identity_hash: hash,
        },
        None => SetupScope::Entitlement,
    };
    let kem_identity = || match &asset_hash {
        Some(hash) => KemIdentity::Asset {
            identity_hash: hash,
        },
        None => KemIdentity::Entitlement {
            canonical: payload.identity,
        },
    };

    let draw = |draws: &mut Vec<u8>| {
        let filled = deps
            .random
            .fill_bytes(
                FillBytesParams,
                FillBytesPayload {
                    length: <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH,
                },
            )
            .map_err(ProofVectorsErrorReturn::FillBytes)?;
        draws.extend_from_slice(filled.bytes.expose());
        Ok(filled.bytes)
    };

    let encode_g1 = |point: P::G1| -> Vec<u8> {
        let Ok(encoded) = deps
            .pairing
            .encode_g1(EncodeG1Params, EncodeG1Payload { point });
        encoded.bytes.as_ref().to_vec()
    };
    let encode_g2 = |point: P::G2| -> Vec<u8> {
        let Ok(encoded) = deps
            .pairing
            .encode_g2(EncodeG2Params, EncodeG2Payload { point });
        encoded.bytes.as_ref().to_vec()
    };
    let scalar_bytes = |scalar: P::Scalar| -> Result<[u8; 32], ProofVectorsErrorReturn> {
        let Ok(encoded) = deps
            .pairing
            .encode_scalar(EncodeScalarParams, EncodeScalarPayload { scalar });
        let bytes = encoded.bytes.expose().as_ref();
        match <[u8; 32]>::try_from(bytes) {
            Ok(value) => Ok(value),
            Err(_) => Err(ProofVectorsErrorReturn::ScalarLength {
                actual: bytes.len(),
            }),
        }
    };

    let rebuild_forms = |relation: DeliveryRelation, canonical: &CanonicalDeliveryContext| {
        let chain = <D::Forms as IChainForms>::ChainIdentifier::from_field(
            FromFieldParams,
            canonical.chain.clone(),
        )
        .map(|success| success.value)
        .map_err(|error| ProofVectorsErrorReturn::ChainIdentifier { relation, error })?;
        let entitlement_contract = <D::Forms as IChainForms>::Identity::from_field(
            FromFieldParams,
            canonical.entitlement_contract.clone(),
        )
        .map(|success| success.value)
        .map_err(|error| ProofVectorsErrorReturn::Identity {
            relation,
            field: DeliveryContextField::EntitlementContract,
            error,
        })?;
        let source_entitlement = <D::Forms as IChainForms>::Entitlement::from_field(
            FromFieldParams,
            canonical.source_entitlement.clone(),
        )
        .map(|success| success.value)
        .map_err(|error| ProofVectorsErrorReturn::Entitlement {
            relation,
            field: DeliveryContextField::SourceEntitlement,
            error,
        })?;
        let target_entitlement = <D::Forms as IChainForms>::Entitlement::from_field(
            FromFieldParams,
            canonical.target_entitlement.clone(),
        )
        .map(|success| success.value)
        .map_err(|error| ProofVectorsErrorReturn::Entitlement {
            relation,
            field: DeliveryContextField::TargetEntitlement,
            error,
        })?;
        let old_interval = <D::Forms as IChainForms>::Interval::from_field(
            FromFieldParams,
            canonical.old_interval.clone(),
        )
        .map(|success| success.value)
        .map_err(|error| ProofVectorsErrorReturn::Interval {
            relation,
            field: DeliveryContextField::OldInterval,
            error,
        })?;
        let new_interval = <D::Forms as IChainForms>::Interval::from_field(
            FromFieldParams,
            canonical.new_interval.clone(),
        )
        .map(|success| success.value)
        .map_err(|error| ProofVectorsErrorReturn::Interval {
            relation,
            field: DeliveryContextField::NewInterval,
            error,
        })?;
        let seller = <D::Forms as IChainForms>::Identity::from_field(
            FromFieldParams,
            canonical.seller.clone(),
        )
        .map(|success| success.value)
        .map_err(|error| ProofVectorsErrorReturn::Identity {
            relation,
            field: DeliveryContextField::Seller,
            error,
        })?;
        let buyer = <D::Forms as IChainForms>::Identity::from_field(
            FromFieldParams,
            canonical.buyer.clone(),
        )
        .map(|success| success.value)
        .map_err(|error| ProofVectorsErrorReturn::Identity {
            relation,
            field: DeliveryContextField::Buyer,
            error,
        })?;
        Ok((
            chain,
            entitlement_contract,
            source_entitlement,
            target_entitlement,
            old_interval,
            new_interval,
            seller,
            buyer,
        ))
    };

    let issue_credential = |draws: &mut Vec<u8>| {
        let master_uniform = draw(draws)?;
        let u0_uniform = draw(draws)?;
        let u1_uniform = draw(draws)?;
        let setup = deps
            .kem
            .setup(
                SetupParams {
                    scope: setup_scope(),
                },
                SetupPayload {
                    master_uniform,
                    u0_uniform,
                    u1_uniform,
                },
            )
            .map_err(ProofVectorsErrorReturn::Setup)?;
        let identity_element = deps
            .kem
            .derive_identity(
                DeriveIdentityParams,
                DeriveIdentityPayload {
                    parameter_set: &setup.parameter_set,
                    identity: kem_identity(),
                },
            )
            .map_err(ProofVectorsErrorReturn::DeriveIdentity)?
            .identity_element;
        let issue_uniform = draw(draws)?;
        let issued = deps
            .kem
            .issue(
                IssueParams,
                IssuePayload {
                    parameter_set: &setup.parameter_set,
                    master_scalar: &setup.master_scalar,
                    identity_element: &identity_element,
                    uniform: issue_uniform,
                },
            )
            .map_err(ProofVectorsErrorReturn::Issue)?;
        let x_uniform = draw(draws)?;
        let y_uniform = draw(draws)?;
        let generated = deps
            .key_agreement
            .generate_keys(
                GenerateKeysParams,
                GenerateKeysPayload {
                    x_uniform,
                    y_uniform,
                },
            )
            .map_err(ProofVectorsErrorReturn::GenerateKeys)?;
        let Ok(public_components) = deps.key_agreement.key_pair_public_keys(
            KeyPairPublicKeysParams,
            KeyPairPublicKeysPayload {
                key_pair: &generated.key_pair,
            },
        );
        let public_keys = deps
            .key_agreement
            .public_keys_from_components(
                PublicKeysFromComponentsParams,
                PublicKeysFromComponentsPayload {
                    components: PublicKeysComponents {
                        pk1: public_components.components.pk1.clone(),
                        pk2: public_components.components.pk2.clone(),
                    },
                    possession: &generated.possession,
                },
            )
            .map_err(ProofVectorsErrorReturn::PublicKeysFromComponents)?
            .public_keys;
        let Ok(credential) = deps.kem.credential_components(
            CredentialComponentsParams,
            CredentialComponentsPayload {
                credential: &issued.credential,
            },
        );
        let wrap = deps
            .key_agreement
            .wrap_to(
                WrapToParams,
                WrapToPayload {
                    public_keys: &public_keys,
                    credential: credential.components,
                },
            )
            .map_err(ProofVectorsErrorReturn::WrapTo)?;
        Ok((
            setup,
            identity_element,
            issued,
            generated,
            public_components.components,
            wrap,
        ))
    };

    let mut mint = Vec::new();
    for _ in 0..params.mint_count.get() {
        let (
            chain,
            entitlement_contract,
            source_entitlement,
            target_entitlement,
            old_interval,
            new_interval,
            seller,
            buyer,
        ) = rebuild_forms(DeliveryRelation::Mint, payload.mint)?;
        let context = DeliveryContext {
            suite_identifier: payload.mint.suite_identifier.clone(),
            chain,
            entitlement_contract,
            asset_identity_hash: payload.mint.asset_identity_hash.clone(),
            parameter_set_identifier: payload.mint.parameter_set_identifier.clone(),
            source_entitlement,
            target_entitlement,
            old_interval,
            new_interval,
            purpose: payload.mint_purpose,
            seller,
            buyer,
            expiry: payload.mint.expiry,
        };
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
        let record_context = ProofVectorContext {
            suite_identifier: *context.suite_identifier.identifier(),
            suite_version: context.suite_identifier.version(),
            chain: chain.field,
            entitlement_contract: entitlement_contract.field,
            asset_identity_hash: *context.asset_identity_hash.as_bytes(),
            parameter_set: *context.parameter_set_identifier.as_bytes(),
            source_entitlement: source_entitlement.field,
            target_entitlement: target_entitlement.field,
            old_interval: old_interval.field,
            new_interval: new_interval.field,
            purpose: context.purpose.code(),
            seller: seller.field,
            buyer: buyer.field,
            expiry: context.expiry,
        };
        let mut draws = Vec::new();
        let (setup, identity_element, issued, _generated, public_components, wrap) =
            issue_credential(&mut draws)?;
        let Ok(parameter_set) = deps.kem.parameter_set_components(
            ParameterSetComponentsParams,
            ParameterSetComponentsPayload {
                parameter_set: &setup.parameter_set,
            },
        );
        let Ok(element) = deps.kem.identity_element_components(
            IdentityElementComponentsParams,
            IdentityElementComponentsPayload {
                identity_element: &identity_element,
            },
        );
        let Ok(envelope) = deps.key_agreement.envelope_components(
            EnvelopeComponentsParams,
            EnvelopeComponentsPayload {
                envelope: &wrap.envelope,
            },
        );
        let hpub = encode_g2(parameter_set.components.hpub.clone());
        let identity_element_bytes = encode_g1(element.components.element.clone());
        let buyer_pk1 = encode_g1(public_components.pk1.clone());
        let buyer_pk2 = encode_g2(public_components.pk2.clone());
        let envelope_c1 = encode_g1(envelope.components.c1.clone());
        let envelope_c2 = encode_g1(envelope.components.c2.clone());
        let envelope_d1 = encode_g2(envelope.components.d1.clone());
        let envelope_d2 = encode_g2(envelope.components.d2.clone());
        let statement = AlgebraicMintStatement {
            context,
            hpub: parameter_set.components.hpub,
            identity_element: element.components.element,
            buyer_keys: public_components,
            envelope: envelope.components,
        };
        let Ok(master_scalar) = deps.kem.master_scalar_components(
            MasterScalarComponentsParams,
            MasterScalarComponentsPayload {
                master_scalar: &setup.master_scalar,
            },
        );
        let proved = deps
            .delivery_proof
            .prove_mint(
                ProveMintParams,
                ProveMintPayload {
                    master_scalar: master_scalar.components,
                    credential_randomness: issued.randomness,
                    coins: wrap.coins,
                    statement: &statement,
                },
            )
            .map_err(ProofVectorsErrorReturn::ProveMint)?;
        let Ok(proof) = deps.delivery_proof.proof_components(
            ProofComponentsParams,
            ProofComponentsPayload {
                proof: &proved.proof,
            },
        );
        let (challenge, alpha, r, rho, sigma, second_group_first_messages) = match proof.components
        {
            DeliveryProofComponents::MintBothGroups {
                challenge,
                responses,
            } => (
                challenge,
                responses.alpha,
                responses.r,
                responses.rho,
                responses.sigma,
                Vec::new(),
            ),
            DeliveryProofComponents::MintFirstGroupOnly {
                challenge,
                responses,
                first_messages,
            } => {
                let mut messages = encode_g2(first_messages.hpub);
                messages.extend_from_slice(&encode_g2(first_messages.d1));
                messages.extend_from_slice(&encode_g2(first_messages.d2));
                (
                    challenge,
                    responses.alpha,
                    responses.r,
                    responses.rho,
                    responses.sigma,
                    messages,
                )
            }
            DeliveryProofComponents::TransferBothGroups { .. }
            | DeliveryProofComponents::TransferFirstGroupOnly { .. } => {
                return Err(ProofVectorsErrorReturn::ProofComponents {
                    relation: DeliveryRelation::Mint,
                });
            }
        };
        mint.push(MintProofVector {
            draws,
            context: record_context,
            hpub,
            identity_element: identity_element_bytes,
            buyer_pk1,
            buyer_pk2,
            envelope_c1,
            envelope_c2,
            envelope_d1,
            envelope_d2,
            challenge: scalar_bytes(challenge)?,
            alpha: scalar_bytes(alpha)?,
            r: scalar_bytes(r)?,
            rho: scalar_bytes(rho)?,
            sigma: scalar_bytes(sigma)?,
            second_group_first_messages,
        });
    }

    let mut transfer = Vec::new();
    for _ in 0..params.transfer_count.get() {
        let (
            chain,
            entitlement_contract,
            source_entitlement,
            target_entitlement,
            old_interval,
            new_interval,
            seller,
            buyer,
        ) = rebuild_forms(DeliveryRelation::Transfer, payload.transfer)?;
        let context = DeliveryContext {
            suite_identifier: payload.transfer.suite_identifier.clone(),
            chain,
            entitlement_contract,
            asset_identity_hash: payload.transfer.asset_identity_hash.clone(),
            parameter_set_identifier: payload.transfer.parameter_set_identifier.clone(),
            source_entitlement,
            target_entitlement,
            old_interval,
            new_interval,
            purpose: payload.transfer_purpose,
            seller,
            buyer,
            expiry: payload.transfer.expiry,
        };
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
        let record_context = ProofVectorContext {
            suite_identifier: *context.suite_identifier.identifier(),
            suite_version: context.suite_identifier.version(),
            chain: chain.field,
            entitlement_contract: entitlement_contract.field,
            asset_identity_hash: *context.asset_identity_hash.as_bytes(),
            parameter_set: *context.parameter_set_identifier.as_bytes(),
            source_entitlement: source_entitlement.field,
            target_entitlement: target_entitlement.field,
            old_interval: old_interval.field,
            new_interval: new_interval.field,
            purpose: context.purpose.code(),
            seller: seller.field,
            buyer: buyer.field,
            expiry: context.expiry,
        };
        let mut draws = Vec::new();
        let (setup, identity_element, _, seller_generated, seller_components, old_wrap) =
            issue_credential(&mut draws)?;
        let buyer_x_uniform = draw(&mut draws)?;
        let buyer_y_uniform = draw(&mut draws)?;
        let buyer_generated = deps
            .key_agreement
            .generate_keys(
                GenerateKeysParams,
                GenerateKeysPayload {
                    x_uniform: buyer_x_uniform,
                    y_uniform: buyer_y_uniform,
                },
            )
            .map_err(ProofVectorsErrorReturn::GenerateKeys)?;
        let Ok(buyer_components) = deps.key_agreement.key_pair_public_keys(
            KeyPairPublicKeysParams,
            KeyPairPublicKeysPayload {
                key_pair: &buyer_generated.key_pair,
            },
        );
        let buyer_public = deps
            .key_agreement
            .public_keys_from_components(
                PublicKeysFromComponentsParams,
                PublicKeysFromComponentsPayload {
                    components: PublicKeysComponents {
                        pk1: buyer_components.components.pk1.clone(),
                        pk2: buyer_components.components.pk2.clone(),
                    },
                    possession: &buyer_generated.possession,
                },
            )
            .map_err(ProofVectorsErrorReturn::PublicKeysFromComponents)?
            .public_keys;
        let Ok(unwrapped) = deps.key_agreement.unwrap(
            UnwrapParams,
            UnwrapPayload {
                key_pair: &seller_generated.key_pair,
                envelope: &old_wrap.envelope,
            },
        );
        let Ok(seller_credential) = deps.kem.credential_from_components(
            CredentialFromComponentsParams,
            CredentialFromComponentsPayload {
                components: unwrapped.credential,
            },
        );
        let rerandomize_uniform = draw(&mut draws)?;
        let rerandomized = deps
            .kem
            .rerandomize(
                RerandomizeParams,
                RerandomizePayload {
                    parameter_set: &setup.parameter_set,
                    identity_element: &identity_element,
                    credential: &seller_credential.credential,
                    uniform: rerandomize_uniform,
                },
            )
            .map_err(ProofVectorsErrorReturn::Rerandomize)?;
        let Ok(new_credential) = deps.kem.credential_components(
            CredentialComponentsParams,
            CredentialComponentsPayload {
                credential: &rerandomized.credential,
            },
        );
        let new_wrap = deps
            .key_agreement
            .wrap_to(
                WrapToParams,
                WrapToPayload {
                    public_keys: &buyer_public,
                    credential: new_credential.components,
                },
            )
            .map_err(ProofVectorsErrorReturn::WrapTo)?;
        let Ok(element) = deps.kem.identity_element_components(
            IdentityElementComponentsParams,
            IdentityElementComponentsPayload {
                identity_element: &identity_element,
            },
        );
        let Ok(old_envelope) = deps.key_agreement.envelope_components(
            EnvelopeComponentsParams,
            EnvelopeComponentsPayload {
                envelope: &old_wrap.envelope,
            },
        );
        let Ok(new_envelope) = deps.key_agreement.envelope_components(
            EnvelopeComponentsParams,
            EnvelopeComponentsPayload {
                envelope: &new_wrap.envelope,
            },
        );
        let identity_element_bytes = encode_g1(element.components.element.clone());
        let seller_pk1 = encode_g1(seller_components.pk1.clone());
        let seller_pk2 = encode_g2(seller_components.pk2.clone());
        let buyer_pk1 = encode_g1(buyer_components.components.pk1.clone());
        let buyer_pk2 = encode_g2(buyer_components.components.pk2.clone());
        let old_envelope_c1 = encode_g1(old_envelope.components.c1.clone());
        let old_envelope_c2 = encode_g1(old_envelope.components.c2.clone());
        let old_envelope_d1 = encode_g2(old_envelope.components.d1.clone());
        let old_envelope_d2 = encode_g2(old_envelope.components.d2.clone());
        let new_envelope_c1 = encode_g1(new_envelope.components.c1.clone());
        let new_envelope_c2 = encode_g1(new_envelope.components.c2.clone());
        let new_envelope_d1 = encode_g2(new_envelope.components.d1.clone());
        let new_envelope_d2 = encode_g2(new_envelope.components.d2.clone());
        let statement = AlgebraicTransferStatement {
            context,
            identity_element: element.components.element,
            seller_keys: seller_components,
            buyer_keys: buyer_components.components,
            old_envelope: old_envelope.components,
            new_envelope: new_envelope.components,
        };
        let Ok(seller_secrets) = deps.key_agreement.key_pair_components(
            KeyPairComponentsParams,
            KeyPairComponentsPayload {
                key_pair: &seller_generated.key_pair,
            },
        );
        let proved = deps
            .delivery_proof
            .prove_transfer(
                ProveTransferParams,
                ProveTransferPayload {
                    seller_secrets: seller_secrets.components,
                    offset: rerandomized.offset,
                    coins: new_wrap.coins,
                    statement: &statement,
                },
            )
            .map_err(ProofVectorsErrorReturn::ProveTransfer)?;
        let Ok(proof) = deps.delivery_proof.proof_components(
            ProofComponentsParams,
            ProofComponentsPayload {
                proof: &proved.proof,
            },
        );
        let (challenge, x, y, s, rho, sigma, second_group_first_messages) = match proof.components {
            DeliveryProofComponents::TransferBothGroups {
                challenge,
                responses,
            } => (
                challenge,
                responses.x,
                responses.y,
                responses.s,
                responses.rho,
                responses.sigma,
                Vec::new(),
            ),
            DeliveryProofComponents::TransferFirstGroupOnly {
                challenge,
                responses,
                first_messages,
            } => {
                let mut messages = encode_g2(first_messages.pk2);
                messages.extend_from_slice(&encode_g2(first_messages.d1));
                messages.extend_from_slice(&encode_g2(first_messages.d2));
                (
                    challenge,
                    responses.x,
                    responses.y,
                    responses.s,
                    responses.rho,
                    responses.sigma,
                    messages,
                )
            }
            DeliveryProofComponents::MintBothGroups { .. }
            | DeliveryProofComponents::MintFirstGroupOnly { .. } => {
                return Err(ProofVectorsErrorReturn::ProofComponents {
                    relation: DeliveryRelation::Transfer,
                });
            }
        };
        transfer.push(TransferProofVector {
            draws,
            context: record_context,
            identity_element: identity_element_bytes,
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
            challenge: scalar_bytes(challenge)?,
            x: scalar_bytes(x)?,
            y: scalar_bytes(y)?,
            s: scalar_bytes(s)?,
            rho: scalar_bytes(rho)?,
            sigma: scalar_bytes(sigma)?,
            second_group_first_messages,
        });
    }

    Ok(ProofVectorsSuccessReturn { mint, transfer })
}
