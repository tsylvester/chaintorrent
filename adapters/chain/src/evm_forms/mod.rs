#[cfg(test)]
mod integration_test;
mod interface;
#[cfg(test)]
mod mock;
pub(crate) mod provides;
#[cfg(test)]
mod test;

use crate::factory::provides::{
    CHAIN_FORMS_INTERFACE_VERSION, ChainFormsDeclaration, ChainFormsIdentifier,
    DeriveKeyIdentifierParams, DeriveKeyIdentifierPayload, DeriveKeyIdentifierReturn,
    DeriveKeyIdentifierSuccessReturn, IChainForms, IIdentityForm, IntentCall, IntentMemberNames,
    SignedIntentErrorReturn, SignedIntentParams, SignedIntentPayload, SignedIntentReturn,
    SignedIntentSuccessReturn,
};
use encoding::{
    CanonicalFieldKind, CanonicalFieldValue, EncodingIdentifier, FromFieldErrorReturn,
    FromFieldParams, FromFieldReturn, FromFieldSuccessReturn, ICanonicalField, IEncoderAdapter,
    ToFieldParams, ToFieldReturn, ToFieldSuccessReturn, TypeNameParams,
};
use interface::{
    EIP712_DOMAIN_TYPE, EIP712_PREFIX, EvmChainIdentifierForm,
    EvmChainIdentifierFormConstructorParams, EvmChainIdentifierFormTryNewReturn,
    EvmEntitlementForm, EvmEntitlementFormConstructorParams, EvmEntitlementFormTryNewReturn,
    EvmForms, EvmFormsConstructorParams, EvmFormsTryNewReturn, EvmFormsTypeNames, EvmIdentityForm,
    EvmIdentityFormConstructorParams, EvmIdentityFormTryNewReturn, EvmIntervalForm,
    EvmIntervalFormConstructorParams, EvmIntervalFormTryNewReturn, EvmKeyIdentifierForm,
    EvmKeyIdentifierFormConstructorParams, EvmKeyIdentifierFormTryNewReturn, EvmSignedIntentForm,
    EvmSignedIntentFormConstructorParams, EvmSignedIntentFormTryNewReturn,
};
use sha3::{Digest, Keccak256};

impl EvmForms {
    pub const DECLARATION: ChainFormsDeclaration = ChainFormsDeclaration {
        identifier: ChainFormsIdentifier::EvmV1,
        adapter_version: 1,
        interface_version: CHAIN_FORMS_INTERFACE_VERSION,
        encoding_identifier: EncodingIdentifier::EthereumAbiV1,
        key_identifier_domain_tag: b"chaintorrent/evm/key-identifier/v1",
        intent_domain_name: "ChainTorrent",
        intent_domain_version: "1",
        intent_struct_name: "Intent",
        intent_member_names: IntentMemberNames {
            identity: "identity",
            nonce: "nonce",
            expiry: "expiry",
            call: "call",
        },
        nonce_kind: CanonicalFieldKind::Unsigned64,
        expiry_kind: CanonicalFieldKind::Unsigned64,
    };

    pub fn try_new<E: IEncoderAdapter>(
        params: EvmFormsConstructorParams<'_, E>,
    ) -> EvmFormsTryNewReturn {
        let encoder = params.encoder;
        let Ok(fixed_bytes_32) =
            encoder.type_name(TypeNameParams, CanonicalFieldKind::FixedBytes32);
        let Ok(unsigned_16) = encoder.type_name(TypeNameParams, CanonicalFieldKind::Unsigned16);
        let Ok(unsigned_32) = encoder.type_name(TypeNameParams, CanonicalFieldKind::Unsigned32);
        let Ok(unsigned_64) = encoder.type_name(TypeNameParams, CanonicalFieldKind::Unsigned64);
        let Ok(text) = encoder.type_name(TypeNameParams, CanonicalFieldKind::Text);
        let Ok(bytes) = encoder.type_name(TypeNameParams, CanonicalFieldKind::Bytes);
        let Ok(fixed_bytes_20) =
            encoder.type_name(TypeNameParams, CanonicalFieldKind::FixedBytes20);
        let Ok(unsigned_256) = encoder.type_name(TypeNameParams, CanonicalFieldKind::Unsigned256);
        Ok(EvmForms {
            type_names: EvmFormsTypeNames {
                fixed_bytes_32: fixed_bytes_32.name,
                unsigned_16: unsigned_16.name,
                unsigned_32: unsigned_32.name,
                unsigned_64: unsigned_64.name,
                text: text.name,
                bytes: bytes.name,
                fixed_bytes_20: fixed_bytes_20.name,
                unsigned_256: unsigned_256.name,
            },
        })
    }

