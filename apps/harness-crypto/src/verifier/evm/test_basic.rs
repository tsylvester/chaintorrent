#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use alloy::primitives::Address;
use alloy::transports::mock::Asserter;
use chain::{IChainAdapter, IEntitlementStateAdapter};
use encoding::{CanonicalFieldKind, CanonicalFieldValue, FromFieldErrorReturn};
use pairing::{IPairingArithmetic, IPairingReference};
use proof::IDeliveryProofAdapter;

use super::interface::{
    ConsumeEvmVerifierCaseParams, ConsumeEvmVerifierCasePayload, EvmVerifier, EvmVerifierField,
    EvmVerifierTryNewErrorReturn, IEvmVerifierCaseConsumer,
};
use super::mock::*;

/// Contract: an admitted address → `Ok` holding it as the admitted form
///   and as the twenty bytes it carried.
/// Arrange: `verifier_address` `CanonicalFieldValue::FixedBytes20(
///   [0x42; 20])` on the constructor params overrides.
/// Act:     `EvmVerifier::try_new_mocked` with a fresh `Asserter` over
///   the received chain.
/// Assert:  the instance's `verifier_address` equals `Address::from(
///   [0x42; 20])` by `assert_eq!`.
/// Collaborators: the pairing's, hash-to-scalar's, encoding's,
///   chain-forms', chain's, delivery-proof's, and random-source's mock
///   concretes in their default modes through their factories.
#[test]
fn evm_verifier_try_new_holds_the_configured_verifier_address() {
    // Arrange
    let asserter = Asserter::default();

    struct Consumer<'a> {
        asserter: &'a Asserter,
    }

    impl IEvmVerifierCaseConsumer for Consumer<'_> {
        type Output = ();

        fn consume_evm_verifier_case<P, C, D>(
            &self,
            _params: ConsumeEvmVerifierCaseParams,
            payload: ConsumeEvmVerifierCasePayload<'_, P, C, D>,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            C: IChainAdapter,
            D: IDeliveryProofAdapter<Pairing = P, Forms = <C as IEntitlementStateAdapter>::Forms>,
        {
            // Act
            let result = EvmVerifier::try_new_mocked(
                build_evm_verifier_constructor_params(
                    payload.pairing,
                    payload.chain,
                    payload.delivery_proof,
                    EvmVerifierConstructorParamsOverrides {
                        verifier_address: Some(CanonicalFieldValue::FixedBytes20([0x42; 20])),
                        ..Default::default()
                    },
                ),
                self.asserter.clone(),
            );

            // Assert
            let Ok(instance) = result else {
                panic!("an admitted address constructs the concrete")
            };
            assert_eq!(instance.verifier_address, Address::from([0x42; 20]));
        }
    }

    // Act
    let result = run_evm_verifier_case(
        &build_evm_verifier_case_deps(
            Consumer {
                asserter: &asserter,
            },
            Default::default(),
        ),
        build_evm_verifier_case_params(Default::default()),
        build_evm_verifier_case_payload(Default::default()),
    );

    // Assert
    let Ok(_success) = result else {
        panic!("the case chain admits every selection")
    };
    // evm_verifier_try_new_holds_the_configured_verifier_address
}

/// Contract: an address whose canonical field value the chain forms'
///   identity form refuses → `Err(EvmVerifierTryNewErrorReturn::Identity
///   { field: EvmVerifierField::VerifierAddress, error })`, `error`
///   unchanged.
/// Arrange: `verifier_address` `CanonicalFieldValue::Unsigned16(1)` on
///   the constructor params overrides.
/// Act:     `EvmVerifier::try_new_mocked` over the received chain.
/// Assert:  `matches!(result, Err(EvmVerifierTryNewErrorReturn::Identity
///   { field: EvmVerifierField::VerifierAddress, error:
///   FromFieldErrorReturn::WrongKind { expected: CanonicalFieldKind::
///   FixedBytes20 } }))`, the pattern naming every field because
///   `EvmVerifier` omits `Debug` and `PartialEq`.
/// Collaborators: the pairing's, hash-to-scalar's, encoding's,
///   chain-forms', chain's, delivery-proof's, and random-source's mock
///   concretes in their default modes through their factories.
#[test]
fn evm_verifier_try_new_refuses_a_verifier_address_of_another_kind() {
    // Arrange
    let asserter = Asserter::default();

    struct Consumer<'a> {
        asserter: &'a Asserter,
    }

    impl IEvmVerifierCaseConsumer for Consumer<'_> {
        type Output = ();

        fn consume_evm_verifier_case<P, C, D>(
            &self,
            _params: ConsumeEvmVerifierCaseParams,
            payload: ConsumeEvmVerifierCasePayload<'_, P, C, D>,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            C: IChainAdapter,
            D: IDeliveryProofAdapter<Pairing = P, Forms = <C as IEntitlementStateAdapter>::Forms>,
        {
            // Act
            let result = EvmVerifier::try_new_mocked(
                build_evm_verifier_constructor_params(
                    payload.pairing,
                    payload.chain,
                    payload.delivery_proof,
                    EvmVerifierConstructorParamsOverrides {
                        verifier_address: Some(CanonicalFieldValue::Unsigned16(1)),
                        ..Default::default()
                    },
                ),
                self.asserter.clone(),
            );

            // Assert
            assert!(matches!(
                result,
                Err(EvmVerifierTryNewErrorReturn::Identity {
                    field: EvmVerifierField::VerifierAddress,
                    error: FromFieldErrorReturn::WrongKind {
                        expected: CanonicalFieldKind::FixedBytes20
                    }
                })
            ));
        }
    }

    // Act
    let result = run_evm_verifier_case(
        &build_evm_verifier_case_deps(
            Consumer {
                asserter: &asserter,
            },
            Default::default(),
        ),
        build_evm_verifier_case_params(Default::default()),
        build_evm_verifier_case_payload(Default::default()),
    );

    // Assert
    let Ok(_success) = result else {
        panic!("the case chain admits every selection")
    };
    // evm_verifier_try_new_refuses_a_verifier_address_of_another_kind
}

