#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::as_conversions
)]

use super::super::render::provides::{SolidityLibraryEntry, build_solidity_library_entry};
use super::interface::{
    ConstantsDeps, ConstantsFn, ConstantsParams, ConstantsPayload, ConstantsReturn,
    ConstantsSuccessReturn,
};
use envelope::KeyAgreementDeclaration;
use kem::KemDeclaration;
use pairing::IPairingReference;
use proof::{DELIVERY_STATEMENT_VERSION_ONE, DeliveryProofDeclaration};

#[derive(Default)]
pub struct ConstantsDepsOverrides;

pub fn build_constants_deps<'a, P: IPairingReference>(
    pairing: &'a P,
    _overrides: ConstantsDepsOverrides,
) -> ConstantsDeps<'a, P> {
    ConstantsDeps { pairing }
}

#[derive(Default)]
pub struct ConstantsParamsOverrides {
    pub statement_version: Option<u16>,
}

pub fn build_constants_params(overrides: ConstantsParamsOverrides) -> ConstantsParams {
    ConstantsParams {
        statement_version: overrides
            .statement_version
            .unwrap_or(DELIVERY_STATEMENT_VERSION_ONE),
    }
}

#[derive(Default)]
pub struct ConstantsPayloadOverrides;

pub fn build_constants_payload<'a>(
    kem: &'a KemDeclaration,
    key_agreement: &'a KeyAgreementDeclaration,
    delivery_proof: &'a DeliveryProofDeclaration,
    _overrides: ConstantsPayloadOverrides,
) -> ConstantsPayload<'a> {
    ConstantsPayload {
        kem,
        key_agreement,
        delivery_proof,
    }
}

#[derive(Default)]
pub struct ConstantsSuccessReturnOverrides {
    pub entries: Option<Vec<SolidityLibraryEntry>>,
}

pub fn build_constants_success_return(
    overrides: ConstantsSuccessReturnOverrides,
) -> ConstantsSuccessReturn {
    ConstantsSuccessReturn {
        entries: overrides
            .entries
            .unwrap_or_else(|| vec![build_solidity_library_entry(Default::default())]),
    }
}

pub fn mock_constants<'a, P: IPairingReference>(
    _deps: &ConstantsDeps<'a, P>,
    _params: ConstantsParams,
    _payload: ConstantsPayload<'_>,
) -> ConstantsReturn {
    let _: ConstantsFn<'a, P> = mock_constants;
    Ok(build_constants_success_return(Default::default()))
}
