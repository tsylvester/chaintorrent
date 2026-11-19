#![allow(clippy::expect_used, clippy::unwrap_used, clippy::panic)]

use core::marker::PhantomData;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll};

use super::interface::{
    BaseChain, BaseChainTryNewErrorReturn, BaseCurrentReferenceAtErrorReturn,
    BaseEstimateFeeErrorReturn, BaseEvaluateAuthorizationBatchErrorReturn,
    BaseEvaluateAuthorizationErrorReturn, BaseRecoverEnvelopeErrorReturn,
};
use super::mock::{
    build_base_chain, build_base_chain_constructor_params, push_anchor_root, push_block,
    push_call_error, push_call_result, push_gas_estimate, push_logs, push_missing_block,
    push_oracle_fee, push_transaction, push_transport_failure,
};
use super::settings::provides::{
    BaseChainSettings, BaseChainSettingsOverrides, build_base_chain_settings,
};
use super::{IEntitlement, IIdentityAccount, Registry};
use crate::factory::provides::{
    AttemptContextOverrides, AuthorizationResult, BatchPage, ChainEndpoint,
    CurrentReferenceAtErrorReturn, CurrentReferenceAtParamsOverrides, EstimateFeeErrorReturn,
    EvaluateAuthorizationBatchErrorReturn, EvaluateAuthorizationErrorReturn, FeeEntry, FeeQuantity,
    IChainAdapter, IChainForms, IEntitlementStateAdapter, ISettlementAdapter, MockChainIdentifier,
    MockEntitlement, MockIChainForms, MockIdentity, MockInterval, RecoverEnvelopeErrorReturn,
    ReferenceAgeErrorReturn, SettlementReferenceConstructorParamsOverrides, SettlementTier,
    TierOfErrorReturn, build_attempt_context, build_batch_page_limit_params,
    build_batch_page_limit_payload, build_chain_identifier_params, build_chain_identifier_payload,
    build_current_reference_at_params, build_current_reference_at_payload, build_endpoint_params,
    build_endpoint_payload, build_entitlement_contract_params, build_entitlement_contract_payload,
    build_estimate_fee_params, build_estimate_fee_payload,
    build_evaluate_authorization_batch_params, build_evaluate_authorization_batch_payload,
    build_evaluate_authorization_params, build_evaluate_authorization_payload, build_fee_request,
    build_recover_envelope_params, build_recover_envelope_payload, build_reference_age_params,
    build_reference_age_payload, build_settlement_reference, build_tier_of_params,
    build_tier_of_payload,
};
use alloy::primitives::{Address, B256, Bytes, LogData, U256};
use alloy::providers::{DynProvider, Provider, ProviderBuilder};
use alloy::rpc::client::RpcClient;
use alloy::rpc::json_rpc::{RequestPacket, ResponsePacket};
use alloy::rpc::types::eth::Log;
use alloy::sol_types::{SolCall, SolEvent, SolValue};
use alloy::transports::mock::{Asserter, MockTransport};
use alloy::transports::{TransportError, TransportFut};
use encoding::{CanonicalFieldValue, FromFieldParams, ICanonicalField, ToFieldParams};
use hashing::{RootConstructorParamsOverrides, build_root};
use tokio::runtime::Builder;
use tower::Service;

#[derive(Clone, Default)]
pub(super) struct RequestLog {
    entries: Arc<Mutex<Vec<serde_json::Value>>>,
}

impl RequestLog {
    pub(super) fn take(&self) -> Vec<serde_json::Value> {
        self.entries
            .lock()
            .expect("the request log is unpoisoned")
            .clone()
    }
}

#[derive(Clone)]
struct RecordingTransport {
    asserter: Asserter,
    log: RequestLog,
}

impl Service<RequestPacket> for RecordingTransport {
    type Response = ResponsePacket;
    type Error = TransportError;
    type Future = TransportFut<'static>;

    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, request: RequestPacket) -> Self::Future {
        self.log
            .entries
            .lock()
            .expect("the request log is unpoisoned")
            .push(serde_json::to_value(&request).expect("the request packet serializes to JSON"));
        let mut transport = MockTransport::new(self.asserter.clone());
        transport.call(request)
    }
}

fn connect_recording(asserter: Asserter, log: &RequestLog) -> DynProvider {
    ProviderBuilder::default()
        .connect_client(RpcClient::new(
            RecordingTransport {
                asserter,
                log: log.clone(),
            },
            false,
        ))
        .erased()
}

fn arrange_recording_base_chain(
    settings: &BaseChainSettings,
) -> (
    BaseChain<MockIChainForms>,
    Asserter,
    Asserter,
    RequestLog,
    RequestLog,
) {
    arrange_recording_base_chain_as(settings)
}

pub(super) fn arrange_recording_base_chain_as<F: IChainForms>(
    settings: &BaseChainSettings,
) -> (BaseChain<F>, Asserter, Asserter, RequestLog, RequestLog) {
    let endpoint_transport = Asserter::new();
    let settlement_transport = Asserter::new();
    let endpoint_log = RequestLog::default();
    let settlement_log = RequestLog::default();
    let endpoint_provider = connect_recording(endpoint_transport.clone(), &endpoint_log);
    let settlement_provider = connect_recording(settlement_transport.clone(), &settlement_log);
    let runtime = Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("the current-thread runtime builds");
    (
        BaseChain {
            endpoint_provider,
            settlement_provider,
            runtime,
            settings: settings.clone(),
            forms: PhantomData,
        },
        endpoint_transport,
        settlement_transport,
        endpoint_log,
        settlement_log,
    )
}

fn unsigned_256(value: u64) -> [u8; 32] {
    let mut bytes = [0u8; 32];
    bytes[24..].copy_from_slice(&value.to_be_bytes());
    bytes
}

fn entitlement_form(value: u64) -> MockEntitlement {
    MockEntitlement::from_field(
        FromFieldParams,
        CanonicalFieldValue::Unsigned256(unsigned_256(value)),
    )
    .expect("the canonical unsigned-256 value is admitted")
    .value
}

fn interval_form(value: u64) -> MockInterval {
    MockInterval::from_field(FromFieldParams, CanonicalFieldValue::Unsigned64(value))
        .expect("the canonical unsigned-64 value is admitted")
        .value
}

pub(super) fn settlement_log(contract: [u8; 20], transaction: B256, data: LogData) -> Log {
    Log {
        inner: alloy::primitives::Log {
            address: Address::from(contract),
            data,
        },
        block_hash: Some(B256::ZERO),
        block_number: Some(700),
        block_timestamp: Some(2000),
        transaction_hash: Some(transaction),
        transaction_index: Some(0),
        log_index: Some(0),
        removed: false,
    }
}

