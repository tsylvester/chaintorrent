#[cfg(test)]
mod integration_test;
mod interface;
#[cfg(any(test, feature = "mocks"))]
mod mock;
pub mod provides;
#[cfg(test)]
mod test;

use serde_json;

#[cfg(any(test, feature = "mocks"))]
use super::factory::provides::MockIGenerateAdapterFailureMode;
use super::factory::provides::{GenerateConcrete, GenerateConcreteKind};
use super::lookup_generate_declaration::provides::{
    LookupGenerateDeclarationDeps, LookupGenerateDeclarationParams,
    LookupGenerateDeclarationPayload,
};
use super::solidity::provides::{
    AdmitSolidityGenerateSettingsDeps, AdmitSolidityGenerateSettingsParams,
    AdmitSolidityGenerateSettingsPayload, SolidityGenerateSettingsDocument,
};
use interface::{
    AdmitGenerateSelectionDeps, AdmitGenerateSelectionErrorReturn, AdmitGenerateSelectionFn,
    AdmitGenerateSelectionParams, AdmitGenerateSelectionReturn,
    AdmitGenerateSelectionSolidityErrorReturn, AdmitGenerateSelectionSuccessReturn,
};

pub fn admit_generate_selection(
    deps: &AdmitGenerateSelectionDeps,
    params: AdmitGenerateSelectionParams,
    payload: serde_json::Value,
) -> AdmitGenerateSelectionReturn {
    let _: AdmitGenerateSelectionFn = admit_generate_selection;
    match params.identifier {
        GenerateConcreteKind::Solidity => {
            let document = match SolidityGenerateSettingsDocument::try_from(payload) {
                Ok(document) => document,
                Err(error) => {
                    return Err(AdmitGenerateSelectionErrorReturn::Solidity(
                        AdmitGenerateSelectionSolidityErrorReturn::Document(error),
                    ));
                }
            };
            let Ok(looked_up) = (deps.lookup_declaration)(
                &LookupGenerateDeclarationDeps,
                LookupGenerateDeclarationParams {
                    identifier: params.identifier,
                },
                LookupGenerateDeclarationPayload,
            );
            match (deps.admit_solidity_settings)(
                &AdmitSolidityGenerateSettingsDeps {
                    declaration: &looked_up.declaration,
                },
                AdmitSolidityGenerateSettingsParams,
                AdmitSolidityGenerateSettingsPayload { document },
            ) {
                Ok(admitted) => Ok(AdmitGenerateSelectionSuccessReturn {
                    selection: GenerateConcrete::Solidity {
                        settings: Box::new(admitted.settings),
                    },
                }),
                Err(error) => Err(AdmitGenerateSelectionErrorReturn::Solidity(
                    AdmitGenerateSelectionSolidityErrorReturn::Settings(error),
                )),
            }
        }
        #[cfg(any(test, feature = "mocks"))]
        GenerateConcreteKind::Mock => Ok(AdmitGenerateSelectionSuccessReturn {
            selection: GenerateConcrete::Mock(MockIGenerateAdapterFailureMode::NoFailure),
        }),
    }
}
