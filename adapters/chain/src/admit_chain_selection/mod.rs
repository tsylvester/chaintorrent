#[cfg(test)]
mod integration_test;
mod interface;
#[cfg(any(test, feature = "mocks"))]
mod mock;
pub mod provides;
#[cfg(test)]
mod test;

use crate::base::provides::{
    AdmitBaseChainSettingsDeps, AdmitBaseChainSettingsParams, AdmitBaseChainSettingsPayload,
    BaseChainSettingsDocument,
};
#[cfg(any(test, feature = "mocks"))]
use crate::factory::provides::MockIChainAdapterFailureMode;
use crate::factory::provides::{ChainMemberIdentifier, ChainSelection};
use crate::lookup_chain_declaration::provides::{
    LookupChainDeclarationDeps, LookupChainDeclarationParams, LookupChainDeclarationPayload,
};
use interface::{
    AdmitChainSelectionDeps, AdmitChainSelectionErrorReturn, AdmitChainSelectionFn,
    AdmitChainSelectionParams, AdmitChainSelectionReturn, AdmitChainSelectionSuccessReturn,
};

pub fn admit_chain_selection(
    deps: &AdmitChainSelectionDeps,
    params: AdmitChainSelectionParams,
    payload: serde_json::Value,
) -> AdmitChainSelectionReturn {
    let _: AdmitChainSelectionFn = admit_chain_selection;
    match params.identifier {
        ChainMemberIdentifier::Base => {
            let document = match BaseChainSettingsDocument::try_from(payload) {
                Ok(document) => document,
                Err(error) => {
                    return Err(AdmitChainSelectionErrorReturn::BaseDocument(error));
                }
            };
            let Ok(looked_up) = (deps.lookup_chain_declaration)(
                &LookupChainDeclarationDeps,
                LookupChainDeclarationParams {
                    identifier: params.identifier,
                },
                LookupChainDeclarationPayload,
            );
            match (deps.admit_base_chain_settings)(
                &AdmitBaseChainSettingsDeps {
                    declaration: &looked_up.declaration,
                },
                AdmitBaseChainSettingsParams,
                AdmitBaseChainSettingsPayload { document },
            ) {
                Ok(admitted) => Ok(AdmitChainSelectionSuccessReturn {
                    selection: ChainSelection::Base {
                        settings: admitted.settings,
                    },
                }),
                Err(error) => Err(AdmitChainSelectionErrorReturn::Base(error)),
            }
        }
        #[cfg(any(test, feature = "mocks"))]
        ChainMemberIdentifier::Mock => Ok(AdmitChainSelectionSuccessReturn {
            selection: ChainSelection::Mock {
                failure_mode: MockIChainAdapterFailureMode::NoFailure,
            },
        }),
    }
}