/// Contract: each function of the registry's and the entitlement interface's
///   descriptions has the selector text its element states.
/// Arrange: the bound call types.
/// Act:     read each call type's `SIGNATURE`.
/// Assert:  each equals the literal selector text of its element, in the
///   order the elements state.
#[test]
fn the_abi_descriptions_state_the_selector_text_of_every_function() {
    // Arrange / Act / Assert
    assert_eq!(
        Registry::registerAssetCall::SIGNATURE,
        "registerAsset(bytes32,bytes32,bytes,bytes,bool,uint8)"
    );
    assert_eq!(
        Registry::registerDeploymentCall::SIGNATURE,
        "registerDeployment(bytes32,(bytes32,uint8,uint8,bytes,uint32,uint32,uint64,uint8,(bytes32,bytes,bytes)[],uint8,uint8,uint8,uint8,uint8,uint16,uint32,uint32,uint8,bytes,uint8,bytes,bytes[],bytes32),bytes,bool)"
    );
    assert_eq!(
        Registry::addSidecarCall::SIGNATURE,
        "addSidecar(bytes32,bytes32,bytes,bytes)"
    );
    assert_eq!(
        Registry::registerParameterSetCall::SIGNATURE,
        "registerParameterSet(bytes32,bytes32,bytes32,bytes,uint8)"
    );
    assert_eq!(
        Registry::retireParameterSetCall::SIGNATURE,
        "retireParameterSet(bytes32)"
    );
    assert_eq!(
        Registry::rotateSourceKeyCall::SIGNATURE,
        "rotateSourceKey(uint8,bytes32,bytes,uint64)"
    );
    assert_eq!(
        Registry::revokeSourceKeyCall::SIGNATURE,
        "revokeSourceKey(uint8,bytes32)"
    );
    assert_eq!(Registry::resolveCall::SIGNATURE, "resolve(bytes32,bytes)");
    assert_eq!(
        Registry::resolveBatchCall::SIGNATURE,
        "resolveBatch(bytes32[],bytes[])"
    );
    assert_eq!(
        Registry::requestGrantsCall::SIGNATURE,
        "requestGrants(address,bytes32[],(uint64,uint64,uint8,bytes,bytes))"
    );
    assert_eq!(
        Registry::withdrawRequestsCall::SIGNATURE,
        "withdrawRequests(address,bytes32[],(uint64,uint64,uint8,bytes,bytes))"
    );
    assert_eq!(
        Registry::closeRequestByGrantCall::SIGNATURE,
        "closeRequestByGrant(address,bytes32)"
    );
    assert_eq!(
        Registry::openRequestsCall::SIGNATURE,
        "openRequests(bytes32,uint256)"
    );
    assert_eq!(
        Registry::evaluateAuthorizationCall::SIGNATURE,
        "evaluateAuthorization((bytes32,bytes32,address,uint256,address,uint8,bytes32,uint64,uint64))"
    );
    assert_eq!(
        Registry::evaluateAuthorizationBatchCall::SIGNATURE,
        "evaluateAuthorizationBatch((bytes32,bytes32,address,uint256,address,uint8,bytes32,uint64,uint64)[],uint256)"
    );
    assert_eq!(
        IEntitlement::mintCall::SIGNATURE,
        "mint(address,uint256,address,bytes,bytes,(uint64,uint64,uint8,bytes,bytes))"
    );
    assert_eq!(
        IEntitlement::deliverCall::SIGNATURE,
        "deliver(address,uint256,bytes,bytes,bytes,(uint64,uint64,uint8,bytes,bytes))"
    );
    assert_eq!(
        IEntitlement::grantCall::SIGNATURE,
        "grant(address,uint256,uint256,address,bytes,bytes,bytes,(uint64,uint64,uint8,bytes,bytes))"
    );
    assert_eq!(
        IEntitlement::replaceCall::SIGNATURE,
        "replace(address,uint256,bytes,bytes,(uint64,uint64,uint8,bytes,bytes))"
    );
    assert_eq!(IEntitlement::ownerOfCall::SIGNATURE, "ownerOf(uint256)");
    assert_eq!(
        IEntitlement::intervalOfCall::SIGNATURE,
        "intervalOf(uint256)"
    );
    assert_eq!(
        IEntitlement::envelopeDigestOfCall::SIGNATURE,
        "envelopeDigestOf(uint256)"
    );
    assert_eq!(IEntitlement::assetOfCall::SIGNATURE, "assetOf(uint256)");
    assert_eq!(
        IEntitlement::parameterSetOfCall::SIGNATURE,
        "parameterSetOf(uint256)"
    );
    assert_eq!(
        IEntitlement::liveEntitlementCountOfCall::SIGNATURE,
        "liveEntitlementCountOf(bytes32)"
    );
}
// the_abi_descriptions_state_the_selector_text_of_every_function

/// Contract: each event has the signature its element states.
/// Arrange: the bound event types.
/// Act:     read each `SIGNATURE`.
/// Assert:  each equals the literal event text of its element.
#[test]
fn the_abi_descriptions_state_the_signature_text_of_every_event() {
    // Arrange / Act / Assert
    assert_eq!(
        Registry::AssetRegistered::SIGNATURE,
        "AssetRegistered(bytes32,bytes32)"
    );
    assert_eq!(
        Registry::DeploymentRegistered::SIGNATURE,
        "DeploymentRegistered(bytes32,bytes32)"
    );
    assert_eq!(
        Registry::SidecarAdded::SIGNATURE,
        "SidecarAdded(bytes32,bytes32)"
    );
    assert_eq!(
        Registry::GrantsRequested::SIGNATURE,
        "GrantsRequested(address,bytes32[])"
    );
    assert_eq!(
        Registry::RequestsWithdrawn::SIGNATURE,
        "RequestsWithdrawn(address,bytes32[])"
    );
    assert_eq!(
        Registry::RequestClosedByGrant::SIGNATURE,
        "RequestClosedByGrant(address,bytes32)"
    );
    assert_eq!(
        IEntitlement::Minted::SIGNATURE,
        "Minted(uint256,address,uint64,bytes,bytes)"
    );
    assert_eq!(
        IEntitlement::Granted::SIGNATURE,
        "Granted(uint256,uint256,address,bytes,bytes)"
    );
    assert_eq!(
        IEntitlement::Delivered::SIGNATURE,
        "Delivered(uint256,address,address,uint64,bytes,bytes)"
    );
    assert_eq!(
        IEntitlement::Replaced::SIGNATURE,
        "Replaced(uint256,uint64,bytes,bytes)"
    );
}
// the_abi_descriptions_state_the_signature_text_of_every_event

/// Contract: an endpoint text whose scheme the built-in transports do not
///   serve is returned as the transport's error.
/// Arrange: settings from `build_base_chain_settings` with `endpoint`
///   text `ftp://example.test`.
/// Act:     `BaseChain::try_new`.
/// Assert:  the error matches `Endpoint(error)` with the error's rendered
///   text containing `invalid URL scheme: ftp`.
#[test]
fn try_new_refuses_an_endpoint_text_the_transport_refuses() {
    // Arrange
    let settings = build_base_chain_settings(BaseChainSettingsOverrides {
        endpoint: Some(ChainEndpoint {
            text: "ftp://example.test".to_string(),
        }),
        ..Default::default()
    });

    // Act
    let result = BaseChain::<MockIChainForms>::try_new(build_base_chain_constructor_params(
        &settings,
        Default::default(),
    ));

    // Assert
    let Err(BaseChainTryNewErrorReturn::Endpoint(error)) = result else {
        panic!("expected an endpoint transport refusal");
    };
    assert!(
        error.to_string().contains("invalid URL scheme: ftp"),
        "{error}"
    );
}
// try_new_refuses_an_endpoint_text_the_transport_refuses

/// Contract: a settlement endpoint text whose scheme the built-in transports
///   do not serve is returned as the transport's error.
/// Arrange: settings from `build_base_chain_settings` with
///   `settlement_endpoint` text `ftp://example.test`.
/// Act:     `BaseChain::try_new`.
/// Assert:  the error matches `SettlementEndpoint(error)` with the error's
///   rendered text containing `invalid URL scheme: ftp`.
#[test]
fn try_new_refuses_a_settlement_endpoint_text_the_transport_refuses() {
    // Arrange
    let settings = build_base_chain_settings(BaseChainSettingsOverrides {
        settlement_endpoint: Some(ChainEndpoint {
            text: "ftp://example.test".to_string(),
        }),
        ..Default::default()
    });

    // Act
    let result = BaseChain::<MockIChainForms>::try_new(build_base_chain_constructor_params(
        &settings,
        Default::default(),
    ));

    // Assert
    let Err(BaseChainTryNewErrorReturn::SettlementEndpoint(error)) = result else {
        panic!("expected a settlement endpoint transport refusal");
    };
    assert!(
        error.to_string().contains("invalid URL scheme: ftp"),
        "{error}"
    );
}
// try_new_refuses_a_settlement_endpoint_text_the_transport_refuses

/// Contract: `Included` reads the latest head and records that head's
///   number and hash.
/// Arrange: the endpoint transport answering block `1000` with hash
///   `[0x0a; 32]` and timestamp `5000`.
/// Act:     `current_reference_at` for `Included`.
/// Assert:  the reference's ordinal and identity equal `1000` and
///   `[0x0a; 32]`, and the request the transport recorded names the block
///   tag `latest`.
#[test]
fn current_reference_at_returns_the_latest_head_for_included() {
    // Arrange
    let settings = build_base_chain_settings(Default::default());
    let (chain, endpoint_transport, _settlement_transport, endpoint_log, _settlement_log) =
        arrange_recording_base_chain(&settings);
    push_block(&endpoint_transport, 1000, B256::from([0x0a; 32]), 5000);

    // Act
    let success = chain
        .current_reference_at(
            build_current_reference_at_params(CurrentReferenceAtParamsOverrides {
                tier: Some(SettlementTier::Included),
            }),
            build_current_reference_at_payload(Default::default()),
        )
        .expect("the latest head is attained");

    // Assert
    assert_eq!(success.reference.ordinal(), 1000);
    assert_eq!(success.reference.identity(), &[0x0a; 32]);
    let requests = endpoint_log.take();
    let request = requests.last().expect("the transport received a request");
    assert_eq!(request["method"], "eth_getBlockByNumber");
    assert_eq!(request["params"][0], "latest");
}
// current_reference_at_returns_the_latest_head_for_included

