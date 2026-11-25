#[cfg(test)]
mod integration_test;
mod interface;
#[cfg(any(test, feature = "mocks"))]
mod mock;
pub mod provides;
#[cfg(test)]
mod test;

use crate::factory::provides::HashToScalarConcrete;
use crate::lookup_hash_to_scalar_declaration::provides::{
    LookupHashToScalarDeclarationDeps, LookupHashToScalarDeclarationParams,
    LookupHashToScalarDeclarationPayload,
};
use interface::{
    AdmitHashToScalarSelectionDeps, AdmitHashToScalarSelectionErrorReturn,
    AdmitHashToScalarSelectionParams, AdmitHashToScalarSelectionReturn,
    AdmitHashToScalarSelectionSuccessReturn,
};

pub fn admit_hash_to_scalar_selection(
    deps: &AdmitHashToScalarSelectionDeps,
    params: AdmitHashToScalarSelectionParams,
    payload: serde_json::Value,
) -> AdmitHashToScalarSelectionReturn {
    match params.concrete {
        HashToScalarConcrete::Keccak256 => {
            if payload.is_null() || payload.as_object().is_some_and(|map| map.is_empty()) {
                let result = (deps.lookup_declaration)(
                    &LookupHashToScalarDeclarationDeps,
                    LookupHashToScalarDeclarationParams {
                        concrete: params.concrete,
                    },
                    LookupHashToScalarDeclarationPayload,
                );
                let Ok(success) = result;
                Ok(AdmitHashToScalarSelectionSuccessReturn {
                    selection: success.declaration.concrete,
                })
            } else {
                Err(AdmitHashToScalarSelectionErrorReturn::SettingSupplied {
                    concrete: params.concrete,
                })
            }
        }
        #[cfg(any(test, feature = "mocks"))]
        HashToScalarConcrete::Mock(_) => {
            if payload.is_null() || payload.as_object().is_some_and(|map| map.is_empty()) {
                let result = (deps.lookup_declaration)(
                    &LookupHashToScalarDeclarationDeps,
                    LookupHashToScalarDeclarationParams {
                        concrete: params.concrete,
                    },
                    LookupHashToScalarDeclarationPayload,
                );
                let Ok(success) = result;
                Ok(AdmitHashToScalarSelectionSuccessReturn {
                    selection: success.declaration.concrete,
                })
            } else {
                Err(AdmitHashToScalarSelectionErrorReturn::SettingSupplied {
                    concrete: params.concrete,
                })
            }
        }
    }
}
