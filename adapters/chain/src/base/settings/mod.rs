#[cfg(test)]
mod integration_test;
mod interface;
#[cfg(any(test, feature = "mocks"))]
mod mock;
pub mod provides;
#[cfg(test)]
mod test;

use crate::factory::provides::{ChainEndpoint, ChainSettingBounds, ChainSettingDisposition};
use interface::{
    AdmitBaseChainSettingsDeps, AdmitBaseChainSettingsErrorReturn, AdmitBaseChainSettingsParams,
    AdmitBaseChainSettingsPayload, AdmitBaseChainSettingsReturn,
    AdmitBaseChainSettingsSuccessReturn, BaseChainSettings, BaseChainSettingsDocument,
    BaseChainSettingsDocumentErrorReturn, BaseChainSettingsDocumentTryFromReturn,
};
use serde_json::Value;
use url::Url;

const BASE_CHAIN_SETTINGS_DOCUMENT_KEYS: &[&str] = &[
    "endpoint",
    "settlement_endpoint",
    "chain_identifier",
    "registry",
    "entitlement_contract",
    "gas_price_oracle",
    "anchor_state_registry",
    "batch_page_limit",
];

impl TryFrom<Value> for BaseChainSettingsDocument {
    type Error = BaseChainSettingsDocumentErrorReturn;

    fn try_from(value: Value) -> BaseChainSettingsDocumentTryFromReturn {
        let Value::Object(table) = value else {
            return Err(BaseChainSettingsDocumentErrorReturn::NotATable {
                path: String::new(),
            });
        };
        let mut keys: Vec<&str> = table.keys().map(String::as_str).collect();
        keys.sort_unstable();
        for key in &keys {
            if !BASE_CHAIN_SETTINGS_DOCUMENT_KEYS.contains(key) {
                return Err(BaseChainSettingsDocumentErrorReturn::UnknownKey {
                    path: String::new(),
                    key: (*key).to_string(),
                });
            }
        }
        for key in &keys {
            if !table[*key].is_string() {
                return Err(BaseChainSettingsDocumentErrorReturn::NotText {
                    path: (*key).to_string(),
                });
            }
        }
        Ok(BaseChainSettingsDocument {
            endpoint: table
                .get("endpoint")
                .and_then(Value::as_str)
                .map(String::from),
            settlement_endpoint: table
                .get("settlement_endpoint")
                .and_then(Value::as_str)
                .map(String::from),
            chain_identifier: table
                .get("chain_identifier")
                .and_then(Value::as_str)
                .map(String::from),
            registry: table
                .get("registry")
                .and_then(Value::as_str)
                .map(String::from),
            entitlement_contract: table
                .get("entitlement_contract")
                .and_then(Value::as_str)
                .map(String::from),
            gas_price_oracle: table
                .get("gas_price_oracle")
                .and_then(Value::as_str)
                .map(String::from),
            anchor_state_registry: table
                .get("anchor_state_registry")
                .and_then(Value::as_str)
                .map(String::from),
            batch_page_limit: table
                .get("batch_page_limit")
                .and_then(Value::as_str)
                .map(String::from),
        })
    }
}

pub fn admit_base_chain_settings<'a>(
    deps: &AdmitBaseChainSettingsDeps<'a>,
    _params: AdmitBaseChainSettingsParams,
    payload: AdmitBaseChainSettingsPayload,
) -> AdmitBaseChainSettingsReturn {
    let document = payload.document;
    Ok(AdmitBaseChainSettingsSuccessReturn {
        settings: BaseChainSettings {
            endpoint: admit_endpoint(deps, "endpoint", document.endpoint.as_deref())?,
            settlement_endpoint: admit_endpoint(
                deps,
                "settlement_endpoint",
                document.settlement_endpoint.as_deref(),
            )?,
            chain_identifier: admit_chain_identifier(
                deps,
                "chain_identifier",
                document.chain_identifier.as_deref(),
            )?,
            registry: admit_address(deps, "registry", document.registry.as_deref())?,
            entitlement_contract: admit_address(
                deps,
                "entitlement_contract",
                document.entitlement_contract.as_deref(),
            )?,
            gas_price_oracle: admit_address(
                deps,
                "gas_price_oracle",
                document.gas_price_oracle.as_deref(),
            )?,
            anchor_state_registry: admit_address(
                deps,
                "anchor_state_registry",
                document.anchor_state_registry.as_deref(),
            )?,
            batch_page_limit: admit_batch_page_limit(
                deps,
                "batch_page_limit",
                document.batch_page_limit.as_deref(),
            )?,
        },
    })
}

fn resolve_setting_text(
    deps: &AdmitBaseChainSettingsDeps<'_>,
    setting: &str,
    supplied: Option<&str>,
) -> Result<String, AdmitBaseChainSettingsErrorReturn> {
    let statement = deps
        .declaration
        .settings
        .iter()
        .find(|statement| statement.name == setting)
        .ok_or_else(|| AdmitBaseChainSettingsErrorReturn::UndeclaredSetting {
            setting: setting.to_string(),
        })?;
    match statement.disposition {
        ChainSettingDisposition::Fixed { value } => match supplied {
            Some(supplied) => Err(AdmitBaseChainSettingsErrorReturn::FixedSetting {
                setting: setting.to_string(),
                supplied: supplied.to_string(),
            }),
            None => Ok(value.to_string()),
        },
        ChainSettingDisposition::Settable { bounds, default } => check_setting_bounds(
            setting,
            bounds,
            supplied
                .map(str::to_string)
                .unwrap_or_else(|| default.to_string()),
        ),
    }
}

