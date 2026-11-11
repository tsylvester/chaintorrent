mod interface;
#[cfg(any(test, feature = "mocks"))]
mod mock;
pub mod provides;
#[cfg(test)]
mod test;

use chain::IChainForms;
use core::marker::PhantomData;
use domain::{
    ParameterSetIdentifier, ParameterSetIdentifierConstructorParams, SuiteIdentifier,
    SuiteIdentifierConstructorParams,
};
use encoding::{
    CanonicalFieldKind, CanonicalFieldReader, CanonicalFieldValue, CanonicalFields,
    FromFieldsParams, FromFieldsReturn, FromFieldsSuccessReturn, ICanonicalField,
    IEncodingContract, ToFieldParams, ToFieldsParams, ToFieldsReturn, ToFieldsSuccessReturn,
};
use hashing::{Root, RootConstructorParams};
use interface::{
    MintFirstMessages, MintPurpose, MintStatement, MintStatementDescription,
    MintStatementDescriptionConstructorParams, MintStatementDescriptionTryNewReturn,
    MintStatementFromFieldsErrorReturn,
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
        bytes: Vec<u8>,
    ) -> Result<P::G1, MintStatementFromFieldsErrorReturn> {
        match self.pairing.decode_g1(DecodeG1Params, &bytes) {
            Ok(success) => Ok(success.point),
            Err(error) => Err(MintStatementFromFieldsErrorReturn::InvalidG1 { index, error }),
        }
    }

    fn decode_g2(
        &self,
        index: usize,
        bytes: Vec<u8>,
    ) -> Result<P::G2, MintStatementFromFieldsErrorReturn> {
        match self.pairing.decode_g2(DecodeG2Params, &bytes) {
            Ok(success) => Ok(success.point),
            Err(error) => Err(MintStatementFromFieldsErrorReturn::InvalidG2 { index, error }),
        }
    }
}