/// Contract: `Soft` reads the safe head and records that head's number and
///   hash.
/// Arrange: the endpoint transport answering block `900` with hash
///   `[0x0b; 32]` and timestamp `4000`.
/// Act:     `current_reference_at` for `Soft`.
/// Assert:  the reference's ordinal and identity equal `900` and
///   `[0x0b; 32]`, and the request the transport recorded names the block
///   tag `safe`.
#[test]
fn current_reference_at_returns_the_safe_head_for_soft() {
    // Arrange
    let settings = build_base_chain_settings(Default::default());
    let (chain, endpoint_transport, _settlement_transport, endpoint_log, _settlement_log) =
        arrange_recording_base_chain(&settings);
    push_block(&endpoint_transport, 900, B256::from([0x0b; 32]), 4000);

    // Act
    let success = chain
        .current_reference_at(
            build_current_reference_at_params(CurrentReferenceAtParamsOverrides {
                tier: Some(SettlementTier::Soft),
            }),
            build_current_reference_at_payload(Default::default()),
        )
        .expect("the safe head is attained");

    // Assert
    assert_eq!(success.reference.ordinal(), 900);
    assert_eq!(success.reference.identity(), &[0x0b; 32]);
    let requests = endpoint_log.take();
    let request = requests.last().expect("the transport received a request");
    assert_eq!(request["method"], "eth_getBlockByNumber");
    assert_eq!(request["params"][0], "safe");
}
// current_reference_at_returns_the_safe_head_for_soft

/// Contract: `Hard` reads the finalized head and records that head's
///   number and hash.
/// Arrange: the endpoint transport answering block `800` with hash
///   `[0x0c; 32]` and timestamp `3000`.
/// Act:     `current_reference_at` for `Hard`.
/// Assert:  the reference's ordinal and identity equal `800` and
///   `[0x0c; 32]`, and the request the transport recorded names the block
///   tag `finalized`.
#[test]
fn current_reference_at_returns_the_finalized_head_for_hard() {
    // Arrange
    let settings = build_base_chain_settings(Default::default());
    let (chain, endpoint_transport, _settlement_transport, endpoint_log, _settlement_log) =
        arrange_recording_base_chain(&settings);
    push_block(&endpoint_transport, 800, B256::from([0x0c; 32]), 3000);

    // Act
    let success = chain
        .current_reference_at(
            build_current_reference_at_params(CurrentReferenceAtParamsOverrides {
                tier: Some(SettlementTier::Hard),
            }),
            build_current_reference_at_payload(Default::default()),
        )
        .expect("the finalized head is attained");

    // Assert
    assert_eq!(success.reference.ordinal(), 800);
    assert_eq!(success.reference.identity(), &[0x0c; 32]);
    let requests = endpoint_log.take();
    let request = requests.last().expect("the transport received a request");
    assert_eq!(request["method"], "eth_getBlockByNumber");
    assert_eq!(request["params"][0], "finalized");
}
// current_reference_at_returns_the_finalized_head_for_hard

/// Contract: `Settled` reads the anchor's height from the settlement
///   endpoint and the block from the endpoint.
/// Arrange: the settlement transport answering an anchor root of height
///   `700` and the endpoint transport answering block `700` with hash
///   `[0x0d; 32]` and timestamp `2000`.
/// Act:     `current_reference_at` for `Settled`.
/// Assert:  the reference's ordinal is `700`, the settlement transport
///   recorded the anchor call, and the endpoint transport recorded the
///   block read.
#[test]
fn current_reference_at_reads_the_anchor_state_from_the_settlement_endpoint_for_settled() {
    // Arrange
    let settings = build_base_chain_settings(Default::default());
    let (chain, endpoint_transport, settlement_transport, endpoint_log, settlement_log) =
        arrange_recording_base_chain(&settings);
    push_anchor_root(&settlement_transport, 700);
    push_block(&endpoint_transport, 700, B256::from([0x0d; 32]), 2000);

    // Act
    let success = chain
        .current_reference_at(
            build_current_reference_at_params(CurrentReferenceAtParamsOverrides {
                tier: Some(SettlementTier::Settled),
            }),
            build_current_reference_at_payload(Default::default()),
        )
        .expect("the settled head is attained");

    // Assert
    assert_eq!(success.reference.ordinal(), 700);
    let settlement_requests = settlement_log.take();
    assert_eq!(settlement_requests.len(), 1);
    assert_eq!(settlement_requests[0]["method"], "eth_call");
    let endpoint_requests = endpoint_log.take();
    assert_eq!(endpoint_requests.len(), 1);
    assert_eq!(endpoint_requests[0]["method"], "eth_getBlockByNumber");
    assert_eq!(endpoint_requests[0]["params"][0], "0x2bc");
}
// current_reference_at_reads_the_anchor_state_from_the_settlement_endpoint_for_settled

/// Contract: a transport failure reaches the caller intact.
/// Arrange: a transport failure with the message `refused` pushed to the
///   endpoint transport.
/// Act:     `current_reference_at` for `Included`.
/// Assert:  the error matches
///   `Base(BaseCurrentReferenceAtErrorReturn::Transport(error))` with the
///   error's rendered text containing `refused`.
#[test]
fn current_reference_at_returns_a_transport_failure_whole() {
    // Arrange
    let settings = build_base_chain_settings(Default::default());
    let endpoint_transport = Asserter::new();
    let settlement_transport = Asserter::new();
    let chain: BaseChain<MockIChainForms> =
        build_base_chain(&settings, endpoint_transport.clone(), settlement_transport);
    push_transport_failure(&endpoint_transport, "refused");

    // Act
    let result = chain.current_reference_at(
        build_current_reference_at_params(CurrentReferenceAtParamsOverrides {
            tier: Some(SettlementTier::Included),
        }),
        build_current_reference_at_payload(Default::default()),
    );

    // Assert
    let Err(CurrentReferenceAtErrorReturn::Base(BaseCurrentReferenceAtErrorReturn::Transport(
        error,
    ))) = result
    else {
        panic!("expected an endpoint transport failure");
    };
    assert!(error.to_string().contains("refused"), "{error}");
}
// current_reference_at_returns_a_transport_failure_whole

/// Contract: an absent block is the tier not attained.
/// Arrange: the endpoint transport answering `null` for the safe block.
/// Act:     `current_reference_at` for `Soft`.
/// Assert:  `Err(NotAttained { tier: SettlementTier::Soft })`.
#[test]
fn current_reference_at_refuses_a_tier_the_chain_has_not_attained() {
    // Arrange
    let settings = build_base_chain_settings(Default::default());
    let endpoint_transport = Asserter::new();
    let settlement_transport = Asserter::new();
    let chain: BaseChain<MockIChainForms> =
        build_base_chain(&settings, endpoint_transport.clone(), settlement_transport);
    push_missing_block(&endpoint_transport);

    // Act
    let result = chain.current_reference_at(
        build_current_reference_at_params(CurrentReferenceAtParamsOverrides {
            tier: Some(SettlementTier::Soft),
        }),
        build_current_reference_at_payload(Default::default()),
    );

    // Assert
    assert!(matches!(
        result,
        Err(CurrentReferenceAtErrorReturn::NotAttained {
            tier: SettlementTier::Soft
        })
    ));
}
// current_reference_at_refuses_a_tier_the_chain_has_not_attained

/// Contract: an anchor call returning bytes the binding cannot decode is
///   refused.
/// Arrange: the settlement transport answering `0x1234`.
/// Act:     `current_reference_at` for `Settled`.
/// Assert:  the error matches
///   `Base(BaseCurrentReferenceAtErrorReturn::AnchorDecode)`.
#[test]
fn current_reference_at_refuses_an_anchor_return_that_does_not_decode() {
    // Arrange
    let settings = build_base_chain_settings(Default::default());
    let endpoint_transport = Asserter::new();
    let settlement_transport = Asserter::new();
    let chain: BaseChain<MockIChainForms> =
        build_base_chain(&settings, endpoint_transport, settlement_transport.clone());
    push_call_result(&settlement_transport, Bytes::from(vec![0x12, 0x34]));

    // Act
    let result = chain.current_reference_at(
        build_current_reference_at_params(CurrentReferenceAtParamsOverrides {
            tier: Some(SettlementTier::Settled),
        }),
        build_current_reference_at_payload(Default::default()),
    );

    // Assert
    assert!(matches!(
        result,
        Err(CurrentReferenceAtErrorReturn::Base(
            BaseCurrentReferenceAtErrorReturn::AnchorDecode
        ))
    ));
}
// current_reference_at_refuses_an_anchor_return_that_does_not_decode

/// Contract: the tier is the highest whose current reference is at or
///   after the reference.
/// Arrange: the four heads — latest `1000`, safe `900`, finalized `800`,
///   anchor `700` — and a reference at ordinal `850` with the hash block
///   `850` holds.
/// Act:     `tier_of`.
/// Assert:  `Soft`.
#[test]
fn tier_of_returns_the_highest_tier_a_reference_has_attained() {
    // Arrange
    let settings = build_base_chain_settings(Default::default());
    let endpoint_transport = Asserter::new();
    let settlement_transport = Asserter::new();
    let chain: BaseChain<MockIChainForms> = build_base_chain(
        &settings,
        endpoint_transport.clone(),
        settlement_transport.clone(),
    );
    push_block(&endpoint_transport, 850, B256::from([0x85; 32]), 4500);
    push_block(&endpoint_transport, 1000, B256::from([0x0a; 32]), 5000);
    push_anchor_root(&settlement_transport, 700);
    push_block(&endpoint_transport, 700, B256::from([0x0d; 32]), 2000);
    push_block(&endpoint_transport, 800, B256::from([0x0c; 32]), 3000);
    push_block(&endpoint_transport, 900, B256::from([0x0b; 32]), 4000);
    push_block(&endpoint_transport, 1000, B256::from([0x0a; 32]), 5000);
    let reference = build_settlement_reference(SettlementReferenceConstructorParamsOverrides {
        ordinal: Some(850),
        identity: Some([0x85; 32]),
    });

    // Act
    let result = chain.tier_of(
        build_tier_of_params(Default::default()),
        build_tier_of_payload(&reference, Default::default()),
    );

    // Assert
    assert!(matches!(
        result,
        Ok(success) if success.tier == SettlementTier::Soft
    ));
}
// tier_of_returns_the_highest_tier_a_reference_has_attained

