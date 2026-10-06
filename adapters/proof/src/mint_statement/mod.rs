mod interface;
#[cfg(any(test, feature = "mocks"))]
mod mock;
pub mod provides;
#[cfg(test)]
mod test;

use chain::IChainForms;
use core::marker::PhantomData;
use domain::{
    AssetIdentityHash, AssetIdentityHashConstructorParams, ParameterSetIdentifier,
    ParameterSetIdentifierConstructorParams, SuiteIdentifier, SuiteIdentifierConstructorParams,
};
use encoding::{
    CanonicalFieldKind, CanonicalFieldValue, CanonicalFields, FromFieldParams, FromFieldsParams,
    FromFieldsReturn, FromFieldsSuccessReturn, ICanonicalField, IEncodingContract, ToFieldParams,
    ToFieldsParams, ToFieldsReturn, ToFieldsSuccessReturn,
};
use interface::{
    MINT_STATEMENT_FIELD_COUNT, MintFirstMessages, MintPurpose, MintStatement,
    MintStatementDescription, MintStatementDescriptionConstructorParams,
    MintStatementDescriptionTryNewReturn, MintStatementFromFieldsErrorReturn,
};
use pairing::{
    DecodeG1Params, DecodeG2Params, EncodeG1Params, EncodeG1Payload, EncodeG2Params,
    EncodeG2Payload, IPairingAdapter,
};

impl<'a, P: IPairingAdapter, F: IChainForms> MintStatementDescription<'a, P, F> {
    pub fn try_new(
        params: MintStatementDescriptionConstructorParams<'a, P>,
    ) -> MintStatementDescriptionTryNewReturn<'a, P, F> {
        Ok(MintStatementDescription {
            pairing: params.pairing,
            forms: PhantomData,
        })
    }
}

impl<'a, P: IPairingAdapter, F: IChainForms> MintStatementDescription<'a, P, F> {
    fn encode_g1(&self, point: P::G1) -> CanonicalFieldValue {
        let Ok(encoded) = self
            .pairing
            .encode_g1(EncodeG1Params, EncodeG1Payload { point });
        CanonicalFieldValue::Bytes(encoded.bytes.as_ref().to_vec())
    }

    fn encode_g2(&self, point: P::G2) -> CanonicalFieldValue {
        let Ok(encoded) = self
            .pairing
            .encode_g2(EncodeG2Params, EncodeG2Payload { point });
        CanonicalFieldValue::Bytes(encoded.bytes.as_ref().to_vec())
    }

    fn decode_g1(
        &self,
        index: usize,
        value: CanonicalFieldValue,
    ) -> Result<P::G1, MintStatementFromFieldsErrorReturn<F>> {
        let CanonicalFieldValue::Bytes(bytes) = value else {
            return Err(MintStatementFromFieldsErrorReturn::FieldKind {
                index,
                expected: CanonicalFieldKind::Bytes,
            });
        };
        match self.pairing.decode_g1(DecodeG1Params, &bytes) {
            Ok(success) => Ok(success.point),
            Err(error) => Err(MintStatementFromFieldsErrorReturn::InvalidG1 { index, error }),
        }
    }

    fn decode_g2(
        &self,
        index: usize,
        value: CanonicalFieldValue,
    ) -> Result<P::G2, MintStatementFromFieldsErrorReturn<F>> {
        let CanonicalFieldValue::Bytes(bytes) = value else {
            return Err(MintStatementFromFieldsErrorReturn::FieldKind {
                index,
                expected: CanonicalFieldKind::Bytes,
            });
        };
        match self.pairing.decode_g2(DecodeG2Params, &bytes) {
            Ok(success) => Ok(success.point),
            Err(error) => Err(MintStatementFromFieldsErrorReturn::InvalidG2 { index, error }),
        }
    }
}

