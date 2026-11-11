mod interface;
#[cfg(any(test, feature = "mocks"))]
mod mock;
pub mod provides;
#[cfg(test)]
mod test;

use crate::mint_statement::provides::TransferPurpose;
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
    TransferFirstMessages, TransferStatement, TransferStatementDescription,
    TransferStatementDescriptionConstructorParams, TransferStatementDescriptionTryNewReturn,
    TransferStatementFromFieldsErrorReturn,
};
use pairing::{
    DecodeG1Params, DecodeG2Params, EncodeG1Params, EncodeG1Payload, EncodeG2Params,
    EncodeG2Payload, IPairingAdapter,
};

impl<'a, P: IPairingAdapter, F: IChainForms> TransferStatementDescription<'a, P, F> {
    pub fn try_new(
        params: TransferStatementDescriptionConstructorParams<'a, P>,
    ) -> TransferStatementDescriptionTryNewReturn<'a, P, F> {
        Ok(TransferStatementDescription {
            pairing: params.pairing,
            forms: PhantomData,
        })
    }
}

impl<'a, P: IPairingAdapter, F: IChainForms> TransferStatementDescription<'a, P, F> {
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
    ) -> Result<P::G1, TransferStatementFromFieldsErrorReturn> {
        match self.pairing.decode_g1(DecodeG1Params, &bytes) {
            Ok(success) => Ok(success.point),
            Err(error) => Err(TransferStatementFromFieldsErrorReturn::InvalidG1 { index, error }),
        }
    }

    fn decode_g2(
        &self,
        index: usize,
        bytes: Vec<u8>,
    ) -> Result<P::G2, TransferStatementFromFieldsErrorReturn> {
        match self.pairing.decode_g2(DecodeG2Params, &bytes) {
            Ok(success) => Ok(success.point),
            Err(error) => Err(TransferStatementFromFieldsErrorReturn::InvalidG2 { index, error }),
        }
    }
}