/// Contract: a tier the chain has not attained is skipped.
/// Arrange: the endpoint answering `null` for the safe block and the
///   other heads as arranged, and a reference at `950`.
/// Act:     `tier_of`.
/// Assert:  `Included`.
#[test]
fn tier_of_skips_a_tier_the_chain_has_not_attained() {
    // Arrange
    let settings = build_base_chain_settings(Default::default());
    let endpoint_transport = Asserter::new();
    let settlement_transport = Asserter::new();
    let chain: BaseChain<MockIChainForms> = build_base_chain(
        &settings,
        endpoint_transport.clone(),
        settlement_transport.clone(),
    );
    push_block(&endpoint_transport, 950, B256::from([0x95; 32]), 4500);
    push_block(&endpoint_transport, 1000, B256::from([0x0a; 32]), 5000);
    push_anchor_root(&settlement_transport, 700);
    push_block(&endpoint_transport, 700, B256::from([0x0d; 32]), 2000);
    push_block(&endpoint_transport, 800, B256::from([0x0c; 32]), 3000);
    push_missing_block(&endpoint_transport);
    push_block(&endpoint_transport, 1000, B256::from([0x0a; 32]), 5000);
    let reference = build_settlement_reference(SettlementReferenceConstructorParamsOverrides {
        ordinal: Some(950),
        identity: Some([0x95; 32]),
    });

    // Act
    let result = chain.tier_of(
        build_tier_of_params(Default::default()),
        build_tier_of_payload(&reference, Default::default()),
    );

    // Assert
    assert!(matches!(
        result,
        Ok(success) if success.tier == SettlementTier::Included
    ));
}
// tier_of_skips_a_tier_the_chain_has_not_attained

/// Contract: a reference above the latest number is not included.
/// Arrange: a reference at ordinal `1001` and the endpoint answering
///   `null` for block `1001`.
/// Act:     `tier_of`.
/// Assert:  `Err(NotIncluded { reference })` with that reference.
#[test]
fn tier_of_refuses_a_reference_past_the_latest_head() {
    // Arrange
    let settings = build_base_chain_settings(Default::default());
    let endpoint_transport = Asserter::new();
    let settlement_transport = Asserter::new();
    let chain: BaseChain<MockIChainForms> =
        build_base_chain(&settings, endpoint_transport.clone(), settlement_transport);
    push_missing_block(&endpoint_transport);
    let reference = build_settlement_reference(SettlementReferenceConstructorParamsOverrides {
        ordinal: Some(1001),
        identity: Some([0x99; 32]),
    });

    // Act
    let result = chain.tier_of(
        build_tier_of_params(Default::default()),
        build_tier_of_payload(&reference, Default::default()),
    );

    // Assert
    match result {
        Err(TierOfErrorReturn::NotIncluded {
            reference: returned,
        }) => {
            assert_eq!(returned, reference);
        }
        _ => panic!("expected NotIncluded"),
    }
}
// tier_of_refuses_a_reference_past_the_latest_head

/// Contract: a reference whose hash the chain does not hold at its number
///   is reorganized.
/// Arrange: a reference at ordinal `900` with the hash `[0x99; 32]` while
///   block `900` holds `[0x0b; 32]` and the latest head is `1000`.
/// Act:     `tier_of`.
/// Assert:  `Err(Reorganized { reference })`.
#[test]
fn tier_of_refuses_a_reference_whose_hash_differs() {
    // Arrange
    let settings = build_base_chain_settings(Default::default());
    let endpoint_transport = Asserter::new();
    let settlement_transport = Asserter::new();
    let chain: BaseChain<MockIChainForms> =
        build_base_chain(&settings, endpoint_transport.clone(), settlement_transport);
    push_block(&endpoint_transport, 900, B256::from([0x0b; 32]), 4000);
    push_block(&endpoint_transport, 1000, B256::from([0x0a; 32]), 5000);
    let reference = build_settlement_reference(SettlementReferenceConstructorParamsOverrides {
        ordinal: Some(900),
        identity: Some([0x99; 32]),
    });

    // Act
    let result = chain.tier_of(
        build_tier_of_params(Default::default()),
        build_tier_of_payload(&reference, Default::default()),
    );

    // Assert
    match result {
        Err(TierOfErrorReturn::Reorganized {
            reference: returned,
        }) => {
            assert_eq!(returned, reference);
        }
        _ => panic!("expected Reorganized"),
    }
}
// tier_of_refuses_a_reference_whose_hash_differs

/// Contract: the age is the latest block's timestamp less the reference
///   block's.
/// Arrange: the latest block `1000` at `5000` seconds and block `900` at
///   `4000`, and a reference at `900` holding `[0x0b; 32]`.
/// Act:     `reference_age` for the reference at `900`.
/// Assert:  seconds `1000`.
#[test]
fn reference_age_is_the_latest_timestamp_less_the_reference_timestamp() {
    // Arrange
    let settings = build_base_chain_settings(Default::default());
    let endpoint_transport = Asserter::new();
    let settlement_transport = Asserter::new();
    let chain: BaseChain<MockIChainForms> =
        build_base_chain(&settings, endpoint_transport.clone(), settlement_transport);
    push_block(&endpoint_transport, 900, B256::from([0x0b; 32]), 4000);
    push_block(&endpoint_transport, 1000, B256::from([0x0a; 32]), 5000);
    let reference = build_settlement_reference(SettlementReferenceConstructorParamsOverrides {
        ordinal: Some(900),
        identity: Some([0x0b; 32]),
    });

    // Act
    let success = chain
        .reference_age(
            build_reference_age_params(Default::default()),
            build_reference_age_payload(&reference, Default::default()),
        )
        .expect("the reference is included");

    // Assert
    assert_eq!(success.age.seconds, 1000);
}
// reference_age_is_the_latest_timestamp_less_the_reference_timestamp

/// Contract: subtraction saturates.
/// Arrange: the latest block `1000` at `5000` seconds and block `900`
///   answered at `6000`, and a reference at `900` holding `[0x0b; 32]`.
/// Act:     `reference_age`.
/// Assert:  seconds `0`.
#[test]
fn reference_age_is_zero_when_the_endpoint_answers_a_reference_block_later_than_the_latest() {
    // Arrange
    let settings = build_base_chain_settings(Default::default());
    let endpoint_transport = Asserter::new();
    let settlement_transport = Asserter::new();
    let chain: BaseChain<MockIChainForms> =
        build_base_chain(&settings, endpoint_transport.clone(), settlement_transport);
    push_block(&endpoint_transport, 900, B256::from([0x0b; 32]), 6000);
    push_block(&endpoint_transport, 1000, B256::from([0x0a; 32]), 5000);
    let reference = build_settlement_reference(SettlementReferenceConstructorParamsOverrides {
        ordinal: Some(900),
        identity: Some([0x0b; 32]),
    });

    // Act
    let success = chain
        .reference_age(
            build_reference_age_params(Default::default()),
            build_reference_age_payload(&reference, Default::default()),
        )
        .expect("the reference is included");

    // Assert
    assert_eq!(success.age.seconds, 0);
}
// reference_age_is_zero_when_the_endpoint_answers_a_reference_block_later_than_the_latest

/// Contract: a reference above the latest number is not included.
/// Arrange: a reference at `1001` and the endpoint answering `null` for
///   block `1001`.
/// Act:     `reference_age`.
/// Assert:  `Err(NotIncluded { reference })`.
#[test]
fn reference_age_refuses_a_reference_past_the_latest_head() {
    // Arrange
    let settings = build_base_chain_settings(Default::default());
    let endpoint_transport = Asserter::new();
    let settlement_transport = Asserter::new();
    let chain: BaseChain<MockIChainForms> =
        build_base_chain(&settings, endpoint_transport.clone(), settlement_transport);
    push_missing_block(&endpoint_transport);
    let reference = build_settlement_reference(SettlementReferenceConstructorParamsOverrides {
        ordinal: Some(1001),
        identity: Some([0x99; 32]),
    });

    // Act
    let result = chain.reference_age(
        build_reference_age_params(Default::default()),
        build_reference_age_payload(&reference, Default::default()),
    );

    // Assert
    match result {
        Err(ReferenceAgeErrorReturn::NotIncluded {
            reference: returned,
        }) => {
            assert_eq!(returned, reference);
        }
        _ => panic!("expected NotIncluded"),
    }
}
// reference_age_refuses_a_reference_past_the_latest_head

