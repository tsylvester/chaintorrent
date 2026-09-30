mod interface;
pub mod provides;
#[cfg(test)]
mod test;

use crate::factory::provides::{
    CanonicalFieldKind, CanonicalFieldValue, CanonicalFields, FromFieldsParams, FromFieldsReturn,
    FromFieldsSuccessReturn, IEncodingContract, ToFieldsParams, ToFieldsReturn,
    ToFieldsSuccessReturn,
};
use domain::{
    AssetIdentity, AssetIdentityConstructorParams, DeploymentIdentity,
    DeploymentIdentityConstructorParams, DerivationContext, DerivationContextConstructorParams,
    GroupIndex, GroupIndexConstructorParams, ParameterSetIdentifier,
    ParameterSetIdentifierConstructorParams, PieceGeometry, PieceGeometryConstructorParams,
    SuiteIdentifier, SuiteIdentifierConstructorParams,
};
use interface::{
    DERIVATION_CONTEXT_FIELD_COUNT, DERIVATION_CONTEXT_FIELD_KINDS, DerivationContextDescription,
    DerivationContextDescriptionConstructorParams, DerivationContextDescriptionTryNewReturn,
    DerivationContextFromFieldsErrorReturn,
};

impl DerivationContextDescription {
    pub fn try_new(
        _params: DerivationContextDescriptionConstructorParams,
    ) -> DerivationContextDescriptionTryNewReturn {
        Ok(DerivationContextDescription)
    }
}

impl IEncodingContract for DerivationContextDescription {
    type Described = DerivationContext;
    type FromFieldsErrorReturn = DerivationContextFromFieldsErrorReturn;
    const FIELDS: &'static [CanonicalFieldKind] = &DERIVATION_CONTEXT_FIELD_KINDS;

    fn to_fields(&self, _params: ToFieldsParams, payload: &Self::Described) -> ToFieldsReturn {
        Ok(ToFieldsSuccessReturn {
            fields: CanonicalFields {
                values: vec![
                    CanonicalFieldValue::Text(payload.asset().name().to_string()),
                    CanonicalFieldValue::Text(payload.asset().version().to_string()),
                    CanonicalFieldValue::FixedBytes32(*payload.deployment().as_bytes()),
                    CanonicalFieldValue::FixedBytes32(*payload.suite().identifier()),
                    CanonicalFieldValue::Unsigned16(payload.suite().version()),
                    CanonicalFieldValue::FixedBytes32(*payload.parameter_set().as_bytes()),
                    CanonicalFieldValue::Unsigned64(payload.group_index().value()),
                    CanonicalFieldValue::Unsigned32(payload.geometry().piece_size()),
                    CanonicalFieldValue::Unsigned32(payload.geometry().piece_group_size()),
                    CanonicalFieldValue::Unsigned64(payload.geometry().total_extent()),
                ],
            },
        })
    }

