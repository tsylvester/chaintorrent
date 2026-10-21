#![allow(
    dead_code,
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::too_many_arguments
)]

use core::convert::Infallible;
use pairing::VerifierGroupArithmetic;
use proof::{
    AlgebraicStatement, DELIVERY_STATEMENT_VERSION_ONE, DeliveryProofComponents, DeliveryRelation,
    IDeliveryProofAdapter, ProofComponentsParams, ProofComponentsPayload, VerifyAdapterPayload,
};
use serde_json::{Map, Value};

use super::interface::{
    IVerifierAdapter, L1DataFee, MockIVerifierAdapterFailureMode,
    MockIVerifierAdapterVerifyOnChainErrorReturn, VERIFIER_INTERFACE_VERSION, VerifierDeclaration,
    VerifyCost, VerifyOnChainErrorReturn, VerifyOnChainParams, VerifyOnChainReturn,
    VerifyOnChainSuccessReturn,
};

#[derive(Default)]
pub struct VerifierDeclarationOverrides {
    pub verifier_forms: Option<&'static [VerifierGroupArithmetic]>,
    pub statement_versions: Option<&'static [u16]>,
    pub adapter_version: Option<u32>,
    pub interface_version: Option<u32>,
}

pub fn build_verifier_declaration(overrides: VerifierDeclarationOverrides) -> VerifierDeclaration {
    VerifierDeclaration {
        verifier_forms: overrides
            .verifier_forms
            .unwrap_or(&[VerifierGroupArithmetic::BothGroups]),
        statement_versions: overrides
            .statement_versions
            .unwrap_or(&[DELIVERY_STATEMENT_VERSION_ONE]),
        adapter_version: overrides.adapter_version.unwrap_or(1),
        interface_version: overrides
            .interface_version
            .unwrap_or(VERIFIER_INTERFACE_VERSION),
    }
}

#[derive(Default)]
pub struct VerifierDeclarationCorruptions {
    pub verifier_forms: Option<Value>,
    pub statement_versions: Option<Value>,
    pub adapter_version: Option<Value>,
    pub interface_version: Option<Value>,
}

pub fn invalidate_verifier_declaration(corruptions: VerifierDeclarationCorruptions) -> Value {
    let declaration = build_verifier_declaration(Default::default());
    let mut fields = Map::new();
    fields.insert(
        "verifier_forms".to_string(),
        Value::Array(
            declaration
                .verifier_forms
                .iter()
                .map(|form| {
                    Value::from(match form {
                        VerifierGroupArithmetic::BothGroups => "BothGroups",
                        VerifierGroupArithmetic::FirstGroupOnly => "FirstGroupOnly",
                    })
                })
                .collect(),
        ),
    );
    fields.insert(
        "statement_versions".to_string(),
        Value::Array(
            declaration
                .statement_versions
                .iter()
                .map(|version| Value::from(*version))
                .collect(),
        ),
    );
    fields.insert(
        "adapter_version".to_string(),
        Value::from(declaration.adapter_version),
    );
    fields.insert(
        "interface_version".to_string(),
        Value::from(declaration.interface_version),
    );
    if let Some(verifier_forms) = corruptions.verifier_forms {
        fields.insert("verifier_forms".to_string(), verifier_forms);
    }
    if let Some(statement_versions) = corruptions.statement_versions {
        fields.insert("statement_versions".to_string(), statement_versions);
    }
    if let Some(adapter_version) = corruptions.adapter_version {
        fields.insert("adapter_version".to_string(), adapter_version);
    }
    if let Some(interface_version) = corruptions.interface_version {
        fields.insert("interface_version".to_string(), interface_version);
    }
    Value::Object(fields)
}

#[derive(Default)]
pub struct VerifyOnChainParamsOverrides;

pub fn build_verify_on_chain_params(
    _overrides: VerifyOnChainParamsOverrides,
) -> VerifyOnChainParams {
    VerifyOnChainParams
}

#[derive(Default)]
pub struct VerifyCostOverrides {
    pub l2_execution_gas: Option<u64>,
    pub l1_data_fee: Option<L1DataFee>,
}

pub fn build_verify_cost(overrides: VerifyCostOverrides) -> VerifyCost {
    VerifyCost {
        l2_execution_gas: overrides.l2_execution_gas.unwrap_or(100_000),
        l1_data_fee: overrides
            .l1_data_fee
            .unwrap_or(L1DataFee::Charged { wei: 1_000 }),
    }
}

#[derive(Default)]
pub struct VerifyCostCorruptions {
    pub l2_execution_gas: Option<Value>,
    pub l1_data_fee: Option<Value>,
}

