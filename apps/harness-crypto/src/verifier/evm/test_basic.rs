#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::marker::PhantomData;

use alloy::primitives::Address;
use alloy::transports::mock::Asserter;
use chain::{
    ConsumeChainFormsParams, ConsumeChainFormsPayload, IChainForms, IChainFormsConsumer,
    build_create_chain_forms_deps, build_create_chain_forms_params,
    build_create_chain_forms_payload, create_chain_forms,
};
use encoding::{
    CanonicalFieldKind, CanonicalFieldValue, ConsumeEncodingParams, ConsumeEncodingPayload,
    FromFieldErrorReturn, IDecoderAdapter, IEncoderAdapter, IEncodingConsumer,
    build_create_encoding_deps, build_create_encoding_params, build_create_encoding_payload,
    create_encoding,
};
use hash_to_scalar::{
    IHashToScalarAdapter, build_create_hash_to_scalar_deps, build_create_hash_to_scalar_params,
    build_create_hash_to_scalar_payload, create_hash_to_scalar,
};
use pairing::{
    ConsumePairingParams, ConsumePairingPayload, IPairingArithmetic, IPairingConsumer,
    IPairingReference, VerifierGroupArithmetic, build_create_pairing_deps,
    build_create_pairing_params, build_create_pairing_payload, create_pairing,
};
use proof::{
    ConsumeDeliveryProofParams, ConsumeDeliveryProofPayload, DELIVERY_STATEMENT_VERSION_ONE,
    IDeliveryProofAdapter, IDeliveryProofConsumer, build_create_delivery_proof_deps,
    build_create_delivery_proof_params, build_create_delivery_proof_payload, create_delivery_proof,
};
use random::{
    CreateRandomSourceDeps, CreateRandomSourcePayload, IRandomSourceAdapter,
    build_create_random_source_params, create_random_source,
};
use url::ParseError;

use super::super::factory::provides::VERIFIER_INTERFACE_VERSION;
use super::interface::{
    EVM_VERIFIER_DECLARATION, EvmVerifier, EvmVerifierEndpoint,
    EvmVerifierEndpointConstructorParams, EvmVerifierEndpointTryNewErrorReturn, EvmVerifierField,
    EvmVerifierSettings, EvmVerifierTryNewErrorReturn,
};
use super::mock::*;

/// Contract: text that `Url::parse` admits → `Ok(EvmVerifierEndpoint {
///   url })`.
/// Arrange: `build_evm_verifier_endpoint_constructor_params` with `text`
///   overridden to `"https://sepolia.base.org/rpc"`, a path-bearing text
///   `Url` does not normalize.
/// Act:     `EvmVerifierEndpoint::try_new`.
/// Assert:  `result.map(|endpoint| endpoint.url.as_str().to_string())`
///   equals `Ok("https://sepolia.base.org/rpc".to_string())` by
///   `assert_eq!`.
#[test]
fn evm_verifier_endpoint_admits_a_url_text() {
    // Arrange
    let params: EvmVerifierEndpointConstructorParams =
        build_evm_verifier_endpoint_constructor_params(
            EvmVerifierEndpointConstructorParamsOverrides {
                text: Some("https://sepolia.base.org/rpc".to_string()),
            },
        );

    // Act
    let result = EvmVerifierEndpoint::try_new(params);

    // Assert
    assert_eq!(
        result.map(|endpoint| endpoint.url.as_str().to_string()),
        Ok("https://sepolia.base.org/rpc".to_string())
    );
    // evm_verifier_endpoint_admits_a_url_text
}

/// Contract: text `Url::parse` refuses → `Err(EvmVerifierEndpointTryNew
///   ErrorReturn::Unparsable { error })`, `error` unchanged.
/// Arrange: `build_evm_verifier_endpoint_constructor_params` with `text`
///   overridden to `"not a url"`.
/// Act:     `EvmVerifierEndpoint::try_new`.
/// Assert:  the result equals `Err(EvmVerifierEndpointTryNewErrorReturn::
///   Unparsable { error: ParseError::RelativeUrlWithoutBase })` by
///   `assert_eq!`.
#[test]
fn evm_verifier_endpoint_refuses_text_that_is_not_a_url() {
    // Arrange
    let params: EvmVerifierEndpointConstructorParams =
        build_evm_verifier_endpoint_constructor_params(
            EvmVerifierEndpointConstructorParamsOverrides {
                text: Some("not a url".to_string()),
            },
        );

    // Act
    let result = EvmVerifierEndpoint::try_new(params);

    // Assert
    assert_eq!(
        result,
        Err(EvmVerifierEndpointTryNewErrorReturn::Unparsable {
            error: ParseError::RelativeUrlWithoutBase
        })
    );
    // evm_verifier_endpoint_refuses_text_that_is_not_a_url
}

/// Contract: `EVM_VERIFIER_DECLARATION`, the declaration `EvmVerifier`'s
///   `IVerifierAdapter` implementation names, holds the forms, versions,
///   and adapter version the interaction spec lists.
/// Arrange: none; the declaration depends on no type parameter of
///   `EvmVerifier`.
/// Act:     read `EVM_VERIFIER_DECLARATION`.
/// Assert:  its `verifier_forms` equals `[VerifierGroupArithmetic::
///   FirstGroupOnly, VerifierGroupArithmetic::BothGroups]`, its
///   `statement_versions` equals `[DELIVERY_STATEMENT_VERSION_ONE]`, its
///   `adapter_version` equals `1`, and its `interface_version` equals
///   `VERIFIER_INTERFACE_VERSION`, by `assert_eq!`, the two forms
///   differing so a swapped or dropped entry fails.
#[test]
fn evm_verifier_declares_both_verifier_forms_and_statement_version_one() {
    // Act
    let declaration = EVM_VERIFIER_DECLARATION;

    // Assert
    assert_eq!(
        declaration.verifier_forms,
        [
            VerifierGroupArithmetic::FirstGroupOnly,
            VerifierGroupArithmetic::BothGroups
        ]
        .as_slice()
    );
    assert_eq!(
        declaration.statement_versions,
        [DELIVERY_STATEMENT_VERSION_ONE].as_slice()
    );
    assert_eq!(declaration.adapter_version, 1);
    assert_eq!(declaration.interface_version, VERIFIER_INTERFACE_VERSION);
    // evm_verifier_declares_both_verifier_forms_and_statement_version_one
}

