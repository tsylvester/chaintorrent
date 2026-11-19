#![allow(
    dead_code,
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::too_many_arguments
)]

use super::interface::{
    AdmitEvmVerifierSettingsDeps, AdmitEvmVerifierSettingsFn, AdmitEvmVerifierSettingsParams,
    AdmitEvmVerifierSettingsPayload, AdmitEvmVerifierSettingsReturn,
    AdmitEvmVerifierSettingsSuccessReturn, EvmVerifierSettings, EvmVerifierSettingsDocument,
};

#[derive(Default)]
pub struct EvmVerifierSettingsOverrides;

pub fn build_evm_verifier_settings(
    _overrides: EvmVerifierSettingsOverrides,
) -> EvmVerifierSettings {
    EvmVerifierSettings
}

#[derive(Default)]
pub struct EvmVerifierSettingsDocumentOverrides;

pub fn build_evm_verifier_settings_document(
    _overrides: EvmVerifierSettingsDocumentOverrides,
) -> EvmVerifierSettingsDocument {
    EvmVerifierSettingsDocument
}

#[derive(Default)]
pub struct AdmitEvmVerifierSettingsDepsOverrides;

pub fn build_admit_evm_verifier_settings_deps(
    _overrides: AdmitEvmVerifierSettingsDepsOverrides,
) -> AdmitEvmVerifierSettingsDeps {
    AdmitEvmVerifierSettingsDeps
}

#[derive(Default)]
pub struct AdmitEvmVerifierSettingsParamsOverrides;

pub fn build_admit_evm_verifier_settings_params(
    _overrides: AdmitEvmVerifierSettingsParamsOverrides,
) -> AdmitEvmVerifierSettingsParams {
    AdmitEvmVerifierSettingsParams
}

#[derive(Default)]
pub struct AdmitEvmVerifierSettingsPayloadOverrides {
    pub document: Option<EvmVerifierSettingsDocument>,
}

pub fn build_admit_evm_verifier_settings_payload(
    overrides: AdmitEvmVerifierSettingsPayloadOverrides,
) -> AdmitEvmVerifierSettingsPayload {
    AdmitEvmVerifierSettingsPayload {
        document: overrides
            .document
            .unwrap_or_else(|| build_evm_verifier_settings_document(Default::default())),
    }
}

#[derive(Default)]
pub struct AdmitEvmVerifierSettingsSuccessReturnOverrides {
    pub settings: Option<EvmVerifierSettings>,
}

pub fn build_admit_evm_verifier_settings_success_return(
    overrides: AdmitEvmVerifierSettingsSuccessReturnOverrides,
) -> AdmitEvmVerifierSettingsSuccessReturn {
    AdmitEvmVerifierSettingsSuccessReturn {
        settings: overrides
            .settings
            .unwrap_or_else(|| build_evm_verifier_settings(Default::default())),
    }
}

pub fn mock_admit_evm_verifier_settings(
    _deps: &AdmitEvmVerifierSettingsDeps,
    _params: AdmitEvmVerifierSettingsParams,
    _payload: AdmitEvmVerifierSettingsPayload,
) -> AdmitEvmVerifierSettingsReturn {
    let _: AdmitEvmVerifierSettingsFn = mock_admit_evm_verifier_settings;
    Ok(build_admit_evm_verifier_settings_success_return(
        Default::default(),
    ))
}