pub fn invalidate_verify_cost(corruptions: VerifyCostCorruptions) -> Value {
    let cost = build_verify_cost(Default::default());
    let mut fields = Map::new();
    fields.insert(
        "l2_execution_gas".to_string(),
        Value::from(cost.l2_execution_gas),
    );
    fields.insert(
        "l1_data_fee".to_string(),
        match cost.l1_data_fee {
            L1DataFee::Charged { wei } => {
                let mut charged = Map::new();
                let mut wei_field = Map::new();
                wei_field.insert("wei".to_string(), serde_json::to_value(wei).unwrap());
                charged.insert("Charged".to_string(), Value::Object(wei_field));
                Value::Object(charged)
            }
            L1DataFee::NotApplicable => Value::from("NotApplicable"),
        },
    );
    if let Some(l2_execution_gas) = corruptions.l2_execution_gas {
        fields.insert("l2_execution_gas".to_string(), l2_execution_gas);
    }
    if let Some(l1_data_fee) = corruptions.l1_data_fee {
        fields.insert("l1_data_fee".to_string(), l1_data_fee);
    }
    Value::Object(fields)
}

pub(crate) const MOCK_I_VERIFIER_ADAPTER_DECLARATION: VerifierDeclaration = VerifierDeclaration {
    verifier_forms: &[
        VerifierGroupArithmetic::FirstGroupOnly,
        VerifierGroupArithmetic::BothGroups,
    ],
    statement_versions: &[DELIVERY_STATEMENT_VERSION_ONE],
    adapter_version: 1,
    interface_version: VERIFIER_INTERFACE_VERSION,
};

pub(crate) struct MockIVerifierAdapterConstructorParams<'a, D: IDeliveryProofAdapter> {
    pub failure_mode: MockIVerifierAdapterFailureMode,
    pub delivery_proof: &'a D,
}

#[derive(Default)]
pub struct MockIVerifierAdapterConstructorParamsOverrides {
    pub failure_mode: Option<MockIVerifierAdapterFailureMode>,
}

pub(crate) fn build_mock_i_verifier_adapter_constructor_params<'a, D: IDeliveryProofAdapter>(
    delivery_proof: &'a D,
    overrides: MockIVerifierAdapterConstructorParamsOverrides,
) -> MockIVerifierAdapterConstructorParams<'a, D> {
    MockIVerifierAdapterConstructorParams {
        failure_mode: overrides
            .failure_mode
            .unwrap_or(MockIVerifierAdapterFailureMode::NoFailure),
        delivery_proof,
    }
}

pub(crate) struct MockIVerifierAdapter<'a, D: IDeliveryProofAdapter> {
    pub(super) failure_mode: MockIVerifierAdapterFailureMode,
    pub(super) delivery_proof: &'a D,
}

pub(crate) type MockIVerifierAdapterTryNewReturn<'a, D> =
    Result<MockIVerifierAdapter<'a, D>, Infallible>;

impl<'a, D: IDeliveryProofAdapter> MockIVerifierAdapter<'a, D> {
    pub(crate) fn try_new(
        params: MockIVerifierAdapterConstructorParams<'a, D>,
    ) -> MockIVerifierAdapterTryNewReturn<'a, D> {
        Ok(MockIVerifierAdapter {
            failure_mode: params.failure_mode,
            delivery_proof: params.delivery_proof,
        })
    }
}

pub(crate) fn build_mock_i_verifier_adapter<'a, D: IDeliveryProofAdapter>(
    delivery_proof: &'a D,
    overrides: MockIVerifierAdapterConstructorParamsOverrides,
) -> MockIVerifierAdapter<'a, D> {
    let Ok(adapter) = MockIVerifierAdapter::try_new(
        build_mock_i_verifier_adapter_constructor_params(delivery_proof, overrides),
    );
    adapter
}

impl<'a, D: IDeliveryProofAdapter> IVerifierAdapter for MockIVerifierAdapter<'a, D> {
    const DECLARATION: VerifierDeclaration = MOCK_I_VERIFIER_ADAPTER_DECLARATION;

    type DeliveryProof = D;

    fn verify_on_chain(
        &self,
        _params: VerifyOnChainParams,
        payload: VerifyAdapterPayload<'_, D>,
    ) -> VerifyOnChainReturn {
        let statement = match &payload.statement {
            AlgebraicStatement::Mint(_) => DeliveryRelation::Mint,
            AlgebraicStatement::Transfer(_) => DeliveryRelation::Transfer,
        };
        let Ok(components) = self.delivery_proof.proof_components(
            ProofComponentsParams,
            ProofComponentsPayload {
                proof: payload.proof,
            },
        );
        let proof = match components.components {
            DeliveryProofComponents::MintBothGroups { .. }
            | DeliveryProofComponents::MintFirstGroupOnly { .. } => DeliveryRelation::Mint,
            DeliveryProofComponents::TransferBothGroups { .. }
            | DeliveryProofComponents::TransferFirstGroupOnly { .. } => DeliveryRelation::Transfer,
        };
        if statement != proof {
            return Err(VerifyOnChainErrorReturn::RelationMismatch { statement, proof });
        }
        match self.failure_mode {
            MockIVerifierAdapterFailureMode::RpcRefused => Err(VerifyOnChainErrorReturn::Mock(
                MockIVerifierAdapterVerifyOnChainErrorReturn::RpcRefused,
            )),
            MockIVerifierAdapterFailureMode::NoFailure => {
                Ok(VerifyOnChainSuccessReturn::Accepted {
                    cost: build_verify_cost(Default::default()),
                })
            }
        }
    }
}
