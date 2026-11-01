#[cfg(test)]
mod integration_test;
mod interface;
#[cfg(any(test, feature = "mocks"))]
mod mock;
pub mod provides;
#[cfg(test)]
mod test;

use domain::Secret;
use encoding::{
    CanonicalFieldKind, CanonicalFieldValue, CanonicalFields, FromFieldsParams, FromFieldsReturn,
    FromFieldsSuccessReturn, IEncodingContract, ToFieldsParams, ToFieldsReturn,
    ToFieldsSuccessReturn,
};
use hash_to_scalar::{
    DomainTag, DomainTagConstructorParams, HashToScalarParams, HashToScalarPayload,
};
use kem::{
    DeriveIdentityParams, DeriveIdentityPayload, ICredentialKemAdapter,
    IdentityElementComponentsParams, IdentityElementComponentsPayload, KemIdentity, SetupParams,
    SetupPayload,
};
use pairing::{EncodeScalarParams, EncodeScalarPayload, IPairingArithmetic, ISampleUniformScalar};
use random::{FillBytesParams, FillBytesPayload};

use self::interface::{
    HASH_TO_SCALAR_VECTOR_FIELD_COUNT, HASH_TO_SCALAR_VECTOR_FIELD_KINDS,
    HASH_TO_SCALAR_VECTOR_MEMBER_NAMES, HashToScalarVector, HashToScalarVectorDescription,
    HashToScalarVectorDescriptionConstructorParams, HashToScalarVectorDescriptionTryNewReturn,
    HashToScalarVectorFromFieldsErrorReturn, IDENTITY_MAPPING_VECTOR_FIELD_COUNT,
    IDENTITY_MAPPING_VECTOR_FIELD_KINDS, IDENTITY_MAPPING_VECTOR_MEMBER_NAMES,
    IdentityMappingVector, IdentityMappingVectorDescription,
    IdentityMappingVectorDescriptionConstructorParams,
    IdentityMappingVectorDescriptionTryNewReturn, IdentityMappingVectorFromFieldsErrorReturn,
    MappingVectorMemberNamesErrorReturn, MappingVectorMemberNamesReturn, MappingVectorsDeps,
    MappingVectorsErrorReturn, MappingVectorsParams, MappingVectorsPayload, MappingVectorsReturn,
    MappingVectorsSuccessReturn,
};
use super::render::provides::{SolidityMemberName, SolidityMemberNameConstructorParams};

impl HashToScalarVectorDescription {
    pub fn try_new(
        _params: HashToScalarVectorDescriptionConstructorParams,
    ) -> HashToScalarVectorDescriptionTryNewReturn {
        Ok(HashToScalarVectorDescription)
    }

    pub fn member_names(&self) -> MappingVectorMemberNamesReturn {
        let mut names = Vec::new();
        for text in HASH_TO_SCALAR_VECTOR_MEMBER_NAMES {
            match SolidityMemberName::try_new(SolidityMemberNameConstructorParams {
                text: text.to_string(),
            }) {
                Err(error) => {
                    return Err(MappingVectorMemberNamesErrorReturn::MemberName(error));
                }
                Ok(name) => names.push(name),
            }
        }
        Ok(names)
    }
}

impl IdentityMappingVectorDescription {
    pub fn try_new(
        _params: IdentityMappingVectorDescriptionConstructorParams,
    ) -> IdentityMappingVectorDescriptionTryNewReturn {
        Ok(IdentityMappingVectorDescription)
    }

    pub fn member_names(&self) -> MappingVectorMemberNamesReturn {
        let mut names = Vec::new();
        for text in IDENTITY_MAPPING_VECTOR_MEMBER_NAMES {
            match SolidityMemberName::try_new(SolidityMemberNameConstructorParams {
                text: text.to_string(),
            }) {
                Err(error) => {
                    return Err(MappingVectorMemberNamesErrorReturn::MemberName(error));
                }
                Ok(name) => names.push(name),
            }
        }
        Ok(names)
    }
}

impl IEncodingContract for HashToScalarVectorDescription {
    type Described = HashToScalarVector;
    type FromFieldsErrorReturn = HashToScalarVectorFromFieldsErrorReturn;

    const FIELDS: &'static [CanonicalFieldKind] = &HASH_TO_SCALAR_VECTOR_FIELD_KINDS;

