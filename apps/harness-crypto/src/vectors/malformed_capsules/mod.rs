#[cfg(test)]
mod integration_test;
mod interface;
#[cfg(any(test, feature = "mocks"))]
mod mock;
pub mod provides;
#[cfg(test)]
mod test_basic;
#[cfg(test)]
mod test_error;

use domain::{ASSET_IDENTITY_HASH_LENGTH, AssetIdentityHash, AssetIdentityHashConstructorParams};
use kem::{
    CapsuleComponents, CapsuleComponentsParams, CapsuleComponentsPayload,
    CapsuleFromComponentsParams, CapsuleFromComponentsPayload, EncapsulateParams,
    EncapsulatePayload, ICredentialKemAdapter, IdentityScope, IsWellFormedParams,
    IsWellFormedPayload, ParameterSetComponentsParams, ParameterSetComponentsPayload, SetupParams,
    SetupPayload, SetupScope,
};
use pairing::{IPairingAdapter, ISampleUniformScalar};
use random::{FillBytesParams, FillBytesPayload};

use interface::{
    MalformedCapsulesDeps, MalformedCapsulesErrorReturn, MalformedCapsulesParams,
    MalformedCapsulesPayload, MalformedCapsulesReturn, MalformedCapsulesSuccessReturn,
    MalformedCapsulesVector,
};

pub fn malformed_capsules<'a, K: ICredentialKemAdapter>(
    deps: &MalformedCapsulesDeps<'a, K>,
    params: MalformedCapsulesParams,
    payload: MalformedCapsulesPayload<'_>,
) -> MalformedCapsulesReturn<K> {
    let record = |vectors: &mut Vec<MalformedCapsulesVector<K>>,
                  scope: SetupScope<'_>|
     -> Result<(), MalformedCapsulesErrorReturn> {
        let mut draws = Vec::new();
        let draw = |draws: &mut Vec<u8>| {
            deps.random
                .fill_bytes(
                    FillBytesParams,
                    FillBytesPayload {
                        length:
                            <<K::Pairing as IPairingAdapter>::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH,
                    },
                )
                .map(|success| {
                    draws.extend_from_slice(success.bytes.expose());
                    success.bytes
                })
                .map_err(MalformedCapsulesErrorReturn::FillBytes)
        };
        let setup = deps
            .kem
            .setup(
                SetupParams { scope },
                SetupPayload {
                    master_uniform: draw(&mut draws)?,
                    u0_uniform: draw(&mut draws)?,
                    u1_uniform: draw(&mut draws)?,
                },
            )
            .map_err(MalformedCapsulesErrorReturn::Setup)?;
        let encapsulated = deps
            .kem
            .encapsulate(
                EncapsulateParams,
                EncapsulatePayload {
                    parameter_set: &setup.parameter_set,
                    uniform: draw(&mut draws)?,
                },
            )
            .map_err(MalformedCapsulesErrorReturn::Encapsulate)?;
        let Ok(parameter_set_components) = deps.kem.parameter_set_components(
            ParameterSetComponentsParams,
            ParameterSetComponentsPayload {
                parameter_set: &setup.parameter_set,
            },
        );
        let Ok(capsule_components) = deps.kem.capsule_components(
            CapsuleComponentsParams,
            CapsuleComponentsPayload {
                capsule: &encapsulated.capsule,
            },
        );
        let g1 = parameter_set_components.components.g1;
        let components = match capsule_components.components {
            CapsuleComponents::Entitlement { u, v: _, w } => {
                CapsuleComponents::Entitlement { u, v: g1, w }
            }
            CapsuleComponents::Asset { u, v: _ } => CapsuleComponents::Asset { u, v: g1 },
        };
        let Ok(displaced) = deps
            .kem
            .capsule_from_components(
                CapsuleFromComponentsParams,
                CapsuleFromComponentsPayload { components },
            )
            .map(|success| success.capsule);
        let Ok(verdict) = deps.kem.is_well_formed(
            IsWellFormedParams,
            IsWellFormedPayload {
                parameter_set: &setup.parameter_set,
                capsule: &encapsulated.capsule,
            },
        );
        let Ok(displaced_verdict) = deps.kem.is_well_formed(
            IsWellFormedParams,
            IsWellFormedPayload {
                parameter_set: &setup.parameter_set,
                capsule: &displaced,
            },
        );
        vectors.push(MalformedCapsulesVector {
            draws,
            parameter_set: setup.parameter_set,
            capsule: encapsulated.capsule,
            displaced,
            is_well_formed: verdict.is_well_formed,
            displaced_is_well_formed: displaced_verdict.is_well_formed,
        });
        Ok(())
    };
    let mut vectors = Vec::new();
    match params.scope {
        IdentityScope::Entitlement => {
            for _ in 0..params.count.get() {
                record(&mut vectors, SetupScope::Entitlement)?;
            }
        }
        IdentityScope::Asset => {
            let Ok(bytes) = <[u8; ASSET_IDENTITY_HASH_LENGTH]>::try_from(payload.identity) else {
                return Err(MalformedCapsulesErrorReturn::AssetIdentityLength {
                    actual: payload.identity.len(),
                });
            };
            let hash =
                match AssetIdentityHash::try_new(AssetIdentityHashConstructorParams { bytes }) {
                    Ok(hash) => hash,
                    Err(error) => {
                        return Err(MalformedCapsulesErrorReturn::AssetIdentityHash(error));
                    }
                };
            for _ in 0..params.count.get() {
                record(
                    &mut vectors,
                    SetupScope::Asset {
                        identity_hash: &hash,
                    },
                )?;
            }
        }
    }
    Ok(MalformedCapsulesSuccessReturn { vectors })
}
