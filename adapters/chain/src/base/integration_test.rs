#![allow(clippy::expect_used, clippy::unwrap_used, clippy::panic)]

use super::interface::BaseChain;
use super::interface::BaseEstimateFeeErrorReturn;
use super::mock::{
    build_base_chain, push_anchor_root, push_block, push_call_error, push_call_result,
    push_gas_estimate, push_logs, push_oracle_fee, push_transaction,
};
use super::settings::provides::{BaseChainSettingsOverrides, build_base_chain_settings};
use super::test::{arrange_recording_base_chain_as, settlement_log};
use super::{IEntitlement, IIdentityAccount, Registry};
use crate::create_chain_forms::provides::{
    CHAIN_FORMS_CONCRETES, ChainFormsConcrete, ConsumeCaseParams, ConsumeCasePayload,
    ConsumeChainFormsParams, ConsumeChainFormsPayload, CreateChainFormsCaseParamsOverrides,
    CreateChainFormsParams, CreateChainFormsParamsOverrides, CreateChainFormsReturn,
    IChainFormsConsumer, ICreateChainFormsCaseConsumer, build_create_chain_forms_case_deps,
    build_create_chain_forms_case_params, build_create_chain_forms_case_payload,
    build_create_chain_forms_deps, build_create_chain_forms_params,
    build_create_chain_forms_payload, create_chain_forms, create_chain_forms_case,
};
use crate::factory::provides::{
    AttemptContextOverrides, AuthorizationResult, BatchPage, CurrentReferenceAtParamsOverrides,
    EstimateFeeErrorReturn, EvaluateAuthorizationBatchParamsOverrides,
    EvaluateAuthorizationErrorReturn, FeeEntry, FeeQuantity, IChainAdapter, IChainForms,
    IEntitlementStateAdapter, ISettlementAdapter, SettlementReferenceConstructorParamsOverrides,
    SettlementTier, TierOfErrorReturn, build_attempt_context, build_chain_identifier_params,
    build_chain_identifier_payload, build_current_reference_at_params,
    build_current_reference_at_payload, build_entitlement_contract_params,
    build_entitlement_contract_payload, build_estimate_fee_params, build_estimate_fee_payload,
    build_evaluate_authorization_batch_params, build_evaluate_authorization_batch_payload,
    build_evaluate_authorization_params, build_evaluate_authorization_payload, build_fee_request,
    build_recover_envelope_params, build_recover_envelope_payload, build_settlement_reference,
    build_tier_of_params, build_tier_of_payload,
};
use alloy::primitives::{Address, B256, Bytes, U256};
use alloy::sol_types::{SolCall, SolEvent, SolValue};
use alloy::transports::mock::Asserter;
use domain::{
    DeploymentIdentityConstructorParamsOverrides, HandshakeKeyCommitment,
    HandshakeKeyCommitmentConstructorParams, SignatureSchemeIdentifierConstructorParamsOverrides,
    build_deployment_identity, build_signature_scheme_identifier,
};
use encoding::{
    CanonicalFieldValue, CreateEncodingParams, DECLARED_ENCODING_SELECTIONS, EncodingIdentifier,
    FromFieldParams, ICanonicalField, IEncoderAdapter, ToFieldParams,
};
use hashing::{RootConstructorParamsOverrides, build_root};

struct CaseConsumer<C> {
    params: CreateChainFormsParams,
    consumer: C,
}

impl<C: IChainFormsConsumer + Clone> ICreateChainFormsCaseConsumer for CaseConsumer<C> {
    type Output = CreateChainFormsReturn<C::Output>;

    fn consume_case<E: IEncoderAdapter>(
        &self,
        _params: ConsumeCaseParams,
        payload: ConsumeCasePayload<'_, E>,
    ) -> Self::Output {
        create_chain_forms(
            &build_create_chain_forms_deps(
                self.consumer.clone(),
                payload.encoder,
                Default::default(),
            ),
            self.params,
            build_create_chain_forms_payload(Default::default()),
        )
    }
}

fn required_encodings(concrete: ChainFormsConcrete) -> Vec<CreateEncodingParams> {
    match concrete {
        ChainFormsConcrete::Evm => DECLARED_ENCODING_SELECTIONS
            .iter()
            .filter(|selection| selection.identifier == EncodingIdentifier::EthereumAbiV1)
            .map(|selection| CreateEncodingParams {
                concrete: selection.concrete,
                identifier: selection.identifier,
            })
            .collect(),
        ChainFormsConcrete::Mock => Vec::new(),
    }
}

fn run_form_concretes<C: IChainFormsConsumer + Clone>(consumer: C) {
    for concrete in CHAIN_FORMS_CONCRETES.iter().copied() {
        let params = build_create_chain_forms_params(CreateChainFormsParamsOverrides {
            concrete: Some(concrete),
            ..Default::default()
        });
        let deps = build_create_chain_forms_case_deps(
            CaseConsumer {
                params,
                consumer: consumer.clone(),
            },
            Default::default(),
        );
        for encoding in required_encodings(concrete) {
            let result = create_chain_forms_case(
                &deps,
                build_create_chain_forms_case_params(CreateChainFormsCaseParamsOverrides {
                    encoding: Some(encoding),
                }),
                build_create_chain_forms_case_payload(Default::default()),
            );
            assert!(result.is_ok());
            assert!(result.is_ok_and(|success| success.output.is_ok()));
        }
    }
}

fn word256(value: u64) -> [u8; 32] {
    let mut bytes = [0u8; 32];
    bytes[24..].copy_from_slice(&value.to_be_bytes());
    bytes
}

fn entitlement_form<F: IChainForms>(value: u64) -> F::Entitlement {
    F::Entitlement::from_field(
        FromFieldParams,
        CanonicalFieldValue::Unsigned256(word256(value)),
    )
    .expect("the canonical unsigned-256 value is admitted")
    .value
}

fn interval_form<F: IChainForms>(value: u64) -> F::Interval {
    F::Interval::from_field(FromFieldParams, CanonicalFieldValue::Unsigned64(value))
        .expect("the canonical unsigned-64 value is admitted")
        .value
}

fn signed_intent() -> IIdentityAccount::SignedIntent {
    IIdentityAccount::SignedIntent {
        nonce: 0,
        expiry: 0,
        scheme: 0,
        key: Bytes::new(),
        signature: Bytes::new(),
    }
}