    fn to_fields(&self, _params: ToFieldsParams, payload: &Self::Described) -> ToFieldsReturn {
        Ok(ToFieldsSuccessReturn {
            fields: CanonicalFields {
                values: vec![
                    CanonicalFieldValue::Bytes(payload.tag.clone()),
                    CanonicalFieldValue::Bytes(payload.message.clone()),
                    CanonicalFieldValue::Unsigned256(payload.scalar),
                ],
            },
        })
    }

    fn fields_to_value(
        &self,
        _params: FromFieldsParams,
        payload: CanonicalFields,
    ) -> FromFieldsReturn<Self::Described, Self::FromFieldsErrorReturn> {
        let [tag, message, scalar] =
            match <[CanonicalFieldValue; HASH_TO_SCALAR_VECTOR_FIELD_COUNT]>::try_from(
                payload.values,
            ) {
                Err(values) => {
                    return Err(HashToScalarVectorFromFieldsErrorReturn::FieldCount {
                        expected: HASH_TO_SCALAR_VECTOR_FIELD_COUNT,
                        actual: values.len(),
                    });
                }
                Ok(values) => values,
            };
        let CanonicalFieldValue::Bytes(tag) = tag else {
            return Err(HashToScalarVectorFromFieldsErrorReturn::FieldKind {
                index: 0,
                expected: HASH_TO_SCALAR_VECTOR_FIELD_KINDS[0],
            });
        };
        let CanonicalFieldValue::Bytes(message) = message else {
            return Err(HashToScalarVectorFromFieldsErrorReturn::FieldKind {
                index: 1,
                expected: HASH_TO_SCALAR_VECTOR_FIELD_KINDS[1],
            });
        };
        let CanonicalFieldValue::Unsigned256(scalar) = scalar else {
            return Err(HashToScalarVectorFromFieldsErrorReturn::FieldKind {
                index: 2,
                expected: HASH_TO_SCALAR_VECTOR_FIELD_KINDS[2],
            });
        };
        Ok(FromFieldsSuccessReturn {
            described: HashToScalarVector {
                tag,
                message,
                scalar,
            },
        })
    }
}

impl IEncodingContract for IdentityMappingVectorDescription {
    type Described = IdentityMappingVector;
    type FromFieldsErrorReturn = IdentityMappingVectorFromFieldsErrorReturn;

    const FIELDS: &'static [CanonicalFieldKind] = &IDENTITY_MAPPING_VECTOR_FIELD_KINDS;

    fn to_fields(&self, _params: ToFieldsParams, payload: &Self::Described) -> ToFieldsReturn {
        Ok(ToFieldsSuccessReturn {
            fields: CanonicalFields {
                values: vec![
                    CanonicalFieldValue::Bytes(payload.identity.clone()),
                    CanonicalFieldValue::Unsigned256(payload.scalar),
                ],
            },
        })
    }

    fn fields_to_value(
        &self,
        _params: FromFieldsParams,
        payload: CanonicalFields,
    ) -> FromFieldsReturn<Self::Described, Self::FromFieldsErrorReturn> {
        let [identity, scalar] =
            match <[CanonicalFieldValue; IDENTITY_MAPPING_VECTOR_FIELD_COUNT]>::try_from(
                payload.values,
            ) {
                Err(values) => {
                    return Err(IdentityMappingVectorFromFieldsErrorReturn::FieldCount {
                        expected: IDENTITY_MAPPING_VECTOR_FIELD_COUNT,
                        actual: values.len(),
                    });
                }
                Ok(values) => values,
            };
        let CanonicalFieldValue::Bytes(identity) = identity else {
            return Err(IdentityMappingVectorFromFieldsErrorReturn::FieldKind {
                index: 0,
                expected: IDENTITY_MAPPING_VECTOR_FIELD_KINDS[0],
            });
        };
        let CanonicalFieldValue::Unsigned256(scalar) = scalar else {
            return Err(IdentityMappingVectorFromFieldsErrorReturn::FieldKind {
                index: 1,
                expected: IDENTITY_MAPPING_VECTOR_FIELD_KINDS[1],
            });
        };
        Ok(FromFieldsSuccessReturn {
            described: IdentityMappingVector { identity, scalar },
        })
    }
}