/// Contract: two admitted addresses → `Ok` holding each as the twenty
///   bytes its field carried.
/// Arrange: `settings` by `build_evm_verifier_settings` with
///   `verifier_address` `CanonicalFieldValue::FixedBytes20([0x42; 20])`
///   and `gas_price_oracle` `CanonicalFieldValue::FixedBytes20([0x43;
///   20])`, two distinct values so a swapped field fails.
/// Act:     `EvmVerifier::try_new_mocked` with a fresh `Asserter`.
/// Assert:  the instance's `verifier` equals `Address::from([0x42; 20])`
///   and its `oracle` equals `Address::from([0x43; 20])` by `assert_eq!`.
/// Collaborators: the pairing's, hash-to-scalar's, encoding's,
///   chain-forms', delivery-proof's, and random-source's mock concretes
///   in their default modes through their factories.
#[test]
fn evm_verifier_try_new_holds_the_configured_addresses() {
    // Arrange
    let settings = build_evm_verifier_settings(EvmVerifierSettingsOverrides {
        verifier_address: Some(CanonicalFieldValue::FixedBytes20([0x42; 20])),
        gas_price_oracle: Some(CanonicalFieldValue::FixedBytes20([0x43; 20])),
        ..Default::default()
    });
    let asserter = Asserter::default();

    struct DeliveryProofConsumer<'a, P: IPairingArithmetic, F: IChainForms> {
        pairing: &'a P,
        settings: &'a EvmVerifierSettings,
        asserter: &'a Asserter,
        forms: PhantomData<F>,
    }

    impl<'a, P, F> IDeliveryProofConsumer<P, F> for DeliveryProofConsumer<'a, P, F>
    where
        P: IPairingArithmetic + IPairingReference,
        F: IChainForms,
    {
        type Output = ();

        fn consume_delivery_proof<D: IDeliveryProofAdapter<Pairing = P, Forms = F>>(
            &self,
            _params: ConsumeDeliveryProofParams,
            payload: ConsumeDeliveryProofPayload<D>,
        ) -> Self::Output {
            // Act
            let result = EvmVerifier::try_new_mocked(
                build_evm_verifier_constructor_params(
                    self.pairing,
                    &payload.adapter,
                    EvmVerifierConstructorParamsOverrides {
                        settings: Some(self.settings.clone()),
                    },
                ),
                self.asserter.clone(),
            );

            // Assert
            let Ok(instance) = result else {
                panic!("two admitted addresses construct the concrete")
            };
            assert_eq!(instance.verifier, Address::from([0x42; 20]));
            assert_eq!(instance.oracle, Address::from([0x43; 20]));
        }
    }

    struct ChainFormsConsumer<'a, P: IPairingArithmetic, E> {
        pairing: &'a P,
        hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
        encoder: &'a E,
        random: &'a dyn IRandomSourceAdapter,
        settings: &'a EvmVerifierSettings,
        asserter: &'a Asserter,
    }

    impl<'a, P, E> IChainFormsConsumer for ChainFormsConsumer<'a, P, E>
    where
        P: IPairingArithmetic + IPairingReference,
        E: IEncoderAdapter,
    {
        type Output = ();

        fn consume_chain_forms<F: IChainForms>(
            &self,
            _params: ConsumeChainFormsParams,
            _payload: ConsumeChainFormsPayload<F>,
        ) -> Self::Output {
            let created = create_delivery_proof(
                &build_create_delivery_proof_deps(
                    self.pairing,
                    self.hashing,
                    self.encoder,
                    self.random,
                    DeliveryProofConsumer {
                        pairing: self.pairing,
                        settings: self.settings,
                        asserter: self.asserter,
                        forms: PhantomData::<F>,
                    },
                    Default::default(),
                ),
                build_create_delivery_proof_params(Default::default()),
                build_create_delivery_proof_payload(Default::default()),
            );
            let Ok(_success) = created else {
                panic!("the delivery-proof factory admits the selection")
            };
        }
    }

    struct EncodingConsumer<'a, P: IPairingArithmetic> {
        pairing: &'a P,
        hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
        random: &'a dyn IRandomSourceAdapter,
        settings: &'a EvmVerifierSettings,
        asserter: &'a Asserter,
    }

    impl<'a, P> IEncodingConsumer for EncodingConsumer<'a, P>
    where
        P: IPairingArithmetic + IPairingReference,
    {
        type Output = ();

        fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
            &self,
            _params: ConsumeEncodingParams,
            payload: ConsumeEncodingPayload<E>,
        ) -> Self::Output {
            let created = create_chain_forms(
                &build_create_chain_forms_deps(
                    ChainFormsConsumer {
                        pairing: self.pairing,
                        hashing: self.hashing,
                        encoder: &payload.adapter,
                        random: self.random,
                        settings: self.settings,
                        asserter: self.asserter,
                    },
                    Default::default(),
                ),
                build_create_chain_forms_params(Default::default()),
                build_create_chain_forms_payload(Default::default()),
            );
            let Ok(_success) = created else {
                panic!("the chain-forms factory admits the selection")
            };
        }
    }

    struct PairingConsumer<'a> {
        settings: &'a EvmVerifierSettings,
        asserter: &'a Asserter,
    }

    impl<'a> IPairingConsumer for PairingConsumer<'a> {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            let hashing = create_hash_to_scalar::<P::Scalar>(
                &build_create_hash_to_scalar_deps(Default::default()),
                build_create_hash_to_scalar_params(Default::default()),
                build_create_hash_to_scalar_payload(Default::default()),
            );
            let Ok(hashing) = hashing else {
                panic!("the hash-to-scalar factory admits the selection")
            };
            let Ok(source) = create_random_source(
                &CreateRandomSourceDeps,
                build_create_random_source_params(Default::default()),
                CreateRandomSourcePayload,
            );
            let created = create_encoding(
                &build_create_encoding_deps(EncodingConsumer {
                    pairing: &payload.adapter,
                    hashing: &*hashing.adapter,
                    random: &*source.adapter,
                    settings: self.settings,
                    asserter: self.asserter,
                }),
                build_create_encoding_params(Default::default()),
                build_create_encoding_payload(Default::default()),
            );
            let Ok(_success) = created else {
                panic!("the encoding factory admits the selection")
            };
        }
    }

    let consumer = PairingConsumer {
        settings: &settings,
        asserter: &asserter,
    };
    let created = create_pairing(
        &build_create_pairing_deps(consumer, Default::default()),
        build_create_pairing_params(Default::default()),
        build_create_pairing_payload(Default::default()),
    );
    let Ok(_success) = created else {
        panic!("the pairing factory ran the consumer")
    };
    // evm_verifier_try_new_holds_the_configured_addresses
}

