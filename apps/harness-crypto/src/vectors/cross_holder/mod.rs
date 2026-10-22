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
    DecapsulateParams, DecapsulatePayload, DeriveIdentityParams, DeriveIdentityPayload,
    EncapsulateParams, EncapsulatePayload, ICredentialKemAdapter, IdentityScope, IssueParams,
    IssuePayload, KemIdentity, RerandomizeParams, RerandomizePayload, SetupParams, SetupPayload,
    SetupScope,
};
use pairing::{IPairingAdapter, ISampleUniformScalar};
use random::{FillBytesParams, FillBytesPayload};

use interface::{
    CrossHolderDeps, CrossHolderErrorReturn, CrossHolderParams, CrossHolderPayload,
    CrossHolderReturn, CrossHolderSuccessReturn, CrossHolderVector,
};

pub fn cross_holder<'a, K: ICredentialKemAdapter>(
    deps: &CrossHolderDeps<'a, K>,
    params: CrossHolderParams,
    payload: CrossHolderPayload<'_>,
) -> CrossHolderReturn<K> {
    let record = |vectors: &mut Vec<CrossHolderVector<K>>,
                  identity: KemIdentity<'_>,
                  scope: SetupScope<'_>|
     -> Result<(), CrossHolderErrorReturn> {
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
                .map_err(CrossHolderErrorReturn::FillBytes)
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
            .map_err(CrossHolderErrorReturn::Setup)?;
        let derived = deps
            .kem
            .derive_identity(
                DeriveIdentityParams,
                DeriveIdentityPayload {
                    parameter_set: &setup.parameter_set,
                    identity,
                },
            )
            .map_err(CrossHolderErrorReturn::DeriveIdentity)?;
        let issued = deps
            .kem
            .issue(
                IssueParams,
                IssuePayload {
                    parameter_set: &setup.parameter_set,
                    master_scalar: &setup.master_scalar,
                    identity_element: &derived.identity_element,
                    uniform: draw(&mut draws)?,
                },
            )
            .map_err(CrossHolderErrorReturn::Issue)?;
        let rerandomized = deps
            .kem
            .rerandomize(
                RerandomizeParams,
                RerandomizePayload {
                    parameter_set: &setup.parameter_set,
                    identity_element: &derived.identity_element,
                    credential: &issued.credential,
                    uniform: draw(&mut draws)?,
                },
            )
            .map_err(CrossHolderErrorReturn::Rerandomize)?;
        let encapsulated = deps
            .kem
            .encapsulate(
                EncapsulateParams,
                EncapsulatePayload {
                    parameter_set: &setup.parameter_set,
                    uniform: draw(&mut draws)?,
                },
            )
            .map_err(CrossHolderErrorReturn::Encapsulate)?;
        let Ok(issued_decapsulated) = deps.kem.decapsulate(
            DecapsulateParams,
            DecapsulatePayload {
                identity_element: &derived.identity_element,
                credential: &issued.credential,
                capsule: &encapsulated.capsule,
            },
        );
        let Ok(successor_decapsulated) = deps.kem.decapsulate(
            DecapsulateParams,
            DecapsulatePayload {
                identity_element: &derived.identity_element,
                credential: &rerandomized.credential,
                capsule: &encapsulated.capsule,
            },
        );
        vectors.push(CrossHolderVector {
            draws,
            parameter_set: setup.parameter_set,
            identity_element: derived.identity_element,
            issued: issued.credential,
            successor: rerandomized.credential,
            capsule: encapsulated.capsule,
            encapsulated: encapsulated.encapsulated,
            issued_decapsulated: issued_decapsulated.encapsulated,
            successor_decapsulated: successor_decapsulated.encapsulated,
        });
        Ok(())
    };
    let mut vectors = Vec::new();
    match params.scope {
        IdentityScope::Entitlement => {
            for _ in 0..params.count.get() {
                record(
                    &mut vectors,
                    KemIdentity::Entitlement {
                        canonical: payload.identity,
                    },
                    SetupScope::Entitlement,
                )?;
            }
        }
        IdentityScope::Asset => {
            let Ok(bytes) = <[u8; ASSET_IDENTITY_HASH_LENGTH]>::try_from(payload.identity) else {
                return Err(CrossHolderErrorReturn::AssetIdentityLength {
                    actual: payload.identity.len(),
                });
            };
            let hash =
                match AssetIdentityHash::try_new(AssetIdentityHashConstructorParams { bytes }) {
                    Ok(hash) => hash,
                    Err(error) => {
                        return Err(CrossHolderErrorReturn::AssetIdentityHash(error));
                    }
                };
            for _ in 0..params.count.get() {
                record(
                    &mut vectors,
                    KemIdentity::Asset {
                        identity_hash: &hash,
                    },
                    SetupScope::Asset {
                        identity_hash: &hash,
                    },
                )?;
            }
        }
    }
    Ok(CrossHolderSuccessReturn { vectors })
}
