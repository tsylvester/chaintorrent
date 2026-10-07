mod interface;
#[cfg(any(test, feature = "mocks"))]
mod mock;
pub mod provides;
#[cfg(test)]
mod test;

use domain::{ASSET_IDENTITY_HASH_LENGTH, AssetIdentityHash, AssetIdentityHashConstructorParams};
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
    IdentityElementComponentsParams, IdentityElementComponentsPayload, IdentityScope, KemIdentity,
    SetupParams, SetupPayload, SetupScope,
};
use pairing::{EncodeScalarParams, EncodeScalarPayload, IPairingArithmetic, ISampleUniformScalar};
use random::{FillBytesParams, FillBytesPayload};

use super::render::provides::{
    SolidityMemberName, SolidityMemberNameConstructorParams, SolidityRecordMember,
    SolidityRecordMembers, SolidityRecordMembersConstructorParams,
};
use interface::{
    HASH_TO_SCALAR_VECTOR_FIELD_COUNT, HASH_TO_SCALAR_VECTOR_FIELD_KINDS,
    HASH_TO_SCALAR_VECTOR_MEMBER_NAMES, HashToScalarVector, HashToScalarVectorDescription,
    HashToScalarVectorDescriptionConstructorParams, HashToScalarVectorDescriptionTryNewReturn,
    HashToScalarVectorFromFieldsErrorReturn, IDENTITY_MAPPING_VECTOR_FIELD_COUNT,
    IDENTITY_MAPPING_VECTOR_FIELD_KINDS, IDENTITY_MAPPING_VECTOR_MEMBER_NAMES,
    IdentityMappingVector, IdentityMappingVectorDescription,
    IdentityMappingVectorDescriptionConstructorParams,
    IdentityMappingVectorDescriptionTryNewReturn, IdentityMappingVectorFromFieldsErrorReturn,
    MappingVectorMembersErrorReturn, MappingVectorMembersReturn, MappingVectorsDeps,
    MappingVectorsErrorReturn, MappingVectorsParams, MappingVectorsPayload, MappingVectorsReturn,
    MappingVectorsSuccessReturn,
};

impl HashToScalarVectorDescription {
    pub fn try_new(
        _params: HashToScalarVectorDescriptionConstructorParams,
    ) -> HashToScalarVectorDescriptionTryNewReturn {
        Ok(HashToScalarVectorDescription)
    }