/// Contract: the registry's `uint8` result `0` maps to `Authorized`.
/// Arrange: the call returning `0`.
/// Act:     `evaluate_authorization`.
/// Assert:  `Authorized`.
#[test]
fn evaluate_authorization_returns_authorized_for_zero() {
    // Arrange
    let settings = build_base_chain_settings(Default::default());
    let endpoint_transport = Asserter::new();
    let settlement_transport = Asserter::new();
    let chain: BaseChain<MockIChainForms> =
        build_base_chain(&settings, endpoint_transport.clone(), settlement_transport);
    push_call_result(
        &endpoint_transport,
        Bytes::from(Registry::evaluateAuthorizationCall::abi_encode_returns(
            &0u8,
        )),
    );
    let context = build_attempt_context::<MockIChainForms>(Default::default());

    // Act
    let success = chain
        .evaluate_authorization(
            build_evaluate_authorization_params(Default::default()),
            build_evaluate_authorization_payload(&context, Default::default()),
        )
        .expect("the call decodes");

    // Assert
    assert_eq!(success.result, AuthorizationResult::Authorized);
}
// evaluate_authorization_returns_authorized_for_zero

/// Contract: the registry's `uint8` result `1` maps to
///   `PendingSettlement`.
/// Arrange: the call returning `1`.
/// Act:     `evaluate_authorization`.
/// Assert:  `PendingSettlement`.
#[test]
fn evaluate_authorization_returns_pending_settlement_for_one() {
    // Arrange
    let settings = build_base_chain_settings(Default::default());
    let endpoint_transport = Asserter::new();
    let settlement_transport = Asserter::new();
    let chain: BaseChain<MockIChainForms> =
        build_base_chain(&settings, endpoint_transport.clone(), settlement_transport);
    push_call_result(
        &endpoint_transport,
        Bytes::from(Registry::evaluateAuthorizationCall::abi_encode_returns(
            &1u8,
        )),
    );
    let context = build_attempt_context::<MockIChainForms>(Default::default());

    // Act
    let success = chain
        .evaluate_authorization(
            build_evaluate_authorization_params(Default::default()),
            build_evaluate_authorization_payload(&context, Default::default()),
        )
        .expect("the call decodes");

    // Assert
    assert_eq!(success.result, AuthorizationResult::PendingSettlement);
}
// evaluate_authorization_returns_pending_settlement_for_one

/// Contract: the registry's `uint8` result `2` maps to `Denied`.
/// Arrange: the call returning `2`.
/// Act:     `evaluate_authorization`.
/// Assert:  `Denied`.
#[test]
fn evaluate_authorization_returns_denied_for_two() {
    // Arrange
    let settings = build_base_chain_settings(Default::default());
    let endpoint_transport = Asserter::new();
    let settlement_transport = Asserter::new();
    let chain: BaseChain<MockIChainForms> =
        build_base_chain(&settings, endpoint_transport.clone(), settlement_transport);
    push_call_result(
        &endpoint_transport,
        Bytes::from(Registry::evaluateAuthorizationCall::abi_encode_returns(
            &2u8,
        )),
    );
    let context = build_attempt_context::<MockIChainForms>(Default::default());

    // Act
    let success = chain
        .evaluate_authorization(
            build_evaluate_authorization_params(Default::default()),
            build_evaluate_authorization_payload(&context, Default::default()),
        )
        .expect("the call decodes");

    // Assert
    assert_eq!(success.result, AuthorizationResult::Denied);
}
// evaluate_authorization_returns_denied_for_two

/// Contract: a result other than `0`, `1`, or `2` is refused.
/// Arrange: the call returning `3`.
/// Act:     `evaluate_authorization`.
/// Assert:  the error matches
///   `Base(BaseEvaluateAuthorizationErrorReturn::UnknownAuthorizationState { value: 3 })`.
#[test]
fn evaluate_authorization_refuses_a_state_outside_the_three() {
    // Arrange
    let settings = build_base_chain_settings(Default::default());
    let endpoint_transport = Asserter::new();
    let settlement_transport = Asserter::new();
    let chain: BaseChain<MockIChainForms> =
        build_base_chain(&settings, endpoint_transport.clone(), settlement_transport);
    push_call_result(
        &endpoint_transport,
        Bytes::from(Registry::evaluateAuthorizationCall::abi_encode_returns(
            &3u8,
        )),
    );
    let context = build_attempt_context::<MockIChainForms>(Default::default());

    // Act
    let result = chain.evaluate_authorization(
        build_evaluate_authorization_params(Default::default()),
        build_evaluate_authorization_payload(&context, Default::default()),
    );

    // Assert
    assert!(matches!(
        result,
        Err(EvaluateAuthorizationErrorReturn::Base(
            BaseEvaluateAuthorizationErrorReturn::UnknownAuthorizationState { value: 3 }
        ))
    ));
}
// evaluate_authorization_refuses_a_state_outside_the_three

/// Contract: the view is read at the canonical block whose hash the
///   reference records.
/// Arrange: a reference at `900` with `[0x0b; 32]` and the call
///   returning `0`.
/// Act:     `evaluate_authorization`.
/// Assert:  the request the transport recorded names the block hash `0x`
///   followed by sixty-four `b` digits, the literal of `[0x0b; 32]`, and
///   carries the canonical-block requirement.
#[test]
fn evaluate_authorization_reads_at_the_block_of_the_reference() {
    // Arrange
    let settings = build_base_chain_settings(Default::default());
    let (chain, endpoint_transport, _settlement_transport, endpoint_log, _settlement_log) =
        arrange_recording_base_chain(&settings);
    push_call_result(
        &endpoint_transport,
        Bytes::from(Registry::evaluateAuthorizationCall::abi_encode_returns(
            &0u8,
        )),
    );
    let reference = build_settlement_reference(SettlementReferenceConstructorParamsOverrides {
        ordinal: Some(900),
        identity: Some([0x0b; 32]),
    });
    let context = build_attempt_context::<MockIChainForms>(AttemptContextOverrides {
        reference: Some(reference),
        ..Default::default()
    });

    // Act
    let success = chain
        .evaluate_authorization(
            build_evaluate_authorization_params(Default::default()),
            build_evaluate_authorization_payload(&context, Default::default()),
        )
        .expect("the call decodes");

    // Assert
    assert_eq!(success.result, AuthorizationResult::Authorized);
    let requests = endpoint_log.take();
    let request = requests.last().expect("the transport received a request");
    assert_eq!(request["method"], "eth_call");
    assert_eq!(
        request["params"][1]["blockHash"],
        "0x0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b"
    );
    assert_eq!(request["params"][1]["requireCanonical"], true);
}
// evaluate_authorization_reads_at_the_block_of_the_reference

/// Contract: a call at a block the endpoint reports as not canonical is
///   refused as reorganized.
/// Arrange: the transport answering the call with a JSON-RPC error whose
///   message is `block is not canonical`.
/// Act:     `evaluate_authorization`.
/// Assert:  `Err(Reorganized { reference })`.
#[test]
fn evaluate_authorization_refuses_a_reference_that_is_not_canonical() {
    // Arrange
    let settings = build_base_chain_settings(Default::default());
    let endpoint_transport = Asserter::new();
    let settlement_transport = Asserter::new();
    let chain: BaseChain<MockIChainForms> =
        build_base_chain(&settings, endpoint_transport.clone(), settlement_transport);
    push_call_error(&endpoint_transport, "block is not canonical");
    let context = build_attempt_context::<MockIChainForms>(Default::default());
    let reference = context.reference;

    // Act
    let result = chain.evaluate_authorization(
        build_evaluate_authorization_params(Default::default()),
        build_evaluate_authorization_payload(&context, Default::default()),
    );

    // Assert
    match result {
        Err(EvaluateAuthorizationErrorReturn::Reorganized {
            reference: returned,
        }) => {
            assert_eq!(returned, reference);
        }
        _ => panic!("expected Reorganized"),
    }
}
// evaluate_authorization_refuses_a_reference_that_is_not_canonical

/// Contract: a return the binding cannot decode is refused.
/// Arrange: the call returning `0x1234`.
/// Act:     `evaluate_authorization`.
/// Assert:  the error matches
///   `Base(BaseEvaluateAuthorizationErrorReturn::Decode)`.
#[test]
fn evaluate_authorization_refuses_a_return_that_does_not_decode() {
    // Arrange
    let settings = build_base_chain_settings(Default::default());
    let endpoint_transport = Asserter::new();
    let settlement_transport = Asserter::new();
    let chain: BaseChain<MockIChainForms> =
        build_base_chain(&settings, endpoint_transport.clone(), settlement_transport);
    push_call_result(&endpoint_transport, Bytes::from(vec![0x12, 0x34]));
    let context = build_attempt_context::<MockIChainForms>(Default::default());

    // Act
    let result = chain.evaluate_authorization(
        build_evaluate_authorization_params(Default::default()),
        build_evaluate_authorization_payload(&context, Default::default()),
    );

    // Assert
    assert!(matches!(
        result,
        Err(EvaluateAuthorizationErrorReturn::Base(
            BaseEvaluateAuthorizationErrorReturn::Decode
        ))
    ));
}
// evaluate_authorization_refuses_a_return_that_does_not_decode