/// Contract: `try_new` over an admitted address and the chain adapter's
///   endpoint → `Ok` holding the address, the HTTP provider built over
///   the endpoint's text and the runtime built, and no request made.
/// Arrange: `build_evm_verifier_constructor_params` over the received
///   pairing, chain, and delivery proof adapter with `verifier_address`
///   `FixedBytes20([0x42; 20])`; the endpoint the mock chain returns is
///   a loopback text nothing listens at, so a request made would fail
///   the block.
/// Act:     `EvmVerifier::try_new`.
/// Assert:  the instance's `verifier_address` equals `Address::from(
///   [0x42; 20])` by `assert_eq!`.
/// Collaborators: the pairing's, hash-to-scalar's, encoding's,
///   chain-forms', chain's, delivery-proof's, and random-source's mock
///   concretes in their default modes through their factories.
#[test]
fn evm_verifier_try_new_builds_the_http_provider_and_runtime_at_the_chains_endpoint_without_reading_the_chain()
 {
    struct Consumer;

    impl IEvmVerifierCaseConsumer for Consumer {
        type Output = ();

        fn consume_evm_verifier_case<P, C, D>(
            &self,
            _params: ConsumeEvmVerifierCaseParams,
            payload: ConsumeEvmVerifierCasePayload<'_, P, C, D>,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            C: IChainAdapter,
            D: IDeliveryProofAdapter<Pairing = P, Forms = <C as IEntitlementStateAdapter>::Forms>,
        {
            // Act
            let result = EvmVerifier::try_new(build_evm_verifier_constructor_params(
                payload.pairing,
                payload.chain,
                payload.delivery_proof,
                EvmVerifierConstructorParamsOverrides {
                    verifier_address: Some(CanonicalFieldValue::FixedBytes20([0x42; 20])),
                    ..Default::default()
                },
            ));

            // Assert
            let Ok(instance) = result else {
                panic!("an admitted address builds the provider and runtime")
            };
            assert_eq!(instance.verifier_address, Address::from([0x42; 20]));
        }
    }

    // Act
    let result = run_evm_verifier_case(
        &build_evm_verifier_case_deps(Consumer, Default::default()),
        build_evm_verifier_case_params(Default::default()),
        build_evm_verifier_case_payload(Default::default()),
    );

    // Assert
    let Ok(_success) = result else {
        panic!("the case chain admits every selection")
    };
    // evm_verifier_try_new_builds_the_http_provider_and_runtime_at_the_chains_endpoint_without_reading_the_chain
}

/// Contract: the production constructor decides the address exactly as
///   the scripted one does → `Err(EvmVerifierTryNewErrorReturn::Identity
///   { field, error })`.
/// Arrange: `verifier_address` `CanonicalFieldValue::Unsigned16(1)` on
///   the constructor params overrides.
/// Act:     `EvmVerifier::try_new` over the received chain.
/// Assert:  `matches!(result, Err(EvmVerifierTryNewErrorReturn::Identity
///   { field: EvmVerifierField::VerifierAddress, error:
///   FromFieldErrorReturn::WrongKind { expected: CanonicalFieldKind::
///   FixedBytes20 } }))`.
/// Collaborators: the pairing's, hash-to-scalar's, encoding's,
///   chain-forms', chain's, delivery-proof's, and random-source's mock
///   concretes in their default modes through their factories.
#[test]
fn evm_verifier_try_new_refuses_an_address_of_another_kind_as_try_new_mocked_does() {
    struct Consumer;

    impl IEvmVerifierCaseConsumer for Consumer {
        type Output = ();

        fn consume_evm_verifier_case<P, C, D>(
            &self,
            _params: ConsumeEvmVerifierCaseParams,
            payload: ConsumeEvmVerifierCasePayload<'_, P, C, D>,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            C: IChainAdapter,
            D: IDeliveryProofAdapter<Pairing = P, Forms = <C as IEntitlementStateAdapter>::Forms>,
        {
            // Act
            let result = EvmVerifier::try_new(build_evm_verifier_constructor_params(
                payload.pairing,
                payload.chain,
                payload.delivery_proof,
                EvmVerifierConstructorParamsOverrides {
                    verifier_address: Some(CanonicalFieldValue::Unsigned16(1)),
                    ..Default::default()
                },
            ));

            // Assert
            assert!(matches!(
                result,
                Err(EvmVerifierTryNewErrorReturn::Identity {
                    field: EvmVerifierField::VerifierAddress,
                    error: FromFieldErrorReturn::WrongKind {
                        expected: CanonicalFieldKind::FixedBytes20
                    }
                })
            ));
        }
    }

    // Act
    let result = run_evm_verifier_case(
        &build_evm_verifier_case_deps(Consumer, Default::default()),
        build_evm_verifier_case_params(Default::default()),
        build_evm_verifier_case_payload(Default::default()),
    );

    // Assert
    let Ok(_success) = result else {
        panic!("the case chain admits every selection")
    };
    // evm_verifier_try_new_refuses_an_address_of_another_kind_as_try_new_mocked_does
}