impl<P: IPairingAdapter, F: IChainForms> IEncodingContract
    for TransferStatementDescription<'_, P, F>
{
    type Described = TransferStatement<P, F>;
    type FromFieldsErrorReturn = TransferStatementFromFieldsErrorReturn;
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
                    self.encode_g1(payload.seller_keys.pk1.clone()),
                    self.encode_g2(payload.seller_keys.pk2.clone()),
                    self.encode_g1(payload.buyer_keys.pk1.clone()),
                    self.encode_g2(payload.buyer_keys.pk2.clone()),
                    self.encode_g1(payload.old_envelope.c1.clone()),
                    self.encode_g1(payload.old_envelope.c2.clone()),
                    self.encode_g2(payload.old_envelope.d1.clone()),
                    self.encode_g2(payload.old_envelope.d2.clone()),
                    self.encode_g1(payload.new_envelope.c1.clone()),
                    self.encode_g1(payload.new_envelope.c2.clone()),
                    self.encode_g2(payload.new_envelope.d1.clone()),
                    self.encode_g2(payload.new_envelope.d2.clone()),
                    CanonicalFieldValue::Unsigned64(payload.expiry),
                    self.encode_g1(payload.first_messages.pk1.clone()),
                    self.encode_g1(payload.first_messages.c1.clone()),
                    self.encode_g1(payload.first_messages.c2.clone()),
                    self.encode_g2(payload.first_messages.pk2.clone()),
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
            .map_err(TransferStatementFromFieldsErrorReturn::Fields)?;
        let identifier = reader
            .read_fixed_bytes32()
            .map_err(TransferStatementFromFieldsErrorReturn::Fields)?;
        let version = reader
            .read_unsigned16()
            .map_err(TransferStatementFromFieldsErrorReturn::Fields)?;
        let suite_identifier = SuiteIdentifier::try_new(SuiteIdentifierConstructorParams {
            identifier,
            version,
        })
        .map_err(TransferStatementFromFieldsErrorReturn::SuiteIdentifier)?;
        let chain = reader
            .read_form::<F::ChainIdentifier>()
            .map_err(TransferStatementFromFieldsErrorReturn::Fields)?;
        let entitlement_contract = reader
            .read_form::<F::Identity>()
            .map_err(TransferStatementFromFieldsErrorReturn::Fields)?;
        let asset_root_bytes = reader
            .read_fixed_bytes32()
            .map_err(TransferStatementFromFieldsErrorReturn::Fields)?;
        let asset_root = Root::try_new(RootConstructorParams {
            bytes: asset_root_bytes,
        })
        .map_err(TransferStatementFromFieldsErrorReturn::AssetRoot)?;
        let parameter_set_identifier_bytes = reader
            .read_fixed_bytes32()
            .map_err(TransferStatementFromFieldsErrorReturn::Fields)?;
        let parameter_set_identifier =
            ParameterSetIdentifier::try_new(ParameterSetIdentifierConstructorParams {
                bytes: parameter_set_identifier_bytes,
            })
            .map_err(TransferStatementFromFieldsErrorReturn::ParameterSetIdentifier)?;
        let source_entitlement = reader
            .read_form::<F::Entitlement>()
            .map_err(TransferStatementFromFieldsErrorReturn::Fields)?;
        let target_entitlement = reader
            .read_form::<F::Entitlement>()
            .map_err(TransferStatementFromFieldsErrorReturn::Fields)?;
        let old_interval = reader
            .read_form::<F::Interval>()
            .map_err(TransferStatementFromFieldsErrorReturn::Fields)?;
        let new_interval = reader
            .read_form::<F::Interval>()
            .map_err(TransferStatementFromFieldsErrorReturn::Fields)?;
        let code = reader
            .read_unsigned16()
            .map_err(TransferStatementFromFieldsErrorReturn::Fields)?;
        let purpose = match TransferPurpose::try_from(code) {
            Ok(purpose) => purpose,
            Err(refusal) => {
                return Err(TransferStatementFromFieldsErrorReturn::PurposeCode {
                    index: 10,
                    code: refusal.code,
                });
            }
        };
        let seller = reader
            .read_form::<F::Identity>()
            .map_err(TransferStatementFromFieldsErrorReturn::Fields)?;
        let buyer = reader
            .read_form::<F::Identity>()
            .map_err(TransferStatementFromFieldsErrorReturn::Fields)?;
        let seller_pk1_bytes = reader
            .read_bytes()
            .map_err(TransferStatementFromFieldsErrorReturn::Fields)?;
        let seller_pk2_bytes = reader
            .read_bytes()
            .map_err(TransferStatementFromFieldsErrorReturn::Fields)?;
        let buyer_pk1_bytes = reader
            .read_bytes()
            .map_err(TransferStatementFromFieldsErrorReturn::Fields)?;
        let buyer_pk2_bytes = reader
            .read_bytes()
            .map_err(TransferStatementFromFieldsErrorReturn::Fields)?;
        let old_envelope_c1_bytes = reader
            .read_bytes()
            .map_err(TransferStatementFromFieldsErrorReturn::Fields)?;
        let old_envelope_c2_bytes = reader
            .read_bytes()
            .map_err(TransferStatementFromFieldsErrorReturn::Fields)?;
        let old_envelope_d1_bytes = reader
            .read_bytes()
            .map_err(TransferStatementFromFieldsErrorReturn::Fields)?;
        let old_envelope_d2_bytes = reader
            .read_bytes()
            .map_err(TransferStatementFromFieldsErrorReturn::Fields)?;
        let new_envelope_c1_bytes = reader
            .read_bytes()
            .map_err(TransferStatementFromFieldsErrorReturn::Fields)?;
        let new_envelope_c2_bytes = reader
            .read_bytes()
            .map_err(TransferStatementFromFieldsErrorReturn::Fields)?;
        let new_envelope_d1_bytes = reader
            .read_bytes()
            .map_err(TransferStatementFromFieldsErrorReturn::Fields)?;
        let new_envelope_d2_bytes = reader
            .read_bytes()
            .map_err(TransferStatementFromFieldsErrorReturn::Fields)?;
        let expiry = reader
            .read_unsigned64()
            .map_err(TransferStatementFromFieldsErrorReturn::Fields)?;
        let message_pk1_bytes = reader
            .read_bytes()
            .map_err(TransferStatementFromFieldsErrorReturn::Fields)?;
        let message_c1_bytes = reader
            .read_bytes()
            .map_err(TransferStatementFromFieldsErrorReturn::Fields)?;
        let message_c2_bytes = reader
            .read_bytes()
            .map_err(TransferStatementFromFieldsErrorReturn::Fields)?;
        let message_pk2_bytes = reader
            .read_bytes()
            .map_err(TransferStatementFromFieldsErrorReturn::Fields)?;
        let message_d1_bytes = reader
            .read_bytes()
            .map_err(TransferStatementFromFieldsErrorReturn::Fields)?;
        let message_d2_bytes = reader
            .read_bytes()
            .map_err(TransferStatementFromFieldsErrorReturn::Fields)?;
        let seller_pk1 = self.decode_g1(13, seller_pk1_bytes)?;
        let seller_pk2 = self.decode_g2(14, seller_pk2_bytes)?;
        let buyer_pk1 = self.decode_g1(15, buyer_pk1_bytes)?;
        let buyer_pk2 = self.decode_g2(16, buyer_pk2_bytes)?;
        let old_envelope_c1 = self.decode_g1(17, old_envelope_c1_bytes)?;
        let old_envelope_c2 = self.decode_g1(18, old_envelope_c2_bytes)?;
        let old_envelope_d1 = self.decode_g2(19, old_envelope_d1_bytes)?;
        let old_envelope_d2 = self.decode_g2(20, old_envelope_d2_bytes)?;
        let new_envelope_c1 = self.decode_g1(21, new_envelope_c1_bytes)?;
        let new_envelope_c2 = self.decode_g1(22, new_envelope_c2_bytes)?;
        let new_envelope_d1 = self.decode_g2(23, new_envelope_d1_bytes)?;
        let new_envelope_d2 = self.decode_g2(24, new_envelope_d2_bytes)?;
        let message_pk1 = self.decode_g1(26, message_pk1_bytes)?;
        let message_c1 = self.decode_g1(27, message_c1_bytes)?;
        let message_c2 = self.decode_g1(28, message_c2_bytes)?;
        let message_pk2 = self.decode_g2(29, message_pk2_bytes)?;
        let message_d1 = self.decode_g2(30, message_d1_bytes)?;
        let message_d2 = self.decode_g2(31, message_d2_bytes)?;
        Ok(FromFieldsSuccessReturn {
            described: TransferStatement {
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
                seller_keys: envelope::PublicKeysComponents {
                    pk1: seller_pk1,
                    pk2: seller_pk2,
                },
                buyer_keys: envelope::PublicKeysComponents {
                    pk1: buyer_pk1,
                    pk2: buyer_pk2,
                },
                old_envelope: envelope::EnvelopeComponents {
                    c1: old_envelope_c1,
                    c2: old_envelope_c2,
                    d1: old_envelope_d1,
                    d2: old_envelope_d2,
                },
                new_envelope: envelope::EnvelopeComponents {
                    c1: new_envelope_c1,
                    c2: new_envelope_c2,
                    d1: new_envelope_d1,
                    d2: new_envelope_d2,
                },
                expiry,
                first_messages: TransferFirstMessages {
                    pk1: message_pk1,
                    c1: message_c1,
                    c2: message_c2,
                    pk2: message_pk2,
                    d1: message_d1,
                    d2: message_d2,
                },
            },
        })
    }
}