impl<P: IPairingAdapter, F: IChainForms> IEncodingContract for MintStatementDescription<'_, P, F> {
    type Described = MintStatement<P, F>;
    type FromFieldsErrorReturn = MintStatementFromFieldsErrorReturn;
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
                    CanonicalFieldValue::FixedBytes32(*payload.asset_root.as_bytes()),
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
        let mut reader = CanonicalFieldReader::try_new(payload, Self::FIELDS)
            .map_err(MintStatementFromFieldsErrorReturn::Fields)?;
        let identifier = reader
            .read_fixed_bytes32()
            .map_err(MintStatementFromFieldsErrorReturn::Fields)?;
        let version = reader
            .read_unsigned16()
            .map_err(MintStatementFromFieldsErrorReturn::Fields)?;
        let suite_identifier = SuiteIdentifier::try_new(SuiteIdentifierConstructorParams {
            identifier,
            version,
        })
        .map_err(MintStatementFromFieldsErrorReturn::SuiteIdentifier)?;
        let chain = reader
            .read_form::<F::ChainIdentifier>()
            .map_err(MintStatementFromFieldsErrorReturn::Fields)?;
        let entitlement_contract = reader
            .read_form::<F::Identity>()
            .map_err(MintStatementFromFieldsErrorReturn::Fields)?;
        let asset_root_bytes = reader
            .read_fixed_bytes32()
            .map_err(MintStatementFromFieldsErrorReturn::Fields)?;
        let asset_root = Root::try_new(RootConstructorParams {
            bytes: asset_root_bytes,
        })
        .map_err(MintStatementFromFieldsErrorReturn::AssetRoot)?;
        let parameter_set_identifier_bytes = reader
            .read_fixed_bytes32()
            .map_err(MintStatementFromFieldsErrorReturn::Fields)?;
        let parameter_set_identifier =
            ParameterSetIdentifier::try_new(ParameterSetIdentifierConstructorParams {
                bytes: parameter_set_identifier_bytes,
            })
            .map_err(MintStatementFromFieldsErrorReturn::ParameterSetIdentifier)?;
        let source_entitlement = reader
            .read_form::<F::Entitlement>()
            .map_err(MintStatementFromFieldsErrorReturn::Fields)?;
        let target_entitlement = reader
            .read_form::<F::Entitlement>()
            .map_err(MintStatementFromFieldsErrorReturn::Fields)?;
        let old_interval = reader
            .read_form::<F::Interval>()
            .map_err(MintStatementFromFieldsErrorReturn::Fields)?;
        let new_interval = reader
            .read_form::<F::Interval>()
            .map_err(MintStatementFromFieldsErrorReturn::Fields)?;
        let code = reader
            .read_unsigned16()
            .map_err(MintStatementFromFieldsErrorReturn::Fields)?;
        let purpose = match MintPurpose::try_from(code) {
            Ok(purpose) => purpose,
            Err(refusal) => {
                return Err(MintStatementFromFieldsErrorReturn::PurposeCode {
                    index: 10,
                    code: refusal.code,
                });
            }
        };
        let seller = reader
            .read_form::<F::Identity>()
            .map_err(MintStatementFromFieldsErrorReturn::Fields)?;
        let buyer = reader
            .read_form::<F::Identity>()
            .map_err(MintStatementFromFieldsErrorReturn::Fields)?;
        let pk1_bytes = reader
            .read_bytes()
            .map_err(MintStatementFromFieldsErrorReturn::Fields)?;
        let pk2_bytes = reader
            .read_bytes()
            .map_err(MintStatementFromFieldsErrorReturn::Fields)?;
        let envelope_c1_bytes = reader
            .read_bytes()
            .map_err(MintStatementFromFieldsErrorReturn::Fields)?;
        let envelope_c2_bytes = reader
            .read_bytes()
            .map_err(MintStatementFromFieldsErrorReturn::Fields)?;
        let envelope_d1_bytes = reader
            .read_bytes()
            .map_err(MintStatementFromFieldsErrorReturn::Fields)?;
        let envelope_d2_bytes = reader
            .read_bytes()
            .map_err(MintStatementFromFieldsErrorReturn::Fields)?;
        let expiry = reader
            .read_unsigned64()
            .map_err(MintStatementFromFieldsErrorReturn::Fields)?;
        let hpub_bytes = reader
            .read_bytes()
            .map_err(MintStatementFromFieldsErrorReturn::Fields)?;
        let message_c1_bytes = reader
            .read_bytes()
            .map_err(MintStatementFromFieldsErrorReturn::Fields)?;
        let message_c2_bytes = reader
            .read_bytes()
            .map_err(MintStatementFromFieldsErrorReturn::Fields)?;
        let message_d1_bytes = reader
            .read_bytes()
            .map_err(MintStatementFromFieldsErrorReturn::Fields)?;
        let message_d2_bytes = reader
            .read_bytes()
            .map_err(MintStatementFromFieldsErrorReturn::Fields)?;
        let pk1 = self.decode_g1(13, pk1_bytes)?;
        let pk2 = self.decode_g2(14, pk2_bytes)?;
        let envelope_c1 = self.decode_g1(15, envelope_c1_bytes)?;
        let envelope_c2 = self.decode_g1(16, envelope_c2_bytes)?;
        let envelope_d1 = self.decode_g2(17, envelope_d1_bytes)?;
        let envelope_d2 = self.decode_g2(18, envelope_d2_bytes)?;
        let hpub = self.decode_g2(20, hpub_bytes)?;
        let message_c1 = self.decode_g1(21, message_c1_bytes)?;
        let message_c2 = self.decode_g1(22, message_c2_bytes)?;
        let message_d1 = self.decode_g2(23, message_d1_bytes)?;
        let message_d2 = self.decode_g2(24, message_d2_bytes)?;
        Ok(FromFieldsSuccessReturn {
            described: MintStatement {
                suite_identifier,
                chain,
                entitlement_contract,
                asset_root,
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
