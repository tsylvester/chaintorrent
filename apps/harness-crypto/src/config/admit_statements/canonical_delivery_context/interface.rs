use domain::{ParameterSetIdentifier, SuiteIdentifier};
use encoding::CanonicalFieldValue;
use hashing::Root;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CanonicalDeliveryContext {
    pub suite_identifier: SuiteIdentifier,
    pub chain: CanonicalFieldValue,
    pub entitlement_contract: CanonicalFieldValue,
    pub asset_root: Root,
    pub parameter_set_identifier: ParameterSetIdentifier,
    pub source_entitlement: CanonicalFieldValue,
    pub target_entitlement: CanonicalFieldValue,
    pub old_interval: CanonicalFieldValue,
    pub new_interval: CanonicalFieldValue,
    pub seller: CanonicalFieldValue,
    pub buyer: CanonicalFieldValue,
    pub expiry: u64,
}
