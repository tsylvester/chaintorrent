mod cross_holder;
mod cross_set;
#[cfg(test)]
mod integration_test;
mod interface;
mod malformed_capsules;
#[cfg(any(test, feature = "mocks"))]
mod mock;
mod non_convertibility;
pub mod provides;
mod replay;
mod rerandomization;
mod statement_mutations;
#[cfg(test)]
mod test_basic;
#[cfg(test)]
mod test_error;

use encoding::IEncoderAdapter;
use envelope::IKeyAgreementAdapter;
use kem::ICredentialKemAdapter;
use pairing::IPairingArithmetic;
use proof::IDeliveryProofAdapter;

use self::cross_holder::provides::{
    CrossHolderDeps, CrossHolderParams, CrossHolderPayload, cross_holder,
};
use self::cross_set::provides::{CrossSetDeps, CrossSetParams, CrossSetPayload, cross_set};
use self::interface::{
    VectorsDeps, VectorsErrorReturn, VectorsFn, VectorsParams, VectorsPayload, VectorsReturn,
    VectorsSuccessReturn,
};
use self::malformed_capsules::provides::{
    MalformedCapsulesDeps, MalformedCapsulesParams, MalformedCapsulesPayload, malformed_capsules,
};
use self::non_convertibility::provides::{
    NonConvertibilityDeps, NonConvertibilityParams, NonConvertibilityPayload, non_convertibility,
};
use self::replay::provides::{ReplayDeps, ReplayParams, ReplayPayload, replay};
use self::rerandomization::provides::{
    RerandomizationDeps, RerandomizationParams, RerandomizationPayload, rerandomization,
};
use self::statement_mutations::provides::{
    StatementMutationsDeps, StatementMutationsParams, StatementMutationsPayload,
    statement_mutations,
};

pub fn vectors<
    'a,
    P: IPairingArithmetic,
    K: ICredentialKemAdapter<Pairing = P>,
    A: IKeyAgreementAdapter<Pairing = P>,
    D: IDeliveryProofAdapter<Pairing = P>,
    E: IEncoderAdapter,
>(
    deps: &VectorsDeps<'a, P, K, A, D, E>,
    params: VectorsParams,
    payload: VectorsPayload<'_, D::Forms>,
) -> VectorsReturn<P, K, D> {
    let _: VectorsFn<'a, P, K, A, D, E> = vectors;
    let cross_holder = cross_holder::<K>(
        &CrossHolderDeps {
            kem: deps.kem,
            random: deps.random,
        },
        CrossHolderParams {
            count: params.cross_holder,
        },
        CrossHolderPayload {
            identity: payload.identity,
        },
    )
    .map_err(VectorsErrorReturn::CrossHolder)?;
    let rerandomization = rerandomization::<K>(
        &RerandomizationDeps {
            kem: deps.kem,
            random: deps.random,
        },
        RerandomizationParams {
            count: params.rerandomization,
        },
        RerandomizationPayload {
            identity: payload.identity,
        },
    )
    .map_err(VectorsErrorReturn::Rerandomization)?;
    let non_convertibility = non_convertibility::<K>(
        &NonConvertibilityDeps {
            kem: deps.kem,
            random: deps.random,
        },
        NonConvertibilityParams {
            count: params.non_convertibility,
        },
        NonConvertibilityPayload {
            identity: payload.identity,
            other_identity: payload.other_identity,
        },
    )
    .map_err(VectorsErrorReturn::NonConvertibility)?;
    let malformed_capsules = malformed_capsules::<K>(
        &MalformedCapsulesDeps {
            kem: deps.kem,
            random: deps.random,
        },
        MalformedCapsulesParams {
            count: params.malformed_capsules,
        },
        MalformedCapsulesPayload {
            identity: payload.identity,
        },
    )
    .map_err(VectorsErrorReturn::MalformedCapsules)?;
    let cross_set = cross_set::<K, E>(
        &CrossSetDeps {
            kem: deps.kem,
            random: deps.random,
            kdf: deps.kdf,
            encoder: deps.encoder,
        },
        CrossSetParams {
            count: params.cross_set,
        },
        CrossSetPayload {
            identity: payload.identity,
            context: payload.context,
        },
    )
    .map_err(VectorsErrorReturn::CrossSet)?;
    let statement_mutations = statement_mutations::<P, K, A, D>(
        &StatementMutationsDeps {
            pairing: deps.pairing,
            kem: deps.kem,
            key_agreement: deps.key_agreement,
            delivery_proof: deps.delivery_proof,
            random: deps.random,
        },
        StatementMutationsParams {
            count: params.statement_mutations,
        },
        StatementMutationsPayload {
            mint: payload.mint,
            transfer: payload.transfer,
            identity: payload.identity,
        },
    )
    .map_err(VectorsErrorReturn::StatementMutations)?;
    let replay = replay::<P, K, A, D>(
        &ReplayDeps {
            kem: deps.kem,
            key_agreement: deps.key_agreement,
            delivery_proof: deps.delivery_proof,
            random: deps.random,
        },
        ReplayParams {
            count: params.replay,
        },
        ReplayPayload {
            mint: payload.mint,
            transfer: payload.transfer,
            next: payload.next,
            identity: payload.identity,
        },
    )
    .map_err(VectorsErrorReturn::Replay)?;
    Ok(VectorsSuccessReturn {
        cross_holder,
        rerandomization,
        non_convertibility,
        malformed_capsules,
        cross_set,
        statement_mutations,
        replay,
    })
}
