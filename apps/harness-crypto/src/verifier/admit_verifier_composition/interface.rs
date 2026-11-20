use chain::{
    ChainDeclaration, ChainFormsDeclaration, ChainFormsIdentifierDisagreement,
    ChainMemberIdentifier,
};
use encoding::EncodingDeclaration;
use pairing::{PairingDeclaration, VerifierGroupArithmetic};

use crate::generate::provides::{EncodingIdentifierDisagreement, GenerateDeclaration};

use super::super::factory::provides::VerifierConcreteKind;
use super::super::lookup_verifier_declaration::provides::{
    LookupVerifierDeclarationFn, lookup_verifier_declaration,
};

#[derive(Clone, Copy)]
pub struct AdmitVerifierCompositionDeps {
    pub lookup_verifier_declaration: LookupVerifierDeclarationFn,
}

pub(crate) const ADMIT_VERIFIER_COMPOSITION_DEPS: AdmitVerifierCompositionDeps =
    AdmitVerifierCompositionDeps {
        lookup_verifier_declaration,
    };

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AdmitVerifierCompositionParams {
    pub identifier: VerifierConcreteKind,
}

#[derive(Clone, Copy)]
pub struct AdmitVerifierCompositionPayload<'a> {
    pub chain_forms: &'a ChainFormsDeclaration,
    pub chain: &'a ChainDeclaration,
    pub encoder: &'a EncodingDeclaration,
    pub pairing: &'a PairingDeclaration,
    pub generate: &'a GenerateDeclaration,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AdmitVerifierCompositionSuccessReturn;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ChainMemberIdentifierDisagreement {
    pub required: ChainMemberIdentifier,
    pub declared: ChainMemberIdentifier,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct VerifierFormDisagreement {
    pub required: &'static [VerifierGroupArithmetic],
    pub declared: VerifierGroupArithmetic,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ContractSuiteDisagreement {
    pub required: &'static str,
    pub declared: &'static str,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AdmitVerifierCompositionErrorReturn {
    Incompatible {
        chain_forms: Option<ChainFormsIdentifierDisagreement>,
        chain_member: Option<ChainMemberIdentifierDisagreement>,
        encoding: Option<EncodingIdentifierDisagreement>,
        verifier_form: Option<VerifierFormDisagreement>,
        contract_suite: Option<ContractSuiteDisagreement>,
    },
}

pub type AdmitVerifierCompositionReturn =
    Result<AdmitVerifierCompositionSuccessReturn, AdmitVerifierCompositionErrorReturn>;

pub type AdmitVerifierCompositionFn = for<'a> fn(
    &AdmitVerifierCompositionDeps,
    AdmitVerifierCompositionParams,
    AdmitVerifierCompositionPayload<'a>,
) -> AdmitVerifierCompositionReturn;
