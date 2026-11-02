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
    CanonicalFieldKind, CanonicalFieldValue, CanonicalFields, FromFieldsParams, FromFieldsReturn,
    FromFieldsSuccessReturn, ICanonicalField, IEncodingContract, ToFieldParams, ToFieldsParams,
    ToFieldsReturn, ToFieldsSuccessReturn,
};
use envelope::IKeyAgreementAdapter;
use kem::ICredentialKemAdapter;
use pairing::{
    EncodeG1Params, EncodeG1Payload, EncodeG2Params, EncodeG2Payload, EncodeScalarParams,
    EncodeScalarPayload, IPairingAdapter,
};
use proof::{
    DeliveryContext, DeliveryProofComponents, DeliveryRelation, IDeliveryProofAdapter,
    ProofComponentsParams, ProofComponentsPayload,
};

use super::render::provides::{SolidityMemberName, SolidityMemberNameConstructorParams};
use crate::delivery::provides::{
    MintDeps, MintParams, MintPayload, TransferDeps, TransferParams, TransferPayload, mint,
    transfer,
};
use interface::{
    MintProofVector, MintProofVectorDescription, MintProofVectorDescriptionConstructorParams,
    MintProofVectorDescriptionTryNewReturn, MintProofVectorFromFieldsErrorReturn,
    ProofVectorContext, ProofVectorMemberNamesErrorReturn, ProofVectorMemberNamesReturn,
    ProofVectorsDeps, ProofVectorsErrorReturn, ProofVectorsParams, ProofVectorsPayload,
    ProofVectorsReturn, ProofVectorsSuccessReturn, TransferProofVector,
    TransferProofVectorDescription, TransferProofVectorDescriptionConstructorParams,
    TransferProofVectorDescriptionTryNewReturn, TransferProofVectorFromFieldsErrorReturn,
};

impl<F: IChainForms> MintProofVectorDescription<F> {
    pub fn try_new(
        _params: MintProofVectorDescriptionConstructorParams,
    ) -> MintProofVectorDescriptionTryNewReturn<F> {
        Ok(MintProofVectorDescription { forms: PhantomData })
    }

