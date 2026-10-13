#![allow(clippy::expect_used)]

use super::create_key_agreement;
use super::provides::{
    CreateKeyAgreementErrorReturn, CreateKeyAgreementParamsOverrides,
    DECLARED_KEY_AGREEMENT_SELECTIONS, MockIKeyAgreementConsumer, build_create_key_agreement_deps,
    build_create_key_agreement_params, build_create_key_agreement_payload,
};
use encoding::{
    ConsumeEncodingParams, ConsumeEncodingPayload, IDecoderAdapter, IEncoderAdapter,
    IEncodingConsumer, build_create_encoding_deps, build_create_encoding_params,
    build_create_encoding_payload, create_encoding,
};
use hash_to_scalar::{
    build_create_hash_to_scalar_deps, build_create_hash_to_scalar_params,
    build_create_hash_to_scalar_payload, create_hash_to_scalar,
};
use pairing::{
    ConsumePairingParams, ConsumePairingPayload, CreatePairingPayload, IPairingAdapter,
    IPairingArithmetic, IPairingConsumer, IPairingReference, build_create_pairing_deps,
    build_create_pairing_params, create_pairing,
};
use random::{
    CreateRandomSourceDeps, CreateRandomSourcePayload, build_create_random_source_params,
    create_random_source,
};

/// Contract: `create_key_agreement`, admitted: the arm's `try_new` returns
///   `Ok(adapter)` → `deps.consumer.consume_key_agreement` runs exactly once
///   and the return is `Ok(CreateKeyAgreementSuccessReturn { output })`
///   holding the consumer's output.
/// Arrange: `build_create_key_agreement_params` at its defaults, the mock
///   selection its declaration admits; `build_create_key_agreement_deps`
///   over the received borrows and `MockIKeyAgreementConsumer`; `Consumer`
///   whose method calls `create_encoding` with `EncodingConsumer`, whose
///   method obtains the hash-to-scalar adapter and the source and makes the
///   single call.
/// Act:     `create_key_agreement` once, inside `EncodingConsumer`'s method.
/// Assert:  `assert_eq!(result.map(|success| success.output), Ok(()))`, `()`
///   the output type `MockIKeyAgreementConsumer` declares; each `created`
///   `is_ok()`, proving the consumers ran.
/// Collaborators: the pairing from `create_pairing`, the encoder from
///   `create_encoding`, the hash-to-scalar adapter from
///   `create_hash_to_scalar`, and the source from `create_random_source`,
///   each reached through its factory at the defaults of its official params
///   builder, which select that family's mock concrete; the consumer,
///   `MockIKeyAgreementConsumer`; the key agreement is the family's mock
///   concrete the mock selection constructs.
#[test]
fn create_key_agreement_returns_the_consumers_output_for_the_mock_selection() {
    // Arrange
    struct Consumer;

    impl IPairingConsumer for Consumer {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            consumed: ConsumePairingPayload<P>,
        ) -> Self::Output {
            // Arrange
            struct EncodingConsumer<'a, P: IPairingAdapter> {
                pairing: &'a P,
            }

            impl<P: IPairingArithmetic + IPairingReference> IEncodingConsumer for EncodingConsumer<'_, P> {
                type Output = ();

                fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
                    &self,
                    _params: ConsumeEncodingParams,
                    consumed: ConsumeEncodingPayload<E>,
                ) -> Self::Output {
                    // Arrange
                    let hash_to_scalar = create_hash_to_scalar::<P::Scalar>(
                        &build_create_hash_to_scalar_deps(Default::default()),
                        build_create_hash_to_scalar_params(Default::default()),
                        build_create_hash_to_scalar_payload(Default::default()),
                    )
                    .expect("the default selection constructs the hash-to-scalar mock concrete")
                    .adapter;
                    let random = create_random_source(
                        &CreateRandomSourceDeps,
                        build_create_random_source_params(Default::default()),
                        CreateRandomSourcePayload,
                    )
                    .expect("the default selection constructs the random-source mock concrete")
                    .adapter;

                    // Act
                    let result = create_key_agreement(
                        &build_create_key_agreement_deps(
                            self.pairing,
                            &*hash_to_scalar,
                            &consumed.adapter,
                            &*random,
                            MockIKeyAgreementConsumer,
                            Default::default(),
                        ),
                        build_create_key_agreement_params(Default::default()),
                        build_create_key_agreement_payload(Default::default()),
                    );

                    // Assert
                    assert_eq!(result.map(|success| success.output), Ok(()));
                }
            }

            // Act
            let created = create_encoding(
                &build_create_encoding_deps(EncodingConsumer {
                    pairing: &consumed.adapter,
                }),
                build_create_encoding_params(Default::default()),
                build_create_encoding_payload(Default::default()),
            );

            // Assert
            assert!(created.is_ok());
        }
    }

    // Act
    let created = create_pairing(
        &build_create_pairing_deps(Consumer, Default::default()),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    );

    // Assert
    assert!(created.is_ok());
}