/// Contract: an address whose canonical field value the chain forms'
///   identity form refuses → `Err(EvmVerifierTryNewErrorReturn::Identity
///   { field: EvmVerifierField::VerifierAddress, error })`, `error`
///   unchanged.
/// Arrange: `verifier_address` `CanonicalFieldValue::Unsigned16(1)` and
///   the oracle's default.
/// Act:     `EvmVerifier::try_new_mocked`.
/// Assert:  `matches!(result, Err(EvmVerifierTryNewErrorReturn::Identity
///   { field: EvmVerifierField::VerifierAddress, error:
///   FromFieldErrorReturn::WrongKind { expected: CanonicalFieldKind::
///   FixedBytes20 } }))`, the pattern naming every field because
///   `EvmVerifier` omits `Debug` and `PartialEq`.
/// Collaborators: the pairing's, hash-to-scalar's, encoding's,
///   chain-forms', delivery-proof's, and random-source's mock concretes
///   in their default modes through their factories.
#[test]
fn evm_verifier_try_new_refuses_a_verifier_address_of_another_kind() {
    // Arrange
    let settings = build_evm_verifier_settings(EvmVerifierSettingsOverrides {
        verifier_address: Some(CanonicalFieldValue::Unsigned16(1)),
        ..Default::default()
    });
    let asserter = Asserter::default();

    struct DeliveryProofConsumer<'a, P: IPairingArithmetic, F: IChainForms> {
        pairing: &'a P,
        settings: &'a EvmVerifierSettings,
        asserter: &'a Asserter,
        forms: PhantomData<F>,
    }

    impl<'a, P, F> IDeliveryProofConsumer<P, F> for DeliveryProofConsumer<'a, P, F>
    where
        P: IPairingArithmetic + IPairingReference,
        F: IChainForms,
    {
        type Output = ();

        fn consume_delivery_proof<D: IDeliveryProofAdapter<Pairing = P, Forms = F>>(
            &self,
            _params: ConsumeDeliveryProofParams,
            payload: ConsumeDeliveryProofPayload<D>,
        ) -> Self::Output {
            // Act
            let result = EvmVerifier::try_new_mocked(
                build_evm_verifier_constructor_params(
                    self.pairing,
                    &payload.adapter,
                    EvmVerifierConstructorParamsOverrides {
                        settings: Some(self.settings.clone()),
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

    struct ChainFormsConsumer<'a, P: IPairingArithmetic, E> {
        pairing: &'a P,
        hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
        encoder: &'a E,
        random: &'a dyn IRandomSourceAdapter,
        settings: &'a EvmVerifierSettings,
        asserter: &'a Asserter,
    }

    impl<'a, P, E> IChainFormsConsumer for ChainFormsConsumer<'a, P, E>
    where
        P: IPairingArithmetic + IPairingReference,
        E: IEncoderAdapter,
    {
        type Output = ();

        fn consume_chain_forms<F: IChainForms>(
            &self,
            _params: ConsumeChainFormsParams,
            _payload: ConsumeChainFormsPayload<F>,
        ) -> Self::Output {
            let created = create_delivery_proof(
                &build_create_delivery_proof_deps(
                    self.pairing,
                    self.hashing,
                    self.encoder,
                    self.random,
                    DeliveryProofConsumer {
                        pairing: self.pairing,
                        settings: self.settings,
                        asserter: self.asserter,
                        forms: PhantomData::<F>,
                    },
                    Default::default(),
                ),
                build_create_delivery_proof_params(Default::default()),
                build_create_delivery_proof_payload(Default::default()),
            );
            let Ok(_success) = created else {
                panic!("the delivery-proof factory admits the selection")
            };
        }
    }

    struct EncodingConsumer<'a, P: IPairingArithmetic> {
        pairing: &'a P,
        hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
        random: &'a dyn IRandomSourceAdapter,
        settings: &'a EvmVerifierSettings,
        asserter: &'a Asserter,
    }

    impl<'a, P> IEncodingConsumer for EncodingConsumer<'a, P>
    where
        P: IPairingArithmetic + IPairingReference,
    {
        type Output = ();

        fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
            &self,
            _params: ConsumeEncodingParams,
            payload: ConsumeEncodingPayload<E>,
        ) -> Self::Output {
            let created = create_chain_forms(
                &build_create_chain_forms_deps(
                    ChainFormsConsumer {
                        pairing: self.pairing,
                        hashing: self.hashing,
                        encoder: &payload.adapter,
                        random: self.random,
                        settings: self.settings,
                        asserter: self.asserter,
                    },
                    Default::default(),
                ),
                build_create_chain_forms_params(Default::default()),
                build_create_chain_forms_payload(Default::default()),
            );
            let Ok(_success) = created else {
                panic!("the chain-forms factory admits the selection")
            };
        }
    }

    struct PairingConsumer<'a> {
        settings: &'a EvmVerifierSettings,
        asserter: &'a Asserter,
    }

    impl<'a> IPairingConsumer for PairingConsumer<'a> {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            let hashing = create_hash_to_scalar::<P::Scalar>(
                &build_create_hash_to_scalar_deps(Default::default()),
                build_create_hash_to_scalar_params(Default::default()),
                build_create_hash_to_scalar_payload(Default::default()),
            );
            let Ok(hashing) = hashing else {
                panic!("the hash-to-scalar factory admits the selection")
            };
            let Ok(source) = create_random_source(
                &CreateRandomSourceDeps,
                build_create_random_source_params(Default::default()),
                CreateRandomSourcePayload,
            );
            let created = create_encoding(
                &build_create_encoding_deps(EncodingConsumer {
                    pairing: &payload.adapter,
                    hashing: &*hashing.adapter,
                    random: &*source.adapter,
                    settings: self.settings,
                    asserter: self.asserter,
                }),
                build_create_encoding_params(Default::default()),
                build_create_encoding_payload(Default::default()),
            );
            let Ok(_success) = created else {
                panic!("the encoding factory admits the selection")
            };
        }
    }

    let consumer = PairingConsumer {
        settings: &settings,
        asserter: &asserter,
    };
    let created = create_pairing(
        &build_create_pairing_deps(consumer, Default::default()),
        build_create_pairing_params(Default::default()),
        build_create_pairing_payload(Default::default()),
    );
    let Ok(_success) = created else {
        panic!("the pairing factory ran the consumer")
    };
    // evm_verifier_try_new_refuses_a_verifier_address_of_another_kind
}

