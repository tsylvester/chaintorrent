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
    DecapsulateParams, DecapsulatePayload, DeriveIdentityParams, DeriveIdentityPayload,
    EncapsulateParams, EncapsulatePayload, ICredentialKemAdapter, IssueParams, IssuePayload,
    RerandomizeParams, RerandomizePayload, SetupParams, SetupPayload,
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
    let record = |vectors: &mut Vec<CrossHolderVector<K>>| -> Result<(), CrossHolderErrorReturn> {
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
                SetupParams {
                    scope: payload.identity.setup_scope,
                },
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
                    identity: payload.identity.identity,
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
    for _ in 0..params.count.get() {
        record(&mut vectors)?;
    }
    Ok(CrossHolderSuccessReturn { vectors })
}