    fn type_name(&self, kind: CanonicalFieldKind) -> &str {
        match kind {
            CanonicalFieldKind::FixedBytes32 => self.type_names.fixed_bytes_32.as_str(),
            CanonicalFieldKind::Unsigned16 => self.type_names.unsigned_16.as_str(),
            CanonicalFieldKind::Unsigned32 => self.type_names.unsigned_32.as_str(),
            CanonicalFieldKind::Unsigned64 => self.type_names.unsigned_64.as_str(),
            CanonicalFieldKind::Text => self.type_names.text.as_str(),
            CanonicalFieldKind::Bytes => self.type_names.bytes.as_str(),
            CanonicalFieldKind::FixedBytes20 => self.type_names.fixed_bytes_20.as_str(),
            CanonicalFieldKind::Unsigned256 => self.type_names.unsigned_256.as_str(),
        }
    }

    fn is_identifier(text: &str) -> bool {
        let mut characters = text.chars();
        match characters.next() {
            Some(first) if first.is_ascii_alphabetic() || first == '_' => {
                characters.all(|character| character.is_ascii_alphanumeric() || character == '_')
            }
            _ => false,
        }
    }

    fn value_kind(value: &CanonicalFieldValue) -> CanonicalFieldKind {
        match value {
            CanonicalFieldValue::FixedBytes32(_) => CanonicalFieldKind::FixedBytes32,
            CanonicalFieldValue::Unsigned16(_) => CanonicalFieldKind::Unsigned16,
            CanonicalFieldValue::Unsigned32(_) => CanonicalFieldKind::Unsigned32,
            CanonicalFieldValue::Unsigned64(_) => CanonicalFieldKind::Unsigned64,
            CanonicalFieldValue::Text(_) => CanonicalFieldKind::Text,
            CanonicalFieldValue::Bytes(_) => CanonicalFieldKind::Bytes,
            CanonicalFieldValue::FixedBytes20(_) => CanonicalFieldKind::FixedBytes20,
            CanonicalFieldValue::Unsigned256(_) => CanonicalFieldKind::Unsigned256,
        }
    }

    fn value_word(value: &CanonicalFieldValue) -> [u8; 32] {
        match value {
            CanonicalFieldValue::FixedBytes32(bytes) | CanonicalFieldValue::Unsigned256(bytes) => {
                *bytes
            }
            CanonicalFieldValue::Unsigned16(value) => {
                let mut word = [0u8; 32];
                word[30..].copy_from_slice(&value.to_be_bytes());
                word
            }
            CanonicalFieldValue::Unsigned32(value) => {
                let mut word = [0u8; 32];
                word[28..].copy_from_slice(&value.to_be_bytes());
                word
            }
            CanonicalFieldValue::Unsigned64(value) => {
                let mut word = [0u8; 32];
                word[24..].copy_from_slice(&value.to_be_bytes());
                word
            }
            CanonicalFieldValue::Text(text) => Keccak256::digest(text.as_bytes()).into(),
            CanonicalFieldValue::Bytes(bytes) => Keccak256::digest(bytes).into(),
            CanonicalFieldValue::FixedBytes20(bytes) => {
                let mut word = [0u8; 32];
                word[..20].copy_from_slice(bytes);
                word
            }
        }
    }

    fn intent_type_string(&self, call: &IntentCall) -> String {
        let members = Self::DECLARATION.intent_member_names;
        format!(
            "{}({} {},{} {},{} {},{} {})",
            Self::DECLARATION.intent_struct_name,
            self.type_name(EvmIdentityForm::KIND),
            members.identity,
            self.type_name(Self::DECLARATION.nonce_kind),
            members.nonce,
            self.type_name(Self::DECLARATION.expiry_kind),
            members.expiry,
            call.operation,
            members.call,
        )
    }

