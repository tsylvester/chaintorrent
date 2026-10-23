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

use kem::{
    DeriveIdentityParams, DeriveIdentityPayload, ICredentialKemAdapter, IdentityScope,
    IsValidParams, IsValidPayload, IssueParams, IssuePayload, KemIdentity, SetupParams,
    SetupPayload, SetupScope,
};
use pairing::{IPairingAdapter, ISampleUniformScalar};
use random::{FillBytesParams, FillBytesPayload};

use interface::{
    NonConvertibilityDeps, NonConvertibilityErrorReturn, NonConvertibilityParams,
    NonConvertibilityPayload, NonConvertibilityReturn, NonConvertibilitySuccessReturn,
    NonConvertibilityVector,
};

pub fn non_convertibility<'a, K: ICredentialKemAdapter>(
    deps: &NonConvertibilityDeps<'a, K>,
    params: NonConvertibilityParams,
    payload: NonConvertibilityPayload<'_>,
) -> NonConvertibilityReturn<K> {
    let record =
        |vectors: &mut Vec<NonConvertibilityVector<K>>| -> Result<(),
            NonConvertibilityErrorReturn> {
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
                    .map_err(NonConvertibilityErrorReturn::FillBytes)
            };
            let setup = deps
                .kem
                .setup(
                    SetupParams {
                        scope: SetupScope::Entitlement,
                    },
                    SetupPayload {
                        master_uniform: draw(&mut draws)?,
                        u0_uniform: draw(&mut draws)?,
                        u1_uniform: draw(&mut draws)?,
                    },
                )
                .map_err(NonConvertibilityErrorReturn::Setup)?;
            let derived = deps
                .kem
                .derive_identity(
                    DeriveIdentityParams,
                    DeriveIdentityPayload {
                        parameter_set: &setup.parameter_set,
                        identity: KemIdentity::Entitlement {
                            canonical: payload.identity,
                        },
                    },
                )
                .map_err(NonConvertibilityErrorReturn::DeriveIdentity)?;
            let other_derived = deps
                .kem
                .derive_identity(
                    DeriveIdentityParams,
                    DeriveIdentityPayload {
                        parameter_set: &setup.parameter_set,
                        identity: KemIdentity::Entitlement {
                            canonical: payload.other_identity,
                        },
                    },
                )
                .map_err(NonConvertibilityErrorReturn::DeriveOtherIdentity)?;
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
                .map_err(NonConvertibilityErrorReturn::Issue)?;
            let Ok(verdict) = deps.kem.is_valid(
                IsValidParams,
                IsValidPayload {
                    parameter_set: &setup.parameter_set,
                    identity_element: &derived.identity_element,
                    credential: &issued.credential,
                },
            );
            let Ok(other_verdict) = deps.kem.is_valid(
                IsValidParams,
                IsValidPayload {
                    parameter_set: &setup.parameter_set,
                    identity_element: &other_derived.identity_element,
                    credential: &issued.credential,
                },
            );
            vectors.push(NonConvertibilityVector {
                draws,
                credential: issued.credential,
                is_valid: verdict.is_valid,
                other_is_valid: other_verdict.is_valid,
            });
            Ok(())
        };
    let mut vectors = Vec::new();
    match params.scope {
        IdentityScope::Entitlement => {
            for _ in 0..params.count.get() {
                record(&mut vectors)?;
            }
        }
        IdentityScope::Asset => {}
    }
    Ok(NonConvertibilitySuccessReturn { vectors })
}