/// Contract: the own entry `each_tier_reads_its_own_head_and_orders_the_references`:
///   `current_reference_at` called for each of the four tiers against an
///   endpoint whose latest, safe, and finalized blocks and whose anchor
///   state hold four distinct heights → each reference's ordinal is its
///   tier's height and the references compare in the order of the tiers,
///   `Included` greatest.
/// Arrange: the endpoint transport answering `latest` with block `1000`,
///   `safe` with `900`, `finalized` with `800`, and number `700` with
///   `700`; the settlement transport answering an anchor root of height
///   `700`; the run over each concrete of `CHAIN_FORMS_CONCRETES`, each
///   over the encoding selections its declaration requires.
/// Act:     `current_reference_at` for `Included`, `Soft`, `Hard`, and
///   `Settled`, in that order, inside the forms consumer.
/// Assert:  the references' ordinals are `1000`, `900`, `800`, and `700`
///   with identities `[0x0a; 32]`, `[0x0b; 32]`, `[0x0c; 32]`, and
///   `[0x0d; 32]`; `included > soft > hard > settled`; the endpoint
///   transport received block reads naming `latest`, `safe`, `finalized`,
///   and `0x2bc` in order, and the settlement transport received one
///   `eth_call` to the anchor state registry.
/// Boundary: `create_chain_forms_case`, `create_encoding`,
///   `create_chain_forms`, the concrete's producer, `consume_chain_forms`,
///   the provider stack over the recorded `MockTransport`, the sol
///   bindings, and `current_reference_at`.
/// Mocked:  the two providers' transports through `Asserter`, recorded by
///   the `RecordingTransport` wrapper.
#[test]
fn each_tier_reads_its_own_head_and_orders_the_references() {
    #[derive(Clone, Copy)]
    struct Consumer;

    impl IChainFormsConsumer for Consumer {
        type Output = ();

        fn consume_chain_forms<F: IChainForms>(
            &self,
            _params: ConsumeChainFormsParams,
            _payload: ConsumeChainFormsPayload<F>,
        ) -> Self::Output {
            // Arrange
            let settings = build_base_chain_settings(Default::default());
            let (chain, endpoint_transport, settlement_transport, endpoint_log, settlement_log) =
                arrange_recording_base_chain_as::<F>(&settings);
            push_block(&endpoint_transport, 1000, B256::from([0x0a; 32]), 5000);
            push_block(&endpoint_transport, 900, B256::from([0x0b; 32]), 4000);
            push_block(&endpoint_transport, 800, B256::from([0x0c; 32]), 3000);
            push_anchor_root(&settlement_transport, 700);
            push_block(&endpoint_transport, 700, B256::from([0x0d; 32]), 2000);

            // Act
            let included = chain
                .current_reference_at(
                    build_current_reference_at_params(CurrentReferenceAtParamsOverrides {
                        tier: Some(SettlementTier::Included),
                    }),
                    build_current_reference_at_payload(Default::default()),
                )
                .expect("the latest block answers")
                .reference;
            let soft = chain
                .current_reference_at(
                    build_current_reference_at_params(CurrentReferenceAtParamsOverrides {
                        tier: Some(SettlementTier::Soft),
                    }),
                    build_current_reference_at_payload(Default::default()),
                )
                .expect("the safe block answers")
                .reference;
            let hard = chain
                .current_reference_at(
                    build_current_reference_at_params(CurrentReferenceAtParamsOverrides {
                        tier: Some(SettlementTier::Hard),
                    }),
                    build_current_reference_at_payload(Default::default()),
                )
                .expect("the finalized block answers")
                .reference;
            let settled = chain
                .current_reference_at(
                    build_current_reference_at_params(CurrentReferenceAtParamsOverrides {
                        tier: Some(SettlementTier::Settled),
                    }),
                    build_current_reference_at_payload(Default::default()),
                )
                .expect("the anchor height's block answers")
                .reference;

            // Assert
            assert_eq!(included.ordinal(), 1000);
            assert_eq!(*included.identity(), [0x0a; 32]);
            assert_eq!(soft.ordinal(), 900);
            assert_eq!(*soft.identity(), [0x0b; 32]);
            assert_eq!(hard.ordinal(), 800);
            assert_eq!(*hard.identity(), [0x0c; 32]);
            assert_eq!(settled.ordinal(), 700);
            assert_eq!(*settled.identity(), [0x0d; 32]);
            assert!(included > soft);
            assert!(soft > hard);
            assert!(hard > settled);
            let endpoint_requests = endpoint_log.take();
            assert_eq!(endpoint_requests.len(), 4);
            for request in &endpoint_requests {
                assert_eq!(request["method"], "eth_getBlockByNumber");
            }
            assert_eq!(endpoint_requests[0]["params"][0], "latest");
            assert_eq!(endpoint_requests[1]["params"][0], "safe");
            assert_eq!(endpoint_requests[2]["params"][0], "finalized");
            assert_eq!(endpoint_requests[3]["params"][0], "0x2bc");
            let settlement_requests = settlement_log.take();
            assert_eq!(settlement_requests.len(), 1);
            assert_eq!(settlement_requests[0]["method"], "eth_call");
            assert_eq!(
                serde_json::from_value::<Address>(
                    settlement_requests[0]["params"][0]["to"].clone()
                )
                .expect("the call names an address"),
                Address::from([
                    0x90, 0x9f, 0x6c, 0xf4, 0x7e, 0xd1, 0x2f, 0x01, 0x0a, 0x79, 0x65, 0x27, 0xf5,
                    0x62, 0xbf, 0xc2, 0x6c, 0x7f, 0x4e, 0x72,
                ])
            );
        }
    }

    run_form_concretes(Consumer);
}
// each_tier_reads_its_own_head_and_orders_the_references