fn check_setting_bounds(
    setting: &str,
    bounds: ChainSettingBounds,
    text: String,
) -> Result<String, AdmitBaseChainSettingsErrorReturn> {
    match bounds {
        ChainSettingBounds::Unbounded => Ok(text),
        ChainSettingBounds::TextPrefixedBy { prefixes } => {
            if prefixes.iter().any(|prefix| text.starts_with(prefix)) {
                Ok(text)
            } else {
                Err(AdmitBaseChainSettingsErrorReturn::OutOfBounds {
                    setting: setting.to_string(),
                    supplied: text,
                })
            }
        }
    }
}

fn admit_endpoint(
    deps: &AdmitBaseChainSettingsDeps<'_>,
    setting: &str,
    supplied: Option<&str>,
) -> Result<ChainEndpoint, AdmitBaseChainSettingsErrorReturn> {
    let text = resolve_setting_text(deps, setting, supplied)?;
    Url::parse(&text).map_err(|error| AdmitBaseChainSettingsErrorReturn::Endpoint {
        setting: setting.to_string(),
        error,
    })?;
    Ok(ChainEndpoint { text })
}

fn admit_chain_identifier(
    deps: &AdmitBaseChainSettingsDeps<'_>,
    setting: &str,
    supplied: Option<&str>,
) -> Result<[u8; 32], AdmitBaseChainSettingsErrorReturn> {
    let text = resolve_setting_text(deps, setting, supplied)?;
    if text.is_empty() || !text.bytes().all(|byte| byte.is_ascii_digit()) {
        return Err(AdmitBaseChainSettingsErrorReturn::ChainIdentifier {
            setting: setting.to_string(),
            text,
        });
    }
    let mut value = [0u8; 32];
    let mut overflow = false;
    for byte in text.bytes() {
        let mut carry = u16::from(byte - b'0');
        for slot in value.iter_mut().rev() {
            let acc = u16::from(*slot) * 10 + carry;
            let [slot_byte, carry_byte] = acc.to_le_bytes();
            *slot = slot_byte;
            carry = u16::from(carry_byte);
        }
        if carry != 0 {
            overflow = true;
            break;
        }
    }
    if overflow {
        return Err(AdmitBaseChainSettingsErrorReturn::ChainIdentifier {
            setting: setting.to_string(),
            text,
        });
    }
    Ok(value)
}

fn admit_address(
    deps: &AdmitBaseChainSettingsDeps<'_>,
    setting: &str,
    supplied: Option<&str>,
) -> Result<[u8; 20], AdmitBaseChainSettingsErrorReturn> {
    let text = resolve_setting_text(deps, setting, supplied)?;
    let Some(body) = text.strip_prefix("0x") else {
        return Err(AdmitBaseChainSettingsErrorReturn::Address {
            setting: setting.to_string(),
            text,
        });
    };
    if body.len() != 40 || !body.bytes().all(|byte| byte.is_ascii_hexdigit()) {
        return Err(AdmitBaseChainSettingsErrorReturn::Address {
            setting: setting.to_string(),
            text,
        });
    }
    let mut bytes = [0u8; 20];
    for (index, pair) in body.as_bytes().as_chunks::<2>().0.iter().enumerate() {
        bytes[index] = (hex_digit(pair[0]) << 4) | hex_digit(pair[1]);
    }
    Ok(bytes)
}

fn hex_digit(byte: u8) -> u8 {
    match byte {
        b'0'..=b'9' => byte - b'0',
        b'a'..=b'f' => byte - b'a' + 10,
        _ => byte - b'A' + 10,
    }
}

fn admit_batch_page_limit(
    deps: &AdmitBaseChainSettingsDeps<'_>,
    setting: &str,
    supplied: Option<&str>,
) -> Result<u64, AdmitBaseChainSettingsErrorReturn> {
    let text = resolve_setting_text(deps, setting, supplied)?;
    if text.is_empty() || !text.bytes().all(|byte| byte.is_ascii_digit()) {
        return Err(AdmitBaseChainSettingsErrorReturn::BatchPageLimit {
            setting: setting.to_string(),
            text,
        });
    }
    let mut value: u64 = 0;
    let mut overflow = false;
    for byte in text.bytes() {
        match value
            .checked_mul(10)
            .and_then(|next| next.checked_add(u64::from(byte - b'0')))
        {
            Some(next) => value = next,
            None => {
                overflow = true;
                break;
            }
        }
    }
    if overflow || value == 0 {
        return Err(AdmitBaseChainSettingsErrorReturn::BatchPageLimit {
            setting: setting.to_string(),
            text,
        });
    }
    Ok(value)
}
