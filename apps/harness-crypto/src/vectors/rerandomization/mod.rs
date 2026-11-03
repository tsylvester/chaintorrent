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
    DeriveIdentityParams, DeriveIdentityPayload, ICredentialKemAdapter, IsValidParams,
    IsValidPayload, IssueParams, IssuePayload, RerandomizeParams, RerandomizePayload, SetupParams,
    SetupPayload,
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
    let record =
        |vectors: &mut Vec<RerandomizationVector<K>>| -> Result<(), RerandomizationErrorReturn> {
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
                    SetupParams {
                        scope: payload.identity.setup_scope,
                    },
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
                        identity: payload.identity.identity,
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
    for _ in 0..params.count.get() {
        record(&mut vectors)?;
    }
    Ok(RerandomizationSuccessReturn { vectors })
}