/// Contract: the batch returns the registry's results in the contexts'
///   order.
/// Arrange: three contexts whose assets are `[0x01; 32]`, `[0x02; 32]`,
///   and `[0x03; 32]` and the call returning `[2, 0, 1]` and next page
///   `0`.
/// Act:     `evaluate_authorization_batch` at page `0`.
/// Assert:  results `[Denied, Authorized, PendingSettlement]` and `next`
///   absent.
#[test]
fn evaluate_authorization_batch_returns_one_result_per_context_in_order() {
    // Arrange
    let settings = build_base_chain_settings(Default::default());
    let endpoint_transport = Asserter::new();
    let settlement_transport = Asserter::new();
    let chain: BaseChain<MockIChainForms> =
        build_base_chain(&settings, endpoint_transport.clone(), settlement_transport);
    let contexts = [
        build_attempt_context::<MockIChainForms>(AttemptContextOverrides {
            asset: Some(build_root(RootConstructorParamsOverrides {
                bytes: Some([0x01; 32]),
            })),
            ..Default::default()
        }),
        build_attempt_context::<MockIChainForms>(AttemptContextOverrides {
            asset: Some(build_root(RootConstructorParamsOverrides {
                bytes: Some([0x02; 32]),
            })),
            ..Default::default()
        }),
        build_attempt_context::<MockIChainForms>(AttemptContextOverrides {
            asset: Some(build_root(RootConstructorParamsOverrides {
                bytes: Some([0x03; 32]),
            })),
            ..Default::default()
        }),
    ];
    push_call_result(
        &endpoint_transport,
        Bytes::from((vec![2u8, 0, 1], U256::ZERO).abi_encode()),
    );

    // Act
    let success = chain
        .evaluate_authorization_batch(
            build_evaluate_authorization_batch_params(Default::default()),
            build_evaluate_authorization_batch_payload(&contexts, Default::default()),
        )
        .expect("the call decodes");

    // Assert
    assert_eq!(
        success.results,
        [
            AuthorizationResult::Denied,
            AuthorizationResult::Authorized,
            AuthorizationResult::PendingSettlement,
        ]
    );
    assert_eq!(success.next, None);
}
// evaluate_authorization_batch_returns_one_result_per_context_in_order

/// Contract: a nonzero next page is returned.
/// Arrange: the call returning `[0]` and next page `7`.
/// Act:     the batch over one context.
/// Assert:  `next` equal to `Some(BatchPage { index: 7 })`.
#[test]
fn evaluate_authorization_batch_returns_the_next_page() {
    // Arrange
    let settings = build_base_chain_settings(Default::default());
    let endpoint_transport = Asserter::new();
    let settlement_transport = Asserter::new();
    let chain: BaseChain<MockIChainForms> =
        build_base_chain(&settings, endpoint_transport.clone(), settlement_transport);
    let contexts = [build_attempt_context::<MockIChainForms>(Default::default())];
    push_call_result(
        &endpoint_transport,
        Bytes::from((vec![0u8], U256::from(7)).abi_encode()),
    );

    // Act
    let success = chain
        .evaluate_authorization_batch(
            build_evaluate_authorization_batch_params(Default::default()),
            build_evaluate_authorization_batch_payload(&contexts, Default::default()),
        )
        .expect("the call decodes");

    // Assert
    assert_eq!(success.results, [AuthorizationResult::Authorized]);
    assert_eq!(success.next, Some(BatchPage { index: 7 }));
}
// evaluate_authorization_batch_returns_the_next_page

/// Contract: more contexts than the settings' page limit are refused
///   before any call.
/// Arrange: settings whose `batch_page_limit` is `2` and three contexts.
/// Act:     the batch.
/// Assert:  `Err(PageTooLarge { limit: 2, supplied: 3 })` and no request
///   recorded.
#[test]
fn evaluate_authorization_batch_refuses_a_page_above_the_limit() {
    // Arrange
    let settings = build_base_chain_settings(BaseChainSettingsOverrides {
        batch_page_limit: Some(2),
        ..Default::default()
    });
    let (chain, _endpoint_transport, _settlement_transport, endpoint_log, settlement_log) =
        arrange_recording_base_chain(&settings);
    let contexts = [
        build_attempt_context::<MockIChainForms>(Default::default()),
        build_attempt_context::<MockIChainForms>(Default::default()),
        build_attempt_context::<MockIChainForms>(Default::default()),
    ];

    // Act
    let result = chain.evaluate_authorization_batch(
        build_evaluate_authorization_batch_params(Default::default()),
        build_evaluate_authorization_batch_payload(&contexts, Default::default()),
    );

    // Assert
    assert!(matches!(
        result,
        Err(EvaluateAuthorizationBatchErrorReturn::PageTooLarge {
            limit: 2,
            supplied: 3,
        })
    ));
    assert!(endpoint_log.take().is_empty());
    assert!(settlement_log.take().is_empty());
}
// evaluate_authorization_batch_refuses_a_page_above_the_limit

/// Contract: a number of results other than the contexts' is refused.
/// Arrange: two contexts and the call returning one result.
/// Act:     the batch.
/// Assert:  the error matches
///   `Base(BaseEvaluateAuthorizationBatchErrorReturn::ResultCount { expected: 2, actual: 1 })`.
#[test]
fn evaluate_authorization_batch_refuses_a_result_count_that_differs() {
    // Arrange
    let settings = build_base_chain_settings(Default::default());
    let endpoint_transport = Asserter::new();
    let settlement_transport = Asserter::new();
    let chain: BaseChain<MockIChainForms> =
        build_base_chain(&settings, endpoint_transport.clone(), settlement_transport);
    let contexts = [
        build_attempt_context::<MockIChainForms>(Default::default()),
        build_attempt_context::<MockIChainForms>(Default::default()),
    ];
    push_call_result(
        &endpoint_transport,
        Bytes::from((vec![0u8], U256::ZERO).abi_encode()),
    );

    // Act
    let result = chain.evaluate_authorization_batch(
        build_evaluate_authorization_batch_params(Default::default()),
        build_evaluate_authorization_batch_payload(&contexts, Default::default()),
    );

    // Assert
    assert!(matches!(
        result,
        Err(EvaluateAuthorizationBatchErrorReturn::Base(
            BaseEvaluateAuthorizationBatchErrorReturn::ResultCount {
                expected: 2,
                actual: 1,
            }
        ))
    ));
}
// evaluate_authorization_batch_refuses_a_result_count_that_differs

/// Contract: each returns the setting's value, the forms rebuilt from
///   the setting's bytes.
/// Arrange: settings whose page limit is `5`, endpoint
///   `https://example.test`, chain identifier `7`, and entitlement
///   contract the bytes `0x01` to `0x14` ascending.
/// Act:     each method.
/// Assert:  `5`, the endpoint text `https://example.test`, the chain
///   identifier form whose `to_field` is `Unsigned256` holding `7`
///   big-endian, and the contract form whose `to_field` is
///   `FixedBytes20` holding the ascending bytes.
#[test]
fn batch_page_limit_endpoint_chain_identifier_and_entitlement_contract_return_the_settings_values()
{
    // Arrange
    let mut chain_identifier = [0u8; 32];
    chain_identifier[31] = 7;
    let entitlement_contract = [
        0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f,
        0x10, 0x11, 0x12, 0x13, 0x14,
    ];
    let settings = build_base_chain_settings(BaseChainSettingsOverrides {
        batch_page_limit: Some(5),
        endpoint: Some(ChainEndpoint {
            text: "https://example.test".to_string(),
        }),
        chain_identifier: Some(chain_identifier),
        entitlement_contract: Some(entitlement_contract),
        ..Default::default()
    });
    let chain: BaseChain<MockIChainForms> =
        build_base_chain(&settings, Asserter::new(), Asserter::new());

    // Act
    let limit = chain
        .batch_page_limit(
            build_batch_page_limit_params(Default::default()),
            build_batch_page_limit_payload(Default::default()),
        )
        .expect("the page limit is infallible")
        .limit;
    let endpoint = chain
        .endpoint(
            build_endpoint_params(Default::default()),
            build_endpoint_payload(Default::default()),
        )
        .expect("the endpoint is infallible")
        .endpoint;
    let chain_identifier_form = chain
        .chain_identifier(
            build_chain_identifier_params(Default::default()),
            build_chain_identifier_payload(Default::default()),
        )
        .expect("the settings' bytes admit")
        .chain_identifier;
    let contract_form = chain
        .entitlement_contract(
            build_entitlement_contract_params(Default::default()),
            build_entitlement_contract_payload(Default::default()),
        )
        .expect("the settings' bytes admit")
        .contract;

    // Assert
    assert_eq!(limit, 5);
    assert_eq!(endpoint.text, "https://example.test");
    assert_eq!(
        MockChainIdentifier::to_field(ToFieldParams, &chain_identifier_form)
            .expect("the form yields its field")
            .field,
        CanonicalFieldValue::Unsigned256(chain_identifier)
    );
    assert_eq!(
        MockIdentity::to_field(ToFieldParams, &contract_form)
            .expect("the form yields its field")
            .field,
        CanonicalFieldValue::FixedBytes20(entitlement_contract)
    );
}
// batch_page_limit_endpoint_chain_identifier_and_entitlement_contract_return_the_settings_values