    fn call_type_string(&self, call: &IntentCall) -> String {
        let members = call
            .fields
            .iter()
            .map(|field| {
                format!(
                    "{} {}",
                    self.type_name(Self::value_kind(&field.value)),
                    field.name
                )
            })
            .collect::<Vec<_>>()
            .join(",");
        format!("{}({})", call.operation, members)
    }
}

impl IChainForms for EvmForms {
    const DECLARATION: ChainFormsDeclaration = EvmForms::DECLARATION;

    type Identity = EvmIdentityForm;
    type Entitlement = EvmEntitlementForm;
    type Interval = EvmIntervalForm;
    type ChainIdentifier = EvmChainIdentifierForm;
    type SignedIntent = EvmSignedIntentForm;

    fn signed_intent(
        &self,
        _params: SignedIntentParams,
        payload: SignedIntentPayload<'_, Self>,
    ) -> SignedIntentReturn<Self::SignedIntent> {
        if !Self::is_identifier(&payload.call.operation) {
            return Err(SignedIntentErrorReturn::OperationNotIdentifier {
                operation: payload.call.operation.clone(),
            });
        }
        if payload.call.operation == Self::DECLARATION.intent_struct_name {
            return Err(SignedIntentErrorReturn::OperationIsStructName {
                operation: payload.call.operation.clone(),
            });
        }
        for (index, field) in payload.call.fields.iter().enumerate() {
            if !Self::is_identifier(&field.name) {
                return Err(SignedIntentErrorReturn::FieldNotIdentifier {
                    name: field.name.clone(),
                });
            }
            if payload.call.fields[..index]
                .iter()
                .any(|earlier| earlier.name == field.name)
            {
                return Err(SignedIntentErrorReturn::FieldRepeated {
                    name: field.name.clone(),
                });
            }
        }
        let nonce_kind = Self::value_kind(&payload.nonce);
        if nonce_kind != Self::DECLARATION.nonce_kind {
            return Err(SignedIntentErrorReturn::NonceKind {
                expected: Self::DECLARATION.nonce_kind,
                actual: nonce_kind,
            });
        }
        let expiry_kind = Self::value_kind(&payload.expiry);
        if expiry_kind != Self::DECLARATION.expiry_kind {
            return Err(SignedIntentErrorReturn::ExpiryKind {
                expected: Self::DECLARATION.expiry_kind,
                actual: expiry_kind,
            });
        }

        let Ok(chain) = EvmChainIdentifierForm::to_field(ToFieldParams, payload.chain);
        let Ok(_contract) = EvmIdentityForm::to_field(ToFieldParams, payload.contract);
        let Ok(identity) = EvmIdentityForm::to_field(ToFieldParams, payload.identity);

        let intent_type = self.intent_type_string(payload.call);
        let call_type = self.call_type_string(payload.call);
        let mut hasher = Keccak256::new();
        Digest::update(&mut hasher, intent_type.as_bytes());
        Digest::update(&mut hasher, call_type.as_bytes());
        let intent_type_digest: [u8; 32] = hasher.finalize().into();
        let call_type_digest: [u8; 32] = Keccak256::digest(call_type.as_bytes()).into();

        let mut hasher = Keccak256::new();
        Digest::update(&mut hasher, call_type_digest);
        for field in &payload.call.fields {
            Digest::update(&mut hasher, Self::value_word(&field.value));
        }
        let call_hash: [u8; 32] = hasher.finalize().into();

        let mut hasher = Keccak256::new();
        Digest::update(&mut hasher, intent_type_digest);
        Digest::update(&mut hasher, Self::value_word(&identity.field));
        Digest::update(&mut hasher, Self::value_word(&payload.nonce));
        Digest::update(&mut hasher, Self::value_word(&payload.expiry));
        Digest::update(&mut hasher, call_hash);
        let intent_hash: [u8; 32] = hasher.finalize().into();

        let domain_type_digest = Keccak256::digest(EIP712_DOMAIN_TYPE);
        let name_digest = Keccak256::digest(Self::DECLARATION.intent_domain_name);
        let version_digest = Keccak256::digest(Self::DECLARATION.intent_domain_version);
        let mut contract_word = [0u8; 32];
        contract_word[12..].copy_from_slice(&payload.contract.bytes);
        let mut hasher = Keccak256::new();
        Digest::update(&mut hasher, domain_type_digest);
        Digest::update(&mut hasher, name_digest);
        Digest::update(&mut hasher, version_digest);
        Digest::update(&mut hasher, Self::value_word(&chain.field));
        Digest::update(&mut hasher, contract_word);
        let domain_separator: [u8; 32] = hasher.finalize().into();

        let mut hasher = Keccak256::new();
        Digest::update(&mut hasher, EIP712_PREFIX);
        Digest::update(&mut hasher, domain_separator);
        Digest::update(&mut hasher, intent_hash);
        let bytes: [u8; 32] = hasher.finalize().into();
        let Ok(signed_intent) =
            EvmSignedIntentForm::try_new(EvmSignedIntentFormConstructorParams { bytes });
        Ok(SignedIntentSuccessReturn { signed_intent })
    }
}