/// Contract: the same refusal for the oracle → `Err(EvmVerifierTryNew
///   ErrorReturn::Identity { field: EvmVerifierField::GasPriceOracle,
///   error })`.
/// Arrange: `gas_price_oracle` `CanonicalFieldValue::Unsigned16(1)` and
///   the verifier's default.
/// Act:     `EvmVerifier::try_new_mocked`.
/// Assert:  `matches!` over the whole error with `field`
///   `GasPriceOracle`.
/// Collaborators: the pairing's, hash-to-scalar's, encoding's,
///   chain-forms', delivery-proof's, and random-source's mock concretes
///   in their default modes through their factories.
#[test]
fn evm_verifier_try_new_refuses_an_oracle_address_of_another_kind() {
    // Arrange
    let settings = build_evm_verifier_settings(EvmVerifierSettingsOverrides {
        gas_price_oracle: Some(CanonicalFieldValue::Unsigned16(1)),
        ..Default::default()
    });
    let asserter = Asserter::default();

    struct DeliveryProofConsumer<'a, P: IPairingArithmetic, F: IChainForms> {
        pairing: &'a P,
        settings: &'a EvmVerifierSettings,
        asserter: &'a Asserter,
        forms: PhantomData<F>,
    }

    impl<'a, P, F> IDeliveryProofConsumer<P, F> for DeliveryProofConsumer<'a, P, F>
    where
        P: IPairingArithmetic + IPairingReference,
        F: IChainForms,
    {
        type Output = ();

        fn consume_delivery_proof<D: IDeliveryProofAdapter<Pairing = P, Forms = F>>(
            &self,
            _params: ConsumeDeliveryProofParams,
            payload: ConsumeDeliveryProofPayload<D>,
        ) -> Self::Output {
            // Act
            let result = EvmVerifier::try_new_mocked(
                build_evm_verifier_constructor_params(
                    self.pairing,
                    &payload.adapter,
                    EvmVerifierConstructorParamsOverrides {
                        settings: Some(self.settings.clone()),
                    },
                ),
                self.asserter.clone(),
            );

            // Assert
            assert!(matches!(
                result,
                Err(EvmVerifierTryNewErrorReturn::Identity {
                    field: EvmVerifierField::GasPriceOracle,
                    error: FromFieldErrorReturn::WrongKind {
                        expected: CanonicalFieldKind::FixedBytes20
                    }
                })
            ));
        }
    }

    struct ChainFormsConsumer<'a, P: IPairingArithmetic, E> {
        pairing: &'a P,
        hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
        encoder: &'a E,
        random: &'a dyn IRandomSourceAdapter,
        settings: &'a EvmVerifierSettings,
        asserter: &'a Asserter,
    }

    impl<'a, P, E> IChainFormsConsumer for ChainFormsConsumer<'a, P, E>
    where
        P: IPairingArithmetic + IPairingReference,
        E: IEncoderAdapter,
    {
        type Output = ();

        fn consume_chain_forms<F: IChainForms>(
            &self,
            _params: ConsumeChainFormsParams,
            _payload: ConsumeChainFormsPayload<F>,
        ) -> Self::Output {
            let created = create_delivery_proof(
                &build_create_delivery_proof_deps(
                    self.pairing,
                    self.hashing,
                    self.encoder,
                    self.random,
                    DeliveryProofConsumer {
                        pairing: self.pairing,
                        settings: self.settings,
                        asserter: self.asserter,
                        forms: PhantomData::<F>,
                    },
                    Default::default(),
                ),
                build_create_delivery_proof_params(Default::default()),
                build_create_delivery_proof_payload(Default::default()),
            );
            let Ok(_success) = created else {
                panic!("the delivery-proof factory admits the selection")
            };
        }
    }

    struct EncodingConsumer<'a, P: IPairingArithmetic> {
        pairing: &'a P,
        hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
        random: &'a dyn IRandomSourceAdapter,
        settings: &'a EvmVerifierSettings,
        asserter: &'a Asserter,
    }

    impl<'a, P> IEncodingConsumer for EncodingConsumer<'a, P>
    where
        P: IPairingArithmetic + IPairingReference,
    {
        type Output = ();

        fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
            &self,
            _params: ConsumeEncodingParams,
            payload: ConsumeEncodingPayload<E>,
        ) -> Self::Output {
            let created = create_chain_forms(
                &build_create_chain_forms_deps(
                    ChainFormsConsumer {
                        pairing: self.pairing,
                        hashing: self.hashing,
                        encoder: &payload.adapter,
                        random: self.random,
                        settings: self.settings,
                        asserter: self.asserter,
                    },
                    Default::default(),
                ),
                build_create_chain_forms_params(Default::default()),
                build_create_chain_forms_payload(Default::default()),
            );
            let Ok(_success) = created else {
                panic!("the chain-forms factory admits the selection")
            };
        }
    }

    struct PairingConsumer<'a> {
        settings: &'a EvmVerifierSettings,
        asserter: &'a Asserter,
    }

    impl<'a> IPairingConsumer for PairingConsumer<'a> {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            let hashing = create_hash_to_scalar::<P::Scalar>(
                &build_create_hash_to_scalar_deps(Default::default()),
                build_create_hash_to_scalar_params(Default::default()),
                build_create_hash_to_scalar_payload(Default::default()),
            );
            let Ok(hashing) = hashing else {
                panic!("the hash-to-scalar factory admits the selection")
            };
            let Ok(source) = create_random_source(
                &CreateRandomSourceDeps,
                build_create_random_source_params(Default::default()),
                CreateRandomSourcePayload,
            );
            let created = create_encoding(
                &build_create_encoding_deps(EncodingConsumer {
                    pairing: &payload.adapter,
                    hashing: &*hashing.adapter,
                    random: &*source.adapter,
                    settings: self.settings,
                    asserter: self.asserter,
                }),
                build_create_encoding_params(Default::default()),
                build_create_encoding_payload(Default::default()),
            );
            let Ok(_success) = created else {
                panic!("the encoding factory admits the selection")
            };
        }
    }

    let consumer = PairingConsumer {
        settings: &settings,
        asserter: &asserter,
    };
    let created = create_pairing(
        &build_create_pairing_deps(consumer, Default::default()),
        build_create_pairing_params(Default::default()),
        build_create_pairing_payload(Default::default()),
    );
    let Ok(_success) = created else {
        panic!("the pairing factory ran the consumer")
    };
    // evm_verifier_try_new_refuses_an_oracle_address_of_another_kind
}

