#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::marker::PhantomData;
use std::path::PathBuf;

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
use envelope::{
    ConsumeKeyAgreementParams, ConsumeKeyAgreementPayload, IKeyAgreementAdapter,
    IKeyAgreementConsumer, build_create_key_agreement_deps, build_create_key_agreement_params,
    build_create_key_agreement_payload, create_key_agreement,
};
use hash_to_scalar::{
    IHashToScalarAdapter, build_create_hash_to_scalar_deps, build_create_hash_to_scalar_params,
    build_create_hash_to_scalar_payload, create_hash_to_scalar,
};
use kem::{
    ConsumeKemParams, ConsumeKemPayload, ICredentialKemAdapter, IKemConsumer,
    build_create_kem_deps, build_create_kem_params, build_create_kem_payload, create_kem,
};
use pairing::{
    ConsumePairingParams, ConsumePairingPayload, IPairingArithmetic, IPairingConsumer,
    IPairingReference, build_create_pairing_deps, build_create_pairing_params,
    build_create_pairing_payload, create_pairing,
};
use proof::{
    ConsumeDeliveryProofParams, ConsumeDeliveryProofPayload, IDeliveryProofAdapter,
    IDeliveryProofConsumer, build_create_delivery_proof_deps, build_create_delivery_proof_params,
    build_create_delivery_proof_payload, create_delivery_proof,
};
use random::{
    CreateRandomSourceDeps, CreateRandomSourcePayload, IRandomSourceAdapter,
    build_create_random_source_params, create_random_source,
};

use super::mock::build_generate_consumer;
use crate::config::provides::{HarnessStatements, build_harness_statements};
use crate::generate::provides::{
    CreateGenerateParams, CreateGenerateParamsOverrides, EmitErrorReturn, EmitReturn,
    EmitSuccessReturn, EmittedFileOverrides, GenerateConcrete, MockIGenerateAdapterEmitErrorReturn,
    MockIGenerateAdapterFailureMode, VectorKind, build_create_generate_deps,
    build_create_generate_params, build_create_generate_payload, build_emitted_file,
    create_generate,
};

