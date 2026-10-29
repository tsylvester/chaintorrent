mod interface;
#[cfg(any(test, feature = "mocks"))]
mod mock;
pub mod provides;
#[cfg(test)]
mod test_basic;
#[cfg(test)]
mod test_error;

use domain::{
    DeploymentIdentity, DeploymentIdentityConstructorParams, GroupIndex,
    GroupIndexConstructorParams, ParameterSetIdentifier, ParameterSetIdentifierConstructorParams,
    PieceGeometry, PieceGeometryConstructorParams, SuiteIdentifier,
    SuiteIdentifierConstructorParams,
};
use encoding::{
    CanonicalFieldKind, CanonicalFieldValue, CanonicalFields, FromFieldsParams, FromFieldsReturn,
    FromFieldsSuccessReturn, IEncodingContract, ToFieldsParams, ToFieldsReturn,
    ToFieldsSuccessReturn,
};
use hashing::{Root, RootConstructorParams};
use interface::{
    DERIVATION_CONTEXT_FIELD_COUNT, DERIVATION_CONTEXT_FIELD_KINDS, DerivationContext,
    DerivationContextConstructorParams, DerivationContextDescription,
    DerivationContextDescriptionConstructorParams, DerivationContextDescriptionTryNewReturn,
    DerivationContextFromFieldsErrorReturn, DerivationContextTryNewErrorReturn,
    DerivationContextTryNewReturn,
};

impl DerivationContext {
    pub fn try_new(params: DerivationContextConstructorParams) -> DerivationContextTryNewReturn {
        let group_index = params.group_index.value();
        let group_count = params.geometry.group_count();
        if group_index >= group_count {
            return Err(DerivationContextTryNewErrorReturn::GroupIndexOutOfRange {
                group_index,
                group_count,
            });
        }
        Ok(DerivationContext {
            asset: params.asset,
            deployment: params.deployment,
            suite: params.suite,
            parameter_set: params.parameter_set,
            group_index: params.group_index,
            geometry: params.geometry,
        })
    }
}

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
                    CanonicalFieldValue::FixedBytes32(*payload.asset.as_bytes()),
                    CanonicalFieldValue::FixedBytes32(*payload.deployment.as_bytes()),
                    CanonicalFieldValue::FixedBytes32(*payload.suite.identifier()),
                    CanonicalFieldValue::Unsigned16(payload.suite.version()),
                    CanonicalFieldValue::FixedBytes32(*payload.parameter_set.as_bytes()),
                    CanonicalFieldValue::Unsigned64(payload.group_index.value()),
                    CanonicalFieldValue::Unsigned32(payload.geometry.piece_size()),
                    CanonicalFieldValue::Unsigned32(payload.geometry.piece_group_size()),
                    CanonicalFieldValue::Unsigned64(payload.geometry.total_extent()),
                ],
            },
        })
    }

    fn fields_to_value(
        &self,
        _params: FromFieldsParams,
        payload: CanonicalFields,
    ) -> FromFieldsReturn<Self::Described, Self::FromFieldsErrorReturn> {
        let values =
            <[CanonicalFieldValue; DERIVATION_CONTEXT_FIELD_COUNT]>::try_from(payload.values)
                .map_err(
                    |values| DerivationContextFromFieldsErrorReturn::FieldCount {
                        expected: DERIVATION_CONTEXT_FIELD_COUNT,
                        actual: values.len(),
                    },
                )?;
        let [
            asset_value,
            deployment_value,
            suite_identifier_value,
            suite_version_value,
            parameter_set_value,
            group_index_value,
            piece_size_value,
            piece_group_size_value,
            total_extent_value,
        ] = values;
        let CanonicalFieldValue::FixedBytes32(asset_bytes) = asset_value else {
            return Err(DerivationContextFromFieldsErrorReturn::FieldKind {
                index: 0,
                expected: CanonicalFieldKind::FixedBytes32,
            });
        };
        let CanonicalFieldValue::FixedBytes32(deployment_bytes) = deployment_value else {
            return Err(DerivationContextFromFieldsErrorReturn::FieldKind {
                index: 1,
                expected: CanonicalFieldKind::FixedBytes32,
            });
        };
        let CanonicalFieldValue::FixedBytes32(suite_identifier) = suite_identifier_value else {
            return Err(DerivationContextFromFieldsErrorReturn::FieldKind {
                index: 2,
                expected: CanonicalFieldKind::FixedBytes32,
            });
        };
        let CanonicalFieldValue::Unsigned16(version) = suite_version_value else {
            return Err(DerivationContextFromFieldsErrorReturn::FieldKind {
                index: 3,
                expected: CanonicalFieldKind::Unsigned16,
            });
        };
        let CanonicalFieldValue::FixedBytes32(parameter_set_bytes) = parameter_set_value else {
            return Err(DerivationContextFromFieldsErrorReturn::FieldKind {
                index: 4,
                expected: CanonicalFieldKind::FixedBytes32,
            });
        };
        let CanonicalFieldValue::Unsigned64(index_value) = group_index_value else {
            return Err(DerivationContextFromFieldsErrorReturn::FieldKind {
                index: 5,
                expected: CanonicalFieldKind::Unsigned64,
            });
        };
        let CanonicalFieldValue::Unsigned32(piece_size) = piece_size_value else {
            return Err(DerivationContextFromFieldsErrorReturn::FieldKind {
                index: 6,
                expected: CanonicalFieldKind::Unsigned32,
            });
        };
        let CanonicalFieldValue::Unsigned32(piece_group_size) = piece_group_size_value else {
            return Err(DerivationContextFromFieldsErrorReturn::FieldKind {
                index: 7,
                expected: CanonicalFieldKind::Unsigned32,
            });
        };
        let CanonicalFieldValue::Unsigned64(total_extent) = total_extent_value else {
            return Err(DerivationContextFromFieldsErrorReturn::FieldKind {
                index: 8,
                expected: CanonicalFieldKind::Unsigned64,
            });
        };
        let asset = match Root::try_new(RootConstructorParams { bytes: asset_bytes }) {
            Ok(asset) => asset,
            Err(error) => return Err(DerivationContextFromFieldsErrorReturn::Asset(error)),
        };
        let deployment = match DeploymentIdentity::try_new(DeploymentIdentityConstructorParams {
            bytes: deployment_bytes,
        }) {
            Ok(deployment) => deployment,
            Err(error) => return Err(DerivationContextFromFieldsErrorReturn::Deployment(error)),
        };
        let suite = match SuiteIdentifier::try_new(SuiteIdentifierConstructorParams {
            identifier: suite_identifier,
            version,
        }) {
            Ok(suite) => suite,
            Err(error) => return Err(DerivationContextFromFieldsErrorReturn::Suite(error)),
        };
        let parameter_set =
            match ParameterSetIdentifier::try_new(ParameterSetIdentifierConstructorParams {
                bytes: parameter_set_bytes,
            }) {
                Ok(parameter_set) => parameter_set,
                Err(error) => {
                    return Err(DerivationContextFromFieldsErrorReturn::ParameterSet(error));
                }
            };
        let Ok(group_index) =
            GroupIndex::try_new(GroupIndexConstructorParams { value: index_value });
        let geometry = match PieceGeometry::try_new(PieceGeometryConstructorParams {
            piece_size,
            piece_group_size,
            total_extent,
        }) {
            Ok(geometry) => geometry,
            Err(error) => return Err(DerivationContextFromFieldsErrorReturn::Geometry(error)),
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
            Err(error) => return Err(DerivationContextFromFieldsErrorReturn::Context(error)),
        };
        Ok(FromFieldsSuccessReturn { described })
    }
}