/// Contract: the verifier's address is admitted before the oracle's →
///   the verifier's refusal.
/// Arrange: both addresses `CanonicalFieldValue::Unsigned16(1)`.
/// Act:     `EvmVerifier::try_new_mocked`.
/// Assert:  `matches!` over the whole error with `field`
///   `VerifierAddress`.
/// Collaborators: the pairing's, hash-to-scalar's, encoding's,
///   chain-forms', delivery-proof's, and random-source's mock concretes
///   in their default modes through their factories.
#[test]
fn evm_verifier_try_new_checks_the_verifier_address_before_the_oracle_address() {
    // Arrange
    let settings = build_evm_verifier_settings(EvmVerifierSettingsOverrides {
        verifier_address: Some(CanonicalFieldValue::Unsigned16(1)),
        gas_price_oracle: Some(CanonicalFieldValue::Unsigned16(1)),
        ..Default::default()
    });
    let asserter = Asserter::default();

    struct DeliveryProofConsumer<'a, P: IPairingArithmetic, F: IChainForms> {
        pairing: &'a P,
        settings: &'a EvmVerifierSettings,
        asserter: &'a Asserter,
        forms: PhantomData<F>,
    }

    impl<'a, P, F> IDeliveryProofConsumer<P, F> for DeliveryProofConsumer<'a, P, F>
    where
        P: IPairingArithmetic + IPairingReference,
        F: IChainForms,
    {
        type Output = ();

        fn consume_delivery_proof<D: IDeliveryProofAdapter<Pairing = P, Forms = F>>(
            &self,
            _params: ConsumeDeliveryProofParams,
            payload: ConsumeDeliveryProofPayload<D>,
        ) -> Self::Output {
            // Act
            let result = EvmVerifier::try_new_mocked(
                build_evm_verifier_constructor_params(
                    self.pairing,
                    &payload.adapter,
                    EvmVerifierConstructorParamsOverrides {
                        settings: Some(self.settings.clone()),
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

    struct ChainFormsConsumer<'a, P: IPairingArithmetic, E> {
        pairing: &'a P,
        hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
        encoder: &'a E,
        random: &'a dyn IRandomSourceAdapter,
        settings: &'a EvmVerifierSettings,
        asserter: &'a Asserter,
    }

    impl<'a, P, E> IChainFormsConsumer for ChainFormsConsumer<'a, P, E>
    where
        P: IPairingArithmetic + IPairingReference,
        E: IEncoderAdapter,
    {
        type Output = ();

        fn consume_chain_forms<F: IChainForms>(
            &self,
            _params: ConsumeChainFormsParams,
            _payload: ConsumeChainFormsPayload<F>,
        ) -> Self::Output {
            let created = create_delivery_proof(
                &build_create_delivery_proof_deps(
                    self.pairing,
                    self.hashing,
                    self.encoder,
                    self.random,
                    DeliveryProofConsumer {
                        pairing: self.pairing,
                        settings: self.settings,
                        asserter: self.asserter,
                        forms: PhantomData::<F>,
                    },
                    Default::default(),
                ),
                build_create_delivery_proof_params(Default::default()),
                build_create_delivery_proof_payload(Default::default()),
            );
            let Ok(_success) = created else {
                panic!("the delivery-proof factory admits the selection")
            };
        }
    }

    struct EncodingConsumer<'a, P: IPairingArithmetic> {
        pairing: &'a P,
        hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
        random: &'a dyn IRandomSourceAdapter,
        settings: &'a EvmVerifierSettings,
        asserter: &'a Asserter,
    }

    impl<'a, P> IEncodingConsumer for EncodingConsumer<'a, P>
    where
        P: IPairingArithmetic + IPairingReference,
    {
        type Output = ();

        fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
            &self,
            _params: ConsumeEncodingParams,
            payload: ConsumeEncodingPayload<E>,
        ) -> Self::Output {
            let created = create_chain_forms(
                &build_create_chain_forms_deps(
                    ChainFormsConsumer {
                        pairing: self.pairing,
                        hashing: self.hashing,
                        encoder: &payload.adapter,
                        random: self.random,
                        settings: self.settings,
                        asserter: self.asserter,
                    },
                    Default::default(),
                ),
                build_create_chain_forms_params(Default::default()),
                build_create_chain_forms_payload(Default::default()),
            );
            let Ok(_success) = created else {
                panic!("the chain-forms factory admits the selection")
            };
        }
    }

    struct PairingConsumer<'a> {
        settings: &'a EvmVerifierSettings,
        asserter: &'a Asserter,
    }

    impl<'a> IPairingConsumer for PairingConsumer<'a> {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            let hashing = create_hash_to_scalar::<P::Scalar>(
                &build_create_hash_to_scalar_deps(Default::default()),
                build_create_hash_to_scalar_params(Default::default()),
                build_create_hash_to_scalar_payload(Default::default()),
            );
            let Ok(hashing) = hashing else {
                panic!("the hash-to-scalar factory admits the selection")
            };
            let Ok(source) = create_random_source(
                &CreateRandomSourceDeps,
                build_create_random_source_params(Default::default()),
                CreateRandomSourcePayload,
            );
            let created = create_encoding(
                &build_create_encoding_deps(EncodingConsumer {
                    pairing: &payload.adapter,
                    hashing: &*hashing.adapter,
                    random: &*source.adapter,
                    settings: self.settings,
                    asserter: self.asserter,
                }),
                build_create_encoding_params(Default::default()),
                build_create_encoding_payload(Default::default()),
            );
            let Ok(_success) = created else {
                panic!("the encoding factory admits the selection")
            };
        }
    }

    let consumer = PairingConsumer {
        settings: &settings,
        asserter: &asserter,
    };
    let created = create_pairing(
        &build_create_pairing_deps(consumer, Default::default()),
        build_create_pairing_params(Default::default()),
        build_create_pairing_payload(Default::default()),
    );
    let Ok(_success) = created else {
        panic!("the pairing factory ran the consumer")
    };
    // evm_verifier_try_new_checks_the_verifier_address_before_the_oracle_address
}