impl IIdentityForm for EvmIdentityForm {
    type KeyIdentifier = EvmKeyIdentifierForm;

    fn derive_key_identifier(
        _params: DeriveKeyIdentifierParams,
        payload: DeriveKeyIdentifierPayload<'_>,
    ) -> DeriveKeyIdentifierReturn<Self::KeyIdentifier> {
        let mut hasher = Keccak256::new();
        Digest::update(&mut hasher, EvmForms::DECLARATION.key_identifier_domain_tag);
        Digest::update(&mut hasher, [payload.scheme.identifier()]);
        Digest::update(&mut hasher, payload.key);
        let bytes: [u8; 32] = hasher.finalize().into();
        let Ok(key_identifier) =
            EvmKeyIdentifierForm::try_new(EvmKeyIdentifierFormConstructorParams { bytes });
        Ok(DeriveKeyIdentifierSuccessReturn { key_identifier })
    }
}

impl EvmIdentityForm {
    pub fn try_new(params: EvmIdentityFormConstructorParams) -> EvmIdentityFormTryNewReturn {
        Ok(EvmIdentityForm {
            bytes: params.bytes,
        })
    }
}

impl ICanonicalField for EvmIdentityForm {
    const KIND: CanonicalFieldKind = CanonicalFieldKind::FixedBytes20;

    fn to_field(_params: ToFieldParams, payload: &Self) -> ToFieldReturn {
        Ok(ToFieldSuccessReturn {
            field: CanonicalFieldValue::FixedBytes20(payload.bytes),
        })
    }

    fn from_field(_params: FromFieldParams, payload: CanonicalFieldValue) -> FromFieldReturn<Self> {
        let CanonicalFieldValue::FixedBytes20(bytes) = payload else {
            return Err(FromFieldErrorReturn::WrongKind {
                expected: CanonicalFieldKind::FixedBytes20,
            });
        };
        let Ok(value) = EvmIdentityForm::try_new(EvmIdentityFormConstructorParams { bytes });
        Ok(FromFieldSuccessReturn { value })
    }
}

impl EvmEntitlementForm {
    pub fn try_new(params: EvmEntitlementFormConstructorParams) -> EvmEntitlementFormTryNewReturn {
        Ok(EvmEntitlementForm {
            bytes: params.bytes,
        })
    }
}

impl ICanonicalField for EvmEntitlementForm {
    const KIND: CanonicalFieldKind = CanonicalFieldKind::Unsigned256;

    fn to_field(_params: ToFieldParams, payload: &Self) -> ToFieldReturn {
        Ok(ToFieldSuccessReturn {
            field: CanonicalFieldValue::Unsigned256(payload.bytes),
        })
    }

    fn from_field(_params: FromFieldParams, payload: CanonicalFieldValue) -> FromFieldReturn<Self> {
        let CanonicalFieldValue::Unsigned256(bytes) = payload else {
            return Err(FromFieldErrorReturn::WrongKind {
                expected: CanonicalFieldKind::Unsigned256,
            });
        };
        let Ok(value) = EvmEntitlementForm::try_new(EvmEntitlementFormConstructorParams { bytes });
        Ok(FromFieldSuccessReturn { value })
    }
}

impl EvmIntervalForm {
    pub fn try_new(params: EvmIntervalFormConstructorParams) -> EvmIntervalFormTryNewReturn {
        Ok(EvmIntervalForm {
            value: params.value,
        })
    }
}

impl ICanonicalField for EvmIntervalForm {
    const KIND: CanonicalFieldKind = CanonicalFieldKind::Unsigned64;

