#![allow(
    dead_code,
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::too_many_arguments
)]

use super::interface::{BaseChain, BaseChainConstructorParams, BaseChainMockedConstructorParams};
use super::settings::provides::BaseChainSettings;
use crate::factory::provides::IChainForms;
use alloy::primitives::{B256, Bytes, U256};
use alloy::rpc::json_rpc::ErrorPayload;
use alloy::rpc::types::eth::{Block, Log, Transaction};
use alloy::sol_types::SolValue;
use alloy::transports::mock::Asserter;
use serde_json::{Value, json};

#[derive(Default)]
pub struct BaseChainConstructorParamsOverrides;

pub fn build_base_chain_constructor_params<'a>(
    settings: &'a BaseChainSettings,
    _overrides: BaseChainConstructorParamsOverrides,
) -> BaseChainConstructorParams<'a> {
    BaseChainConstructorParams { settings }
}

#[derive(Default)]
pub struct BaseChainMockedConstructorParamsOverrides;

pub fn build_base_chain_mocked_constructor_params<'a>(
    settings: &'a BaseChainSettings,
    endpoint_transport: Asserter,
    settlement_transport: Asserter,
    _overrides: BaseChainMockedConstructorParamsOverrides,
) -> BaseChainMockedConstructorParams<'a> {
    BaseChainMockedConstructorParams {
        settings,
        endpoint_transport,
        settlement_transport,
    }
}

pub fn build_base_chain<F: IChainForms>(
    settings: &BaseChainSettings,
    endpoint_transport: Asserter,
    settlement_transport: Asserter,
) -> BaseChain<F> {
    BaseChain::try_new_mocked(build_base_chain_mocked_constructor_params(
        settings,
        endpoint_transport,
        settlement_transport,
        Default::default(),
    ))
    .expect("built mocked constructor params construct a chain")
}

pub fn push_block(asserter: &Asserter, number: u64, hash: B256, timestamp: u64) {
    let mut block = serde_json::to_value(Block::<Transaction>::default()).unwrap();
    block["number"] = format!("0x{number:x}").into();
    block["hash"] = format!("{hash:#x}").into();
    block["timestamp"] = format!("0x{timestamp:x}").into();
    asserter.push_success(&block);
}

pub fn push_missing_block(asserter: &Asserter) {
    asserter.push_success(&Value::Null);
}

pub fn push_anchor_root(asserter: &Asserter, height: u64) {
    push_call_result(
        asserter,
        Bytes::from((B256::ZERO, U256::from(height)).abi_encode()),
    );
}

pub fn push_call_result(asserter: &Asserter, result: Bytes) {
    asserter.push_success(&result);
}

pub fn push_call_error(asserter: &Asserter, message: &'static str) {
    asserter.push_failure(ErrorPayload::internal_error_message(message.into()));
}

pub fn push_gas_estimate(asserter: &Asserter, gas: u64) {
    asserter.push_success(&format!("0x{gas:x}"));
}

pub fn push_oracle_fee(asserter: &Asserter, fee: U256) {
    push_call_result(asserter, Bytes::from(fee.abi_encode()));
}

pub fn push_logs(asserter: &Asserter, logs: Vec<Log>) {
    asserter.push_success(&logs);
}

pub fn push_transaction(asserter: &Asserter, transaction_hash: B256, input: Bytes) {
    let transaction = json!({
        "type": "0x2",
        "chainId": "0x0",
        "nonce": "0x0",
        "gas": "0x0",
        "maxFeePerGas": "0x0",
        "maxPriorityFeePerGas": "0x0",
        "to": "0x0000000000000000000000000000000000000000",
        "value": "0x0",
        "accessList": [],
        "input": input.to_string(),
        "yParity": "0x0",
        "r": "0x0",
        "s": "0x0",
        "hash": format!("{transaction_hash:#x}"),
        "from": "0x0000000000000000000000000000000000000000",
        "blockHash": "0x0000000000000000000000000000000000000000000000000000000000000000",
        "blockNumber": "0x0",
        "transactionIndex": "0x0",
    });
    asserter.push_success(&transaction);
}

pub fn push_transport_failure(asserter: &Asserter, message: &'static str) {
    asserter.push_failure_msg(message);
}