    pub fn members(&self) -> MappingVectorMembersReturn {
        let mut members = Vec::new();
        for (index, text) in HASH_TO_SCALAR_VECTOR_MEMBER_NAMES.iter().enumerate() {
            let name = SolidityMemberName::try_new(SolidityMemberNameConstructorParams {
                text: text.to_string(),
            })
            .map_err(MappingVectorMembersErrorReturn::MemberName)?;
            members.push(SolidityRecordMember {
                name,
                kind: HASH_TO_SCALAR_VECTOR_FIELD_KINDS[index],
            });
        }
        SolidityRecordMembers::try_new(SolidityRecordMembersConstructorParams { members })
            .map_err(MappingVectorMembersErrorReturn::RecordMembers)
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
        let actual = payload.values.len();
        let Ok(fields) =
            <[CanonicalFieldValue; HASH_TO_SCALAR_VECTOR_FIELD_COUNT]>::try_from(payload.values)
        else {
            return Err(HashToScalarVectorFromFieldsErrorReturn::FieldCount {
                expected: HASH_TO_SCALAR_VECTOR_FIELD_COUNT,
                actual,
            });
        };
        let [tag, message, scalar] = fields;
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

impl IdentityMappingVectorDescription {
    pub fn try_new(
        _params: IdentityMappingVectorDescriptionConstructorParams,
    ) -> IdentityMappingVectorDescriptionTryNewReturn {
        Ok(IdentityMappingVectorDescription)
    }

    pub fn members(&self) -> MappingVectorMembersReturn {
        let mut members = Vec::new();
        for (index, text) in IDENTITY_MAPPING_VECTOR_MEMBER_NAMES.iter().enumerate() {
            let name = SolidityMemberName::try_new(SolidityMemberNameConstructorParams {
                text: text.to_string(),
            })
            .map_err(MappingVectorMembersErrorReturn::MemberName)?;
            members.push(SolidityRecordMember {
                name,
                kind: IDENTITY_MAPPING_VECTOR_FIELD_KINDS[index],
            });
        }
        SolidityRecordMembers::try_new(SolidityRecordMembersConstructorParams { members })
            .map_err(MappingVectorMembersErrorReturn::RecordMembers)
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
        let actual = payload.values.len();
        let Ok(fields) =
            <[CanonicalFieldValue; IDENTITY_MAPPING_VECTOR_FIELD_COUNT]>::try_from(payload.values)
        else {
            return Err(IdentityMappingVectorFromFieldsErrorReturn::FieldCount {
                expected: IDENTITY_MAPPING_VECTOR_FIELD_COUNT,
                actual,
            });
        };
        let [identity, scalar] = fields;
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
    let mut tags = Vec::new();
    for bytes in [
        K::DECLARATION.identity_tag,
        payload.key_agreement.possession_g1_tag,
        payload.key_agreement.possession_g2_tag,
        payload.delivery_proof.challenge_tag,
        payload.delivery_proof.weight_tag,
    ] {
        let tag = DomainTag::try_new(DomainTagConstructorParams {
            bytes: bytes.to_vec(),
        })
        .map_err(MappingVectorsErrorReturn::DomainTag)?;
        tags.push((bytes, tag));
    }

    let scalar_bytes = |scalar: P::Scalar| -> Result<[u8; 32], MappingVectorsErrorReturn> {
        let Ok(encoded) = deps
            .pairing
            .encode_scalar(EncodeScalarParams, EncodeScalarPayload { scalar });
        let actual = encoded.bytes.expose().as_ref().len();
        let Ok(bytes) = <[u8; 32]>::try_from(encoded.bytes.expose().as_ref()) else {
            return Err(MappingVectorsErrorReturn::ScalarLength { actual });
        };
        Ok(bytes)
    };

    let draw_uniform = || -> Result<domain::Secret<Vec<u8>>, MappingVectorsErrorReturn> {
        deps.random
            .fill_bytes(
                FillBytesParams,
                FillBytesPayload {
                    length: <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH,
                },
            )
            .map(|success| success.bytes)
            .map_err(MappingVectorsErrorReturn::FillBytes)
    };

    let mut hash_to_scalar = Vec::new();
    for _ in 0..params.hash_to_scalar_count.get() {
        let filled = deps
            .random
            .fill_bytes(
                FillBytesParams,
                FillBytesPayload {
                    length: params.message_length,
                },
            )
            .map_err(MappingVectorsErrorReturn::FillBytes)?;
        let message = filled.bytes.expose().clone();
        for (bytes, tag) in &tags {
            let hashed = deps.hash_to_scalar.hash_to_scalar(
                HashToScalarParams { tag },
                HashToScalarPayload { message: &message },
            );
            let hashed = hashed.map_err(MappingVectorsErrorReturn::HashToScalar)?;
            hash_to_scalar.push(HashToScalarVector {
                tag: bytes.to_vec(),
                message: message.clone(),
                scalar: scalar_bytes(hashed.scalar)?,
            });
        }
    }

    let mut identity_mapping = Vec::new();
    for (index, bytes) in payload.identities.iter().enumerate() {
        let identity_element = match params.scope {
            IdentityScope::Entitlement => {
                let setup = deps
                    .kem
                    .setup(
                        SetupParams {
                            scope: SetupScope::Entitlement,
                        },
                        SetupPayload {
                            master_uniform: draw_uniform()?,
                            u0_uniform: draw_uniform()?,
                            u1_uniform: draw_uniform()?,
                        },
                    )
                    .map_err(MappingVectorsErrorReturn::Setup)?;
                deps.kem
                    .derive_identity(
                        DeriveIdentityParams,
                        DeriveIdentityPayload {
                            parameter_set: &setup.parameter_set,
                            identity: KemIdentity::Entitlement { canonical: bytes },
                        },
                    )
                    .map_err(MappingVectorsErrorReturn::DeriveIdentity)?
                    .identity_element
            }
            IdentityScope::Asset => {
                let actual = bytes.len();
                let Ok(array) = <[u8; ASSET_IDENTITY_HASH_LENGTH]>::try_from(bytes.as_slice())
                else {
                    return Err(MappingVectorsErrorReturn::AssetIdentityLength { index, actual });
                };
                let hash =
                    AssetIdentityHash::try_new(AssetIdentityHashConstructorParams { bytes: array })
                        .map_err(MappingVectorsErrorReturn::AssetIdentityHash)?;
                let setup = deps
                    .kem
                    .setup(
                        SetupParams {
                            scope: SetupScope::Asset {
                                identity_hash: &hash,
                            },
                        },
                        SetupPayload {
                            master_uniform: draw_uniform()?,
                            u0_uniform: draw_uniform()?,
                            u1_uniform: draw_uniform()?,
                        },
                    )
                    .map_err(MappingVectorsErrorReturn::Setup)?;
                deps.kem
                    .derive_identity(
                        DeriveIdentityParams,
                        DeriveIdentityPayload {
                            parameter_set: &setup.parameter_set,
                            identity: KemIdentity::Asset {
                                identity_hash: &hash,
                            },
                        },
                    )
                    .map_err(MappingVectorsErrorReturn::DeriveIdentity)?
                    .identity_element
            }
        };
        let Ok(components) = deps.kem.identity_element_components(
            IdentityElementComponentsParams,
            IdentityElementComponentsPayload {
                identity_element: &identity_element,
            },
        );
        identity_mapping.push(IdentityMappingVector {
            identity: bytes.clone(),
            scalar: scalar_bytes(components.components.scalar)?,
        });
    }

    Ok(MappingVectorsSuccessReturn {
        hash_to_scalar,
        identity_mapping,
    })
}
