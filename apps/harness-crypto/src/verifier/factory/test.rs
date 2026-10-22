#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::marker::PhantomData;

use chain::{
    ConsumeChainFormsParams, ConsumeChainFormsPayload, IChainForms, IChainFormsConsumer,
    build_create_chain_forms_deps, build_create_chain_forms_params,
    build_create_chain_forms_payload, create_chain_forms,
};
use encoding::{
    ConsumeEncodingParams, ConsumeEncodingPayload, IDecoderAdapter, IEncoderAdapter,
    IEncodingConsumer, build_create_encoding_deps, build_create_encoding_params,
    build_create_encoding_payload, create_encoding,
};
use hash_to_scalar::{
    IHashToScalarAdapter, build_create_hash_to_scalar_deps, build_create_hash_to_scalar_params,
    build_create_hash_to_scalar_payload, create_hash_to_scalar,
};
use pairing::{
    ConsumePairingParams, ConsumePairingPayload, IPairingArithmetic, IPairingConsumer,
    IPairingReference, build_create_pairing_deps, build_create_pairing_params,
    build_create_pairing_payload, create_pairing,
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

use super::create_verifier;
use super::interface::{
    CreateVerifierErrorReturn, CreateVerifierParams, CreateVerifierReturn,
    CreateVerifierSuccessReturn,
};
use super::mock::{
    CreateVerifierParamsOverrides, MockIVerifierConsumer, build_create_verifier_deps,
    build_create_verifier_params, build_create_verifier_payload,
};

/// Contract: admitted, `Mock(failure_mode)` arm: the mock concrete's
///   declaration serves the pairing's form and lists the params' version →
///   the consumer is called once with the constructed mock concrete and
///   `Ok(CreateVerifierSuccessReturn { output })` holds its output.
/// Arrange: `build_create_verifier_params(Default::default())`, the mock
///   selection under `NoFailure` at `DELIVERY_STATEMENT_VERSION_ONE`.
/// Act:     `create_verifier`, called in the innermost consumer's method.
/// Assert:  `matches!(result, Ok(CreateVerifierSuccessReturn { output: ()
///   }))`.
/// Collaborators: the pairing's, hash-to-scalar's, encoding's,
///   chain-forms', delivery-proof's, and random-source's mock concretes in
///   their default modes through their factories; `MockIVerifierConsumer`
///   in the deps.
#[test]
fn create_verifier_hands_the_consumer_the_concrete_the_selection_names() {
    // Arrange
    let params = build_create_verifier_params(Default::default());

    struct DeliveryProofConsumer<'a, P: IPairingArithmetic, F> {
        pairing: &'a P,
        params: &'a CreateVerifierParams,
        forms: PhantomData<F>,
    }

    impl<'a, P, F> IDeliveryProofConsumer<P, F> for DeliveryProofConsumer<'a, P, F>
    where
        P: IPairingArithmetic + IPairingReference,
        F: IChainForms,
    {
        type Output = CreateVerifierReturn<()>;

        fn consume_delivery_proof<D: IDeliveryProofAdapter<Pairing = P, Forms = F>>(
            &self,
            _params: ConsumeDeliveryProofParams,
            payload: ConsumeDeliveryProofPayload<D>,
        ) -> Self::Output {
            // Act
            create_verifier(
                &build_create_verifier_deps(
                    self.pairing,
                    &payload.adapter,
                    MockIVerifierConsumer,
                    Default::default(),
                ),
                (*self.params).clone(),
                build_create_verifier_payload(Default::default()),
            )
        }
    }

    struct ChainFormsConsumer<'a, P: IPairingArithmetic, E> {
        pairing: &'a P,
        hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
        encoder: &'a E,
        random: &'a dyn IRandomSourceAdapter,
        params: &'a CreateVerifierParams,
    }

    impl<'a, P, E> IChainFormsConsumer for ChainFormsConsumer<'a, P, E>
    where
        P: IPairingArithmetic + IPairingReference,
        E: IEncoderAdapter,
    {
        type Output = CreateVerifierReturn<()>;

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
                        params: self.params,
                        forms: PhantomData::<F>,
                    },
                    Default::default(),
                ),
                build_create_delivery_proof_params(Default::default()),
                build_create_delivery_proof_payload(Default::default()),
            );
            let Ok(success) = created else {
                panic!("the delivery-proof factory admits the selection")
            };
            success.output
        }
    }

    struct EncodingConsumer<'a, P: IPairingArithmetic> {
        pairing: &'a P,
        hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
        random: &'a dyn IRandomSourceAdapter,
        params: &'a CreateVerifierParams,
    }

    impl<'a, P> IEncodingConsumer for EncodingConsumer<'a, P>
    where
        P: IPairingArithmetic + IPairingReference,
    {
        type Output = CreateVerifierReturn<()>;

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
                        params: self.params,
                    },
                    Default::default(),
                ),
                build_create_chain_forms_params(Default::default()),
                build_create_chain_forms_payload(Default::default()),
            );
            let Ok(success) = created else {
                panic!("the chain-forms factory admits the selection")
            };
            success.output
        }
    }

    struct PairingConsumer<'a> {
        params: &'a CreateVerifierParams,
    }

    impl IPairingConsumer for PairingConsumer<'_> {
        type Output = CreateVerifierReturn<()>;

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
                    params: self.params,
                }),
                build_create_encoding_params(Default::default()),
                build_create_encoding_payload(Default::default()),
            );
            let Ok(success) = created else {
                panic!("the encoding factory admits the selection")
            };
            success.output
        }
    }

    let consumer = PairingConsumer { params: &params };
    let created = create_pairing(
        &build_create_pairing_deps(consumer, Default::default()),
        build_create_pairing_params(Default::default()),
        build_create_pairing_payload(Default::default()),
    );
    let Ok(success) = created else {
        panic!("the pairing factory ran the consumer")
    };
    let result = success.output;

    // Assert
    assert!(matches!(
        result,
        Ok(CreateVerifierSuccessReturn { output: () })
    ));
    // create_verifier_hands_the_consumer_the_concrete_the_selection_names
}