    pub fn member_names(&self) -> ProofVectorMemberNamesReturn {
        let mut names = Vec::with_capacity(29);
        for text in [
            "draws",
            "suiteIdentifier",
            "suiteVersion",
            "chain",
            "entitlementContract",
            "assetRoot",
            "parameterSet",
            "sourceEntitlement",
            "targetEntitlement",
            "oldInterval",
            "newInterval",
            "purpose",
            "seller",
            "buyer",
            "expiry",
            "hpub",
            "identityElement",
            "buyerPk1",
            "buyerPk2",
            "envelopeC1",
            "envelopeC2",
            "envelopeD1",
            "envelopeD2",
            "challenge",
            "alpha",
            "r",
            "rho",
            "sigma",
            "secondGroupFirstMessages",
        ] {
            names.push(
                SolidityMemberName::try_new(SolidityMemberNameConstructorParams {
                    text: text.to_string(),
                })
                .map_err(ProofVectorMemberNamesErrorReturn::MemberName)?,
            );
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
        let fields: [CanonicalFieldValue; 29] = match payload.values.try_into() {
            Ok(fields) => fields,
            Err(values) => {
                return Err(MintProofVectorFromFieldsErrorReturn::FieldCount {
                    expected: 29,
                    actual: values.len(),
                });
            }
        };
        for (index, (value, kind)) in fields.iter().zip(Self::FIELDS.iter()).enumerate() {
            if canonical_field_kind(value) != *kind {
                return Err(MintProofVectorFromFieldsErrorReturn::FieldKind {
                    index,
                    expected: *kind,
                });
            }
        }
        let [
            draws,
            suite_identifier,
            suite_version,
            chain,
            entitlement_contract,
            asset_root,
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
                expected: CanonicalFieldKind::Bytes,
            });
        };
        let CanonicalFieldValue::FixedBytes32(suite_identifier) = suite_identifier else {
            return Err(MintProofVectorFromFieldsErrorReturn::FieldKind {
                index: 1,
                expected: CanonicalFieldKind::FixedBytes32,
            });
        };
        let CanonicalFieldValue::Unsigned16(suite_version) = suite_version else {
            return Err(MintProofVectorFromFieldsErrorReturn::FieldKind {
                index: 2,
                expected: CanonicalFieldKind::Unsigned16,
            });
        };
        let CanonicalFieldValue::FixedBytes32(asset_root) = asset_root else {
            return Err(MintProofVectorFromFieldsErrorReturn::FieldKind {
                index: 5,
                expected: CanonicalFieldKind::FixedBytes32,
            });
        };
        let CanonicalFieldValue::FixedBytes32(parameter_set) = parameter_set else {
            return Err(MintProofVectorFromFieldsErrorReturn::FieldKind {
                index: 6,
                expected: CanonicalFieldKind::FixedBytes32,
            });
        };
        let CanonicalFieldValue::Unsigned16(purpose) = purpose else {
            return Err(MintProofVectorFromFieldsErrorReturn::FieldKind {
                index: 11,
                expected: CanonicalFieldKind::Unsigned16,
            });
        };
        let CanonicalFieldValue::Unsigned64(expiry) = expiry else {
            return Err(MintProofVectorFromFieldsErrorReturn::FieldKind {
                index: 14,
                expected: CanonicalFieldKind::Unsigned64,
            });
        };
        let CanonicalFieldValue::Bytes(hpub) = hpub else {
            return Err(MintProofVectorFromFieldsErrorReturn::FieldKind {
                index: 15,
                expected: CanonicalFieldKind::Bytes,
            });
        };
        let CanonicalFieldValue::Bytes(identity_element) = identity_element else {
            return Err(MintProofVectorFromFieldsErrorReturn::FieldKind {
                index: 16,
                expected: CanonicalFieldKind::Bytes,
            });
        };
        let CanonicalFieldValue::Bytes(buyer_pk1) = buyer_pk1 else {
            return Err(MintProofVectorFromFieldsErrorReturn::FieldKind {
                index: 17,
                expected: CanonicalFieldKind::Bytes,
            });
        };
        let CanonicalFieldValue::Bytes(buyer_pk2) = buyer_pk2 else {
            return Err(MintProofVectorFromFieldsErrorReturn::FieldKind {
                index: 18,
                expected: CanonicalFieldKind::Bytes,
            });
        };
        let CanonicalFieldValue::Bytes(envelope_c1) = envelope_c1 else {
            return Err(MintProofVectorFromFieldsErrorReturn::FieldKind {
                index: 19,
                expected: CanonicalFieldKind::Bytes,
            });
        };
        let CanonicalFieldValue::Bytes(envelope_c2) = envelope_c2 else {
            return Err(MintProofVectorFromFieldsErrorReturn::FieldKind {
                index: 20,
                expected: CanonicalFieldKind::Bytes,
            });
        };
        let CanonicalFieldValue::Bytes(envelope_d1) = envelope_d1 else {
            return Err(MintProofVectorFromFieldsErrorReturn::FieldKind {
                index: 21,
                expected: CanonicalFieldKind::Bytes,
            });
        };
        let CanonicalFieldValue::Bytes(envelope_d2) = envelope_d2 else {
            return Err(MintProofVectorFromFieldsErrorReturn::FieldKind {
                index: 22,
                expected: CanonicalFieldKind::Bytes,
            });
        };
        let CanonicalFieldValue::Unsigned256(challenge) = challenge else {
            return Err(MintProofVectorFromFieldsErrorReturn::FieldKind {
                index: 23,
                expected: CanonicalFieldKind::Unsigned256,
            });
        };
        let CanonicalFieldValue::Unsigned256(alpha) = alpha else {
            return Err(MintProofVectorFromFieldsErrorReturn::FieldKind {
                index: 24,
                expected: CanonicalFieldKind::Unsigned256,
            });
        };
        let CanonicalFieldValue::Unsigned256(r) = r else {
            return Err(MintProofVectorFromFieldsErrorReturn::FieldKind {
                index: 25,
                expected: CanonicalFieldKind::Unsigned256,
            });
        };
        let CanonicalFieldValue::Unsigned256(rho) = rho else {
            return Err(MintProofVectorFromFieldsErrorReturn::FieldKind {
                index: 26,
                expected: CanonicalFieldKind::Unsigned256,
            });
        };
        let CanonicalFieldValue::Unsigned256(sigma) = sigma else {
            return Err(MintProofVectorFromFieldsErrorReturn::FieldKind {
                index: 27,
                expected: CanonicalFieldKind::Unsigned256,
            });
        };
        let CanonicalFieldValue::Bytes(second_group_first_messages) = second_group_first_messages
        else {
            return Err(MintProofVectorFromFieldsErrorReturn::FieldKind {
                index: 28,
                expected: CanonicalFieldKind::Bytes,
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
                    asset_root,
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

    pub fn member_names(&self) -> ProofVectorMemberNamesReturn {
        let mut names = Vec::with_capacity(35);
        for text in [
            "draws",
            "suiteIdentifier",
            "suiteVersion",
            "chain",
            "entitlementContract",
            "assetRoot",
            "parameterSet",
            "sourceEntitlement",
            "targetEntitlement",
            "oldInterval",
            "newInterval",
            "purpose",
            "seller",
            "buyer",
            "expiry",
            "identityElement",
            "sellerPk1",
            "sellerPk2",
            "buyerPk1",
            "buyerPk2",
            "oldEnvelopeC1",
            "oldEnvelopeC2",
            "oldEnvelopeD1",
            "oldEnvelopeD2",
            "newEnvelopeC1",
            "newEnvelopeC2",
            "newEnvelopeD1",
            "newEnvelopeD2",
            "challenge",
            "x",
            "y",
            "s",
            "rho",
            "sigma",
            "secondGroupFirstMessages",
        ] {
            names.push(
                SolidityMemberName::try_new(SolidityMemberNameConstructorParams {
                    text: text.to_string(),
                })
                .map_err(ProofVectorMemberNamesErrorReturn::MemberName)?,
            );
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
        let fields: [CanonicalFieldValue; 35] = match payload.values.try_into() {
            Ok(fields) => fields,
            Err(values) => {
                return Err(TransferProofVectorFromFieldsErrorReturn::FieldCount {
                    expected: 35,
                    actual: values.len(),
                });
            }
        };
        for (index, (value, kind)) in fields.iter().zip(Self::FIELDS.iter()).enumerate() {
            if canonical_field_kind(value) != *kind {
                return Err(TransferProofVectorFromFieldsErrorReturn::FieldKind {
                    index,
                    expected: *kind,
                });
            }
        }
        let [
            draws,
            suite_identifier,
            suite_version,
            chain,
            entitlement_contract,
            asset_root,
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
                expected: CanonicalFieldKind::Bytes,
            });
        };
        let CanonicalFieldValue::FixedBytes32(suite_identifier) = suite_identifier else {
            return Err(TransferProofVectorFromFieldsErrorReturn::FieldKind {
                index: 1,
                expected: CanonicalFieldKind::FixedBytes32,
            });
        };
        let CanonicalFieldValue::Unsigned16(suite_version) = suite_version else {
            return Err(TransferProofVectorFromFieldsErrorReturn::FieldKind {
                index: 2,
                expected: CanonicalFieldKind::Unsigned16,
            });
        };
        let CanonicalFieldValue::FixedBytes32(asset_root) = asset_root else {
            return Err(TransferProofVectorFromFieldsErrorReturn::FieldKind {
                index: 5,
                expected: CanonicalFieldKind::FixedBytes32,
            });
        };
        let CanonicalFieldValue::FixedBytes32(parameter_set) = parameter_set else {
            return Err(TransferProofVectorFromFieldsErrorReturn::FieldKind {
                index: 6,
                expected: CanonicalFieldKind::FixedBytes32,
            });
        };
        let CanonicalFieldValue::Unsigned16(purpose) = purpose else {
            return Err(TransferProofVectorFromFieldsErrorReturn::FieldKind {
                index: 11,
                expected: CanonicalFieldKind::Unsigned16,
            });
        };
        let CanonicalFieldValue::Unsigned64(expiry) = expiry else {
            return Err(TransferProofVectorFromFieldsErrorReturn::FieldKind {
                index: 14,
                expected: CanonicalFieldKind::Unsigned64,
            });
        };
        let CanonicalFieldValue::Bytes(identity_element) = identity_element else {
            return Err(TransferProofVectorFromFieldsErrorReturn::FieldKind {
                index: 15,
                expected: CanonicalFieldKind::Bytes,
            });
        };
        let CanonicalFieldValue::Bytes(seller_pk1) = seller_pk1 else {
            return Err(TransferProofVectorFromFieldsErrorReturn::FieldKind {
                index: 16,
                expected: CanonicalFieldKind::Bytes,
            });
        };
        let CanonicalFieldValue::Bytes(seller_pk2) = seller_pk2 else {
            return Err(TransferProofVectorFromFieldsErrorReturn::FieldKind {
                index: 17,
                expected: CanonicalFieldKind::Bytes,
            });
        };
        let CanonicalFieldValue::Bytes(buyer_pk1) = buyer_pk1 else {
            return Err(TransferProofVectorFromFieldsErrorReturn::FieldKind {
                index: 18,
                expected: CanonicalFieldKind::Bytes,
            });
        };
        let CanonicalFieldValue::Bytes(buyer_pk2) = buyer_pk2 else {
            return Err(TransferProofVectorFromFieldsErrorReturn::FieldKind {
                index: 19,
                expected: CanonicalFieldKind::Bytes,
            });
        };
        let CanonicalFieldValue::Bytes(old_envelope_c1) = old_envelope_c1 else {
            return Err(TransferProofVectorFromFieldsErrorReturn::FieldKind {
                index: 20,
                expected: CanonicalFieldKind::Bytes,
            });
        };
        let CanonicalFieldValue::Bytes(old_envelope_c2) = old_envelope_c2 else {
            return Err(TransferProofVectorFromFieldsErrorReturn::FieldKind {
                index: 21,
                expected: CanonicalFieldKind::Bytes,
            });
        };
        let CanonicalFieldValue::Bytes(old_envelope_d1) = old_envelope_d1 else {
            return Err(TransferProofVectorFromFieldsErrorReturn::FieldKind {
                index: 22,
                expected: CanonicalFieldKind::Bytes,
            });
        };
        let CanonicalFieldValue::Bytes(old_envelope_d2) = old_envelope_d2 else {
            return Err(TransferProofVectorFromFieldsErrorReturn::FieldKind {
                index: 23,
                expected: CanonicalFieldKind::Bytes,
            });
        };
        let CanonicalFieldValue::Bytes(new_envelope_c1) = new_envelope_c1 else {
            return Err(TransferProofVectorFromFieldsErrorReturn::FieldKind {
                index: 24,
                expected: CanonicalFieldKind::Bytes,
            });
        };
        let CanonicalFieldValue::Bytes(new_envelope_c2) = new_envelope_c2 else {
            return Err(TransferProofVectorFromFieldsErrorReturn::FieldKind {
                index: 25,
                expected: CanonicalFieldKind::Bytes,
            });
        };
        let CanonicalFieldValue::Bytes(new_envelope_d1) = new_envelope_d1 else {
            return Err(TransferProofVectorFromFieldsErrorReturn::FieldKind {
                index: 26,
                expected: CanonicalFieldKind::Bytes,
            });
        };
        let CanonicalFieldValue::Bytes(new_envelope_d2) = new_envelope_d2 else {
            return Err(TransferProofVectorFromFieldsErrorReturn::FieldKind {
                index: 27,
                expected: CanonicalFieldKind::Bytes,
            });
        };
        let CanonicalFieldValue::Unsigned256(challenge) = challenge else {
            return Err(TransferProofVectorFromFieldsErrorReturn::FieldKind {
                index: 28,
                expected: CanonicalFieldKind::Unsigned256,
            });
        };
        let CanonicalFieldValue::Unsigned256(x) = x else {
            return Err(TransferProofVectorFromFieldsErrorReturn::FieldKind {
                index: 29,
                expected: CanonicalFieldKind::Unsigned256,
            });
        };
        let CanonicalFieldValue::Unsigned256(y) = y else {
            return Err(TransferProofVectorFromFieldsErrorReturn::FieldKind {
                index: 30,
                expected: CanonicalFieldKind::Unsigned256,
            });
        };
        let CanonicalFieldValue::Unsigned256(s) = s else {
            return Err(TransferProofVectorFromFieldsErrorReturn::FieldKind {
                index: 31,
                expected: CanonicalFieldKind::Unsigned256,
            });
        };
        let CanonicalFieldValue::Unsigned256(rho) = rho else {
            return Err(TransferProofVectorFromFieldsErrorReturn::FieldKind {
                index: 32,
                expected: CanonicalFieldKind::Unsigned256,
            });
        };
        let CanonicalFieldValue::Unsigned256(sigma) = sigma else {
            return Err(TransferProofVectorFromFieldsErrorReturn::FieldKind {
                index: 33,
                expected: CanonicalFieldKind::Unsigned256,
            });
        };
        let CanonicalFieldValue::Bytes(second_group_first_messages) = second_group_first_messages
        else {
            return Err(TransferProofVectorFromFieldsErrorReturn::FieldKind {
                index: 34,
                expected: CanonicalFieldKind::Bytes,
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
                    asset_root,
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

fn canonical_field_kind(value: &CanonicalFieldValue) -> CanonicalFieldKind {
    match value {
        CanonicalFieldValue::FixedBytes32(_) => CanonicalFieldKind::FixedBytes32,
        CanonicalFieldValue::Unsigned16(_) => CanonicalFieldKind::Unsigned16,
        CanonicalFieldValue::Unsigned32(_) => CanonicalFieldKind::Unsigned32,
        CanonicalFieldValue::Unsigned64(_) => CanonicalFieldKind::Unsigned64,
        CanonicalFieldValue::Text(_) => CanonicalFieldKind::Text,
        CanonicalFieldValue::Bytes(_) => CanonicalFieldKind::Bytes,
        CanonicalFieldValue::FixedBytes20(_) => CanonicalFieldKind::FixedBytes20,
        CanonicalFieldValue::Unsigned256(_) => CanonicalFieldKind::Unsigned256,
    }
}

fn canonical_field<T: ICanonicalField>(form: &T) -> CanonicalFieldValue {
    let Ok(field) = T::to_field(ToFieldParams, form);
    field.field
}

fn record_context<F: IChainForms, Purpose>(
    context: &DeliveryContext<F, Purpose>,
    purpose: u16,
) -> ProofVectorContext {
    ProofVectorContext {
        suite_identifier: *context.suite_identifier.identifier(),
        suite_version: context.suite_identifier.version(),
        chain: canonical_field(&context.chain),
        entitlement_contract: canonical_field(&context.entitlement_contract),
        asset_root: *context.asset_identity_hash.as_bytes(),
        parameter_set: *context.parameter_set_identifier.as_bytes(),
        source_entitlement: canonical_field(&context.source_entitlement),
        target_entitlement: canonical_field(&context.target_entitlement),
        old_interval: canonical_field(&context.old_interval),
        new_interval: canonical_field(&context.new_interval),
        purpose,
        seller: canonical_field(&context.seller),
        buyer: canonical_field(&context.buyer),
        expiry: context.expiry,
    }
}

fn g1_bytes<P: IPairingAdapter>(pairing: &P, point: &P::G1) -> Vec<u8> {
    let Ok(encoded) = pairing.encode_g1(
        EncodeG1Params,
        EncodeG1Payload {
            point: point.clone(),
        },
    );
    encoded.bytes.as_ref().to_vec()
}

fn g2_bytes<P: IPairingAdapter>(pairing: &P, point: &P::G2) -> Vec<u8> {
    let Ok(encoded) = pairing.encode_g2(
        EncodeG2Params,
        EncodeG2Payload {
            point: point.clone(),
        },
    );
    encoded.bytes.as_ref().to_vec()
}

fn scalar_bytes<P: IPairingAdapter>(
    pairing: &P,
    scalar: &P::Scalar,
) -> Result<[u8; 32], ProofVectorsErrorReturn> {
    let Ok(encoded) = pairing.encode_scalar(
        EncodeScalarParams,
        EncodeScalarPayload {
            scalar: scalar.clone(),
        },
    );
    let bytes: &[u8] = encoded.bytes.expose().as_ref();
    let Ok(bytes) = <[u8; 32]>::try_from(bytes) else {
        return Err(ProofVectorsErrorReturn::ScalarLength {
            actual: bytes.len(),
        });
    };
    Ok(bytes)
}

pub fn proof_vectors<'a, P, K, A, D>(
    deps: &ProofVectorsDeps<'a, P, K, A, D>,
    params: ProofVectorsParams,
    payload: ProofVectorsPayload<'_, D::Forms>,
) -> ProofVectorsReturn
where
    P: IPairingAdapter,
    K: ICredentialKemAdapter<Pairing = P>,
    A: IKeyAgreementAdapter<Pairing = P>,
    D: IDeliveryProofAdapter<Pairing = P>,
{
    let mut mint_records = Vec::new();
    for _ in 0..params.mint_count.get() {
        let minted = match mint(
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
        ) {
            Ok(minted) => minted,
            Err(error) => return Err(ProofVectorsErrorReturn::Mint(error)),
        };
        let Ok(components) = deps.delivery_proof.proof_components(
            ProofComponentsParams,
            ProofComponentsPayload {
                proof: &minted.delivery.proof,
            },
        );
        let (challenge, responses, first_messages) = match components.components {
            DeliveryProofComponents::MintBothGroups {
                challenge,
                responses,
            } => (challenge, responses, None),
            DeliveryProofComponents::MintFirstGroupOnly {
                challenge,
                responses,
                first_messages,
            } => (challenge, responses, Some(first_messages)),
            _ => {
                return Err(ProofVectorsErrorReturn::ProofComponents {
                    relation: DeliveryRelation::Mint,
                });
            }
        };
        let statement = &minted.delivery.statement;
        mint_records.push(MintProofVector {
            draws: minted.delivery.draws.clone(),
            context: record_context(&statement.context, statement.context.purpose.code()),
            hpub: g2_bytes(deps.pairing, &statement.hpub),
            identity_element: g1_bytes(deps.pairing, &statement.identity_element),
            buyer_pk1: g1_bytes(deps.pairing, &statement.buyer_keys.pk1),
            buyer_pk2: g2_bytes(deps.pairing, &statement.buyer_keys.pk2),
            envelope_c1: g1_bytes(deps.pairing, &statement.envelope.c1),
            envelope_c2: g1_bytes(deps.pairing, &statement.envelope.c2),
            envelope_d1: g2_bytes(deps.pairing, &statement.envelope.d1),
            envelope_d2: g2_bytes(deps.pairing, &statement.envelope.d2),
            challenge: scalar_bytes(deps.pairing, &challenge)?,
            alpha: scalar_bytes(deps.pairing, &responses.alpha)?,
            r: scalar_bytes(deps.pairing, &responses.r)?,
            rho: scalar_bytes(deps.pairing, &responses.rho)?,
            sigma: scalar_bytes(deps.pairing, &responses.sigma)?,
            second_group_first_messages: match first_messages {
                Some(first_messages) => [
                    g2_bytes(deps.pairing, &first_messages.hpub),
                    g2_bytes(deps.pairing, &first_messages.d1),
                    g2_bytes(deps.pairing, &first_messages.d2),
                ]
                .concat(),
                None => Vec::new(),
            },
        });
    }
    let mut transfer_records = Vec::new();
    for _ in 0..params.transfer_count.get() {
        let minted = match mint(
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
        ) {
            Ok(minted) => minted,
            Err(error) => return Err(ProofVectorsErrorReturn::Mint(error)),
        };
        let transferred = match transfer(
            &TransferDeps {
                kem: deps.kem,
                key_agreement: deps.key_agreement,
                delivery_proof: deps.delivery_proof,
                random: deps.random,
            },
            TransferParams,
            TransferPayload {
                context: payload.transfer,
                parameter_set: &minted.delivery.parameter_set,
                identity_element: &minted.delivery.identity_element,
                holder: &minted.delivery.recipient,
            },
        ) {
            Ok(transferred) => transferred,
            Err(error) => return Err(ProofVectorsErrorReturn::Transfer(error)),
        };
        let Ok(components) = deps.delivery_proof.proof_components(
            ProofComponentsParams,
            ProofComponentsPayload {
                proof: &transferred.delivery.proof,
            },
        );
        let (challenge, responses, first_messages) = match components.components {
            DeliveryProofComponents::TransferBothGroups {
                challenge,
                responses,
            } => (challenge, responses, None),
            DeliveryProofComponents::TransferFirstGroupOnly {
                challenge,
                responses,
                first_messages,
            } => (challenge, responses, Some(first_messages)),
            _ => {
                return Err(ProofVectorsErrorReturn::ProofComponents {
                    relation: DeliveryRelation::Transfer,
                });
            }
        };
        let mut draws = minted.delivery.draws.clone();
        draws.extend_from_slice(&transferred.delivery.draws);
        let statement = &transferred.delivery.statement;
        transfer_records.push(TransferProofVector {
            draws,
            context: record_context(&statement.context, statement.context.purpose.code()),
            identity_element: g1_bytes(deps.pairing, &statement.identity_element),
            seller_pk1: g1_bytes(deps.pairing, &statement.seller_keys.pk1),
            seller_pk2: g2_bytes(deps.pairing, &statement.seller_keys.pk2),
            buyer_pk1: g1_bytes(deps.pairing, &statement.buyer_keys.pk1),
            buyer_pk2: g2_bytes(deps.pairing, &statement.buyer_keys.pk2),
            old_envelope_c1: g1_bytes(deps.pairing, &statement.old_envelope.c1),
            old_envelope_c2: g1_bytes(deps.pairing, &statement.old_envelope.c2),
            old_envelope_d1: g2_bytes(deps.pairing, &statement.old_envelope.d1),
            old_envelope_d2: g2_bytes(deps.pairing, &statement.old_envelope.d2),
            new_envelope_c1: g1_bytes(deps.pairing, &statement.new_envelope.c1),
            new_envelope_c2: g1_bytes(deps.pairing, &statement.new_envelope.c2),
            new_envelope_d1: g2_bytes(deps.pairing, &statement.new_envelope.d1),
            new_envelope_d2: g2_bytes(deps.pairing, &statement.new_envelope.d2),
            challenge: scalar_bytes(deps.pairing, &challenge)?,
            x: scalar_bytes(deps.pairing, &responses.x)?,
            y: scalar_bytes(deps.pairing, &responses.y)?,
            s: scalar_bytes(deps.pairing, &responses.s)?,
            rho: scalar_bytes(deps.pairing, &responses.rho)?,
            sigma: scalar_bytes(deps.pairing, &responses.sigma)?,
            second_group_first_messages: match first_messages {
                Some(first_messages) => [
                    g2_bytes(deps.pairing, &first_messages.pk2),
                    g2_bytes(deps.pairing, &first_messages.d1),
                    g2_bytes(deps.pairing, &first_messages.d2),
                ]
                .concat(),
                None => Vec::new(),
            },
        });
    }
    Ok(ProofVectorsSuccessReturn {
        mint: mint_records,
        transfer: transfer_records,
    })
}