/// Contract: `create_key_agreement`, unsupported identifier: the `Mock`
///   arm's declared identifier is not `params.identifier` →
///   `Err(CreateKeyAgreementErrorReturn::UnsupportedKeyAgreementIdentifier)`,
///   with nothing constructed and the consumer not called.
/// Arrange: `build_create_key_agreement_params` with `identifier`
///   overridden by the identifier of a member of
///   `DECLARED_KEY_AGREEMENT_SELECTIONS`, one case per member, the
///   identifier differing from the one the mock concrete declares; the
///   consumers as
///   `create_key_agreement_returns_the_consumers_output_for_the_mock_selection`.
/// Act:     `create_key_agreement` once per case, inside `EncodingConsumer`'s
///   method.
/// Assert:  per case, `result.err()` equals
///   `Some(CreateKeyAgreementErrorReturn::UnsupportedKeyAgreementIdentifier)`
///   by `assert_eq!`; each `created` `is_ok()`.
/// Collaborators: as
///   `create_key_agreement_returns_the_consumers_output_for_the_mock_selection`.
#[test]
fn create_key_agreement_refuses_the_mock_selection_whose_identifier_the_mock_does_not_declare() {
    // Arrange
    struct Consumer;

    impl IPairingConsumer for Consumer {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            consumed: ConsumePairingPayload<P>,
        ) -> Self::Output {
            // Arrange
            struct EncodingConsumer<'a, P: IPairingAdapter> {
                pairing: &'a P,
            }

            impl<P: IPairingArithmetic + IPairingReference> IEncodingConsumer for EncodingConsumer<'_, P> {
                type Output = ();

                fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
                    &self,
                    _params: ConsumeEncodingParams,
                    consumed: ConsumeEncodingPayload<E>,
                ) -> Self::Output {
                    // Arrange
                    let hash_to_scalar = create_hash_to_scalar::<P::Scalar>(
                        &build_create_hash_to_scalar_deps(Default::default()),
                        build_create_hash_to_scalar_params(Default::default()),
                        build_create_hash_to_scalar_payload(Default::default()),
                    )
                    .expect("the default selection constructs the hash-to-scalar mock concrete")
                    .adapter;
                    let random = create_random_source(
                        &CreateRandomSourceDeps,
                        build_create_random_source_params(Default::default()),
                        CreateRandomSourcePayload,
                    )
                    .expect("the default selection constructs the random-source mock concrete")
                    .adapter;
                    let deps = build_create_key_agreement_deps(
                        self.pairing,
                        &*hash_to_scalar,
                        &consumed.adapter,
                        &*random,
                        MockIKeyAgreementConsumer,
                        Default::default(),
                    );

                    // Act and Assert
                    for selection in DECLARED_KEY_AGREEMENT_SELECTIONS {
                        let result = create_key_agreement(
                            &deps,
                            build_create_key_agreement_params(CreateKeyAgreementParamsOverrides {
                                identifier: Some(selection.identifier),
                                ..Default::default()
                            }),
                            build_create_key_agreement_payload(Default::default()),
                        );
                        assert_eq!(
                            result.err(),
                            Some(CreateKeyAgreementErrorReturn::UnsupportedKeyAgreementIdentifier)
                        );
                    }
                }
            }

            // Act
            let created = create_encoding(
                &build_create_encoding_deps(EncodingConsumer {
                    pairing: &consumed.adapter,
                }),
                build_create_encoding_params(Default::default()),
                build_create_encoding_payload(Default::default()),
            );

            // Assert
            assert!(created.is_ok());
        }
    }

    // Act
    let created = create_pairing(
        &build_create_pairing_deps(Consumer, Default::default()),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    );

    // Assert
    assert!(created.is_ok());
}

