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
    DeriveIdentityParams, DeriveIdentityPayload, ICredentialKemAdapter, IdentityScope,
    IsValidParams, IsValidPayload, IssueParams, IssuePayload, KemIdentity, RerandomizeParams,
    RerandomizePayload, SetupParams, SetupPayload, SetupScope,
};
use pairing::{IPairingAdapter, ISampleUniformScalar};
use random::{FillBytesParams, FillBytesPayload};

use interface::{
    RerandomizationDeps, RerandomizationErrorReturn, RerandomizationParams, RerandomizationPayload,
    RerandomizationReturn, RerandomizationSuccessReturn, RerandomizationVector,
};

pub fn rerandomization<'a, K: ICredentialKemAdapter>(
    deps: &RerandomizationDeps<'a, K>,
    params: RerandomizationParams,
    payload: RerandomizationPayload<'_>,
) -> RerandomizationReturn<K> {
    let record = |vectors: &mut Vec<RerandomizationVector<K>>,
                  identity: KemIdentity<'_>,
                  scope: SetupScope<'_>|
     -> Result<(), RerandomizationErrorReturn> {
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
                .map_err(RerandomizationErrorReturn::FillBytes)
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
            .map_err(RerandomizationErrorReturn::Setup)?;
        let derived = deps
            .kem
            .derive_identity(
                DeriveIdentityParams,
                DeriveIdentityPayload {
                    parameter_set: &setup.parameter_set,
                    identity,
                },
            )
            .map_err(RerandomizationErrorReturn::DeriveIdentity)?;
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
            .map_err(RerandomizationErrorReturn::Issue)?;
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
            .map_err(RerandomizationErrorReturn::Rerandomize)?;
        let Ok(issued_verdict) = deps.kem.is_valid(
            IsValidParams,
            IsValidPayload {
                parameter_set: &setup.parameter_set,
                identity_element: &derived.identity_element,
                credential: &issued.credential,
            },
        );
        let Ok(rerandomized_verdict) = deps.kem.is_valid(
            IsValidParams,
            IsValidPayload {
                parameter_set: &setup.parameter_set,
                identity_element: &derived.identity_element,
                credential: &rerandomized.credential,
            },
        );
        vectors.push(RerandomizationVector {
            draws,
            parameter_set: setup.parameter_set,
            identity_element: derived.identity_element,
            issued: issued.credential,
            rerandomized: rerandomized.credential,
            issued_is_valid: issued_verdict.is_valid,
            rerandomized_is_valid: rerandomized_verdict.is_valid,
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
                return Err(RerandomizationErrorReturn::AssetIdentityLength {
                    actual: payload.identity.len(),
                });
            };
            let hash =
                match AssetIdentityHash::try_new(AssetIdentityHashConstructorParams { bytes }) {
                    Ok(hash) => hash,
                    Err(error) => {
                        return Err(RerandomizationErrorReturn::AssetIdentityHash(error));
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
    Ok(RerandomizationSuccessReturn { vectors })
}
