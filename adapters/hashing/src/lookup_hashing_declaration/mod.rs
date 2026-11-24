mod interface;
#[cfg(any(test, feature = "mocks"))]
mod mock;
pub mod provides;
#[cfg(test)]
mod test;

use crate::blake3_bao::provides::Blake3BaoCommitment;
use crate::factory::provides::CommitmentConcrete;
#[cfg(any(test, feature = "mocks"))]
use crate::factory::provides::{CommitmentDeclaration, MOCK_I_COMMITMENT_ADAPTER_DECLARATION};
use interface::{
    LookupHashingDeclarationDeps, LookupHashingDeclarationParams, LookupHashingDeclarationPayload,
    LookupHashingDeclarationReturn, LookupHashingDeclarationSuccessReturn,
};

pub fn lookup_hashing_declaration(
    _deps: &LookupHashingDeclarationDeps,
    params: LookupHashingDeclarationParams,
    _payload: LookupHashingDeclarationPayload,
) -> LookupHashingDeclarationReturn {
    let declaration = match params.concrete {
        CommitmentConcrete::Blake3Bao => Blake3BaoCommitment::DECLARATION,
        #[cfg(any(test, feature = "mocks"))]
        CommitmentConcrete::Mock(_) => CommitmentDeclaration {
            concrete: params.concrete,
            ..MOCK_I_COMMITMENT_ADAPTER_DECLARATION
        },
    };
    Ok(LookupHashingDeclarationSuccessReturn { declaration })
}