/// Contract: `create_key_agreement`, unsupported identifier: the
///   `PairingElGamal` arm's declared identifier is not `params.identifier` →
///   `Err(CreateKeyAgreementErrorReturn::UnsupportedKeyAgreementIdentifier)`,
///   with nothing constructed and the consumer not called, read before any
///   construction.
/// Arrange: each member of `DECLARED_KEY_AGREEMENT_SELECTIONS`, one case
///   per member, with `identifier` overridden by the identifier
///   `build_create_key_agreement_params` carries at its defaults, the
///   identifier the mock concrete declares; the consumers as
///   `create_key_agreement_returns_the_consumers_output_for_the_mock_selection`.
/// Act:     `create_key_agreement` once per case, inside `EncodingConsumer`'s
///   method.
/// Assert:  per case, `result.err()` equals
///   `Some(CreateKeyAgreementErrorReturn::UnsupportedKeyAgreementIdentifier)`
///   by `assert_eq!`; each `created` `is_ok()`.
/// Collaborators: as
///   `create_key_agreement_returns_the_consumers_output_for_the_mock_selection`;
///   the refusal is decided from the selected concrete's declaration
///   constant, before any construction, so no real concrete's function runs.
#[test]
fn create_key_agreement_refuses_a_declared_selection_whose_identifier_the_concrete_does_not_declare()
 {
    // Arrange
    struct Consumer;

    impl IPairingConsumer for Consumer {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            consumed: ConsumePairingPayload<P>,
        ) -> Self::Output {
            // Arrange
            struct EncodingConsumer<'a, P: IPairingAdapter> {
                pairing: &'a P,
            }

            impl<P: IPairingArithmetic + IPairingReference> IEncodingConsumer for EncodingConsumer<'_, P> {
                type Output = ();

                fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
                    &self,
                    _params: ConsumeEncodingParams,
                    consumed: ConsumeEncodingPayload<E>,
                ) -> Self::Output {
                    // Arrange
                    let hash_to_scalar = create_hash_to_scalar::<P::Scalar>(
                        &build_create_hash_to_scalar_deps(Default::default()),
                        build_create_hash_to_scalar_params(Default::default()),
                        build_create_hash_to_scalar_payload(Default::default()),
                    )
                    .expect("the default selection constructs the hash-to-scalar mock concrete")
                    .adapter;
                    let random = create_random_source(
                        &CreateRandomSourceDeps,
                        build_create_random_source_params(Default::default()),
                        CreateRandomSourcePayload,
                    )
                    .expect("the default selection constructs the random-source mock concrete")
                    .adapter;
                    let deps = build_create_key_agreement_deps(
                        self.pairing,
                        &*hash_to_scalar,
                        &consumed.adapter,
                        &*random,
                        MockIKeyAgreementConsumer,
                        Default::default(),
                    );
                    let mock_identifier =
                        build_create_key_agreement_params(Default::default()).identifier;

                    // Act and Assert
                    for selection in DECLARED_KEY_AGREEMENT_SELECTIONS {
                        let result = create_key_agreement(
                            &deps,
                            build_create_key_agreement_params(CreateKeyAgreementParamsOverrides {
                                concrete: Some(selection.concrete),
                                identifier: Some(mock_identifier),
                                algebra: Some(selection.algebra),
                            }),
                            build_create_key_agreement_payload(Default::default()),
                        );
                        assert_eq!(
                            result.err(),
                            Some(CreateKeyAgreementErrorReturn::UnsupportedKeyAgreementIdentifier)
                        );
                    }
                }
            }

            // Act
            let created = create_encoding(
                &build_create_encoding_deps(EncodingConsumer {
                    pairing: &consumed.adapter,
                }),
                build_create_encoding_params(Default::default()),
                build_create_encoding_payload(Default::default()),
            );

            // Assert
            assert!(created.is_ok());
        }
    }

    // Act
    let created = create_pairing(
        &build_create_pairing_deps(Consumer, Default::default()),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    );

    // Assert
    assert!(created.is_ok());
}