/// Contract: the own entry `a_context_reaches_the_registry_with_every_field`:
///   `evaluate_authorization` called with a context whose every field
///   differs from the builders' defaults → the `evaluateAuthorization`
///   call the provider received decodes to a context equal field by field
///   to the one handed.
/// Arrange: a context whose asset is the ascending bytes from `0x01`,
///   deployment the ascending bytes from `0x21`, entitlement contract the
///   ascending twenty bytes from `0x41`, entitlement the ascending bytes
///   from `0x61`, identity the ascending twenty bytes from `0x81`,
///   handshake scheme `0xa6`, handshake commitment the ascending bytes
///   from `0xa1`, interval `0x0102_0304_0506_0708`, and reference ordinal
///   `900` with identity `[0x0b; 32]`; the endpoint answering `0`.
/// Act:     `evaluate_authorization`, inside the forms consumer.
/// Assert:  `Authorized`; the recorded `eth_call` names the registry
///   address, carries the canonical-block requirement at hash
///   `[0x0b; 32]`, and its input decodes to a context holding every
///   arranged field, with `data` absent.
/// Boundary: `create_chain_forms_case`, `create_encoding`,
///   `create_chain_forms`, the concrete's producer, `consume_chain_forms`,
///   the provider stack over the recorded `MockTransport`, the sol
///   bindings, and `evaluate_authorization`.
/// Mocked:  the two providers' transports through `Asserter`, recorded by
///   the `RecordingTransport` wrapper.
#[test]
fn a_context_reaches_the_registry_with_every_field() {
    #[derive(Clone, Copy)]
    struct Consumer;

    impl IChainFormsConsumer for Consumer {
        type Output = ();

        fn consume_chain_forms<F: IChainForms>(
            &self,
            _params: ConsumeChainFormsParams,
            _payload: ConsumeChainFormsPayload<F>,
        ) -> Self::Output {
            // Arrange
            let settings = build_base_chain_settings(Default::default());
            let (chain, endpoint_transport, _settlement_transport, endpoint_log, _settlement_log) =
                arrange_recording_base_chain_as::<F>(&settings);
            push_call_result(
                &endpoint_transport,
                Bytes::from(Registry::evaluateAuthorizationCall::abi_encode_returns(
                    &0u8,
                )),
            );
            let context = build_attempt_context::<F>(AttemptContextOverrides {
                asset: Some(build_root(RootConstructorParamsOverrides {
                    bytes: Some([
                        0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c,
                        0x0d, 0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18,
                        0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x20,
                    ]),
                })),
                deployment: Some(build_deployment_identity(
                    DeploymentIdentityConstructorParamsOverrides {
                        bytes: Some([
                            0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2a, 0x2b, 0x2c,
                            0x2d, 0x2e, 0x2f, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38,
                            0x39, 0x3a, 0x3b, 0x3c, 0x3d, 0x3e, 0x3f, 0x40,
                        ]),
                    },
                )),
                entitlement_contract: Some(
                    F::Identity::from_field(
                        FromFieldParams,
                        CanonicalFieldValue::FixedBytes20([
                            0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4a, 0x4b, 0x4c,
                            0x4d, 0x4e, 0x4f, 0x50, 0x51, 0x52, 0x53, 0x54,
                        ]),
                    )
                    .expect("the canonical fixed-bytes-20 value is admitted")
                    .value,
                ),
                entitlement: Some(
                    F::Entitlement::from_field(
                        FromFieldParams,
                        CanonicalFieldValue::Unsigned256([
                            0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0x6a, 0x6b, 0x6c,
                            0x6d, 0x6e, 0x6f, 0x70, 0x71, 0x72, 0x73, 0x74, 0x75, 0x76, 0x77, 0x78,
                            0x79, 0x7a, 0x7b, 0x7c, 0x7d, 0x7e, 0x7f, 0x80,
                        ]),
                    )
                    .expect("the canonical unsigned-256 value is admitted")
                    .value,
                ),
                identity: Some(
                    F::Identity::from_field(
                        FromFieldParams,
                        CanonicalFieldValue::FixedBytes20([
                            0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8a, 0x8b, 0x8c,
                            0x8d, 0x8e, 0x8f, 0x90, 0x91, 0x92, 0x93, 0x94,
                        ]),
                    )
                    .expect("the canonical fixed-bytes-20 value is admitted")
                    .value,
                ),
                handshake_key: Some(
                    HandshakeKeyCommitment::try_new(HandshakeKeyCommitmentConstructorParams {
                        bytes: [
                            0xa1, 0xa2, 0xa3, 0xa4, 0xa5, 0xa6, 0xa7, 0xa8, 0xa9, 0xaa, 0xab, 0xac,
                            0xad, 0xae, 0xaf, 0xb0, 0xb1, 0xb2, 0xb3, 0xb4, 0xb5, 0xb6, 0xb7, 0xb8,
                            0xb9, 0xba, 0xbb, 0xbc, 0xbd, 0xbe, 0xbf, 0xc0,
                        ],
                        scheme: build_signature_scheme_identifier(
                            SignatureSchemeIdentifierConstructorParamsOverrides {
                                identifier: Some(0xa6),
                            },
                        ),
                    })
                    .expect("the nonzero commitment is admitted"),
                ),
                interval: Some(
                    F::Interval::from_field(
                        FromFieldParams,
                        CanonicalFieldValue::Unsigned64(0x0102_0304_0506_0708),
                    )
                    .expect("the canonical unsigned-64 value is admitted")
                    .value,
                ),
                reference: Some(build_settlement_reference(
                    SettlementReferenceConstructorParamsOverrides {
                        ordinal: Some(900),
                        identity: Some([0x0b; 32]),
                    },
                )),
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
            assert_eq!(requests.len(), 1);
            let request = &requests[0];
            assert_eq!(request["method"], "eth_call");
            assert_eq!(
                serde_json::from_value::<Address>(request["params"][0]["to"].clone())
                    .expect("the call names an address"),
                Address::from([0xdd; 20])
            );
            assert!(request["params"][0]["data"].is_null());
            assert_eq!(
                request["params"][1]["blockHash"],
                "0x0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b"
            );
            assert_eq!(request["params"][1]["requireCanonical"], true);
            let input: Bytes = serde_json::from_value(request["params"][0]["input"].clone())
                .expect("the call input is bytes");
            let decoded = Registry::evaluateAuthorizationCall::abi_decode(&input)
                .expect("the call input decodes through the binding");
            let context = decoded.context;
            assert_eq!(
                context.asset,
                B256::from([
                    0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d,
                    0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a,
                    0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x20,
                ])
            );
            assert_eq!(
                context.deployment,
                B256::from([
                    0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2a, 0x2b, 0x2c, 0x2d,
                    0x2e, 0x2f, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3a,
                    0x3b, 0x3c, 0x3d, 0x3e, 0x3f, 0x40,
                ])
            );
            assert_eq!(
                context.entitlementContract,
                Address::from([
                    0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4a, 0x4b, 0x4c, 0x4d,
                    0x4e, 0x4f, 0x50, 0x51, 0x52, 0x53, 0x54,
                ])
            );
            assert_eq!(
                context.entitlement,
                U256::from_be_bytes([
                    0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0x6a, 0x6b, 0x6c, 0x6d,
                    0x6e, 0x6f, 0x70, 0x71, 0x72, 0x73, 0x74, 0x75, 0x76, 0x77, 0x78, 0x79, 0x7a,
                    0x7b, 0x7c, 0x7d, 0x7e, 0x7f, 0x80,
                ])
            );
            assert_eq!(
                context.identity,
                Address::from([
                    0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8a, 0x8b, 0x8c, 0x8d,
                    0x8e, 0x8f, 0x90, 0x91, 0x92, 0x93, 0x94,
                ])
            );
            assert_eq!(context.handshakeKeyScheme, 0xa6);
            assert_eq!(
                context.handshakeKeyCommitment,
                B256::from([
                    0xa1, 0xa2, 0xa3, 0xa4, 0xa5, 0xa6, 0xa7, 0xa8, 0xa9, 0xaa, 0xab, 0xac, 0xad,
                    0xae, 0xaf, 0xb0, 0xb1, 0xb2, 0xb3, 0xb4, 0xb5, 0xb6, 0xb7, 0xb8, 0xb9, 0xba,
                    0xbb, 0xbc, 0xbd, 0xbe, 0xbf, 0xc0,
                ])
            );
            assert_eq!(context.interval, 0x0102_0304_0506_0708);
            assert_eq!(context.reference, 900);
        }
    }

    run_form_concretes(Consumer);
}
// a_context_reaches_the_registry_with_every_field

/// Contract: the own entry `batch_evaluation_agrees_with_individual_evaluation`:
///   `evaluate_authorization_batch` and `evaluate_authorization` called
///   over the same contexts against a registry answering each context by
///   its position → the batch's result at each position equals the
///   individual result of that position's context, and the next page is
///   returned unchanged.
/// Arrange: three contexts of different assets `[0x01; 32]`, `[0x02; 32]`,
///   and `[0x03; 32]`, identities `[0xe1; 20]`, `[0xe2; 20]`, and
///   `[0xe3; 20]`, and references `900`/`[0x0b; 32]`, `850`/`[0x85; 32]`,
///   and `700`/`[0x0d; 32]`; the endpoint answering the batch with
///   `[0, 1, 2]` and next page `7` and each individual call with `0`,
///   `1`, and `2`.
/// Act:     the batch at page `5`, then the individual evaluation of each
///   context in order, inside the forms consumer.
/// Assert:  batch results `[Authorized, PendingSettlement, Denied]` and
///   `next` `Some(BatchPage { index: 7 })`; each individual result equals
///   the batch's positional result; the batch call's input decodes to the
///   three arranged contexts and page `5`, read at `[0x0b; 32]`.
/// Boundary: `create_chain_forms_case`, `create_encoding`,
///   `create_chain_forms`, the concrete's producer, `consume_chain_forms`,
///   the provider stack over the recorded `MockTransport`, the sol
///   bindings, and the two evaluation methods.
/// Mocked:  the two providers' transports through `Asserter`, recorded by
///   the `RecordingTransport` wrapper.
#[test]
fn batch_evaluation_agrees_with_individual_evaluation() {
    #[derive(Clone, Copy)]
    struct Consumer;

    impl IChainFormsConsumer for Consumer {
        type Output = ();

        fn consume_chain_forms<F: IChainForms>(
            &self,
            _params: ConsumeChainFormsParams,
            _payload: ConsumeChainFormsPayload<F>,
        ) -> Self::Output {
            // Arrange
            let settings = build_base_chain_settings(Default::default());
            let (chain, endpoint_transport, _settlement_transport, endpoint_log, _settlement_log) =
                arrange_recording_base_chain_as::<F>(&settings);
            let contexts = [
                build_attempt_context::<F>(AttemptContextOverrides {
                    asset: Some(build_root(RootConstructorParamsOverrides {
                        bytes: Some([0x01; 32]),
                    })),
                    identity: Some(
                        F::Identity::from_field(
                            FromFieldParams,
                            CanonicalFieldValue::FixedBytes20([0xe1; 20]),
                        )
                        .expect("the canonical fixed-bytes-20 value is admitted")
                        .value,
                    ),
                    reference: Some(build_settlement_reference(
                        SettlementReferenceConstructorParamsOverrides {
                            ordinal: Some(900),
                            identity: Some([0x0b; 32]),
                        },
                    )),
                    ..Default::default()
                }),
                build_attempt_context::<F>(AttemptContextOverrides {
                    asset: Some(build_root(RootConstructorParamsOverrides {
                        bytes: Some([0x02; 32]),
                    })),
                    identity: Some(
                        F::Identity::from_field(
                            FromFieldParams,
                            CanonicalFieldValue::FixedBytes20([0xe2; 20]),
                        )
                        .expect("the canonical fixed-bytes-20 value is admitted")
                        .value,
                    ),
                    reference: Some(build_settlement_reference(
                        SettlementReferenceConstructorParamsOverrides {
                            ordinal: Some(850),
                            identity: Some([0x85; 32]),
                        },
                    )),
                    ..Default::default()
                }),
                build_attempt_context::<F>(AttemptContextOverrides {
                    asset: Some(build_root(RootConstructorParamsOverrides {
                        bytes: Some([0x03; 32]),
                    })),
                    identity: Some(
                        F::Identity::from_field(
                            FromFieldParams,
                            CanonicalFieldValue::FixedBytes20([0xe3; 20]),
                        )
                        .expect("the canonical fixed-bytes-20 value is admitted")
                        .value,
                    ),
                    reference: Some(build_settlement_reference(
                        SettlementReferenceConstructorParamsOverrides {
                            ordinal: Some(700),
                            identity: Some([0x0d; 32]),
                        },
                    )),
                    ..Default::default()
                }),
            ];
            push_call_result(
                &endpoint_transport,
                Bytes::from((vec![0u8, 1, 2], U256::from(7)).abi_encode()),
            );
            push_call_result(
                &endpoint_transport,
                Bytes::from(Registry::evaluateAuthorizationCall::abi_encode_returns(
                    &0u8,
                )),
            );
            push_call_result(
                &endpoint_transport,
                Bytes::from(Registry::evaluateAuthorizationCall::abi_encode_returns(
                    &1u8,
                )),
            );
            push_call_result(
                &endpoint_transport,
                Bytes::from(Registry::evaluateAuthorizationCall::abi_encode_returns(
                    &2u8,
                )),
            );

            // Act
            let batch = chain
                .evaluate_authorization_batch(
                    build_evaluate_authorization_batch_params(
                        EvaluateAuthorizationBatchParamsOverrides {
                            page: Some(BatchPage { index: 5 }),
                        },
                    ),
                    build_evaluate_authorization_batch_payload(&contexts, Default::default()),
                )
                .expect("the batch call decodes");
            let individual = contexts
                .iter()
                .map(|context| {
                    chain
                        .evaluate_authorization(
                            build_evaluate_authorization_params(Default::default()),
                            build_evaluate_authorization_payload(context, Default::default()),
                        )
                        .expect("the call decodes")
                        .result
                })
                .collect::<Vec<_>>();

            // Assert
            assert_eq!(
                batch.results,
                [
                    AuthorizationResult::Authorized,
                    AuthorizationResult::PendingSettlement,
                    AuthorizationResult::Denied,
                ]
            );
            assert_eq!(batch.next, Some(BatchPage { index: 7 }));
            assert_eq!(
                individual,
                [
                    AuthorizationResult::Authorized,
                    AuthorizationResult::PendingSettlement,
                    AuthorizationResult::Denied,
                ]
            );
            let requests = endpoint_log.take();
            assert_eq!(requests.len(), 4);
            assert_eq!(requests[0]["method"], "eth_call");
            assert_eq!(
                requests[0]["params"][1]["blockHash"],
                "0x0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b"
            );
            let input: Bytes = serde_json::from_value(requests[0]["params"][0]["input"].clone())
                .expect("the batch input is bytes");
            let decoded = Registry::evaluateAuthorizationBatchCall::abi_decode(&input)
                .expect("the batch input decodes through the binding");
            assert_eq!(decoded.contexts.len(), 3);
            assert_eq!(decoded.contexts[0].asset, B256::from([0x01; 32]));
            assert_eq!(decoded.contexts[1].asset, B256::from([0x02; 32]));
            assert_eq!(decoded.contexts[2].asset, B256::from([0x03; 32]));
            assert_eq!(decoded.contexts[0].identity, Address::from([0xe1; 20]));
            assert_eq!(decoded.contexts[1].identity, Address::from([0xe2; 20]));
            assert_eq!(decoded.contexts[2].identity, Address::from([0xe3; 20]));
            assert_eq!(decoded.contexts[0].reference, 900);
            assert_eq!(decoded.contexts[1].reference, 850);
            assert_eq!(decoded.contexts[2].reference, 700);
            assert_eq!(decoded.page, U256::from(5));
        }
    }

    run_form_concretes(Consumer);
}
// batch_evaluation_agrees_with_individual_evaluation

/// Contract: the own entry `a_reorganized_reference_is_refused_whatever_its_number`:
///   `tier_of` and `evaluate_authorization` called with a reference whose
///   number the chain holds under another hash → `Err(Reorganized {
///   reference })` from each.
/// Arrange: three references, at the latest `1000`, safe `900`, and
///   finalized `800` heads, each recording identity `[0x99; 32]` while the
///   chain holds `[0x0a; 32]`, `[0x0b; 32]`, and `[0x0c; 32]` at those
///   numbers; the endpoint answering the call with `block is not
///   canonical`.
/// Act:     `tier_of` and `evaluate_authorization` per reference, inside
///   the forms consumer, over `BaseChain::try_new_mocked`.
/// Assert:  `Err(Reorganized { reference })` with the handed reference,
///   from each of the six calls.
/// Boundary: `create_chain_forms_case`, `create_encoding`,
///   `create_chain_forms`, the concrete's producer, `consume_chain_forms`,
///   `BaseChain::try_new_mocked`, and the two methods.
/// Mocked:  the two providers' transports through `Asserter`.
#[test]
fn a_reorganized_reference_is_refused_whatever_its_number() {
    #[derive(Clone, Copy)]
    struct Consumer;

    impl IChainFormsConsumer for Consumer {
        type Output = ();

        fn consume_chain_forms<F: IChainForms>(
            &self,
            _params: ConsumeChainFormsParams,
            _payload: ConsumeChainFormsPayload<F>,
        ) -> Self::Output {
            // Arrange
            let settings = build_base_chain_settings(Default::default());
            let endpoint_transport = Asserter::new();
            let settlement_transport = Asserter::new();
            let chain: BaseChain<F> =
                build_base_chain(&settings, endpoint_transport.clone(), settlement_transport);
            for (ordinal, held) in [(1000, [0x0a; 32]), (900, [0x0b; 32]), (800, [0x0c; 32])] {
                push_block(&endpoint_transport, ordinal, B256::from(held), 5000);
                push_block(&endpoint_transport, 1000, B256::from([0x0a; 32]), 5000);
                push_call_error(&endpoint_transport, "block is not canonical");
                let reference =
                    build_settlement_reference(SettlementReferenceConstructorParamsOverrides {
                        ordinal: Some(ordinal),
                        identity: Some([0x99; 32]),
                    });
                let context = build_attempt_context::<F>(AttemptContextOverrides {
                    reference: Some(reference),
                    ..Default::default()
                });

                // Act
                let tier = chain.tier_of(
                    build_tier_of_params(Default::default()),
                    build_tier_of_payload(&reference, Default::default()),
                );
                let evaluation = chain.evaluate_authorization(
                    build_evaluate_authorization_params(Default::default()),
                    build_evaluate_authorization_payload(&context, Default::default()),
                );

                // Assert
                match tier {
                    Err(TierOfErrorReturn::Reorganized {
                        reference: returned,
                    }) => assert_eq!(returned, reference),
                    _ => panic!("expected Reorganized"),
                }
                match evaluation {
                    Err(EvaluateAuthorizationErrorReturn::Reorganized {
                        reference: returned,
                    }) => assert_eq!(returned, reference),
                    _ => panic!("expected Reorganized"),
                }
            }
        }
    }

    run_form_concretes(Consumer);
}
// a_reorganized_reference_is_refused_whatever_its_number

/// Contract: the own entry
///   `a_delivered_envelope_is_recovered_from_the_settlement_that_created_the_interval`:
///   `recover_envelope` called for an interval created by a mint, a
///   delivery, a grant, and a replacement in turn against a history
///   holding all four → each returns the envelope of its own settlement
///   and its transaction hash.
/// Arrange: a history for entitlement `5` holding a `Minted` of interval
///   `1` with envelope `0xa1a1a1`, a `Delivered` of interval `1` with
///   `0xa9a9a9` followed by a `Delivered` of interval `2` with
///   `0xa2a2a2`, a `Replaced` of interval `3` with `0xa3a3a3`, and a
///   `Granted` of the new entitlement with `0xa4a4a4`; the transactions
///   `[0x71; 32]` to `[0x74; 32]` carrying the matching settlement calls.
/// Act:     `recover_envelope` for intervals `1`, `2`, `3`, and `0`,
///   inside the forms consumer, over `BaseChain::try_new_mocked`.
/// Assert:  each returns its own settlement's envelope and transaction
///   hash — `0xa1a1a1`/`[0x71; 32]`, `0xa2a2a2`/`[0x72; 32]`,
///   `0xa3a3a3`/`[0x73; 32]`, and `0xa4a4a4`/`[0x74; 32]`.
/// Boundary: `create_chain_forms_case`, `create_encoding`,
///   `create_chain_forms`, the concrete's producer, `consume_chain_forms`,
///   `BaseChain::try_new_mocked`, the sol bindings, and `recover_envelope`.
/// Mocked:  the two providers' transports through `Asserter`.
#[test]
fn a_delivered_envelope_is_recovered_from_the_settlement_that_created_the_interval() {
    #[derive(Clone, Copy)]
    struct Consumer;

    impl IChainFormsConsumer for Consumer {
        type Output = ();

        fn consume_chain_forms<F: IChainForms>(
            &self,
            _params: ConsumeChainFormsParams,
            _payload: ConsumeChainFormsPayload<F>,
        ) -> Self::Output {
            // Arrange
            let settings = build_base_chain_settings(Default::default());
            let endpoint_transport = Asserter::new();
            let settlement_transport = Asserter::new();
            let chain: BaseChain<F> =
                build_base_chain(&settings, endpoint_transport.clone(), settlement_transport);
            let minted = settlement_log(
                settings.entitlement_contract,
                B256::from([0x71; 32]),
                IEntitlement::Minted {
                    entitlement: U256::from(5),
                    recipient: Address::ZERO,
                    interval: 1,
                    envelope: Bytes::from(vec![0xa1, 0xa1, 0xa1]),
                    proof: Bytes::new(),
                }
                .encode_log_data(),
            );
            let earlier_delivered = settlement_log(
                settings.entitlement_contract,
                B256::from([0x79; 32]),
                IEntitlement::Delivered {
                    entitlement: U256::from(5),
                    from: Address::ZERO,
                    to: Address::ZERO,
                    interval: 1,
                    envelope: Bytes::from(vec![0xa9, 0xa9, 0xa9]),
                    proof: Bytes::new(),
                }
                .encode_log_data(),
            );
            let delivered = settlement_log(
                settings.entitlement_contract,
                B256::from([0x72; 32]),
                IEntitlement::Delivered {
                    entitlement: U256::from(5),
                    from: Address::ZERO,
                    to: Address::ZERO,
                    interval: 2,
                    envelope: Bytes::from(vec![0xa2, 0xa2, 0xa2]),
                    proof: Bytes::new(),
                }
                .encode_log_data(),
            );
            let replaced = settlement_log(
                settings.entitlement_contract,
                B256::from([0x73; 32]),
                IEntitlement::Replaced {
                    entitlement: U256::from(5),
                    interval: 3,
                    envelope: Bytes::from(vec![0xa3, 0xa3, 0xa3]),
                    proof: Bytes::new(),
                }
                .encode_log_data(),
            );
            let granted = settlement_log(
                settings.entitlement_contract,
                B256::from([0x74; 32]),
                IEntitlement::Granted {
                    sourceEntitlement: U256::from(4),
                    entitlement: U256::from(5),
                    recipient: Address::ZERO,
                    envelope: Bytes::from(vec![0xa4, 0xa4, 0xa4]),
                    proof: Bytes::new(),
                }
                .encode_log_data(),
            );
            let entitlement = entitlement_form::<F>(5);

            // Act / Assert
            for (interval, envelope, hash) in [
                (1u64, vec![0xa1, 0xa1, 0xa1], [0x71; 32]),
                (2, vec![0xa2, 0xa2, 0xa2], [0x72; 32]),
                (3, vec![0xa3, 0xa3, 0xa3], [0x73; 32]),
                (0, vec![0xa4, 0xa4, 0xa4], [0x74; 32]),
            ] {
                push_logs(&endpoint_transport, vec![minted.clone()]);
                push_logs(
                    &endpoint_transport,
                    vec![earlier_delivered.clone(), delivered.clone()],
                );
                push_logs(&endpoint_transport, vec![replaced.clone()]);
                push_logs(&endpoint_transport, vec![granted.clone()]);
                let calldata = match interval {
                    1 => IEntitlement::mintCall {
                        identity: Address::ZERO,
                        entitlement: U256::from(5),
                        recipient: Address::ZERO,
                        envelope: Bytes::from(vec![0xa1, 0xa1, 0xa1]),
                        mintProof: Bytes::new(),
                        intent: signed_intent(),
                    }
                    .abi_encode(),
                    2 => IEntitlement::deliverCall {
                        identity: Address::ZERO,
                        entitlement: U256::from(5),
                        previousEnvelope: Bytes::new(),
                        newEnvelope: Bytes::from(vec![0xa2, 0xa2, 0xa2]),
                        transferProof: Bytes::new(),
                        intent: signed_intent(),
                    }
                    .abi_encode(),
                    3 => IEntitlement::replaceCall {
                        identity: Address::ZERO,
                        entitlement: U256::from(5),
                        envelope: Bytes::from(vec![0xa3, 0xa3, 0xa3]),
                        mintProof: Bytes::new(),
                        intent: signed_intent(),
                    }
                    .abi_encode(),
                    _ => IEntitlement::grantCall {
                        identity: Address::ZERO,
                        sourceEntitlement: U256::from(4),
                        entitlement: U256::from(5),
                        recipient: Address::ZERO,
                        authorEnvelope: Bytes::new(),
                        envelope: Bytes::from(vec![0xa4, 0xa4, 0xa4]),
                        transferProof: Bytes::new(),
                        intent: signed_intent(),
                    }
                    .abi_encode(),
                };
                push_transaction(&endpoint_transport, B256::from(hash), Bytes::from(calldata));
                let interval = interval_form::<F>(interval);
                let success = chain
                    .recover_envelope(
                        build_recover_envelope_params(Default::default()),
                        build_recover_envelope_payload(&entitlement, &interval, Default::default()),
                    )
                    .expect("the interval's settlement is recovered");
                assert_eq!(success.recovered.envelope, envelope);
                assert_eq!(success.recovered.transaction, hash);
            }
        }
    }

    run_form_concretes(Consumer);
}
// a_delivered_envelope_is_recovered_from_the_settlement_that_created_the_interval

/// Contract: the own entry
///   `an_estimated_fee_carries_its_execution_gas_and_its_l1_data_fee_in_the_declared_order`:
///   `estimate_fee` called against an endpoint answering a gas estimate
///   and an oracle answering an L1 fee → the description's entries are
///   the declaration's names and units in its order with the two values.
/// Arrange: the endpoint answering gas `21000` and the oracle `3000`,
///   then gas `21000` and the oracle fee `2^128`.
/// Act:     `estimate_fee` twice, inside the forms consumer.
/// Assert:  the entries `execution_gas`/`gas`/`21000` and
///   `l1_data_fee`/`wei`/`3000` in that order; the second call is
///   `Err(Base(FeeOutOfRange { fee }))` with `fee` `2^128` as 32
///   big-endian bytes; the recorded requests are gas estimates and calls
///   only — nothing signed or sent.
/// Boundary: `create_chain_forms_case`, `create_encoding`,
///   `create_chain_forms`, the concrete's producer, `consume_chain_forms`,
///   the provider stack over the recorded `MockTransport`, the sol
///   bindings, and `estimate_fee`.
/// Mocked:  the two providers' transports through `Asserter`, recorded by
///   the `RecordingTransport` wrapper.
#[test]
fn an_estimated_fee_carries_its_execution_gas_and_its_l1_data_fee_in_the_declared_order() {
    #[derive(Clone, Copy)]
    struct Consumer;

    impl IChainFormsConsumer for Consumer {
        type Output = ();

        fn consume_chain_forms<F: IChainForms>(
            &self,
            _params: ConsumeChainFormsParams,
            _payload: ConsumeChainFormsPayload<F>,
        ) -> Self::Output {
            // Arrange
            let settings = build_base_chain_settings(Default::default());
            let (chain, endpoint_transport, _settlement_transport, endpoint_log, _settlement_log) =
                arrange_recording_base_chain_as::<F>(&settings);
            push_gas_estimate(&endpoint_transport, 21000);
            push_oracle_fee(&endpoint_transport, U256::from(3000));
            let request = build_fee_request::<F>(Default::default());

            // Act
            let success = chain
                .estimate_fee(
                    build_estimate_fee_params(Default::default()),
                    build_estimate_fee_payload(&request, Default::default()),
                )
                .expect("the fee is described");

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

            // Arrange the variation
            push_gas_estimate(&endpoint_transport, 21000);
            push_oracle_fee(&endpoint_transport, U256::from_limbs([0, 0, 1, 0]));

            // Act the variation
            let result = chain.estimate_fee(
                build_estimate_fee_params(Default::default()),
                build_estimate_fee_payload(&request, Default::default()),
            );

            // Assert
            assert!(matches!(
                result,
                Err(EstimateFeeErrorReturn::Base(
                    BaseEstimateFeeErrorReturn::FeeOutOfRange {
                        fee: [
                            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                            0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                        ]
                    }
                ))
            ));
            let requests = endpoint_log.take();
            assert_eq!(requests.len(), 4);
            assert_eq!(requests[0]["method"], "eth_estimateGas");
            assert_eq!(requests[1]["method"], "eth_call");
            assert_eq!(requests[2]["method"], "eth_estimateGas");
            assert_eq!(requests[3]["method"], "eth_call");
            assert_eq!(
                serde_json::from_value::<Address>(requests[1]["params"][0]["to"].clone())
                    .expect("the oracle call names an address"),
                Address::from([
                    0x42, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x0f,
                ])
            );
        }
    }

    run_form_concretes(Consumer);
}
// an_estimated_fee_carries_its_execution_gas_and_its_l1_data_fee_in_the_declared_order

/// Contract: the carried callee entries
///   `chain_forms_identity_survives_its_field_round_trip`,
///   `chain_forms_entitlement_survives_its_field_round_trip`,
///   `chain_forms_interval_survives_its_field_round_trip`, and
///   `chain_forms_chain_identifier_survives_its_field_round_trip`,
///   restated in this node's terms: an identity, an entitlement, an
///   interval, and a chain identifier form handed to
///   `evaluate_authorization` and the same forms read back through
///   `entitlement_contract` and `chain_identifier` → the value that
///   reaches the call data is the field the form yields, and the form
///   rebuilt equals the form that yielded it.
/// Arrange: the ascending bytes `0x01` to `0x14` admitted into
///   `F::Identity`, `0x01` to `0x20` into `F::Entitlement` and
///   `F::ChainIdentifier`, and `0x0102_0304_0506_0708` into `F::Interval`;
///   settings whose `entitlement_contract` and `chain_identifier` hold the
///   same bytes; a context carrying the identity, entitlement, and
///   interval forms; the endpoint answering `0`.
/// Act:     `evaluate_authorization`, `entitlement_contract`, and
///   `chain_identifier`, inside the forms consumer.
/// Assert:  `Authorized`; the decoded context's `identity` and
///   `entitlementContract` are the ascending twenty bytes, `entitlement`
///   the ascending word, and `interval` the value; `to_field` of each
///   handed form yields the field handed; `entitlement_contract` and
///   `chain_identifier` return forms equal to the handed ones.
/// Boundary: `create_chain_forms_case`, `create_encoding`,
///   `create_chain_forms`, the concrete's producer, `consume_chain_forms`,
///   the provider stack over the recorded `MockTransport`, the sol
///   bindings, the three adapter methods, and each form's `from_field`
///   and `to_field`.
/// Mocked:  the two providers' transports through `Asserter`, recorded by
///   the `RecordingTransport` wrapper.
#[test]
fn chain_forms_survive_their_field_round_trip_through_the_call_data() {
    #[derive(Clone, Copy)]
    struct Consumer;

    impl IChainFormsConsumer for Consumer {
        type Output = ();

        fn consume_chain_forms<F: IChainForms>(
            &self,
            _params: ConsumeChainFormsParams,
            _payload: ConsumeChainFormsPayload<F>,
        ) -> Self::Output {
            // Arrange
            let identity_bytes = [
                0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e,
                0x0f, 0x10, 0x11, 0x12, 0x13, 0x14,
            ];
            let word_bytes = [
                0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e,
                0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c,
                0x1d, 0x1e, 0x1f, 0x20,
            ];
            let settings = build_base_chain_settings(BaseChainSettingsOverrides {
                entitlement_contract: Some(identity_bytes),
                chain_identifier: Some(word_bytes),
                ..Default::default()
            });
            let (chain, endpoint_transport, _settlement_transport, endpoint_log, _settlement_log) =
                arrange_recording_base_chain_as::<F>(&settings);
            let identity_form = F::Identity::from_field(
                FromFieldParams,
                CanonicalFieldValue::FixedBytes20(identity_bytes),
            )
            .expect("the canonical fixed-bytes-20 value is admitted")
            .value;
            let entitlement_form = F::Entitlement::from_field(
                FromFieldParams,
                CanonicalFieldValue::Unsigned256(word_bytes),
            )
            .expect("the canonical unsigned-256 value is admitted")
            .value;
            let interval_form = F::Interval::from_field(
                FromFieldParams,
                CanonicalFieldValue::Unsigned64(0x0102_0304_0506_0708),
            )
            .expect("the canonical unsigned-64 value is admitted")
            .value;
            let chain_identifier_form = F::ChainIdentifier::from_field(
                FromFieldParams,
                CanonicalFieldValue::Unsigned256(word_bytes),
            )
            .expect("the canonical unsigned-256 value is admitted")
            .value;
            let context = build_attempt_context::<F>(AttemptContextOverrides {
                entitlement_contract: Some(identity_form.clone()),
                entitlement: Some(entitlement_form.clone()),
                identity: Some(identity_form.clone()),
                interval: Some(interval_form.clone()),
                ..Default::default()
            });
            push_call_result(
                &endpoint_transport,
                Bytes::from(Registry::evaluateAuthorizationCall::abi_encode_returns(
                    &0u8,
                )),
            );

            // Act
            let success = chain
                .evaluate_authorization(
                    build_evaluate_authorization_params(Default::default()),
                    build_evaluate_authorization_payload(&context, Default::default()),
                )
                .expect("the call decodes");
            let contract = chain
                .entitlement_contract(
                    build_entitlement_contract_params(Default::default()),
                    build_entitlement_contract_payload(Default::default()),
                )
                .expect("the settings' contract admits into the form")
                .contract;
            let chain_identifier = chain
                .chain_identifier(
                    build_chain_identifier_params(Default::default()),
                    build_chain_identifier_payload(Default::default()),
                )
                .expect("the settings' identifier admits into the form")
                .chain_identifier;

            // Assert
            assert_eq!(success.result, AuthorizationResult::Authorized);
            assert_eq!(
                F::Identity::to_field(ToFieldParams, &identity_form)
                    .expect("the identity form yields its field")
                    .field,
                CanonicalFieldValue::FixedBytes20(identity_bytes)
            );
            assert_eq!(
                F::Entitlement::to_field(ToFieldParams, &entitlement_form)
                    .expect("the entitlement form yields its field")
                    .field,
                CanonicalFieldValue::Unsigned256(word_bytes)
            );
            assert_eq!(
                F::Interval::to_field(ToFieldParams, &interval_form)
                    .expect("the interval form yields its field")
                    .field,
                CanonicalFieldValue::Unsigned64(0x0102_0304_0506_0708)
            );
            assert_eq!(
                F::ChainIdentifier::to_field(ToFieldParams, &chain_identifier_form)
                    .expect("the chain identifier form yields its field")
                    .field,
                CanonicalFieldValue::Unsigned256(word_bytes)
            );
            let requests = endpoint_log.take();
            assert_eq!(requests.len(), 1);
            let input: Bytes = serde_json::from_value(requests[0]["params"][0]["input"].clone())
                .expect("the call input is bytes");
            let decoded = Registry::evaluateAuthorizationCall::abi_decode(&input)
                .expect("the call input decodes through the binding");
            assert_eq!(
                decoded.context.entitlementContract,
                Address::from(identity_bytes)
            );
            assert_eq!(decoded.context.identity, Address::from(identity_bytes));
            assert_eq!(decoded.context.entitlement, U256::from_be_bytes(word_bytes));
            assert_eq!(decoded.context.interval, 0x0102_0304_0506_0708);
            assert_eq!(contract, identity_form);
            assert_eq!(chain_identifier, chain_identifier_form);
        }
    }

    run_form_concretes(Consumer);
}
// chain_forms_survive_their_field_round_trip_through_the_call_data

/// Contract: the carried callee entry
///   `an_admitted_commitment_reads_back_its_bytes_and_its_scheme`,
///   restated in this node's terms: a commitment admitted with nonzero
///   bytes and a scheme differing from the builder's as the context's
///   handshake key → the context call data carries the commitment's bytes
///   and the scheme's identifier unchanged.
/// Arrange: the real `HandshakeKeyCommitment::try_new` over the ascending
///   bytes `0x01` to `0x20` and the scheme `0x5a`; a context carrying the
///   commitment; the endpoint answering `0`.
/// Act:     `evaluate_authorization`, inside the forms consumer.
/// Assert:  `as_bytes` and `scheme().identifier()` read back the handed
///   bytes and `0x5a`; the decoded context's `handshakeKeyCommitment` is
///   the ascending bytes and `handshakeKeyScheme` is `0x5a`.
/// Boundary: `create_chain_forms_case`, `create_encoding`,
///   `create_chain_forms`, the concrete's producer, `consume_chain_forms`,
///   the provider stack over the recorded `MockTransport`, the sol
///   bindings, `HandshakeKeyCommitment::try_new`, `as_bytes`, `scheme`,
///   and `evaluate_authorization`.
/// Mocked:  the two providers' transports through `Asserter`, recorded by
///   the `RecordingTransport` wrapper.
#[test]
fn a_handshake_commitment_reaches_the_call_data_unchanged() {
    #[derive(Clone, Copy)]
    struct Consumer;

    impl IChainFormsConsumer for Consumer {
        type Output = ();

        fn consume_chain_forms<F: IChainForms>(
            &self,
            _params: ConsumeChainFormsParams,
            _payload: ConsumeChainFormsPayload<F>,
        ) -> Self::Output {
            // Arrange
            let settings = build_base_chain_settings(Default::default());
            let (chain, endpoint_transport, _settlement_transport, endpoint_log, _settlement_log) =
                arrange_recording_base_chain_as::<F>(&settings);
            let bytes = [
                0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e,
                0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c,
                0x1d, 0x1e, 0x1f, 0x20,
            ];
            let commitment =
                HandshakeKeyCommitment::try_new(HandshakeKeyCommitmentConstructorParams {
                    bytes,
                    scheme: build_signature_scheme_identifier(
                        SignatureSchemeIdentifierConstructorParamsOverrides {
                            identifier: Some(0x5a),
                        },
                    ),
                })
                .expect("the nonzero commitment is admitted");
            let context = build_attempt_context::<F>(AttemptContextOverrides {
                handshake_key: Some(commitment.clone()),
                ..Default::default()
            });
            push_call_result(
                &endpoint_transport,
                Bytes::from(Registry::evaluateAuthorizationCall::abi_encode_returns(
                    &0u8,
                )),
            );

            // Act
            let success = chain
                .evaluate_authorization(
                    build_evaluate_authorization_params(Default::default()),
                    build_evaluate_authorization_payload(&context, Default::default()),
                )
                .expect("the call decodes");

            // Assert
            assert_eq!(success.result, AuthorizationResult::Authorized);
            assert_eq!(*commitment.as_bytes(), bytes);
            assert_eq!(commitment.scheme().identifier(), 0x5a);
            let requests = endpoint_log.take();
            assert_eq!(requests.len(), 1);
            let input: Bytes = serde_json::from_value(requests[0]["params"][0]["input"].clone())
                .expect("the call input is bytes");
            let decoded = Registry::evaluateAuthorizationCall::abi_decode(&input)
                .expect("the call input decodes through the binding");
            assert_eq!(decoded.context.handshakeKeyCommitment, B256::from(bytes));
            assert_eq!(decoded.context.handshakeKeyScheme, 0x5a);
        }
    }

    run_form_concretes(Consumer);
}
// a_handshake_commitment_reaches_the_call_data_unchanged