/// Contract: emitted: `emit` returns `Ok(success)` → the output is
///   `Ok(success)`, unchanged.
/// Arrange: the statements `build_harness_statements(Default::default())` and
///   the params `build_create_generate_params(Default::default())`, the
///   family's mock selection under `NoFailure` with its declared kinds, whose
///   `emit` returns a non-empty list in declared order.
/// Act:     `create_generate` with the subject as the deps' consumer, called
///   in the innermost consumer's method.
/// Assert:  the output unpacks as `Ok(EmitSuccessReturn { files })` and
///   `files` equals, in order, an `EmittedFile` per declared kind, each `path`
///   its kind's name, by `assert_eq!`, so an output emptied, reordered, or
///   not taken from `emit` fails.
/// Collaborators: the pairing's, hash-to-scalar's, encoding's, chain-forms',
///   kem's, key-agreement's, delivery-proof's, and random-source's mock
///   concretes in their default modes through their factories; the subject
///   `GenerateConsumer` in the deps.
#[test]
fn consume_generate_returns_the_files_emit_returns() {
    // Arrange
    let statements = build_harness_statements(Default::default());
    let params = build_create_generate_params(Default::default());

    struct DeliveryProofConsumer<'a, P: IPairingArithmetic, E, K, A, F> {
        pairing: &'a P,
        hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
        encoder: &'a E,
        random: &'a dyn IRandomSourceAdapter,
        kem: &'a K,
        key_agreement: &'a A,
        statements: &'a HarnessStatements,
        params: &'a CreateGenerateParams,
        forms: PhantomData<F>,
    }

    impl<'a, P, E, K, A, F> IDeliveryProofConsumer<P, F> for DeliveryProofConsumer<'a, P, E, K, A, F>
    where
        P: IPairingArithmetic + IPairingReference,
        E: IEncoderAdapter,
        K: ICredentialKemAdapter<Pairing = P>,
        A: IKeyAgreementAdapter<Pairing = P>,
        F: IChainForms,
    {
        type Output = EmitReturn;

        fn consume_delivery_proof<D: IDeliveryProofAdapter<Pairing = P, Forms = F>>(
            &self,
            _params: ConsumeDeliveryProofParams,
            payload: ConsumeDeliveryProofPayload<D>,
        ) -> Self::Output {
            // Act
            let created = create_generate(
                &build_create_generate_deps(
                    self.pairing,
                    self.hashing,
                    self.encoder,
                    self.random,
                    self.kem,
                    self.key_agreement,
                    &payload.adapter,
                    build_generate_consumer(self.statements, Default::default()),
                    Default::default(),
                ),
                (*self.params).clone(),
                build_create_generate_payload(Default::default()),
            );
            let Ok(success) = created else {
                panic!("the generate factory admits the selection")
            };
            success.output
        }
    }

    struct KeyAgreementConsumer<'a, P: IPairingArithmetic, E, K, F> {
        pairing: &'a P,
        hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
        encoder: &'a E,
        random: &'a dyn IRandomSourceAdapter,
        kem: &'a K,
        statements: &'a HarnessStatements,
        params: &'a CreateGenerateParams,
        forms: PhantomData<F>,
    }

    impl<'a, P, E, K, F> IKeyAgreementConsumer<P> for KeyAgreementConsumer<'a, P, E, K, F>
    where
        P: IPairingArithmetic + IPairingReference,
        E: IEncoderAdapter,
        K: ICredentialKemAdapter<Pairing = P>,
        F: IChainForms,
    {
        type Output = EmitReturn;

        fn consume_key_agreement<A: IKeyAgreementAdapter<Pairing = P>>(
            &self,
            _params: ConsumeKeyAgreementParams,
            payload: ConsumeKeyAgreementPayload<A>,
        ) -> Self::Output {
            let created = create_delivery_proof(
                &build_create_delivery_proof_deps(
                    self.pairing,
                    self.hashing,
                    self.encoder,
                    self.random,
                    DeliveryProofConsumer {
                        pairing: self.pairing,
                        hashing: self.hashing,
                        encoder: self.encoder,
                        random: self.random,
                        kem: self.kem,
                        key_agreement: &payload.adapter,
                        statements: self.statements,
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

    struct KemConsumer<'a, P: IPairingArithmetic, E, F> {
        pairing: &'a P,
        hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
        encoder: &'a E,
        random: &'a dyn IRandomSourceAdapter,
        statements: &'a HarnessStatements,
        params: &'a CreateGenerateParams,
        forms: PhantomData<F>,
    }

    impl<'a, P, E, F> IKemConsumer<P> for KemConsumer<'a, P, E, F>
    where
        P: IPairingArithmetic + IPairingReference,
        E: IEncoderAdapter,
        F: IChainForms,
    {
        type Output = EmitReturn;

        fn consume_kem<K: ICredentialKemAdapter<Pairing = P>>(
            &self,
            _params: ConsumeKemParams,
            payload: ConsumeKemPayload<K>,
        ) -> Self::Output {
            let created = create_key_agreement(
                &build_create_key_agreement_deps(
                    self.pairing,
                    self.hashing,
                    self.encoder,
                    self.random,
                    KeyAgreementConsumer {
                        pairing: self.pairing,
                        hashing: self.hashing,
                        encoder: self.encoder,
                        random: self.random,
                        kem: &payload.adapter,
                        statements: self.statements,
                        params: self.params,
                        forms: PhantomData::<F>,
                    },
                    Default::default(),
                ),
                build_create_key_agreement_params(Default::default()),
                build_create_key_agreement_payload(Default::default()),
            );
            let Ok(success) = created else {
                panic!("the key-agreement factory admits the selection")
            };
            success.output
        }
    }

    struct ChainFormsConsumer<'a, P: IPairingArithmetic, E> {
        pairing: &'a P,
        hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
        encoder: &'a E,
        random: &'a dyn IRandomSourceAdapter,
        statements: &'a HarnessStatements,
        params: &'a CreateGenerateParams,
    }

    impl<'a, P, E> IChainFormsConsumer for ChainFormsConsumer<'a, P, E>
    where
        P: IPairingArithmetic + IPairingReference,
        E: IEncoderAdapter,
    {
        type Output = EmitReturn;

        fn consume_chain_forms<F: IChainForms>(
            &self,
            _params: ConsumeChainFormsParams,
            _payload: ConsumeChainFormsPayload<F>,
        ) -> Self::Output {
            let created = create_kem(
                &build_create_kem_deps(
                    self.pairing,
                    self.hashing,
                    KemConsumer {
                        pairing: self.pairing,
                        hashing: self.hashing,
                        encoder: self.encoder,
                        random: self.random,
                        statements: self.statements,
                        params: self.params,
                        forms: PhantomData::<F>,
                    },
                    Default::default(),
                ),
                build_create_kem_params(Default::default()),
                build_create_kem_payload(Default::default()),
            );
            let Ok(success) = created else {
                panic!("the kem factory admits the selection")
            };
            success.output
        }
    }

    struct EncodingConsumer<'a, P: IPairingArithmetic> {
        pairing: &'a P,
        hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
        random: &'a dyn IRandomSourceAdapter,
        statements: &'a HarnessStatements,
        params: &'a CreateGenerateParams,
    }

    impl<'a, P> IEncodingConsumer for EncodingConsumer<'a, P>
    where
        P: IPairingArithmetic + IPairingReference,
    {
        type Output = EmitReturn;

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
                        statements: self.statements,
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
        statements: &'a HarnessStatements,
        params: &'a CreateGenerateParams,
    }

    impl IPairingConsumer for PairingConsumer<'_> {
        type Output = EmitReturn;

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
                    statements: self.statements,
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

    let consumer = PairingConsumer {
        statements: &statements,
        params: &params,
    };
    let created = create_pairing(
        &build_create_pairing_deps(consumer, Default::default()),
        build_create_pairing_params(Default::default()),
        build_create_pairing_payload(Default::default()),
    );
    let Ok(success) = created else {
        panic!("the pairing factory ran the consumer")
    };
    let output = success.output;

    // Assert
    let Ok(EmitSuccessReturn { files }) = output else {
        panic!("emit returns the files")
    };
    assert_eq!(
        files,
        vec![
            build_emitted_file(EmittedFileOverrides {
                kind: Some(Some(VectorKind::G1Addition)),
                path: Some(PathBuf::from("G1Addition")),
            }),
            build_emitted_file(EmittedFileOverrides {
                kind: Some(Some(VectorKind::G1MultiScalarMultiplication)),
                path: Some(PathBuf::from("G1MultiScalarMultiplication")),
            }),
            build_emitted_file(EmittedFileOverrides {
                kind: Some(Some(VectorKind::HashToScalar)),
                path: Some(PathBuf::from("HashToScalar")),
            }),
            build_emitted_file(EmittedFileOverrides {
                kind: Some(Some(VectorKind::DecodeRejection)),
                path: Some(PathBuf::from("DecodeRejection")),
            }),
            build_emitted_file(EmittedFileOverrides {
                kind: Some(Some(VectorKind::IdentityMapping)),
                path: Some(PathBuf::from("IdentityMapping")),
            }),
        ]
    );
    // consume_generate_returns_the_files_emit_returns
}

/// Contract: refused: `emit` returns `Err(error)` → the output is
///   `Err(error)`, unchanged.
/// Arrange: the statements `build_harness_statements(Default::default())` and
///   the params `build_create_generate_params` with `concrete`
///   `GenerateConcrete::Mock(MockIGenerateAdapterFailureMode::WriteRefused)`,
///   a mode other than the builder's default `NoFailure`, so a consumer that
///   replaces the refusal with a success, or alters it, fails.
/// Act:     `create_generate` with the subject as the deps' consumer, called
///   in the innermost consumer's method.
/// Assert:  the output unpacks as `Err(EmitErrorReturn::Mock(error))` and
///   `error` equals `MockIGenerateAdapterEmitErrorReturn::WriteRefused` by
///   `assert_eq!`.
/// Collaborators: as `consume_generate_returns_the_files_emit_returns`; the
///   generate family's mock concrete under `WriteRefused`.
#[test]
fn consume_generate_returns_the_refusal_emit_returns_unchanged() {
    // Arrange
    let statements = build_harness_statements(Default::default());
    let params = build_create_generate_params(CreateGenerateParamsOverrides {
        concrete: Some(GenerateConcrete::Mock(
            MockIGenerateAdapterFailureMode::WriteRefused,
        )),
        ..Default::default()
    });

    struct DeliveryProofConsumer<'a, P: IPairingArithmetic, E, K, A, F> {
        pairing: &'a P,
        hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
        encoder: &'a E,
        random: &'a dyn IRandomSourceAdapter,
        kem: &'a K,
        key_agreement: &'a A,
        statements: &'a HarnessStatements,
        params: &'a CreateGenerateParams,
        forms: PhantomData<F>,
    }

    impl<'a, P, E, K, A, F> IDeliveryProofConsumer<P, F> for DeliveryProofConsumer<'a, P, E, K, A, F>
    where
        P: IPairingArithmetic + IPairingReference,
        E: IEncoderAdapter,
        K: ICredentialKemAdapter<Pairing = P>,
        A: IKeyAgreementAdapter<Pairing = P>,
        F: IChainForms,
    {
        type Output = EmitReturn;

        fn consume_delivery_proof<D: IDeliveryProofAdapter<Pairing = P, Forms = F>>(
            &self,
            _params: ConsumeDeliveryProofParams,
            payload: ConsumeDeliveryProofPayload<D>,
        ) -> Self::Output {
            // Act
            let created = create_generate(
                &build_create_generate_deps(
                    self.pairing,
                    self.hashing,
                    self.encoder,
                    self.random,
                    self.kem,
                    self.key_agreement,
                    &payload.adapter,
                    build_generate_consumer(self.statements, Default::default()),
                    Default::default(),
                ),
                (*self.params).clone(),
                build_create_generate_payload(Default::default()),
            );
            let Ok(success) = created else {
                panic!("the generate factory admits the selection")
            };
            success.output
        }
    }

    struct KeyAgreementConsumer<'a, P: IPairingArithmetic, E, K, F> {
        pairing: &'a P,
        hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
        encoder: &'a E,
        random: &'a dyn IRandomSourceAdapter,
        kem: &'a K,
        statements: &'a HarnessStatements,
        params: &'a CreateGenerateParams,
        forms: PhantomData<F>,
    }

    impl<'a, P, E, K, F> IKeyAgreementConsumer<P> for KeyAgreementConsumer<'a, P, E, K, F>
    where
        P: IPairingArithmetic + IPairingReference,
        E: IEncoderAdapter,
        K: ICredentialKemAdapter<Pairing = P>,
        F: IChainForms,
    {
        type Output = EmitReturn;

        fn consume_key_agreement<A: IKeyAgreementAdapter<Pairing = P>>(
            &self,
            _params: ConsumeKeyAgreementParams,
            payload: ConsumeKeyAgreementPayload<A>,
        ) -> Self::Output {
            let created = create_delivery_proof(
                &build_create_delivery_proof_deps(
                    self.pairing,
                    self.hashing,
                    self.encoder,
                    self.random,
                    DeliveryProofConsumer {
                        pairing: self.pairing,
                        hashing: self.hashing,
                        encoder: self.encoder,
                        random: self.random,
                        kem: self.kem,
                        key_agreement: &payload.adapter,
                        statements: self.statements,
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

    struct KemConsumer<'a, P: IPairingArithmetic, E, F> {
        pairing: &'a P,
        hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
        encoder: &'a E,
        random: &'a dyn IRandomSourceAdapter,
        statements: &'a HarnessStatements,
        params: &'a CreateGenerateParams,
        forms: PhantomData<F>,
    }

    impl<'a, P, E, F> IKemConsumer<P> for KemConsumer<'a, P, E, F>
    where
        P: IPairingArithmetic + IPairingReference,
        E: IEncoderAdapter,
        F: IChainForms,
    {
        type Output = EmitReturn;

        fn consume_kem<K: ICredentialKemAdapter<Pairing = P>>(
            &self,
            _params: ConsumeKemParams,
            payload: ConsumeKemPayload<K>,
        ) -> Self::Output {
            let created = create_key_agreement(
                &build_create_key_agreement_deps(
                    self.pairing,
                    self.hashing,
                    self.encoder,
                    self.random,
                    KeyAgreementConsumer {
                        pairing: self.pairing,
                        hashing: self.hashing,
                        encoder: self.encoder,
                        random: self.random,
                        kem: &payload.adapter,
                        statements: self.statements,
                        params: self.params,
                        forms: PhantomData::<F>,
                    },
                    Default::default(),
                ),
                build_create_key_agreement_params(Default::default()),
                build_create_key_agreement_payload(Default::default()),
            );
            let Ok(success) = created else {
                panic!("the key-agreement factory admits the selection")
            };
            success.output
        }
    }

    struct ChainFormsConsumer<'a, P: IPairingArithmetic, E> {
        pairing: &'a P,
        hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
        encoder: &'a E,
        random: &'a dyn IRandomSourceAdapter,
        statements: &'a HarnessStatements,
        params: &'a CreateGenerateParams,
    }

    impl<'a, P, E> IChainFormsConsumer for ChainFormsConsumer<'a, P, E>
    where
        P: IPairingArithmetic + IPairingReference,
        E: IEncoderAdapter,
    {
        type Output = EmitReturn;

        fn consume_chain_forms<F: IChainForms>(
            &self,
            _params: ConsumeChainFormsParams,
            _payload: ConsumeChainFormsPayload<F>,
        ) -> Self::Output {
            let created = create_kem(
                &build_create_kem_deps(
                    self.pairing,
                    self.hashing,
                    KemConsumer {
                        pairing: self.pairing,
                        hashing: self.hashing,
                        encoder: self.encoder,
                        random: self.random,
                        statements: self.statements,
                        params: self.params,
                        forms: PhantomData::<F>,
                    },
                    Default::default(),
                ),
                build_create_kem_params(Default::default()),
                build_create_kem_payload(Default::default()),
            );
            let Ok(success) = created else {
                panic!("the kem factory admits the selection")
            };
            success.output
        }
    }

    struct EncodingConsumer<'a, P: IPairingArithmetic> {
        pairing: &'a P,
        hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
        random: &'a dyn IRandomSourceAdapter,
        statements: &'a HarnessStatements,
        params: &'a CreateGenerateParams,
    }

    impl<'a, P> IEncodingConsumer for EncodingConsumer<'a, P>
    where
        P: IPairingArithmetic + IPairingReference,
    {
        type Output = EmitReturn;

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
                        statements: self.statements,
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
        statements: &'a HarnessStatements,
        params: &'a CreateGenerateParams,
    }

    impl IPairingConsumer for PairingConsumer<'_> {
        type Output = EmitReturn;

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
                    statements: self.statements,
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

    let consumer = PairingConsumer {
        statements: &statements,
        params: &params,
    };
    let created = create_pairing(
        &build_create_pairing_deps(consumer, Default::default()),
        build_create_pairing_params(Default::default()),
        build_create_pairing_payload(Default::default()),
    );
    let Ok(success) = created else {
        panic!("the pairing factory ran the consumer")
    };
    let output = success.output;

    // Assert
    let Err(EmitErrorReturn::Mock(error)) = output else {
        panic!("emit refuses the write")
    };
    assert_eq!(error, MockIGenerateAdapterEmitErrorReturn::WriteRefused);
    // consume_generate_returns_the_refusal_emit_returns_unchanged
}