/// Contract: the description holds the declaration's entries in order
///   with the execution gas and the L1 fee.
/// Arrange: the endpoint answering a gas estimate of `21000` and the
///   oracle answering a fee of `3000`.
/// Act:     `estimate_fee`.
/// Assert:  the entries `execution_gas` `gas` `21000` and `l1_data_fee`
///   `wei` `3000`, in that order.
#[test]
fn estimate_fee_returns_the_declared_entries_in_order() {
    // Arrange
    let settings = build_base_chain_settings(Default::default());
    let endpoint_transport = Asserter::new();
    let settlement_transport = Asserter::new();
    let chain: BaseChain<MockIChainForms> =
        build_base_chain(&settings, endpoint_transport.clone(), settlement_transport);
    push_gas_estimate(&endpoint_transport, 21000);
    push_oracle_fee(&endpoint_transport, U256::from(3000));
    let request = build_fee_request::<MockIChainForms>(Default::default());

    // Act
    let success = chain
        .estimate_fee(
            build_estimate_fee_params(Default::default()),
            build_estimate_fee_payload(&request, Default::default()),
        )
        .expect("the estimate decodes");

    // Assert
    assert_eq!(
        success.description.entries,
        [
            FeeEntry {
                name: "execution_gas",
                unit: "gas",
                quantity: FeeQuantity { value: 21000 },
            },
            FeeEntry {
                name: "l1_data_fee",
                unit: "wei",
                quantity: FeeQuantity { value: 3000 },
            },
        ]
    );
}
// estimate_fee_returns_the_declared_entries_in_order

/// Contract: an oracle fee above `u128::MAX` is refused.
/// Arrange: the oracle answering `2^128`.
/// Act:     `estimate_fee`.
/// Assert:  the error matches
///   `Base(BaseEstimateFeeErrorReturn::FeeOutOfRange { fee })` with the
///   fee `2^128` as 32 big-endian bytes.
#[test]
fn estimate_fee_refuses_a_fee_above_u128() {
    // Arrange
    let settings = build_base_chain_settings(Default::default());
    let endpoint_transport = Asserter::new();
    let settlement_transport = Asserter::new();
    let chain: BaseChain<MockIChainForms> =
        build_base_chain(&settings, endpoint_transport.clone(), settlement_transport);
    push_gas_estimate(&endpoint_transport, 21000);
    push_oracle_fee(&endpoint_transport, U256::from(1) << 128);
    let request = build_fee_request::<MockIChainForms>(Default::default());

    // Act
    let result = chain.estimate_fee(
        build_estimate_fee_params(Default::default()),
        build_estimate_fee_payload(&request, Default::default()),
    );

    // Assert
    let mut fee = [0u8; 32];
    fee[15] = 1;
    match result {
        Err(EstimateFeeErrorReturn::Base(BaseEstimateFeeErrorReturn::FeeOutOfRange {
            fee: returned,
        })) => {
            assert_eq!(returned, fee);
        }
        _ => panic!("expected FeeOutOfRange"),
    }
}
// estimate_fee_refuses_a_fee_above_u128

/// Contract: a transport failure reaches the caller intact.
/// Arrange: a transport failure pushed for the gas estimate.
/// Act:     `estimate_fee`.
/// Assert:  `Transport`.
#[test]
fn estimate_fee_returns_a_transport_failure_whole() {
    // Arrange
    let settings = build_base_chain_settings(Default::default());
    let endpoint_transport = Asserter::new();
    let settlement_transport = Asserter::new();
    let chain: BaseChain<MockIChainForms> =
        build_base_chain(&settings, endpoint_transport.clone(), settlement_transport);
    push_transport_failure(&endpoint_transport, "refused");
    let request = build_fee_request::<MockIChainForms>(Default::default());

    // Act
    let result = chain.estimate_fee(
        build_estimate_fee_params(Default::default()),
        build_estimate_fee_payload(&request, Default::default()),
    );

    // Assert
    assert!(matches!(
        result,
        Err(EstimateFeeErrorReturn::Base(
            BaseEstimateFeeErrorReturn::Transport(_)
        ))
    ));
}
// estimate_fee_returns_a_transport_failure_whole

/// Contract: an undecodable oracle return reaches the caller.
/// Arrange: the endpoint answering a gas estimate of `21000` and the
///   oracle answering `0x1234`.
/// Act:     `estimate_fee`.
/// Assert:  `Decode`.
#[test]
fn estimate_fee_refuses_an_oracle_return_that_does_not_decode() {
    // Arrange
    let settings = build_base_chain_settings(Default::default());
    let endpoint_transport = Asserter::new();
    let settlement_transport = Asserter::new();
    let chain: BaseChain<MockIChainForms> =
        build_base_chain(&settings, endpoint_transport.clone(), settlement_transport);
    push_gas_estimate(&endpoint_transport, 21000);
    push_call_result(&endpoint_transport, Bytes::from(vec![0x12, 0x34]));
    let request = build_fee_request::<MockIChainForms>(Default::default());

    // Act
    let result = chain.estimate_fee(
        build_estimate_fee_params(Default::default()),
        build_estimate_fee_payload(&request, Default::default()),
    );

    // Assert
    assert!(matches!(
        result,
        Err(EstimateFeeErrorReturn::Base(
            BaseEstimateFeeErrorReturn::Decode
        ))
    ));
}
// estimate_fee_refuses_an_oracle_return_that_does_not_decode

/// Contract: the envelope of the log that created the interval is
///   returned when the calldata carries the same bytes.
/// Arrange: a `Delivered` log for interval `3` carrying the envelope
///   `0x0a0b0c` and the transaction whose input is a `deliver` call
///   carrying `0x0a0b0c` as its new envelope.
/// Act:     `recover_envelope` for interval `3`.
/// Assert:  the envelope `[0x0a, 0x0b, 0x0c]` and the transaction hash
///   `[0x77; 32]`.
#[test]
fn recover_envelope_returns_the_event_envelope_confirmed_by_the_calldata() {
    // Arrange
    let settings = build_base_chain_settings(Default::default());
    let endpoint_transport = Asserter::new();
    let settlement_transport = Asserter::new();
    let chain: BaseChain<MockIChainForms> =
        build_base_chain(&settings, endpoint_transport.clone(), settlement_transport);
    let transaction = B256::from([0x77; 32]);
    let delivered = IEntitlement::Delivered {
        entitlement: U256::from(5),
        from: Address::ZERO,
        to: Address::ZERO,
        interval: 3,
        envelope: Bytes::from(vec![0x0a, 0x0b, 0x0c]),
        proof: Bytes::new(),
    };
    push_logs(&endpoint_transport, Vec::new());
    push_logs(
        &endpoint_transport,
        vec![settlement_log(
            settings.entitlement_contract,
            transaction,
            delivered.encode_log_data(),
        )],
    );
    push_logs(&endpoint_transport, Vec::new());
    push_logs(&endpoint_transport, Vec::new());
    push_transaction(
        &endpoint_transport,
        transaction,
        Bytes::from(
            IEntitlement::deliverCall {
                identity: Address::ZERO,
                entitlement: U256::from(5),
                previousEnvelope: Bytes::new(),
                newEnvelope: Bytes::from(vec![0x0a, 0x0b, 0x0c]),
                transferProof: Bytes::new(),
                intent: IIdentityAccount::SignedIntent {
                    nonce: 0,
                    expiry: 0,
                    scheme: 0,
                    key: Bytes::new(),
                    signature: Bytes::new(),
                },
            }
            .abi_encode(),
        ),
    );
    let entitlement = entitlement_form(5);
    let interval = interval_form(3);

    // Act
    let success = chain
        .recover_envelope(
            build_recover_envelope_params(Default::default()),
            build_recover_envelope_payload(&entitlement, &interval, Default::default()),
        )
        .expect("the delivered envelope is recovered");

    // Assert
    assert_eq!(success.recovered.envelope, [0x0a, 0x0b, 0x0c]);
    assert_eq!(success.recovered.transaction, [0x77; 32]);
}
// recover_envelope_returns_the_event_envelope_confirmed_by_the_calldata