impl<P: IPairingAdapter, F: IChainForms> IEncodingContract for MintStatementDescription<'_, P, F>
where
    MintStatementFromFieldsErrorReturn<F>: core::fmt::Debug + PartialEq + Eq,
{
    type Described = MintStatement<P, F>;
    type FromFieldsErrorReturn = MintStatementFromFieldsErrorReturn<F>;
    const FIELDS: &'static [CanonicalFieldKind] = &[
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
        CanonicalFieldKind::Bytes,
        CanonicalFieldKind::Bytes,
        CanonicalFieldKind::Bytes,
        CanonicalFieldKind::Bytes,
        CanonicalFieldKind::Bytes,
        CanonicalFieldKind::Bytes,
        CanonicalFieldKind::Unsigned64,
        CanonicalFieldKind::Bytes,
        CanonicalFieldKind::Bytes,
        CanonicalFieldKind::Bytes,
        CanonicalFieldKind::Bytes,
        CanonicalFieldKind::Bytes,
    ];

    fn to_fields(&self, _params: ToFieldsParams, payload: &Self::Described) -> ToFieldsReturn {
        let Ok(chain) = F::ChainIdentifier::to_field(ToFieldParams, &payload.chain);
        let Ok(entitlement_contract) =
            F::Identity::to_field(ToFieldParams, &payload.entitlement_contract);
        let Ok(source_entitlement) =
            F::Entitlement::to_field(ToFieldParams, &payload.source_entitlement);
        let Ok(target_entitlement) =
            F::Entitlement::to_field(ToFieldParams, &payload.target_entitlement);
        let Ok(old_interval) = F::Interval::to_field(ToFieldParams, &payload.old_interval);
        let Ok(new_interval) = F::Interval::to_field(ToFieldParams, &payload.new_interval);
        let Ok(seller) = F::Identity::to_field(ToFieldParams, &payload.seller);
        let Ok(buyer) = F::Identity::to_field(ToFieldParams, &payload.buyer);
        Ok(ToFieldsSuccessReturn {
            fields: CanonicalFields {
                values: vec![
                    CanonicalFieldValue::FixedBytes32(*payload.suite_identifier.identifier()),
                    CanonicalFieldValue::Unsigned16(payload.suite_identifier.version()),
                    chain.field,
                    entitlement_contract.field,
                    CanonicalFieldValue::FixedBytes32(*payload.asset_identity_hash.as_bytes()),
                    CanonicalFieldValue::FixedBytes32(*payload.parameter_set_identifier.as_bytes()),
                    source_entitlement.field,
                    target_entitlement.field,
                    old_interval.field,
                    new_interval.field,
                    CanonicalFieldValue::Unsigned16(payload.purpose.code()),
                    seller.field,
                    buyer.field,
                    self.encode_g1(payload.buyer_keys.pk1.clone()),
                    self.encode_g2(payload.buyer_keys.pk2.clone()),
                    self.encode_g1(payload.envelope.c1.clone()),
                    self.encode_g1(payload.envelope.c2.clone()),
                    self.encode_g2(payload.envelope.d1.clone()),
                    self.encode_g2(payload.envelope.d2.clone()),
                    CanonicalFieldValue::Unsigned64(payload.expiry),
                    self.encode_g2(payload.first_messages.hpub.clone()),
                    self.encode_g1(payload.first_messages.c1.clone()),
                    self.encode_g1(payload.first_messages.c2.clone()),
                    self.encode_g2(payload.first_messages.d1.clone()),
                    self.encode_g2(payload.first_messages.d2.clone()),
                ],
            },
        })
    }

    fn fields_to_value(
        &self,
        _params: FromFieldsParams,
        payload: CanonicalFields,
    ) -> FromFieldsReturn<Self::Described, Self::FromFieldsErrorReturn> {
        let fields: [CanonicalFieldValue; MINT_STATEMENT_FIELD_COUNT] =
            match payload.values.try_into() {
                Ok(fields) => fields,
                Err(values) => {
                    return Err(MintStatementFromFieldsErrorReturn::FieldCount {
                        expected: MINT_STATEMENT_FIELD_COUNT,
                        actual: values.len(),
                    });
                }
            };
        let [
            suite_identifier_value,
            suite_version_value,
            chain_value,
            entitlement_contract_value,
            asset_identity_hash_value,
            parameter_set_identifier_value,
            source_entitlement_value,
            target_entitlement_value,
            old_interval_value,
            new_interval_value,
            purpose_value,
            seller_value,
            buyer_value,
            pk1_value,
            pk2_value,
            envelope_c1_value,
            envelope_c2_value,
            envelope_d1_value,
            envelope_d2_value,
            expiry_value,
            hpub_value,
            message_c1_value,
            message_c2_value,
            message_d1_value,
            message_d2_value,
        ] = fields;
        let CanonicalFieldValue::FixedBytes32(identifier) = suite_identifier_value else {
            return Err(MintStatementFromFieldsErrorReturn::FieldKind {
                index: 0,
                expected: CanonicalFieldKind::FixedBytes32,
            });
        };
        let CanonicalFieldValue::Unsigned16(version) = suite_version_value else {
            return Err(MintStatementFromFieldsErrorReturn::FieldKind {
                index: 1,
                expected: CanonicalFieldKind::Unsigned16,
            });
        };
        let suite_identifier = match SuiteIdentifier::try_new(SuiteIdentifierConstructorParams {
            identifier,
            version,
        }) {
            Ok(value) => value,
            Err(error) => {
                return Err(MintStatementFromFieldsErrorReturn::SuiteIdentifier(error));
            }
        };
        let chain = match F::ChainIdentifier::from_field(FromFieldParams, chain_value) {
            Ok(success) => success.value,
            Err(error) => {
                return Err(MintStatementFromFieldsErrorReturn::ChainIdentifier {
                    index: 2,
                    error,
                });
            }
        };
        let entitlement_contract =
            match F::Identity::from_field(FromFieldParams, entitlement_contract_value) {
                Ok(success) => success.value,
                Err(error) => {
                    return Err(MintStatementFromFieldsErrorReturn::Identity { index: 3, error });
                }
            };
        let CanonicalFieldValue::FixedBytes32(asset_identity_hash_bytes) =
            asset_identity_hash_value
        else {
            return Err(MintStatementFromFieldsErrorReturn::FieldKind {
                index: 4,
                expected: CanonicalFieldKind::FixedBytes32,
            });
        };
        let asset_identity_hash =
            match AssetIdentityHash::try_new(AssetIdentityHashConstructorParams {
                bytes: asset_identity_hash_bytes,
            }) {
                Ok(value) => value,
                Err(error) => {
                    return Err(MintStatementFromFieldsErrorReturn::AssetIdentityHash(error));
                }
            };
        let CanonicalFieldValue::FixedBytes32(parameter_set_identifier_bytes) =
            parameter_set_identifier_value
        else {
            return Err(MintStatementFromFieldsErrorReturn::FieldKind {
                index: 5,
                expected: CanonicalFieldKind::FixedBytes32,
            });
        };
        let parameter_set_identifier =
            match ParameterSetIdentifier::try_new(ParameterSetIdentifierConstructorParams {
                bytes: parameter_set_identifier_bytes,
            }) {
                Ok(value) => value,
                Err(error) => {
                    return Err(MintStatementFromFieldsErrorReturn::ParameterSetIdentifier(
                        error,
                    ));
                }
            };
        let source_entitlement =
            match F::Entitlement::from_field(FromFieldParams, source_entitlement_value) {
                Ok(success) => success.value,
                Err(error) => {
                    return Err(MintStatementFromFieldsErrorReturn::Entitlement {
                        index: 6,
                        error,
                    });
                }
            };
        let target_entitlement =
            match F::Entitlement::from_field(FromFieldParams, target_entitlement_value) {
                Ok(success) => success.value,
                Err(error) => {
                    return Err(MintStatementFromFieldsErrorReturn::Entitlement {
                        index: 7,
                        error,
                    });
                }
            };
        let old_interval = match F::Interval::from_field(FromFieldParams, old_interval_value) {
            Ok(success) => success.value,
            Err(error) => {
                return Err(MintStatementFromFieldsErrorReturn::Interval { index: 8, error });
            }
        };
        let new_interval = match F::Interval::from_field(FromFieldParams, new_interval_value) {
            Ok(success) => success.value,
            Err(error) => {
                return Err(MintStatementFromFieldsErrorReturn::Interval { index: 9, error });
            }
        };
        let CanonicalFieldValue::Unsigned16(code) = purpose_value else {
            return Err(MintStatementFromFieldsErrorReturn::FieldKind {
                index: 10,
                expected: CanonicalFieldKind::Unsigned16,
            });
        };
        let purpose = match MintPurpose::try_from(code) {
            Ok(purpose) => purpose,
            Err(refusal) => {
                return Err(MintStatementFromFieldsErrorReturn::PurposeCode {
                    index: 10,
                    code: refusal.code,
                });
            }
        };
        let seller = match F::Identity::from_field(FromFieldParams, seller_value) {
            Ok(success) => success.value,
            Err(error) => {
                return Err(MintStatementFromFieldsErrorReturn::Identity { index: 11, error });
            }
        };
        let buyer = match F::Identity::from_field(FromFieldParams, buyer_value) {
            Ok(success) => success.value,
            Err(error) => {
                return Err(MintStatementFromFieldsErrorReturn::Identity { index: 12, error });
            }
        };
        let pk1 = self.decode_g1(13, pk1_value)?;
        let pk2 = self.decode_g2(14, pk2_value)?;
        let envelope_c1 = self.decode_g1(15, envelope_c1_value)?;
        let envelope_c2 = self.decode_g1(16, envelope_c2_value)?;
        let envelope_d1 = self.decode_g2(17, envelope_d1_value)?;
        let envelope_d2 = self.decode_g2(18, envelope_d2_value)?;
        let CanonicalFieldValue::Unsigned64(expiry) = expiry_value else {
            return Err(MintStatementFromFieldsErrorReturn::FieldKind {
                index: 19,
                expected: CanonicalFieldKind::Unsigned64,
            });
        };
        let hpub = self.decode_g2(20, hpub_value)?;
        let message_c1 = self.decode_g1(21, message_c1_value)?;
        let message_c2 = self.decode_g1(22, message_c2_value)?;
        let message_d1 = self.decode_g2(23, message_d1_value)?;
        let message_d2 = self.decode_g2(24, message_d2_value)?;
        Ok(FromFieldsSuccessReturn {
            described: MintStatement {
                suite_identifier,
                chain,
                entitlement_contract,
                asset_identity_hash,
                parameter_set_identifier,
                source_entitlement,
                target_entitlement,
                old_interval,
                new_interval,
                purpose,
                seller,
                buyer,
                buyer_keys: envelope::PublicKeysComponents { pk1, pk2 },
                envelope: envelope::EnvelopeComponents {
                    c1: envelope_c1,
                    c2: envelope_c2,
                    d1: envelope_d1,
                    d2: envelope_d2,
                },
                expiry,
                first_messages: MintFirstMessages {
                    hpub,
                    c1: message_c1,
                    c2: message_c2,
                    d1: message_d1,
                    d2: message_d2,
                },
            },
        })
    }
}