/// Contract: `try_new` over an admitted endpoint and two admitted
///   addresses → `Ok` holding the addresses, the HTTP provider and the
///   runtime built and no request made.
/// Arrange: `build_evm_verifier_constructor_params` over the received
///   pairing and delivery proof adapter with `settings` by
///   `build_evm_verifier_settings` carrying `verifier_address`
///   `FixedBytes20([0x42; 20])` and `gas_price_oracle`
///   `FixedBytes20([0x43; 20])` and the default endpoint, a loopback
///   address nothing listens at, so a request made would fail the block.
/// Act:     `EvmVerifier::try_new`.
/// Assert:  the instance's `verifier` equals `Address::from([0x42; 20])`
///   and its `oracle` equals `Address::from([0x43; 20])` by `assert_eq!`.
/// Collaborators: the pairing's, hash-to-scalar's, encoding's,
///   chain-forms', delivery-proof's, and random-source's mock concretes
///   in their default modes through their factories.
#[test]
fn evm_verifier_try_new_builds_the_http_provider_and_runtime_without_reading_the_chain() {
    // Arrange
    let settings = build_evm_verifier_settings(EvmVerifierSettingsOverrides {
        verifier_address: Some(CanonicalFieldValue::FixedBytes20([0x42; 20])),
        gas_price_oracle: Some(CanonicalFieldValue::FixedBytes20([0x43; 20])),
        ..Default::default()
    });

    struct DeliveryProofConsumer<'a, P: IPairingArithmetic, F: IChainForms> {
        pairing: &'a P,
        settings: &'a EvmVerifierSettings,
        forms: PhantomData<F>,
    }

    impl<'a, P, F> IDeliveryProofConsumer<P, F> for DeliveryProofConsumer<'a, P, F>
    where
        P: IPairingArithmetic + IPairingReference,
        F: IChainForms,
    {
        type Output = ();

        fn consume_delivery_proof<D: IDeliveryProofAdapter<Pairing = P, Forms = F>>(
            &self,
            _params: ConsumeDeliveryProofParams,
            payload: ConsumeDeliveryProofPayload<D>,
        ) -> Self::Output {
            // Act
            let result = EvmVerifier::try_new(build_evm_verifier_constructor_params(
                self.pairing,
                &payload.adapter,
                EvmVerifierConstructorParamsOverrides {
                    settings: Some(self.settings.clone()),
                },
            ));

            // Assert
            let Ok(instance) = result else {
                panic!("admitted addresses build the provider and runtime")
            };
            assert_eq!(instance.verifier, Address::from([0x42; 20]));
            assert_eq!(instance.oracle, Address::from([0x43; 20]));
        }
    }

    struct ChainFormsConsumer<'a, P: IPairingArithmetic, E> {
        pairing: &'a P,
        hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
        encoder: &'a E,
        random: &'a dyn IRandomSourceAdapter,
        settings: &'a EvmVerifierSettings,
    }

    impl<'a, P, E> IChainFormsConsumer for ChainFormsConsumer<'a, P, E>
    where
        P: IPairingArithmetic + IPairingReference,
        E: IEncoderAdapter,
    {
        type Output = ();

        fn consume_chain_forms<F: IChainForms>(
            &self,
            _params: ConsumeChainFormsParams,
            _payload: ConsumeChainFormsPayload<F>,
        ) -> Self::Output {
            let created = create_delivery_proof(
                &build_create_delivery_proof_deps(
                    self.pairing,
                    self.hashing,
                    self.encoder,
                    self.random,
                    DeliveryProofConsumer {
                        pairing: self.pairing,
                        settings: self.settings,
                        forms: PhantomData::<F>,
                    },
                    Default::default(),
                ),
                build_create_delivery_proof_params(Default::default()),
                build_create_delivery_proof_payload(Default::default()),
            );
            let Ok(_success) = created else {
                panic!("the delivery-proof factory admits the selection")
            };
        }
    }

    struct EncodingConsumer<'a, P: IPairingArithmetic> {
        pairing: &'a P,
        hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
        random: &'a dyn IRandomSourceAdapter,
        settings: &'a EvmVerifierSettings,
    }

    impl<'a, P> IEncodingConsumer for EncodingConsumer<'a, P>
    where
        P: IPairingArithmetic + IPairingReference,
    {
        type Output = ();

        fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
            &self,
            _params: ConsumeEncodingParams,
            payload: ConsumeEncodingPayload<E>,
        ) -> Self::Output {
            let created = create_chain_forms(
                &build_create_chain_forms_deps(
                    ChainFormsConsumer {
                        pairing: self.pairing,
                        hashing: self.hashing,
                        encoder: &payload.adapter,
                        random: self.random,
                        settings: self.settings,
                    },
                    Default::default(),
                ),
                build_create_chain_forms_params(Default::default()),
                build_create_chain_forms_payload(Default::default()),
            );
            let Ok(_success) = created else {
                panic!("the chain-forms factory admits the selection")
            };
        }
    }

    struct PairingConsumer<'a> {
        settings: &'a EvmVerifierSettings,
    }

    impl<'a> IPairingConsumer for PairingConsumer<'a> {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            let hashing = create_hash_to_scalar::<P::Scalar>(
                &build_create_hash_to_scalar_deps(Default::default()),
                build_create_hash_to_scalar_params(Default::default()),
                build_create_hash_to_scalar_payload(Default::default()),
            );
            let Ok(hashing) = hashing else {
                panic!("the hash-to-scalar factory admits the selection")
            };
            let Ok(source) = create_random_source(
                &CreateRandomSourceDeps,
                build_create_random_source_params(Default::default()),
                CreateRandomSourcePayload,
            );
            let created = create_encoding(
                &build_create_encoding_deps(EncodingConsumer {
                    pairing: &payload.adapter,
                    hashing: &*hashing.adapter,
                    random: &*source.adapter,
                    settings: self.settings,
                }),
                build_create_encoding_params(Default::default()),
                build_create_encoding_payload(Default::default()),
            );
            let Ok(_success) = created else {
                panic!("the encoding factory admits the selection")
            };
        }
    }

    let consumer = PairingConsumer {
        settings: &settings,
    };
    let created = create_pairing(
        &build_create_pairing_deps(consumer, Default::default()),
        build_create_pairing_params(Default::default()),
        build_create_pairing_payload(Default::default()),
    );
    let Ok(_success) = created else {
        panic!("the pairing factory ran the consumer")
    };
    // evm_verifier_try_new_builds_the_http_provider_and_runtime_without_reading_the_chain
}