    fn to_field(_params: ToFieldParams, payload: &Self) -> ToFieldReturn {
        Ok(ToFieldSuccessReturn {
            field: CanonicalFieldValue::Unsigned64(payload.value),
        })
    }

    fn from_field(_params: FromFieldParams, payload: CanonicalFieldValue) -> FromFieldReturn<Self> {
        let CanonicalFieldValue::Unsigned64(value) = payload else {
            return Err(FromFieldErrorReturn::WrongKind {
                expected: CanonicalFieldKind::Unsigned64,
            });
        };
        let Ok(value) = EvmIntervalForm::try_new(EvmIntervalFormConstructorParams { value });
        Ok(FromFieldSuccessReturn { value })
    }
}

impl EvmChainIdentifierForm {
    pub fn try_new(
        params: EvmChainIdentifierFormConstructorParams,
    ) -> EvmChainIdentifierFormTryNewReturn {
        Ok(EvmChainIdentifierForm {
            bytes: params.bytes,
        })
    }
}

impl ICanonicalField for EvmChainIdentifierForm {
    const KIND: CanonicalFieldKind = CanonicalFieldKind::Unsigned256;

    fn to_field(_params: ToFieldParams, payload: &Self) -> ToFieldReturn {
        Ok(ToFieldSuccessReturn {
            field: CanonicalFieldValue::Unsigned256(payload.bytes),
        })
    }

    fn from_field(_params: FromFieldParams, payload: CanonicalFieldValue) -> FromFieldReturn<Self> {
        let CanonicalFieldValue::Unsigned256(bytes) = payload else {
            return Err(FromFieldErrorReturn::WrongKind {
                expected: CanonicalFieldKind::Unsigned256,
            });
        };
        let Ok(value) =
            EvmChainIdentifierForm::try_new(EvmChainIdentifierFormConstructorParams { bytes });
        Ok(FromFieldSuccessReturn { value })
    }
}

impl EvmKeyIdentifierForm {
    pub fn try_new(
        params: EvmKeyIdentifierFormConstructorParams,
    ) -> EvmKeyIdentifierFormTryNewReturn {
        Ok(EvmKeyIdentifierForm {
            bytes: params.bytes,
        })
    }
}

impl ICanonicalField for EvmKeyIdentifierForm {
    const KIND: CanonicalFieldKind = CanonicalFieldKind::FixedBytes32;

    fn to_field(_params: ToFieldParams, payload: &Self) -> ToFieldReturn {
        Ok(ToFieldSuccessReturn {
            field: CanonicalFieldValue::FixedBytes32(payload.bytes),
        })
    }

    fn from_field(_params: FromFieldParams, payload: CanonicalFieldValue) -> FromFieldReturn<Self> {
        let CanonicalFieldValue::FixedBytes32(bytes) = payload else {
            return Err(FromFieldErrorReturn::WrongKind {
                expected: CanonicalFieldKind::FixedBytes32,
            });
        };
        let Ok(value) =
            EvmKeyIdentifierForm::try_new(EvmKeyIdentifierFormConstructorParams { bytes });
        Ok(FromFieldSuccessReturn { value })
    }
}

impl EvmSignedIntentForm {
    pub fn try_new(
        params: EvmSignedIntentFormConstructorParams,
    ) -> EvmSignedIntentFormTryNewReturn {
        Ok(EvmSignedIntentForm {
            bytes: params.bytes,
        })
    }
}

impl ICanonicalField for EvmSignedIntentForm {
    const KIND: CanonicalFieldKind = CanonicalFieldKind::FixedBytes32;

    fn to_field(_params: ToFieldParams, payload: &Self) -> ToFieldReturn {
        Ok(ToFieldSuccessReturn {
            field: CanonicalFieldValue::FixedBytes32(payload.bytes),
        })
    }

    fn from_field(_params: FromFieldParams, payload: CanonicalFieldValue) -> FromFieldReturn<Self> {
        let CanonicalFieldValue::FixedBytes32(bytes) = payload else {
            return Err(FromFieldErrorReturn::WrongKind {
                expected: CanonicalFieldKind::FixedBytes32,
            });
        };
        let Ok(value) =
            EvmSignedIntentForm::try_new(EvmSignedIntentFormConstructorParams { bytes });
        Ok(FromFieldSuccessReturn { value })
    }
}
