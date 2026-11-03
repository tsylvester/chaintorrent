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
    CanonicalFieldKind, CanonicalFieldReader, CanonicalFieldValue, CanonicalFields,
    FromFieldsParams, FromFieldsReturn, FromFieldsSuccessReturn, IEncodingContract, ToFieldsParams,
    ToFieldsReturn, ToFieldsSuccessReturn,
};
use hashing::{Root, RootConstructorParams};
use interface::{
    DERIVATION_CONTEXT_FIELD_KINDS, DerivationContext, DerivationContextConstructorParams,
    DerivationContextDescription, DerivationContextDescriptionConstructorParams,
    DerivationContextDescriptionTryNewReturn, DerivationContextFromFieldsErrorReturn,
    DerivationContextTryNewErrorReturn, DerivationContextTryNewReturn,
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
        let mut reader = match CanonicalFieldReader::try_new(payload, Self::FIELDS) {
            Ok(reader) => reader,
            Err(error) => return Err(DerivationContextFromFieldsErrorReturn::Fields(error)),
        };
        let asset_bytes = match reader.read_fixed_bytes32() {
            Ok(bytes) => bytes,
            Err(error) => return Err(DerivationContextFromFieldsErrorReturn::Fields(error)),
        };
        let deployment_bytes = match reader.read_fixed_bytes32() {
            Ok(bytes) => bytes,
            Err(error) => return Err(DerivationContextFromFieldsErrorReturn::Fields(error)),
        };
        let suite_identifier = match reader.read_fixed_bytes32() {
            Ok(bytes) => bytes,
            Err(error) => return Err(DerivationContextFromFieldsErrorReturn::Fields(error)),
        };
        let version = match reader.read_unsigned16() {
            Ok(version) => version,
            Err(error) => return Err(DerivationContextFromFieldsErrorReturn::Fields(error)),
        };
        let parameter_set_bytes = match reader.read_fixed_bytes32() {
            Ok(bytes) => bytes,
            Err(error) => return Err(DerivationContextFromFieldsErrorReturn::Fields(error)),
        };
        let index_value = match reader.read_unsigned64() {
            Ok(value) => value,
            Err(error) => return Err(DerivationContextFromFieldsErrorReturn::Fields(error)),
        };
        let piece_size = match reader.read_unsigned32() {
            Ok(size) => size,
            Err(error) => return Err(DerivationContextFromFieldsErrorReturn::Fields(error)),
        };
        let piece_group_size = match reader.read_unsigned32() {
            Ok(size) => size,
            Err(error) => return Err(DerivationContextFromFieldsErrorReturn::Fields(error)),
        };
        let total_extent = match reader.read_unsigned64() {
            Ok(extent) => extent,
            Err(error) => return Err(DerivationContextFromFieldsErrorReturn::Fields(error)),
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
