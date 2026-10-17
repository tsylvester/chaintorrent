#![allow(
    dead_code,
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::too_many_arguments
)]

use envelope::IKeyAgreementAdapter;
use pairing::IPairingArithmetic;
use random::IRandomSourceAdapter;

use super::super::group_vectors::provides::{VectorCount, build_vector_count};
use super::interface::{
    PossessionVector, PossessionVectorDescription, PossessionVectorDescriptionConstructorParams,
    PossessionVectorsDeps, PossessionVectorsFn, PossessionVectorsParams, PossessionVectorsPayload,
    PossessionVectorsReturn, PossessionVectorsSuccessReturn,
};

#[derive(Default)]
pub struct PossessionVectorOverrides {
    pub x: Option<[u8; 32]>,
    pub y: Option<[u8; 32]>,
    pub pk1: Option<Vec<u8>>,
    pub pk2: Option<Vec<u8>>,
    pub r1: Option<Vec<u8>>,
    pub z1: Option<[u8; 32]>,
    pub r2: Option<Vec<u8>>,
    pub z2: Option<[u8; 32]>,
}

pub fn build_possession_vector(overrides: PossessionVectorOverrides) -> PossessionVector {
    PossessionVector {
        x: overrides.x.unwrap_or([0x01; 32]),
        y: overrides.y.unwrap_or([0x02; 32]),
        pk1: overrides.pk1.unwrap_or_else(|| vec![0x03; 64]),
        pk2: overrides.pk2.unwrap_or_else(|| vec![0x04; 128]),
        r1: overrides.r1.unwrap_or_else(|| vec![0x05; 64]),
        z1: overrides.z1.unwrap_or([0x06; 32]),
        r2: overrides.r2.unwrap_or_else(|| vec![0x07; 128]),
        z2: overrides.z2.unwrap_or([0x08; 32]),
    }
}

#[derive(Default)]
pub struct PossessionVectorsParamsOverrides {
    pub count: Option<VectorCount>,
}

pub fn build_possession_vectors_params(
    overrides: PossessionVectorsParamsOverrides,
) -> PossessionVectorsParams {
    PossessionVectorsParams {
        count: overrides
            .count
            .unwrap_or_else(|| build_vector_count(Default::default())),
    }
}

#[derive(Default)]
pub struct PossessionVectorDescriptionConstructorParamsOverrides;

pub fn build_possession_vector_description_constructor_params(
    _overrides: PossessionVectorDescriptionConstructorParamsOverrides,
) -> PossessionVectorDescriptionConstructorParams {
    PossessionVectorDescriptionConstructorParams
}

pub fn build_possession_vector_description(
    overrides: PossessionVectorDescriptionConstructorParamsOverrides,
) -> PossessionVectorDescription {
    PossessionVectorDescription::try_new(build_possession_vector_description_constructor_params(
        overrides,
    ))
    .expect("built params are valid")
}

#[derive(Default)]
pub struct PossessionVectorsPayloadOverrides;

pub fn build_possession_vectors_payload(
    _overrides: PossessionVectorsPayloadOverrides,
) -> PossessionVectorsPayload {
    PossessionVectorsPayload
}

#[derive(Default)]
pub struct PossessionVectorsDepsOverrides;

pub fn build_possession_vectors_deps<
    'a,
    P: IPairingArithmetic,
    K: IKeyAgreementAdapter<Pairing = P>,
>(
    pairing: &'a P,
    key_agreement: &'a K,
    random: &'a dyn IRandomSourceAdapter,
    _overrides: PossessionVectorsDepsOverrides,
) -> PossessionVectorsDeps<'a, P, K> {
    PossessionVectorsDeps {
        pairing,
        key_agreement,
        random,
    }
}

#[derive(Default)]
pub struct PossessionVectorsSuccessReturnOverrides {
    pub records: Option<Vec<PossessionVector>>,
}

pub fn build_possession_vectors_success_return(
    overrides: PossessionVectorsSuccessReturnOverrides,
) -> PossessionVectorsSuccessReturn {
    PossessionVectorsSuccessReturn {
        records: overrides
            .records
            .unwrap_or_else(|| vec![build_possession_vector(Default::default())]),
    }
}

pub fn mock_possession_vectors<'a, P: IPairingArithmetic, K: IKeyAgreementAdapter<Pairing = P>>(
    _deps: &PossessionVectorsDeps<'a, P, K>,
    _params: PossessionVectorsParams,
    _payload: PossessionVectorsPayload,
) -> PossessionVectorsReturn {
    let _: PossessionVectorsFn<'a, P, K> = mock_possession_vectors;
    Ok(build_possession_vectors_success_return(Default::default()))
}