    fn fields_to_value(
        &self,
        _params: FromFieldsParams,
        payload: CanonicalFields,
    ) -> FromFieldsReturn<Self::Described, Self::FromFieldsErrorReturn> {
        let fields: [CanonicalFieldValue; DERIVATION_CONTEXT_FIELD_COUNT] =
            match payload.values.try_into() {
                Ok(fields) => fields,
                Err(values) => {
                    return Err(DerivationContextFromFieldsErrorReturn::FieldCount {
                        expected: DERIVATION_CONTEXT_FIELD_COUNT,
                        actual: values.len(),
                    });
                }
            };
        let [
            asset_name,
            asset_version,
            deployment_bytes,
            suite_identifier,
            suite_version,
            parameter_set_bytes,
            group_index_value,
            piece_size,
            piece_group_size,
            total_extent,
        ] = fields;

        let CanonicalFieldValue::Text(asset_name) = asset_name else {
            return Err(DerivationContextFromFieldsErrorReturn::FieldKind {
                index: 0,
                expected: CanonicalFieldKind::Text,
            });
        };
        let CanonicalFieldValue::Text(asset_version) = asset_version else {
            return Err(DerivationContextFromFieldsErrorReturn::FieldKind {
                index: 1,
                expected: CanonicalFieldKind::Text,
            });
        };
        let CanonicalFieldValue::FixedBytes32(deployment_bytes) = deployment_bytes else {
            return Err(DerivationContextFromFieldsErrorReturn::FieldKind {
                index: 2,
                expected: CanonicalFieldKind::FixedBytes32,
            });
        };
        let CanonicalFieldValue::FixedBytes32(suite_identifier) = suite_identifier else {
            return Err(DerivationContextFromFieldsErrorReturn::FieldKind {
                index: 3,
                expected: CanonicalFieldKind::FixedBytes32,
            });
        };
        let CanonicalFieldValue::Unsigned16(suite_version) = suite_version else {
            return Err(DerivationContextFromFieldsErrorReturn::FieldKind {
                index: 4,
                expected: CanonicalFieldKind::Unsigned16,
            });
        };
        let CanonicalFieldValue::FixedBytes32(parameter_set_bytes) = parameter_set_bytes else {
            return Err(DerivationContextFromFieldsErrorReturn::FieldKind {
                index: 5,
                expected: CanonicalFieldKind::FixedBytes32,
            });
        };
        let CanonicalFieldValue::Unsigned64(group_index_value) = group_index_value else {
            return Err(DerivationContextFromFieldsErrorReturn::FieldKind {
                index: 6,
                expected: CanonicalFieldKind::Unsigned64,
            });
        };
        let CanonicalFieldValue::Unsigned32(piece_size) = piece_size else {
            return Err(DerivationContextFromFieldsErrorReturn::FieldKind {
                index: 7,
                expected: CanonicalFieldKind::Unsigned32,
            });
        };
        let CanonicalFieldValue::Unsigned32(piece_group_size) = piece_group_size else {
            return Err(DerivationContextFromFieldsErrorReturn::FieldKind {
                index: 8,
                expected: CanonicalFieldKind::Unsigned32,
            });
        };
        let CanonicalFieldValue::Unsigned64(total_extent) = total_extent else {
            return Err(DerivationContextFromFieldsErrorReturn::FieldKind {
                index: 9,
                expected: CanonicalFieldKind::Unsigned64,
            });
        };

        let asset = match AssetIdentity::try_new(AssetIdentityConstructorParams {
            name: asset_name,
            version: asset_version,
        }) {
            Ok(asset) => asset,
            Err(error) => {
                return Err(DerivationContextFromFieldsErrorReturn::AssetIdentity(error));
            }
        };
        let deployment = match DeploymentIdentity::try_new(DeploymentIdentityConstructorParams {
            bytes: deployment_bytes,
        }) {
            Ok(deployment) => deployment,
            Err(error) => {
                return Err(DerivationContextFromFieldsErrorReturn::DeploymentIdentity(
                    error,
                ));
            }
        };
        let suite = match SuiteIdentifier::try_new(SuiteIdentifierConstructorParams {
            identifier: suite_identifier,
            version: suite_version,
        }) {
            Ok(suite) => suite,
            Err(error) => {
                return Err(DerivationContextFromFieldsErrorReturn::SuiteIdentifier(
                    error,
                ));
            }
        };
        let parameter_set =
            match ParameterSetIdentifier::try_new(ParameterSetIdentifierConstructorParams {
                bytes: parameter_set_bytes,
            }) {
                Ok(parameter_set) => parameter_set,
                Err(error) => {
                    return Err(
                        DerivationContextFromFieldsErrorReturn::ParameterSetIdentifier(error),
                    );
                }
            };
        let Ok(group_index) = GroupIndex::try_new(GroupIndexConstructorParams {
            value: group_index_value,
        });
        let geometry = match PieceGeometry::try_new(PieceGeometryConstructorParams {
            piece_size,
            piece_group_size,
            total_extent,
        }) {
            Ok(geometry) => geometry,
            Err(error) => {
                return Err(DerivationContextFromFieldsErrorReturn::PieceGeometry(error));
            }
        };
        let described = match DerivationContext::try_new(DerivationContextConstructorParams {
            asset,
            deployment,
            suite,
            parameter_set,
            group_index,
            geometry,
        }) {
            Ok(described) => described,
            Err(error) => {
                return Err(DerivationContextFromFieldsErrorReturn::DerivationContext(
                    error,
                ));
            }
        };
        Ok(FromFieldsSuccessReturn { described })
    }
}