pub fn mapping_vectors<'a, P: IPairingArithmetic, K: ICredentialKemAdapter<Pairing = P>>(
    deps: &MappingVectorsDeps<'a, P, K>,
    params: MappingVectorsParams,
    payload: MappingVectorsPayload<'_>,
) -> MappingVectorsReturn {
    let scalar_bytes = |scalar: P::Scalar| -> Result<[u8; 32], MappingVectorsErrorReturn> {
        let Ok(encoded) = deps
            .pairing
            .encode_scalar(EncodeScalarParams, EncodeScalarPayload { scalar });
        let raw: &[u8] = encoded.bytes.expose().as_ref();
        match <[u8; 32]>::try_from(raw) {
            Err(_) => Err(MappingVectorsErrorReturn::ScalarLength { actual: raw.len() }),
            Ok(array) => Ok(array),
        }
    };
    let draw_uniform = || -> Result<Secret<Vec<u8>>, MappingVectorsErrorReturn> {
        match deps.random.fill_bytes(
            FillBytesParams,
            FillBytesPayload {
                length: <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH,
            },
        ) {
            Err(error) => Err(MappingVectorsErrorReturn::FillBytes(error)),
            Ok(success) => Ok(success.bytes),
        }
    };
    let mut tags = Vec::new();
    for declared in [
        K::DECLARATION.identity_tag,
        payload.key_agreement.possession_g1_tag,
        payload.key_agreement.possession_g2_tag,
        payload.delivery_proof.challenge_tag,
        payload.delivery_proof.weight_tag,
    ] {
        let bytes = declared.to_vec();
        match DomainTag::try_new(DomainTagConstructorParams {
            bytes: bytes.clone(),
        }) {
            Err(error) => return Err(MappingVectorsErrorReturn::DomainTag(error)),
            Ok(tag) => tags.push((bytes, tag)),
        }
    }
    let mut hash_to_scalar = Vec::new();
    for _ in 0..params.hash_to_scalar_count.get() {
        let message = match deps.random.fill_bytes(
            FillBytesParams,
            FillBytesPayload {
                length: params.message_length,
            },
        ) {
            Err(error) => return Err(MappingVectorsErrorReturn::FillBytes(error)),
            Ok(success) => success.bytes.expose().clone(),
        };
        for (bytes, tag) in &tags {
            let scalar = match deps.hash_to_scalar.hash_to_scalar(
                HashToScalarParams { tag },
                HashToScalarPayload { message: &message },
            ) {
                Err(error) => return Err(MappingVectorsErrorReturn::HashToScalar(error)),
                Ok(success) => success.scalar,
            };
            hash_to_scalar.push(HashToScalarVector {
                tag: bytes.clone(),
                message: message.clone(),
                scalar: scalar_bytes(scalar)?,
            });
        }
    }
    let mut identity_mapping = Vec::new();
    for identity in payload.identities {
        let master_uniform = draw_uniform()?;
        let u0_uniform = draw_uniform()?;
        let u1_uniform = draw_uniform()?;
        let set = match deps.kem.setup(
            SetupParams {
                scope: identity.setup_scope,
            },
            SetupPayload {
                master_uniform,
                u0_uniform,
                u1_uniform,
            },
        ) {
            Err(error) => return Err(MappingVectorsErrorReturn::Setup(error)),
            Ok(success) => success,
        };
        let derived = match deps.kem.derive_identity(
            DeriveIdentityParams,
            DeriveIdentityPayload {
                parameter_set: &set.parameter_set,
                identity: identity.identity,
            },
        ) {
            Err(error) => return Err(MappingVectorsErrorReturn::DeriveIdentity(error)),
            Ok(success) => success,
        };
        let record_bytes = match identity.identity {
            KemIdentity::Entitlement { canonical } => canonical.to_vec(),
            KemIdentity::Asset { root } => root.as_bytes().to_vec(),
        };
        let Ok(element) = deps.kem.identity_element_components(
            IdentityElementComponentsParams,
            IdentityElementComponentsPayload {
                identity_element: &derived.identity_element,
            },
        );
        identity_mapping.push(IdentityMappingVector {
            identity: record_bytes,
            scalar: scalar_bytes(element.components.scalar)?,
        });
    }
    Ok(MappingVectorsSuccessReturn {
        hash_to_scalar,
        identity_mapping,
    })
}