/// Contract: a calldata envelope that differs from the event's is
///   refused.
/// Arrange: the log carrying `0x0a0b0c` and the input carrying
///   `0x0a0b0d`.
/// Act:     `recover_envelope`.
/// Assert:  the error matches
///   `Base(BaseRecoverEnvelopeErrorReturn::CalldataDisagrees)`.
#[test]
fn recover_envelope_refuses_calldata_that_disagrees() {
    // Arrange
    let settings = build_base_chain_settings(Default::default());
    let endpoint_transport = Asserter::new();
    let settlement_transport = Asserter::new();
    let chain: BaseChain<MockIChainForms> =
        build_base_chain(&settings, endpoint_transport.clone(), settlement_transport);
    let transaction = B256::from([0x77; 32]);
    let delivered = IEntitlement::Delivered {
        entitlement: U256::from(5),
        from: Address::ZERO,
        to: Address::ZERO,
        interval: 3,
        envelope: Bytes::from(vec![0x0a, 0x0b, 0x0c]),
        proof: Bytes::new(),
    };
    push_logs(&endpoint_transport, Vec::new());
    push_logs(
        &endpoint_transport,
        vec![settlement_log(
            settings.entitlement_contract,
            transaction,
            delivered.encode_log_data(),
        )],
    );
    push_logs(&endpoint_transport, Vec::new());
    push_logs(&endpoint_transport, Vec::new());
    push_transaction(
        &endpoint_transport,
        transaction,
        Bytes::from(
            IEntitlement::deliverCall {
                identity: Address::ZERO,
                entitlement: U256::from(5),
                previousEnvelope: Bytes::new(),
                newEnvelope: Bytes::from(vec![0x0a, 0x0b, 0x0d]),
                transferProof: Bytes::new(),
                intent: IIdentityAccount::SignedIntent {
                    nonce: 0,
                    expiry: 0,
                    scheme: 0,
                    key: Bytes::new(),
                    signature: Bytes::new(),
                },
            }
            .abi_encode(),
        ),
    );
    let entitlement = entitlement_form(5);
    let interval = interval_form(3);

    // Act
    let result = chain.recover_envelope(
        build_recover_envelope_params(Default::default()),
        build_recover_envelope_payload(&entitlement, &interval, Default::default()),
    );

    // Assert
    assert!(matches!(
        result,
        Err(RecoverEnvelopeErrorReturn::Base(
            BaseRecoverEnvelopeErrorReturn::CalldataDisagrees
        ))
    ));
}
// recover_envelope_refuses_calldata_that_disagrees

/// Contract: a history holding no log of the interval is not found.
/// Arrange: logs for intervals `1` and `2` only.
/// Act:     `recover_envelope` for interval `3`.
/// Assert:  `Err(NotFound)`.
#[test]
fn recover_envelope_refuses_a_history_without_the_interval() {
    // Arrange
    let settings = build_base_chain_settings(Default::default());
    let endpoint_transport = Asserter::new();
    let settlement_transport = Asserter::new();
    let chain: BaseChain<MockIChainForms> =
        build_base_chain(&settings, endpoint_transport.clone(), settlement_transport);
    let minted = IEntitlement::Minted {
        entitlement: U256::from(5),
        recipient: Address::ZERO,
        interval: 1,
        envelope: Bytes::from(vec![0x0a, 0x0b, 0x0c]),
        proof: Bytes::new(),
    };
    let delivered = IEntitlement::Delivered {
        entitlement: U256::from(5),
        from: Address::ZERO,
        to: Address::ZERO,
        interval: 2,
        envelope: Bytes::from(vec![0x0a, 0x0b, 0x0c]),
        proof: Bytes::new(),
    };
    push_logs(
        &endpoint_transport,
        vec![settlement_log(
            settings.entitlement_contract,
            B256::from([0x71; 32]),
            minted.encode_log_data(),
        )],
    );
    push_logs(
        &endpoint_transport,
        vec![settlement_log(
            settings.entitlement_contract,
            B256::from([0x72; 32]),
            delivered.encode_log_data(),
        )],
    );
    push_logs(&endpoint_transport, Vec::new());
    push_logs(&endpoint_transport, Vec::new());
    let entitlement = entitlement_form(5);
    let interval = interval_form(3);

    // Act
    let result = chain.recover_envelope(
        build_recover_envelope_params(Default::default()),
        build_recover_envelope_payload(&entitlement, &interval, Default::default()),
    );

    // Assert
    assert!(matches!(result, Err(RecoverEnvelopeErrorReturn::NotFound)));
}
// recover_envelope_refuses_a_history_without_the_interval

/// Contract: a `Granted` log records interval zero of the new
///   entitlement.
/// Arrange: a `Granted` log whose new entitlement is the request's and
///   a transaction whose input is a `grant` call carrying the same
///   envelope `0x0a0b0c`.
/// Act:     `recover_envelope` for interval `0`.
/// Assert:  that log's envelope.
#[test]
fn recover_envelope_reads_a_grant_as_interval_zero() {
    // Arrange
    let settings = build_base_chain_settings(Default::default());
    let endpoint_transport = Asserter::new();
    let settlement_transport = Asserter::new();
    let chain: BaseChain<MockIChainForms> =
        build_base_chain(&settings, endpoint_transport.clone(), settlement_transport);
    let transaction = B256::from([0x77; 32]);
    let granted = IEntitlement::Granted {
        sourceEntitlement: U256::from(4),
        entitlement: U256::from(5),
        recipient: Address::ZERO,
        envelope: Bytes::from(vec![0x0a, 0x0b, 0x0c]),
        proof: Bytes::new(),
    };
    push_logs(&endpoint_transport, Vec::new());
    push_logs(&endpoint_transport, Vec::new());
    push_logs(&endpoint_transport, Vec::new());
    push_logs(
        &endpoint_transport,
        vec![settlement_log(
            settings.entitlement_contract,
            transaction,
            granted.encode_log_data(),
        )],
    );
    push_transaction(
        &endpoint_transport,
        transaction,
        Bytes::from(
            IEntitlement::grantCall {
                identity: Address::ZERO,
                sourceEntitlement: U256::from(4),
                entitlement: U256::from(5),
                recipient: Address::ZERO,
                authorEnvelope: Bytes::new(),
                envelope: Bytes::from(vec![0x0a, 0x0b, 0x0c]),
                transferProof: Bytes::new(),
                intent: IIdentityAccount::SignedIntent {
                    nonce: 0,
                    expiry: 0,
                    scheme: 0,
                    key: Bytes::new(),
                    signature: Bytes::new(),
                },
            }
            .abi_encode(),
        ),
    );
    let entitlement = entitlement_form(5);
    let interval = interval_form(0);

    // Act
    let success = chain
        .recover_envelope(
            build_recover_envelope_params(Default::default()),
            build_recover_envelope_payload(&entitlement, &interval, Default::default()),
        )
        .expect("the granted envelope is recovered");

    // Assert
    assert_eq!(success.recovered.envelope, [0x0a, 0x0b, 0x0c]);
    assert_eq!(success.recovered.transaction, [0x77; 32]);
}
// recover_envelope_reads_a_grant_as_interval_zero

/// Contract: a log the bound event cannot decode is refused.
/// Arrange: a log whose data is `0x1234`.
/// Act:     `recover_envelope`.
/// Assert:  the error matches `Base(BaseRecoverEnvelopeErrorReturn::Decode)`.
#[test]
fn recover_envelope_refuses_a_log_that_does_not_decode() {
    // Arrange
    let settings = build_base_chain_settings(Default::default());
    let endpoint_transport = Asserter::new();
    let settlement_transport = Asserter::new();
    let chain: BaseChain<MockIChainForms> =
        build_base_chain(&settings, endpoint_transport.clone(), settlement_transport);
    push_logs(
        &endpoint_transport,
        vec![settlement_log(
            settings.entitlement_contract,
            B256::from([0x77; 32]),
            LogData::new_unchecked(
                vec![
                    IEntitlement::Minted::SIGNATURE_HASH,
                    B256::from(unsigned_256(5)),
                ],
                Bytes::from(vec![0x12, 0x34]),
            ),
        )],
    );
    push_logs(&endpoint_transport, Vec::new());
    push_logs(&endpoint_transport, Vec::new());
    push_logs(&endpoint_transport, Vec::new());
    let entitlement = entitlement_form(5);
    let interval = interval_form(3);

    // Act
    let result = chain.recover_envelope(
        build_recover_envelope_params(Default::default()),
        build_recover_envelope_payload(&entitlement, &interval, Default::default()),
    );

    // Assert
    assert!(matches!(
        result,
        Err(RecoverEnvelopeErrorReturn::Base(
            BaseRecoverEnvelopeErrorReturn::Decode
        ))
    ));
}
// recover_envelope_refuses_a_log_that_does_not_decode
