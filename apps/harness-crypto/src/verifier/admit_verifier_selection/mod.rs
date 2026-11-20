#[cfg(test)]
mod integration_test;
mod interface;
#[cfg(any(test, feature = "mocks"))]
mod mock;
pub mod provides;
#[cfg(test)]
mod test;

use serde_json;

use super::evm::provides::{
    AdmitEvmVerifierSettingsDeps, AdmitEvmVerifierSettingsParams, AdmitEvmVerifierSettingsPayload,
    EvmVerifierSettingsDocument,
};
#[cfg(any(test, feature = "mocks"))]
use super::factory::provides::MockIVerifierAdapterFailureMode;
use super::factory::provides::{VerifierConcrete, VerifierConcreteKind};
use super::lookup_verifier_declaration::provides::{
    LookupVerifierDeclarationDeps, LookupVerifierDeclarationParams,
    LookupVerifierDeclarationPayload,
};
use interface::{
    AdmitVerifierSelectionDeps, AdmitVerifierSelectionErrorReturn, AdmitVerifierSelectionParams,
    AdmitVerifierSelectionReturn, AdmitVerifierSelectionSuccessReturn,
};

pub fn admit_verifier_selection(
    deps: &AdmitVerifierSelectionDeps,
    params: AdmitVerifierSelectionParams,
    payload: serde_json::Value,
) -> AdmitVerifierSelectionReturn {
    match params.identifier {
        VerifierConcreteKind::Evm => {
            let document = match EvmVerifierSettingsDocument::try_from(payload) {
                Ok(document) => document,
                Err(error) => {
                    return Err(AdmitVerifierSelectionErrorReturn::EvmDocument(error));
                }
            };
            let Ok(looked_up) = (deps.lookup_verifier_declaration)(
                &LookupVerifierDeclarationDeps,
                LookupVerifierDeclarationParams {
                    identifier: params.identifier,
                },
                LookupVerifierDeclarationPayload,
            );
            let Ok(admitted) = (deps.admit_evm_verifier_settings)(
                &AdmitEvmVerifierSettingsDeps {
                    declaration: &looked_up.declaration,
                },
                AdmitEvmVerifierSettingsParams,
                AdmitEvmVerifierSettingsPayload { document },
            );
            Ok(AdmitVerifierSelectionSuccessReturn {
                selection: VerifierConcrete::Evm {
                    settings: Box::new(admitted.settings),
                },
            })
        }
        #[cfg(any(test, feature = "mocks"))]
        VerifierConcreteKind::Mock => Ok(AdmitVerifierSelectionSuccessReturn {
            selection: VerifierConcrete::Mock(MockIVerifierAdapterFailureMode::NoFailure),
        }),
    }
}