/// Contract: the production constructor decides the addresses exactly as
///   the scripted one does → `Err(EvmVerifierTryNewErrorReturn::Identity
///   { field, error })`.
/// Arrange: `gas_price_oracle` `CanonicalFieldValue::Unsigned16(1)` and
///   the verifier's address its default.
/// Act:     `EvmVerifier::try_new`.
/// Assert:  `matches!(result, Err(EvmVerifierTryNewErrorReturn::Identity
///   { field: EvmVerifierField::GasPriceOracle, error:
///   FromFieldErrorReturn::WrongKind { expected: CanonicalFieldKind::
///   FixedBytes20 } }))`.
/// Collaborators: the pairing's, hash-to-scalar's, encoding's,
///   chain-forms', delivery-proof's, and random-source's mock concretes
///   in their default modes through their factories.
#[test]
fn evm_verifier_try_new_refuses_an_address_of_another_kind_as_try_new_mocked_does() {
    // Arrange
    let settings = build_evm_verifier_settings(EvmVerifierSettingsOverrides {
        gas_price_oracle: Some(CanonicalFieldValue::Unsigned16(1)),
        ..Default::default()
    });

    struct DeliveryProofConsumer<'a, P: IPairingArithmetic, F: IChainForms> {
        pairing: &'a P,
        settings: &'a EvmVerifierSettings,
        forms: PhantomData<F>,
    }

    impl<'a, P, F> IDeliveryProofConsumer<P, F> for DeliveryProofConsumer<'a, P, F>
    where
        P: IPairingArithmetic + IPairingReference,
        F: IChainForms,
    {
        type Output = ();

        fn consume_delivery_proof<D: IDeliveryProofAdapter<Pairing = P, Forms = F>>(
            &self,
            _params: ConsumeDeliveryProofParams,
            payload: ConsumeDeliveryProofPayload<D>,
        ) -> Self::Output {
            // Act
            let result = EvmVerifier::try_new(build_evm_verifier_constructor_params(
                self.pairing,
                &payload.adapter,
                EvmVerifierConstructorParamsOverrides {
                    settings: Some(self.settings.clone()),
                },
            ));

            // Assert
            assert!(matches!(
                result,
                Err(EvmVerifierTryNewErrorReturn::Identity {
                    field: EvmVerifierField::GasPriceOracle,
                    error: FromFieldErrorReturn::WrongKind {
                        expected: CanonicalFieldKind::FixedBytes20
                    }
                })
            ));
        }
    }

    struct ChainFormsConsumer<'a, P: IPairingArithmetic, E> {
        pairing: &'a P,
        hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
        encoder: &'a E,
        random: &'a dyn IRandomSourceAdapter,
        settings: &'a EvmVerifierSettings,
    }

    impl<'a, P, E> IChainFormsConsumer for ChainFormsConsumer<'a, P, E>
    where
        P: IPairingArithmetic + IPairingReference,
        E: IEncoderAdapter,
    {
        type Output = ();

        fn consume_chain_forms<F: IChainForms>(
            &self,
            _params: ConsumeChainFormsParams,
            _payload: ConsumeChainFormsPayload<F>,
        ) -> Self::Output {
            let created = create_delivery_proof(
                &build_create_delivery_proof_deps(
                    self.pairing,
                    self.hashing,
                    self.encoder,
                    self.random,
                    DeliveryProofConsumer {
                        pairing: self.pairing,
                        settings: self.settings,
                        forms: PhantomData::<F>,
                    },
                    Default::default(),
                ),
                build_create_delivery_proof_params(Default::default()),
                build_create_delivery_proof_payload(Default::default()),
            );
            let Ok(_success) = created else {
                panic!("the delivery-proof factory admits the selection")
            };
        }
    }

    struct EncodingConsumer<'a, P: IPairingArithmetic> {
        pairing: &'a P,
        hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
        random: &'a dyn IRandomSourceAdapter,
        settings: &'a EvmVerifierSettings,
    }

    impl<'a, P> IEncodingConsumer for EncodingConsumer<'a, P>
    where
        P: IPairingArithmetic + IPairingReference,
    {
        type Output = ();

        fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
            &self,
            _params: ConsumeEncodingParams,
            payload: ConsumeEncodingPayload<E>,
        ) -> Self::Output {
            let created = create_chain_forms(
                &build_create_chain_forms_deps(
                    ChainFormsConsumer {
                        pairing: self.pairing,
                        hashing: self.hashing,
                        encoder: &payload.adapter,
                        random: self.random,
                        settings: self.settings,
                    },
                    Default::default(),
                ),
                build_create_chain_forms_params(Default::default()),
                build_create_chain_forms_payload(Default::default()),
            );
            let Ok(_success) = created else {
                panic!("the chain-forms factory admits the selection")
            };
        }
    }

    struct PairingConsumer<'a> {
        settings: &'a EvmVerifierSettings,
    }

    impl<'a> IPairingConsumer for PairingConsumer<'a> {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            let hashing = create_hash_to_scalar::<P::Scalar>(
                &build_create_hash_to_scalar_deps(Default::default()),
                build_create_hash_to_scalar_params(Default::default()),
                build_create_hash_to_scalar_payload(Default::default()),
            );
            let Ok(hashing) = hashing else {
                panic!("the hash-to-scalar factory admits the selection")
            };
            let Ok(source) = create_random_source(
                &CreateRandomSourceDeps,
                build_create_random_source_params(Default::default()),
                CreateRandomSourcePayload,
            );
            let created = create_encoding(
                &build_create_encoding_deps(EncodingConsumer {
                    pairing: &payload.adapter,
                    hashing: &*hashing.adapter,
                    random: &*source.adapter,
                    settings: self.settings,
                }),
                build_create_encoding_params(Default::default()),
                build_create_encoding_payload(Default::default()),
            );
            let Ok(_success) = created else {
                panic!("the encoding factory admits the selection")
            };
        }
    }

    let consumer = PairingConsumer {
        settings: &settings,
    };
    let created = create_pairing(
        &build_create_pairing_deps(consumer, Default::default()),
        build_create_pairing_params(Default::default()),
        build_create_pairing_payload(Default::default()),
    );
    let Ok(_success) = created else {
        panic!("the pairing factory ran the consumer")
    };
    // evm_verifier_try_new_refuses_an_address_of_another_kind_as_try_new_mocked_does
}