/// Contract: statement version: the declaration's `statement_versions`
///   does not contain the params' version → `Err(CreateVerifierErrorReturn
///   ::UnsupportedStatementVersion { version })`, nothing constructed.
/// Arrange: `build_create_verifier_params` with `statement_version`
///   `DELIVERY_STATEMENT_VERSION_ONE + 1`, a version no concrete of the
///   family declares.
/// Act:     `create_verifier`, called in the innermost consumer's method.
/// Assert:  `matches!(result, Err(CreateVerifierErrorReturn::
///   UnsupportedStatementVersion { version: 2 }))`, the pattern naming
///   every field because `CreateVerifierErrorReturn` omits `PartialEq`.
/// Collaborators: as
///   `create_verifier_hands_the_consumer_the_concrete_the_selection_names`;
///   the refusal is decided from the selected concrete's declaration,
///   before any construction.
#[test]
fn create_verifier_refuses_a_statement_version_the_declaration_does_not_list() {
    // Arrange
    let params = build_create_verifier_params(CreateVerifierParamsOverrides {
        statement_version: Some(DELIVERY_STATEMENT_VERSION_ONE + 1),
        ..Default::default()
    });

    struct DeliveryProofConsumer<'a, P: IPairingArithmetic, F> {
        pairing: &'a P,
        params: &'a CreateVerifierParams,
        forms: PhantomData<F>,
    }

    impl<'a, P, F> IDeliveryProofConsumer<P, F> for DeliveryProofConsumer<'a, P, F>
    where
        P: IPairingArithmetic + IPairingReference,
        F: IChainForms,
    {
        type Output = CreateVerifierReturn<()>;

        fn consume_delivery_proof<D: IDeliveryProofAdapter<Pairing = P, Forms = F>>(
            &self,
            _params: ConsumeDeliveryProofParams,
            payload: ConsumeDeliveryProofPayload<D>,
        ) -> Self::Output {
            // Act
            create_verifier(
                &build_create_verifier_deps(
                    self.pairing,
                    &payload.adapter,
                    MockIVerifierConsumer,
                    Default::default(),
                ),
                (*self.params).clone(),
                build_create_verifier_payload(Default::default()),
            )
        }
    }

    struct ChainFormsConsumer<'a, P: IPairingArithmetic, E> {
        pairing: &'a P,
        hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
        encoder: &'a E,
        random: &'a dyn IRandomSourceAdapter,
        params: &'a CreateVerifierParams,
    }

    impl<'a, P, E> IChainFormsConsumer for ChainFormsConsumer<'a, P, E>
    where
        P: IPairingArithmetic + IPairingReference,
        E: IEncoderAdapter,
    {
        type Output = CreateVerifierReturn<()>;

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
                        params: self.params,
                        forms: PhantomData::<F>,
                    },
                    Default::default(),
                ),
                build_create_delivery_proof_params(Default::default()),
                build_create_delivery_proof_payload(Default::default()),
            );
            let Ok(success) = created else {
                panic!("the delivery-proof factory admits the selection")
            };
            success.output
        }
    }

    struct EncodingConsumer<'a, P: IPairingArithmetic> {
        pairing: &'a P,
        hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
        random: &'a dyn IRandomSourceAdapter,
        params: &'a CreateVerifierParams,
    }

    impl<'a, P> IEncodingConsumer for EncodingConsumer<'a, P>
    where
        P: IPairingArithmetic + IPairingReference,
    {
        type Output = CreateVerifierReturn<()>;

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
                        params: self.params,
                    },
                    Default::default(),
                ),
                build_create_chain_forms_params(Default::default()),
                build_create_chain_forms_payload(Default::default()),
            );
            let Ok(success) = created else {
                panic!("the chain-forms factory admits the selection")
            };
            success.output
        }
    }

    struct PairingConsumer<'a> {
        params: &'a CreateVerifierParams,
    }

    impl IPairingConsumer for PairingConsumer<'_> {
        type Output = CreateVerifierReturn<()>;

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
                    params: self.params,
                }),
                build_create_encoding_params(Default::default()),
                build_create_encoding_payload(Default::default()),
            );
            let Ok(success) = created else {
                panic!("the encoding factory admits the selection")
            };
            success.output
        }
    }

    let consumer = PairingConsumer { params: &params };
    let created = create_pairing(
        &build_create_pairing_deps(consumer, Default::default()),
        build_create_pairing_params(Default::default()),
        build_create_pairing_payload(Default::default()),
    );
    let Ok(success) = created else {
        panic!("the pairing factory ran the consumer")
    };
    let result = success.output;

    // Assert
    assert!(matches!(
        result,
        Err(CreateVerifierErrorReturn::UnsupportedStatementVersion { version: 2 })
    ));
    // create_verifier_refuses_a_statement_version_the_declaration_does_not_list
}
