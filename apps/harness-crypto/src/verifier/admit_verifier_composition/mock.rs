#![allow(
    dead_code,
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::too_many_arguments
)]

use chain::{ChainDeclaration, ChainFormsDeclaration, ChainMemberIdentifier};
use encoding::EncodingDeclaration;
use pairing::{PairingDeclaration, VerifierGroupArithmetic};

use crate::generate::provides::GenerateDeclaration;

use super::super::factory::provides::VerifierConcreteKind;
use super::super::lookup_verifier_declaration::provides::LookupVerifierDeclarationFn;
use super::interface::{
    AdmitVerifierCompositionDeps, AdmitVerifierCompositionFn, AdmitVerifierCompositionParams,
    AdmitVerifierCompositionPayload, AdmitVerifierCompositionReturn,
    AdmitVerifierCompositionSuccessReturn, ChainMemberIdentifierDisagreement,
    ContractSuiteDisagreement, VerifierFormDisagreement,
};

#[derive(Default)]
pub struct AdmitVerifierCompositionDepsOverrides;

pub fn build_admit_verifier_composition_deps(
    lookup_verifier_declaration: LookupVerifierDeclarationFn,
    _overrides: AdmitVerifierCompositionDepsOverrides,
) -> AdmitVerifierCompositionDeps {
    AdmitVerifierCompositionDeps {
        lookup_verifier_declaration,
    }
}

#[derive(Default)]
pub struct AdmitVerifierCompositionParamsOverrides {
    pub identifier: Option<VerifierConcreteKind>,
}

pub fn build_admit_verifier_composition_params(
    overrides: AdmitVerifierCompositionParamsOverrides,
) -> AdmitVerifierCompositionParams {
    AdmitVerifierCompositionParams {
        identifier: overrides.identifier.unwrap_or(VerifierConcreteKind::Evm),
    }
}

#[derive(Default)]
pub struct AdmitVerifierCompositionPayloadOverrides;

pub fn build_admit_verifier_composition_payload<'a>(
    chain_forms: &'a ChainFormsDeclaration,
    chain: &'a ChainDeclaration,
    encoder: &'a EncodingDeclaration,
    pairing: &'a PairingDeclaration,
    generate: &'a GenerateDeclaration,
    _overrides: AdmitVerifierCompositionPayloadOverrides,
) -> AdmitVerifierCompositionPayload<'a> {
    AdmitVerifierCompositionPayload {
        chain_forms,
        chain,
        encoder,
        pairing,
        generate,
    }
}

#[derive(Default)]
pub struct AdmitVerifierCompositionSuccessReturnOverrides;

pub fn build_admit_verifier_composition_success_return(
    _overrides: AdmitVerifierCompositionSuccessReturnOverrides,
) -> AdmitVerifierCompositionSuccessReturn {
    AdmitVerifierCompositionSuccessReturn
}

#[derive(Default)]
pub struct ChainMemberIdentifierDisagreementOverrides {
    pub required: Option<ChainMemberIdentifier>,
    pub declared: Option<ChainMemberIdentifier>,
}

pub fn build_chain_member_identifier_disagreement(
    overrides: ChainMemberIdentifierDisagreementOverrides,
) -> ChainMemberIdentifierDisagreement {
    ChainMemberIdentifierDisagreement {
        required: overrides.required.unwrap_or(ChainMemberIdentifier::Base),
        declared: overrides.declared.unwrap_or(ChainMemberIdentifier::Mock),
    }
}

#[derive(Default)]
pub struct VerifierFormDisagreementOverrides {
    pub required: Option<&'static [VerifierGroupArithmetic]>,
    pub declared: Option<VerifierGroupArithmetic>,
}

pub fn build_verifier_form_disagreement(
    overrides: VerifierFormDisagreementOverrides,
) -> VerifierFormDisagreement {
    VerifierFormDisagreement {
        required: overrides
            .required
            .unwrap_or(&[VerifierGroupArithmetic::FirstGroupOnly]),
        declared: overrides
            .declared
            .unwrap_or(VerifierGroupArithmetic::BothGroups),
    }
}

#[derive(Default)]
pub struct ContractSuiteDisagreementOverrides {
    pub required: Option<&'static str>,
    pub declared: Option<&'static str>,
}

pub fn build_contract_suite_disagreement(
    overrides: ContractSuiteDisagreementOverrides,
) -> ContractSuiteDisagreement {
    ContractSuiteDisagreement {
        required: overrides.required.unwrap_or("evm"),
        declared: overrides.declared.unwrap_or("other"),
    }
}

pub fn mock_admit_verifier_composition(
    _deps: &AdmitVerifierCompositionDeps,
    _params: AdmitVerifierCompositionParams,
    _payload: AdmitVerifierCompositionPayload<'_>,
) -> AdmitVerifierCompositionReturn {
    let _: AdmitVerifierCompositionFn = mock_admit_verifier_composition;
    Ok(build_admit_verifier_composition_success_return(
        Default::default(),
    ))
}
