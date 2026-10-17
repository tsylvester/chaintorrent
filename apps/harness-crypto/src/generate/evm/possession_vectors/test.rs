#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    
)]

use encoding::{
    CanonicalFieldKind, CanonicalFieldValue, CanonicalFieldsOverrides, ConsumeEncodingParams,
    ConsumeEncodingPayload, FromFieldsParams, IDecoderAdapter, IEncoderAdapter, IEncodingConsumer,
    IEncodingContract, ToFieldsParams, build_canonical_fields, build_create_encoding_deps,
    build_create_encoding_params, build_create_encoding_payload, create_encoding,
};
use envelope::{
    ConsumeKeyAgreementParams, ConsumeKeyAgreementPayload, CreateKeyAgreementParamsOverrides,
    GenerateKeysErrorReturn, IKeyAgreementAdapter, IKeyAgreementConsumer, KeyAgreementConcrete,
    MockIKeyAgreementAdapterFailureMode, MockIKeyAgreementAdapterGenerateKeysErrorReturn,
    build_create_key_agreement_deps, build_create_key_agreement_params,
    build_create_key_agreement_payload, create_key_agreement,
};
use hash_to_scalar::{
    IHashToScalarAdapter, build_create_hash_to_scalar_deps, build_create_hash_to_scalar_params,
    build_create_hash_to_scalar_payload, create_hash_to_scalar,
};
use pairing::{
    ConsumePairingParams, ConsumePairingPayload, CreatePairingPayload, IPairingArithmetic,
    IPairingConsumer, IPairingReference, build_create_pairing_deps, build_create_pairing_params,
    create_pairing,
};
use random::{
    CreateRandomSourceDeps, CreateRandomSourceParamsOverrides, CreateRandomSourcePayload,
    FillBytesErrorReturn, IRandomSourceAdapter, MockIRandomSourceAdapterFailureMode,
    RandomSourceKind, build_create_random_source_params, create_random_source,
};

use super::super::group_vectors::provides::{
    VectorCount, VectorCountConstructorParamsOverrides, build_vector_count,
};
use super::super::render::provides::{
    SolidityMemberNameConstructorParamsOverrides, SolidityRecordMemberOverrides,
    SolidityRecordMembersConstructorParamsOverrides, build_solidity_member_name,
    build_solidity_record_member, build_solidity_record_members,
};
use super::interface::*;
use super::mock::*;
use super::possession_vectors;

/// Contract: a refused draw → `Err(PossessionVectorsErrorReturn::
///   FillBytes(error))`, the error unchanged.
/// Arrange:  the random selection overridden to `RandomSourceKind::Mock(
///   MockIRandomSourceAdapterFailureMode::FillBytesRefused)`, which differs
///   from the builder's default of `Succeeds`; every other selection at its
///   default.
/// Act:      `possession_vectors` in the key-agreement consumer.
/// Assert:   the result equals `Err(PossessionVectorsErrorReturn::FillBytes(
///   FillBytesErrorReturn::MockIRandomSourceAdapter))`.
/// Collaborators: the random family's mock concrete in its `FillBytesRefused`
///   mode through `create_random_source`; the pairing's, hash-to-scalar's,
///   encoding's, and key agreement's mock concretes in their succeeding modes
///   through `create_pairing`, `create_hash_to_scalar`, `create_encoding`, and
///   `create_key_agreement`.
#[test]
fn possession_vectors_return_the_draw_refusal_unchanged() {
    // Arrange
    let created = create_random_source(
        &CreateRandomSourceDeps,
        build_create_random_source_params(CreateRandomSourceParamsOverrides {
            kind: Some(RandomSourceKind::Mock(
                MockIRandomSourceAdapterFailureMode::FillBytesRefused,
            )),
        }),
        CreateRandomSourcePayload,
    );
    let source = created.expect("the random-source factory admits the selection");

    struct KeyAgreementConsumer<'a, P: IPairingArithmetic> {
        pairing: &'a P,
        random: &'a dyn IRandomSourceAdapter,
        count: Option<VectorCount>,
    }

    impl<P: IPairingArithmetic> IKeyAgreementConsumer<P> for KeyAgreementConsumer<'_, P> {
        type Output = PossessionVectorsReturn;

        fn consume_key_agreement<K: IKeyAgreementAdapter<Pairing = P>>(
            &self,
            _params: ConsumeKeyAgreementParams,
            payload: ConsumeKeyAgreementPayload<K>,
        ) -> Self::Output {
            // Act
            possession_vectors::<P, K>(
                &build_possession_vectors_deps(
                    self.pairing,
                    &payload.adapter,
                    self.random,
                    Default::default(),
                ),
                build_possession_vectors_params(PossessionVectorsParamsOverrides {
                    count: self.count,
                }),
                build_possession_vectors_payload(Default::default()),
            )
        }
    }

    struct EncodingConsumer<'a, P: IPairingArithmetic> {
        pairing: &'a P,
        hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
        random: &'a dyn IRandomSourceAdapter,
        key_agreement_concrete: Option<KeyAgreementConcrete>,
        count: Option<VectorCount>,
    }

    impl<P: IPairingArithmetic> IEncodingConsumer for EncodingConsumer<'_, P> {
        type Output = PossessionVectorsReturn;

        fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
            &self,
            _params: ConsumeEncodingParams,
            payload: ConsumeEncodingPayload<E>,
        ) -> Self::Output {
            let created = create_key_agreement(
                &build_create_key_agreement_deps(
                    self.pairing,
                    self.hash_to_scalar,
                    &payload.adapter,
                    self.random,
                    KeyAgreementConsumer {
                        pairing: self.pairing,
                        random: self.random,
                        count: self.count,
                    },
                    Default::default(),
                ),
                build_create_key_agreement_params(CreateKeyAgreementParamsOverrides {
                    concrete: self.key_agreement_concrete,
                    ..Default::default()
                }),
                build_create_key_agreement_payload(Default::default()),
            );
            let Ok(success) = created else {
                panic!("the key-agreement factory admits the selection")
            };
            success.output
        }
    }

    struct PairingConsumer {
        key_agreement_concrete: Option<KeyAgreementConcrete>,
        count: Option<VectorCount>,
        random: Box<dyn IRandomSourceAdapter>,
    }

    impl IPairingConsumer for PairingConsumer {
        type Output = PossessionVectorsReturn;

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
                panic!("the hash-to-scalar adapter is created")
            };
            let created = create_encoding(
                &build_create_encoding_deps(EncodingConsumer {
                    pairing: &payload.adapter,
                    hash_to_scalar: &*hashing.adapter,
                    random: self.random.as_ref(),
                    key_agreement_concrete: self.key_agreement_concrete,
                    count: self.count,
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
        key_agreement_concrete: None,
        count: None,
        random: source.adapter,
    };
    let created = create_pairing(
        &build_create_pairing_deps(consumer, Default::default()),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    );
    let Ok(success) = created else {
        panic!("the pairing factory ran the consumer")
    };

    // Assert
    assert_eq!(
        success.output,
        Err(PossessionVectorsErrorReturn::FillBytes(
            FillBytesErrorReturn::MockIRandomSourceAdapter
        ))
    );
    // possession_vectors_return_the_draw_refusal_unchanged
}

/// Contract: a refused key generation → `Err(PossessionVectorsErrorReturn::
///   GenerateKeys(error))`, the error unchanged.
/// Arrange:  the key-agreement selection overridden to
///   `KeyAgreementConcrete::Mock(MockIKeyAgreementAdapterFailureMode::
///   NonceDrawRefused)`, which differs from the builder's default of
///   `NoFailure`; the random selection at its default, so both draws succeed;
///   every other selection at its default.
/// Act:      `possession_vectors` in the key-agreement consumer.
/// Assert:   the result equals `Err(PossessionVectorsErrorReturn::
///   GenerateKeys(GenerateKeysErrorReturn::Mock(
///   MockIKeyAgreementAdapterGenerateKeysErrorReturn::NonceDrawRefused)))`.
/// Collaborators: the key agreement's mock concrete in its `NonceDrawRefused`
///   mode through `create_key_agreement`; the random source's, pairing's,
///   hash-to-scalar's, and encoding's mock concretes in their succeeding modes
///   through `create_random_source`, `create_pairing`, `create_hash_to_scalar`,
///   and `create_encoding`.
#[test]
fn possession_vectors_return_the_generation_refusal_unchanged() {
    // Arrange
    let created = create_random_source(
        &CreateRandomSourceDeps,
        build_create_random_source_params(Default::default()),
        CreateRandomSourcePayload,
    );
    let source = created.expect("the random-source factory admits the selection");

    struct KeyAgreementConsumer<'a, P: IPairingArithmetic> {
        pairing: &'a P,
        random: &'a dyn IRandomSourceAdapter,
        count: Option<VectorCount>,
    }

    impl<P: IPairingArithmetic> IKeyAgreementConsumer<P> for KeyAgreementConsumer<'_, P> {
        type Output = PossessionVectorsReturn;

        fn consume_key_agreement<K: IKeyAgreementAdapter<Pairing = P>>(
            &self,
            _params: ConsumeKeyAgreementParams,
            payload: ConsumeKeyAgreementPayload<K>,
        ) -> Self::Output {
            // Act
            possession_vectors::<P, K>(
                &build_possession_vectors_deps(
                    self.pairing,
                    &payload.adapter,
                    self.random,
                    Default::default(),
                ),
                build_possession_vectors_params(PossessionVectorsParamsOverrides {
                    count: self.count,
                }),
                build_possession_vectors_payload(Default::default()),
            )
        }
    }

    struct EncodingConsumer<'a, P: IPairingArithmetic> {
        pairing: &'a P,
        hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
        random: &'a dyn IRandomSourceAdapter,
        key_agreement_concrete: Option<KeyAgreementConcrete>,
        count: Option<VectorCount>,
    }

    impl<P: IPairingArithmetic> IEncodingConsumer for EncodingConsumer<'_, P> {
        type Output = PossessionVectorsReturn;

        fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
            &self,
            _params: ConsumeEncodingParams,
            payload: ConsumeEncodingPayload<E>,
        ) -> Self::Output {
            let created = create_key_agreement(
                &build_create_key_agreement_deps(
                    self.pairing,
                    self.hash_to_scalar,
                    &payload.adapter,
                    self.random,
                    KeyAgreementConsumer {
                        pairing: self.pairing,
                        random: self.random,
                        count: self.count,
                    },
                    Default::default(),
                ),
                build_create_key_agreement_params(CreateKeyAgreementParamsOverrides {
                    concrete: self.key_agreement_concrete,
                    ..Default::default()
                }),
                build_create_key_agreement_payload(Default::default()),
            );
            let Ok(success) = created else {
                panic!("the key-agreement factory admits the selection")
            };
            success.output
        }
    }

    struct PairingConsumer {
        key_agreement_concrete: Option<KeyAgreementConcrete>,
        count: Option<VectorCount>,
        random: Box<dyn IRandomSourceAdapter>,
    }

    impl IPairingConsumer for PairingConsumer {
        type Output = PossessionVectorsReturn;

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
                panic!("the hash-to-scalar adapter is created")
            };
            let created = create_encoding(
                &build_create_encoding_deps(EncodingConsumer {
                    pairing: &payload.adapter,
                    hash_to_scalar: &*hashing.adapter,
                    random: self.random.as_ref(),
                    key_agreement_concrete: self.key_agreement_concrete,
                    count: self.count,
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
        key_agreement_concrete: Some(KeyAgreementConcrete::Mock(
            MockIKeyAgreementAdapterFailureMode::NonceDrawRefused,
        )),
        count: None,
        random: source.adapter,
    };
    let created = create_pairing(
        &build_create_pairing_deps(consumer, Default::default()),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    );
    let Ok(success) = created else {
        panic!("the pairing factory ran the consumer")
    };

    // Assert
    assert_eq!(
        success.output,
        Err(PossessionVectorsErrorReturn::GenerateKeys(
            GenerateKeysErrorReturn::Mock(
                MockIKeyAgreementAdapterGenerateKeysErrorReturn::NonceDrawRefused
            )
        ))
    );
    // possession_vectors_return_the_generation_refusal_unchanged
}

/// Contract: every record generated → `Ok` whose `records` hold the count the
///   params carry.
/// Arrange:  `build_possession_vectors_params` with `count` overridden to
///   `build_vector_count` over the literal `3`, which is greater than one and
///   differs from the builder's default of `2`; every selection at its
///   default.
/// Act:      `possession_vectors` in the key-agreement consumer.
/// Assert:   the success's `records` length equals `3`, so a function that
///   emits the default count or one record fails.
/// Collaborators: the random source's, pairing's, hash-to-scalar's,
///   encoding's, and key agreement's mock concretes in their succeeding modes
///   through `create_random_source`, `create_pairing`, `create_hash_to_scalar`,
///   `create_encoding`, and `create_key_agreement`.
#[test]
fn possession_vectors_emit_the_configured_count_of_records() {
    // Arrange
    let created = create_random_source(
        &CreateRandomSourceDeps,
        build_create_random_source_params(Default::default()),
        CreateRandomSourcePayload,
    );
    let source = created.expect("the random-source factory admits the selection");

    struct KeyAgreementConsumer<'a, P: IPairingArithmetic> {
        pairing: &'a P,
        random: &'a dyn IRandomSourceAdapter,
        count: Option<VectorCount>,
    }

    impl<P: IPairingArithmetic> IKeyAgreementConsumer<P> for KeyAgreementConsumer<'_, P> {
        type Output = PossessionVectorsReturn;

        fn consume_key_agreement<K: IKeyAgreementAdapter<Pairing = P>>(
            &self,
            _params: ConsumeKeyAgreementParams,
            payload: ConsumeKeyAgreementPayload<K>,
        ) -> Self::Output {
            // Act
            possession_vectors::<P, K>(
                &build_possession_vectors_deps(
                    self.pairing,
                    &payload.adapter,
                    self.random,
                    Default::default(),
                ),
                build_possession_vectors_params(PossessionVectorsParamsOverrides {
                    count: self.count,
                }),
                build_possession_vectors_payload(Default::default()),
            )
        }
    }

    struct EncodingConsumer<'a, P: IPairingArithmetic> {
        pairing: &'a P,
        hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
        random: &'a dyn IRandomSourceAdapter,
        key_agreement_concrete: Option<KeyAgreementConcrete>,
        count: Option<VectorCount>,
    }

    impl<P: IPairingArithmetic> IEncodingConsumer for EncodingConsumer<'_, P> {
        type Output = PossessionVectorsReturn;

        fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
            &self,
            _params: ConsumeEncodingParams,
            payload: ConsumeEncodingPayload<E>,
        ) -> Self::Output {
            let created = create_key_agreement(
                &build_create_key_agreement_deps(
                    self.pairing,
                    self.hash_to_scalar,
                    &payload.adapter,
                    self.random,
                    KeyAgreementConsumer {
                        pairing: self.pairing,
                        random: self.random,
                        count: self.count,
                    },
                    Default::default(),
                ),
                build_create_key_agreement_params(CreateKeyAgreementParamsOverrides {
                    concrete: self.key_agreement_concrete,
                    ..Default::default()
                }),
                build_create_key_agreement_payload(Default::default()),
            );
            let Ok(success) = created else {
                panic!("the key-agreement factory admits the selection")
            };
            success.output
        }
    }

    struct PairingConsumer {
        key_agreement_concrete: Option<KeyAgreementConcrete>,
        count: Option<VectorCount>,
        random: Box<dyn IRandomSourceAdapter>,
    }

    impl IPairingConsumer for PairingConsumer {
        type Output = PossessionVectorsReturn;

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
                panic!("the hash-to-scalar adapter is created")
            };
            let created = create_encoding(
                &build_create_encoding_deps(EncodingConsumer {
                    pairing: &payload.adapter,
                    hash_to_scalar: &*hashing.adapter,
                    random: self.random.as_ref(),
                    key_agreement_concrete: self.key_agreement_concrete,
                    count: self.count,
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
        key_agreement_concrete: None,
        count: Some(build_vector_count(VectorCountConstructorParamsOverrides {
            count: Some(3),
        })),
        random: source.adapter,
    };
    let created = create_pairing(
        &build_create_pairing_deps(consumer, Default::default()),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    );
    let Ok(created) = created else {
        panic!("the pairing factory ran the consumer")
    };
    let result = created.output;

    // Assert
    let success = result.expect("the configured count is admitted");
    assert_eq!(success.records.len(), 3);
    // possession_vectors_emit_the_configured_count_of_records
}

/// Contract: each record's two draws are the next two draws of the random
///   source → no two records share an `x`, no two share a `y`, and a record's
///   `x` differs from its `y`.
/// Arrange:  the params as in
///   `possession_vectors_emit_the_configured_count_of_records`, the random
///   selection at its default, the family's mock concrete, whose draws differ
///   from call to call.
/// Act:      `possession_vectors` in the key-agreement consumer.
/// Assert:   every pair of records differs in `x` and in `y`, and each
///   record's `x` differs from its `y`, so a function that reuses a draw or a
///   record fails.
/// Collaborators: the random source's, pairing's, hash-to-scalar's,
///   encoding's, and key agreement's mock concretes in their succeeding modes
///   through `create_random_source`, `create_pairing`, `create_hash_to_scalar`,
///   `create_encoding`, and `create_key_agreement`.
#[test]
fn possession_vectors_give_each_record_its_own_secrets() {
    // Arrange
    let created = create_random_source(
        &CreateRandomSourceDeps,
        build_create_random_source_params(Default::default()),
        CreateRandomSourcePayload,
    );
    let source = created.expect("the random-source factory admits the selection");

    struct KeyAgreementConsumer<'a, P: IPairingArithmetic> {
        pairing: &'a P,
        random: &'a dyn IRandomSourceAdapter,
        count: Option<VectorCount>,
    }

    impl<P: IPairingArithmetic> IKeyAgreementConsumer<P> for KeyAgreementConsumer<'_, P> {
        type Output = PossessionVectorsReturn;

        fn consume_key_agreement<K: IKeyAgreementAdapter<Pairing = P>>(
            &self,
            _params: ConsumeKeyAgreementParams,
            payload: ConsumeKeyAgreementPayload<K>,
        ) -> Self::Output {
            // Act
            possession_vectors::<P, K>(
                &build_possession_vectors_deps(
                    self.pairing,
                    &payload.adapter,
                    self.random,
                    Default::default(),
                ),
                build_possession_vectors_params(PossessionVectorsParamsOverrides {
                    count: self.count,
                }),
                build_possession_vectors_payload(Default::default()),
            )
        }
    }

    struct EncodingConsumer<'a, P: IPairingArithmetic> {
        pairing: &'a P,
        hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
        random: &'a dyn IRandomSourceAdapter,
        key_agreement_concrete: Option<KeyAgreementConcrete>,
        count: Option<VectorCount>,
    }

    impl<P: IPairingArithmetic> IEncodingConsumer for EncodingConsumer<'_, P> {
        type Output = PossessionVectorsReturn;

        fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
            &self,
            _params: ConsumeEncodingParams,
            payload: ConsumeEncodingPayload<E>,
        ) -> Self::Output {
            let created = create_key_agreement(
                &build_create_key_agreement_deps(
                    self.pairing,
                    self.hash_to_scalar,
                    &payload.adapter,
                    self.random,
                    KeyAgreementConsumer {
                        pairing: self.pairing,
                        random: self.random,
                        count: self.count,
                    },
                    Default::default(),
                ),
                build_create_key_agreement_params(CreateKeyAgreementParamsOverrides {
                    concrete: self.key_agreement_concrete,
                    ..Default::default()
                }),
                build_create_key_agreement_payload(Default::default()),
            );
            let Ok(success) = created else {
                panic!("the key-agreement factory admits the selection")
            };
            success.output
        }
    }

    struct PairingConsumer {
        key_agreement_concrete: Option<KeyAgreementConcrete>,
        count: Option<VectorCount>,
        random: Box<dyn IRandomSourceAdapter>,
    }

    impl IPairingConsumer for PairingConsumer {
        type Output = PossessionVectorsReturn;

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
                panic!("the hash-to-scalar adapter is created")
            };
            let created = create_encoding(
                &build_create_encoding_deps(EncodingConsumer {
                    pairing: &payload.adapter,
                    hash_to_scalar: &*hashing.adapter,
                    random: self.random.as_ref(),
                    key_agreement_concrete: self.key_agreement_concrete,
                    count: self.count,
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
        key_agreement_concrete: None,
        count: Some(build_vector_count(VectorCountConstructorParamsOverrides {
            count: Some(3),
        })),
        random: source.adapter,
    };
    let created = create_pairing(
        &build_create_pairing_deps(consumer, Default::default()),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    );
    let Ok(created) = created else {
        panic!("the pairing factory ran the consumer")
    };
    let result = created.output;

    // Assert
    let success = result.expect("the mock adapters generate every key pair");
    for (index, left) in success.records.iter().enumerate() {
        assert_ne!(left.x, left.y);
        for right in success.records.iter().skip(index + 1) {
            assert_ne!(left.x, right.x);
            assert_ne!(left.y, right.y);
        }
    }
    // possession_vectors_give_each_record_its_own_secrets
}

/// Contract: `x`, `y`, `z1`, and `z2` are the scalar encodings of the key
///   pair's secrets and the possession's responses → each is the scalar's 32
///   big-endian bytes.
/// Arrange:  the params as in
///   `possession_vectors_emit_the_configured_count_of_records`, every
///   selection at its default, the pairing the family's mock concrete, whose
///   scalars are residues below `65521` encoded as 32 big-endian bytes with
///   the first twenty-four zero.
/// Act:      `possession_vectors` in the key-agreement consumer.
/// Assert:   for every record, each of `x`, `y`, `z1`, and `z2` has its first
///   twenty-four bytes zero and its last eight bytes, read as a big-endian
///   `u64`, below the literal `65521`, so a little-endian, truncated, or
///   padded-on-the-wrong-side encoding fails.
/// Collaborators: the random source's, pairing's, hash-to-scalar's,
///   encoding's, and key agreement's mock concretes in their succeeding modes
///   through `create_random_source`, `create_pairing`, `create_hash_to_scalar`,
///   `create_encoding`, and `create_key_agreement`.
#[test]
fn possession_vectors_carry_their_scalar_fields_as_32_byte_encodings() {
    // Arrange
    let created = create_random_source(
        &CreateRandomSourceDeps,
        build_create_random_source_params(Default::default()),
        CreateRandomSourcePayload,
    );
    let source = created.expect("the random-source factory admits the selection");

    struct KeyAgreementConsumer<'a, P: IPairingArithmetic> {
        pairing: &'a P,
        random: &'a dyn IRandomSourceAdapter,
        count: Option<VectorCount>,
    }

    impl<P: IPairingArithmetic> IKeyAgreementConsumer<P> for KeyAgreementConsumer<'_, P> {
        type Output = PossessionVectorsReturn;

        fn consume_key_agreement<K: IKeyAgreementAdapter<Pairing = P>>(
            &self,
            _params: ConsumeKeyAgreementParams,
            payload: ConsumeKeyAgreementPayload<K>,
        ) -> Self::Output {
            // Act
            possession_vectors::<P, K>(
                &build_possession_vectors_deps(
                    self.pairing,
                    &payload.adapter,
                    self.random,
                    Default::default(),
                ),
                build_possession_vectors_params(PossessionVectorsParamsOverrides {
                    count: self.count,
                }),
                build_possession_vectors_payload(Default::default()),
            )
        }
    }

    struct EncodingConsumer<'a, P: IPairingArithmetic> {
        pairing: &'a P,
        hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
        random: &'a dyn IRandomSourceAdapter,
        key_agreement_concrete: Option<KeyAgreementConcrete>,
        count: Option<VectorCount>,
    }

    impl<P: IPairingArithmetic> IEncodingConsumer for EncodingConsumer<'_, P> {
        type Output = PossessionVectorsReturn;

        fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
            &self,
            _params: ConsumeEncodingParams,
            payload: ConsumeEncodingPayload<E>,
        ) -> Self::Output {
            let created = create_key_agreement(
                &build_create_key_agreement_deps(
                    self.pairing,
                    self.hash_to_scalar,
                    &payload.adapter,
                    self.random,
                    KeyAgreementConsumer {
                        pairing: self.pairing,
                        random: self.random,
                        count: self.count,
                    },
                    Default::default(),
                ),
                build_create_key_agreement_params(CreateKeyAgreementParamsOverrides {
                    concrete: self.key_agreement_concrete,
                    ..Default::default()
                }),
                build_create_key_agreement_payload(Default::default()),
            );
            let Ok(success) = created else {
                panic!("the key-agreement factory admits the selection")
            };
            success.output
        }
    }

    struct PairingConsumer {
        key_agreement_concrete: Option<KeyAgreementConcrete>,
        count: Option<VectorCount>,
        random: Box<dyn IRandomSourceAdapter>,
    }

    impl IPairingConsumer for PairingConsumer {
        type Output = PossessionVectorsReturn;

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
                panic!("the hash-to-scalar adapter is created")
            };
            let created = create_encoding(
                &build_create_encoding_deps(EncodingConsumer {
                    pairing: &payload.adapter,
                    hash_to_scalar: &*hashing.adapter,
                    random: self.random.as_ref(),
                    key_agreement_concrete: self.key_agreement_concrete,
                    count: self.count,
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
        key_agreement_concrete: None,
        count: Some(build_vector_count(VectorCountConstructorParamsOverrides {
            count: Some(3),
        })),
        random: source.adapter,
    };
    let created = create_pairing(
        &build_create_pairing_deps(consumer, Default::default()),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    );
    let Ok(created) = created else {
        panic!("the pairing factory ran the consumer")
    };
    let result = created.output;

    // Assert
    let success = result.expect("the mock adapters generate every key pair");
    for record in &success.records {
        for scalar in [&record.x, &record.y, &record.z1, &record.z2] {
            assert!(scalar[..24].iter().all(|byte| *byte == 0x00));
            assert!(
                u64::from_be_bytes(
                    <[u8; 8]>::try_from(&scalar[24..]).expect("the field holds 32 bytes")
                ) < 65521
            );
        }
    }
    // possession_vectors_carry_their_scalar_fields_as_32_byte_encodings
}

/// Contract: `pk1` is the `encode_g1` bytes of the first-group generator
///   times the key pair's `x` → the record's `pk1` read as a big-endian `u64`
///   equals its `x` read as a big-endian `u64`, the mock concrete's
///   first-group generator being residue `1` and its multiplication a product
///   modulo `65521`.
/// Arrange:  the params as in
///   `possession_vectors_emit_the_configured_count_of_records`, every
///   selection at its default.
/// Act:      `possession_vectors` in the key-agreement consumer.
/// Assert:   for every record, `pk1` is eight bytes and its `u64` equals the
///   `u64` of the last eight bytes of `x`, so a `pk1` formed from `y`, or
///   swapped with `pk2`, fails.
/// Collaborators: the random source's, pairing's, hash-to-scalar's,
///   encoding's, and key agreement's mock concretes in their succeeding modes
///   through `create_random_source`, `create_pairing`, `create_hash_to_scalar`,
///   `create_encoding`, and `create_key_agreement`.
#[test]
fn possession_vectors_carry_the_first_group_public_key_the_secret_forms() {
    // Arrange
    let created = create_random_source(
        &CreateRandomSourceDeps,
        build_create_random_source_params(Default::default()),
        CreateRandomSourcePayload,
    );
    let source = created.expect("the random-source factory admits the selection");

    struct KeyAgreementConsumer<'a, P: IPairingArithmetic> {
        pairing: &'a P,
        random: &'a dyn IRandomSourceAdapter,
        count: Option<VectorCount>,
    }

    impl<P: IPairingArithmetic> IKeyAgreementConsumer<P> for KeyAgreementConsumer<'_, P> {
        type Output = PossessionVectorsReturn;

        fn consume_key_agreement<K: IKeyAgreementAdapter<Pairing = P>>(
            &self,
            _params: ConsumeKeyAgreementParams,
            payload: ConsumeKeyAgreementPayload<K>,
        ) -> Self::Output {
            // Act
            possession_vectors::<P, K>(
                &build_possession_vectors_deps(
                    self.pairing,
                    &payload.adapter,
                    self.random,
                    Default::default(),
                ),
                build_possession_vectors_params(PossessionVectorsParamsOverrides {
                    count: self.count,
                }),
                build_possession_vectors_payload(Default::default()),
            )
        }
    }

    struct EncodingConsumer<'a, P: IPairingArithmetic> {
        pairing: &'a P,
        hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
        random: &'a dyn IRandomSourceAdapter,
        key_agreement_concrete: Option<KeyAgreementConcrete>,
        count: Option<VectorCount>,
    }

    impl<P: IPairingArithmetic> IEncodingConsumer for EncodingConsumer<'_, P> {
        type Output = PossessionVectorsReturn;

        fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
            &self,
            _params: ConsumeEncodingParams,
            payload: ConsumeEncodingPayload<E>,
        ) -> Self::Output {
            let created = create_key_agreement(
                &build_create_key_agreement_deps(
                    self.pairing,
                    self.hash_to_scalar,
                    &payload.adapter,
                    self.random,
                    KeyAgreementConsumer {
                        pairing: self.pairing,
                        random: self.random,
                        count: self.count,
                    },
                    Default::default(),
                ),
                build_create_key_agreement_params(CreateKeyAgreementParamsOverrides {
                    concrete: self.key_agreement_concrete,
                    ..Default::default()
                }),
                build_create_key_agreement_payload(Default::default()),
            );
            let Ok(success) = created else {
                panic!("the key-agreement factory admits the selection")
            };
            success.output
        }
    }

    struct PairingConsumer {
        key_agreement_concrete: Option<KeyAgreementConcrete>,
        count: Option<VectorCount>,
        random: Box<dyn IRandomSourceAdapter>,
    }

    impl IPairingConsumer for PairingConsumer {
        type Output = PossessionVectorsReturn;

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
                panic!("the hash-to-scalar adapter is created")
            };
            let created = create_encoding(
                &build_create_encoding_deps(EncodingConsumer {
                    pairing: &payload.adapter,
                    hash_to_scalar: &*hashing.adapter,
                    random: self.random.as_ref(),
                    key_agreement_concrete: self.key_agreement_concrete,
                    count: self.count,
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
        key_agreement_concrete: None,
        count: Some(build_vector_count(VectorCountConstructorParamsOverrides {
            count: Some(3),
        })),
        random: source.adapter,
    };
    let created = create_pairing(
        &build_create_pairing_deps(consumer, Default::default()),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    );
    let Ok(created) = created else {
        panic!("the pairing factory ran the consumer")
    };
    let result = created.output;

    // Assert
    let success = result.expect("the mock adapters generate every key pair");
    for record in &success.records {
        assert_eq!(record.pk1.len(), 8);
        assert_eq!(
            u64::from_be_bytes(
                <[u8; 8]>::try_from(record.pk1.as_slice()).expect("pk1 is eight bytes")
            ),
            u64::from_be_bytes(<[u8; 8]>::try_from(&record.x[24..]).expect("x holds 32 bytes"))
        );
    }
    // possession_vectors_carry_the_first_group_public_key_the_secret_forms
}

/// Contract: `pk2` is the `encode_g2` bytes of the second-group generator
///   times the key pair's `y` → the record's `pk2` read as a big-endian `u64`
///   equals twice its `y` read as a big-endian `u64`, the mock concrete's
///   second-group generator being residue `2` and its multiplication a
///   product modulo `2 * 65521`.
/// Arrange:  the params as in
///   `possession_vectors_emit_the_configured_count_of_records`, every
///   selection at its default.
/// Act:      `possession_vectors` in the key-agreement consumer.
/// Assert:   for every record, `pk2` is eight bytes and its `u64` equals
///   twice the `u64` of the last eight bytes of `y`, so a `pk2` formed from
///   `x`, or swapped with `pk1`, fails.
/// Collaborators: the random source's, pairing's, hash-to-scalar's,
///   encoding's, and key agreement's mock concretes in their succeeding modes
///   through `create_random_source`, `create_pairing`, `create_hash_to_scalar`,
///   `create_encoding`, and `create_key_agreement`.
#[test]
fn possession_vectors_carry_the_second_group_public_key_the_secret_forms() {
    // Arrange
    let created = create_random_source(
        &CreateRandomSourceDeps,
        build_create_random_source_params(Default::default()),
        CreateRandomSourcePayload,
    );
    let source = created.expect("the random-source factory admits the selection");

    struct KeyAgreementConsumer<'a, P: IPairingArithmetic> {
        pairing: &'a P,
        random: &'a dyn IRandomSourceAdapter,
        count: Option<VectorCount>,
    }

    impl<P: IPairingArithmetic> IKeyAgreementConsumer<P> for KeyAgreementConsumer<'_, P> {
        type Output = PossessionVectorsReturn;

        fn consume_key_agreement<K: IKeyAgreementAdapter<Pairing = P>>(
            &self,
            _params: ConsumeKeyAgreementParams,
            payload: ConsumeKeyAgreementPayload<K>,
        ) -> Self::Output {
            // Act
            possession_vectors::<P, K>(
                &build_possession_vectors_deps(
                    self.pairing,
                    &payload.adapter,
                    self.random,
                    Default::default(),
                ),
                build_possession_vectors_params(PossessionVectorsParamsOverrides {
                    count: self.count,
                }),
                build_possession_vectors_payload(Default::default()),
            )
        }
    }

    struct EncodingConsumer<'a, P: IPairingArithmetic> {
        pairing: &'a P,
        hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
        random: &'a dyn IRandomSourceAdapter,
        key_agreement_concrete: Option<KeyAgreementConcrete>,
        count: Option<VectorCount>,
    }

    impl<P: IPairingArithmetic> IEncodingConsumer for EncodingConsumer<'_, P> {
        type Output = PossessionVectorsReturn;

        fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
            &self,
            _params: ConsumeEncodingParams,
            payload: ConsumeEncodingPayload<E>,
        ) -> Self::Output {
            let created = create_key_agreement(
                &build_create_key_agreement_deps(
                    self.pairing,
                    self.hash_to_scalar,
                    &payload.adapter,
                    self.random,
                    KeyAgreementConsumer {
                        pairing: self.pairing,
                        random: self.random,
                        count: self.count,
                    },
                    Default::default(),
                ),
                build_create_key_agreement_params(CreateKeyAgreementParamsOverrides {
                    concrete: self.key_agreement_concrete,
                    ..Default::default()
                }),
                build_create_key_agreement_payload(Default::default()),
            );
            let Ok(success) = created else {
                panic!("the key-agreement factory admits the selection")
            };
            success.output
        }
    }

    struct PairingConsumer {
        key_agreement_concrete: Option<KeyAgreementConcrete>,
        count: Option<VectorCount>,
        random: Box<dyn IRandomSourceAdapter>,
    }

    impl IPairingConsumer for PairingConsumer {
        type Output = PossessionVectorsReturn;

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
                panic!("the hash-to-scalar adapter is created")
            };
            let created = create_encoding(
                &build_create_encoding_deps(EncodingConsumer {
                    pairing: &payload.adapter,
                    hash_to_scalar: &*hashing.adapter,
                    random: self.random.as_ref(),
                    key_agreement_concrete: self.key_agreement_concrete,
                    count: self.count,
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
        key_agreement_concrete: None,
        count: Some(build_vector_count(VectorCountConstructorParamsOverrides {
            count: Some(3),
        })),
        random: source.adapter,
    };
    let created = create_pairing(
        &build_create_pairing_deps(consumer, Default::default()),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    );
    let Ok(created) = created else {
        panic!("the pairing factory ran the consumer")
    };
    let result = created.output;

    // Assert
    let success = result.expect("the mock adapters generate every key pair");
    for record in &success.records {
        assert_eq!(record.pk2.len(), 8);
        assert_eq!(
            u64::from_be_bytes(
                <[u8; 8]>::try_from(record.pk2.as_slice()).expect("pk2 is eight bytes")
            ),
            2 * u64::from_be_bytes(<[u8; 8]>::try_from(&record.y[24..]).expect("y holds 32 bytes"))
        );
    }
    // possession_vectors_carry_the_second_group_public_key_the_secret_forms
}

/// Contract: `r1` is the `encode_g1` bytes and `r2` the `encode_g2` bytes of
///   the proofs' commitments → each is the eight big-endian bytes of a
///   residue of its group.
/// Arrange:  the params as in
///   `possession_vectors_emit_the_configured_count_of_records`, every
///   selection at its default.
/// Act:      `possession_vectors` in the key-agreement consumer.
/// Assert:   for every record, `r1` is eight bytes whose `u64` is below the
///   literal `65521` and `r2` is eight bytes whose `u64` is even and below
///   the literal `131042`, the second group's subgroup being the even
///   residues, so a commitment encoded in the other group fails.
/// Collaborators: the random source's, pairing's, hash-to-scalar's,
///   encoding's, and key agreement's mock concretes in their succeeding modes
///   through `create_random_source`, `create_pairing`, `create_hash_to_scalar`,
///   `create_encoding`, and `create_key_agreement`.
#[test]
fn possession_vectors_carry_each_commitment_in_its_groups_encoding() {
    // Arrange
    let created = create_random_source(
        &CreateRandomSourceDeps,
        build_create_random_source_params(Default::default()),
        CreateRandomSourcePayload,
    );
    let source = created.expect("the random-source factory admits the selection");

    struct KeyAgreementConsumer<'a, P: IPairingArithmetic> {
        pairing: &'a P,
        random: &'a dyn IRandomSourceAdapter,
        count: Option<VectorCount>,
    }

    impl<P: IPairingArithmetic> IKeyAgreementConsumer<P> for KeyAgreementConsumer<'_, P> {
        type Output = PossessionVectorsReturn;

        fn consume_key_agreement<K: IKeyAgreementAdapter<Pairing = P>>(
            &self,
            _params: ConsumeKeyAgreementParams,
            payload: ConsumeKeyAgreementPayload<K>,
        ) -> Self::Output {
            // Act
            possession_vectors::<P, K>(
                &build_possession_vectors_deps(
                    self.pairing,
                    &payload.adapter,
                    self.random,
                    Default::default(),
                ),
                build_possession_vectors_params(PossessionVectorsParamsOverrides {
                    count: self.count,
                }),
                build_possession_vectors_payload(Default::default()),
            )
        }
    }

    struct EncodingConsumer<'a, P: IPairingArithmetic> {
        pairing: &'a P,
        hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
        random: &'a dyn IRandomSourceAdapter,
        key_agreement_concrete: Option<KeyAgreementConcrete>,
        count: Option<VectorCount>,
    }

    impl<P: IPairingArithmetic> IEncodingConsumer for EncodingConsumer<'_, P> {
        type Output = PossessionVectorsReturn;

        fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
            &self,
            _params: ConsumeEncodingParams,
            payload: ConsumeEncodingPayload<E>,
        ) -> Self::Output {
            let created = create_key_agreement(
                &build_create_key_agreement_deps(
                    self.pairing,
                    self.hash_to_scalar,
                    &payload.adapter,
                    self.random,
                    KeyAgreementConsumer {
                        pairing: self.pairing,
                        random: self.random,
                        count: self.count,
                    },
                    Default::default(),
                ),
                build_create_key_agreement_params(CreateKeyAgreementParamsOverrides {
                    concrete: self.key_agreement_concrete,
                    ..Default::default()
                }),
                build_create_key_agreement_payload(Default::default()),
            );
            let Ok(success) = created else {
                panic!("the key-agreement factory admits the selection")
            };
            success.output
        }
    }

    struct PairingConsumer {
        key_agreement_concrete: Option<KeyAgreementConcrete>,
        count: Option<VectorCount>,
        random: Box<dyn IRandomSourceAdapter>,
    }

    impl IPairingConsumer for PairingConsumer {
        type Output = PossessionVectorsReturn;

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
                panic!("the hash-to-scalar adapter is created")
            };
            let created = create_encoding(
                &build_create_encoding_deps(EncodingConsumer {
                    pairing: &payload.adapter,
                    hash_to_scalar: &*hashing.adapter,
                    random: self.random.as_ref(),
                    key_agreement_concrete: self.key_agreement_concrete,
                    count: self.count,
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
        key_agreement_concrete: None,
        count: Some(build_vector_count(VectorCountConstructorParamsOverrides {
            count: Some(3),
        })),
        random: source.adapter,
    };
    let created = create_pairing(
        &build_create_pairing_deps(consumer, Default::default()),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    );
    let Ok(created) = created else {
        panic!("the pairing factory ran the consumer")
    };
    let result = created.output;

    // Assert
    let success = result.expect("the mock adapters generate every key pair");
    for record in &success.records {
        assert_eq!(record.r1.len(), 8);
        assert!(
            u64::from_be_bytes(
                <[u8; 8]>::try_from(record.r1.as_slice()).expect("r1 is eight bytes")
            ) < 65521
        );
        assert_eq!(record.r2.len(), 8);
        let commitment = u64::from_be_bytes(
            <[u8; 8]>::try_from(record.r2.as_slice()).expect("r2 is eight bytes"),
        );
        assert_eq!(commitment % 2, 0);
        assert!(commitment < 131_042);
    }
    // possession_vectors_carry_each_commitment_in_its_groups_encoding
}

/// Contract: `try_new` over the fieldless params →
///   `Ok(PossessionVectorDescription)`.
/// Arrange:  `build_possession_vector_description_constructor_params(
///   Default::default())`.
/// Act:      `PossessionVectorDescription::try_new`.
/// Assert:   the result matches `Ok(PossessionVectorDescription)`.
/// Collaborators: none — the description is a leaf.
#[test]
fn possession_vector_description_admits_its_fieldless_constructor_params() {
    // Arrange
    let params = build_possession_vector_description_constructor_params(Default::default());

    // Act
    let result = PossessionVectorDescription::try_new(params);

    // Assert
    assert!(matches!(result, Ok(PossessionVectorDescription)));
    // possession_vector_description_admits_its_fieldless_constructor_params
}

/// Contract: `to_fields` over a record → `values` being, in member order,
///   `Unsigned256` of `x` and of `y`, `Bytes` of `pk1`, `pk2`, and `r1`,
///   `Unsigned256` of `z1`, `Bytes` of `r2`, and `Unsigned256` of `z2`.
/// Arrange:  `build_possession_vector(Default::default())` and
///   `build_possession_vector_description(Default::default())`.
/// Act:      `to_fields` with `ToFieldsParams`.
/// Assert:   the values equal `Unsigned256([0x01; 32])`,
///   `Unsigned256([0x02; 32])`, `Bytes(vec![0x03; 64])`,
///   `Bytes(vec![0x04; 128])`, `Bytes(vec![0x05; 64])`,
///   `Unsigned256([0x06; 32])`, `Bytes(vec![0x07; 128])`, and
///   `Unsigned256([0x08; 32])`, each a `CanonicalFieldValue`, in that order;
///   the record's fields differ from each other, so a swap of two fields
///   fails.
/// Collaborators: none — the description is a leaf.
#[test]
fn possession_vector_description_maps_secrets_keys_and_proof_components_in_member_order() {
    // Arrange
    let vector = build_possession_vector(Default::default());
    let description = build_possession_vector_description(Default::default());

    // Act
    let Ok(success) = description.to_fields(ToFieldsParams, &vector);

    // Assert
    assert_eq!(
        success.fields.values,
        vec![
            CanonicalFieldValue::Unsigned256([0x01; 32]),
            CanonicalFieldValue::Unsigned256([0x02; 32]),
            CanonicalFieldValue::Bytes(vec![0x03; 64]),
            CanonicalFieldValue::Bytes(vec![0x04; 128]),
            CanonicalFieldValue::Bytes(vec![0x05; 64]),
            CanonicalFieldValue::Unsigned256([0x06; 32]),
            CanonicalFieldValue::Bytes(vec![0x07; 128]),
            CanonicalFieldValue::Unsigned256([0x08; 32]),
        ]
    );
    // possession_vector_description_maps_secrets_keys_and_proof_components_in_member_order
}

/// Contract: `fields_to_value` over a field list of the declared count and
///   kinds → `Ok` whose `described` is the record with each field moved from
///   its value.
/// Arrange:  `build_canonical_fields` with `values` overridden to
///   `Unsigned256([0x01; 32])`, `Unsigned256([0x02; 32])`,
///   `Bytes(vec![0x03; 64])`, `Bytes(vec![0x04; 128])`,
///   `Bytes(vec![0x05; 64])`, `Unsigned256([0x06; 32])`,
///   `Bytes(vec![0x07; 128])`, and `Unsigned256([0x08; 32])`, the description
///   from `build_possession_vector_description(Default::default())`.
/// Act:      `fields_to_value` with `FromFieldsParams`.
/// Assert:   `described` equals `build_possession_vector(Default::default())`.
/// Collaborators: none — the description is a leaf.
#[test]
fn possession_vector_description_rebuilds_the_record_from_its_fields() {
    // Arrange
    let fields = build_canonical_fields(CanonicalFieldsOverrides {
        values: Some(vec![
            CanonicalFieldValue::Unsigned256([0x01; 32]),
            CanonicalFieldValue::Unsigned256([0x02; 32]),
            CanonicalFieldValue::Bytes(vec![0x03; 64]),
            CanonicalFieldValue::Bytes(vec![0x04; 128]),
            CanonicalFieldValue::Bytes(vec![0x05; 64]),
            CanonicalFieldValue::Unsigned256([0x06; 32]),
            CanonicalFieldValue::Bytes(vec![0x07; 128]),
            CanonicalFieldValue::Unsigned256([0x08; 32]),
        ]),
    });
    let description = build_possession_vector_description(Default::default());

    // Act
    let success = description
        .fields_to_value(FromFieldsParams, fields)
        .expect("fields of the declared kinds rebuild the record");

    // Assert
    assert_eq!(
        success.described,
        build_possession_vector(Default::default())
    );
    // possession_vector_description_rebuilds_the_record_from_its_fields
}

/// Contract: `payload.values` does not convert into the declared count →
///   `Err(PossessionVectorFromFieldsErrorReturn::FieldCount { expected,
///   actual })`.
/// Arrange:  `build_canonical_fields` with `values` overridden to the values
///   `possession_vector_description_rebuilds_the_record_from_its_fields`
///   arranges, less the last, with the value at index `0` replaced by
///   `Bytes(vec![0x01; 32])`, so the count is wrong and a kind is wrong.
/// Act:      `fields_to_value` with `FromFieldsParams`.
/// Assert:   the error equals `PossessionVectorFromFieldsErrorReturn::
///   FieldCount { expected: 8, actual: 7 }` by `assert_eq!`, the count
///   refused before any kind.
/// Collaborators: none — the description is a leaf.
#[test]
fn possession_vector_description_refuses_a_list_one_field_short() {
    // Arrange
    let fields = build_canonical_fields(CanonicalFieldsOverrides {
        values: Some(vec![
            CanonicalFieldValue::Bytes(vec![0x01; 32]),
            CanonicalFieldValue::Unsigned256([0x02; 32]),
            CanonicalFieldValue::Bytes(vec![0x03; 64]),
            CanonicalFieldValue::Bytes(vec![0x04; 128]),
            CanonicalFieldValue::Bytes(vec![0x05; 64]),
            CanonicalFieldValue::Unsigned256([0x06; 32]),
            CanonicalFieldValue::Bytes(vec![0x07; 128]),
        ]),
    });
    let description = build_possession_vector_description(Default::default());

    // Act
    let error = description
        .fields_to_value(FromFieldsParams, fields)
        .err()
        .expect("a field list one short is refused");

    // Assert
    assert_eq!(
        error,
        PossessionVectorFromFieldsErrorReturn::FieldCount {
            expected: 8,
            actual: 7,
        }
    );
    // possession_vector_description_refuses_a_list_one_field_short
}

/// Contract: `payload.values` does not convert into the declared count →
///   `Err(PossessionVectorFromFieldsErrorReturn::FieldCount { expected,
///   actual })`.
/// Arrange:  `build_canonical_fields` with `values` overridden to the values
///   `possession_vector_description_rebuilds_the_record_from_its_fields`
///   arranges, followed by `Unsigned256([0x09; 32])`.
/// Act:      `fields_to_value` with `FromFieldsParams`.
/// Assert:   the error equals `PossessionVectorFromFieldsErrorReturn::
///   FieldCount { expected: 8, actual: 9 }` by `assert_eq!`.
/// Collaborators: none — the description is a leaf.
#[test]
fn possession_vector_description_refuses_a_list_one_field_long() {
    // Arrange
    let fields = build_canonical_fields(CanonicalFieldsOverrides {
        values: Some(vec![
            CanonicalFieldValue::Unsigned256([0x01; 32]),
            CanonicalFieldValue::Unsigned256([0x02; 32]),
            CanonicalFieldValue::Bytes(vec![0x03; 64]),
            CanonicalFieldValue::Bytes(vec![0x04; 128]),
            CanonicalFieldValue::Bytes(vec![0x05; 64]),
            CanonicalFieldValue::Unsigned256([0x06; 32]),
            CanonicalFieldValue::Bytes(vec![0x07; 128]),
            CanonicalFieldValue::Unsigned256([0x08; 32]),
            CanonicalFieldValue::Unsigned256([0x09; 32]),
        ]),
    });
    let description = build_possession_vector_description(Default::default());

    // Act
    let error = description
        .fields_to_value(FromFieldsParams, fields)
        .err()
        .expect("a field list one long is refused");

    // Assert
    assert_eq!(
        error,
        PossessionVectorFromFieldsErrorReturn::FieldCount {
            expected: 8,
            actual: 9,
        }
    );
    // possession_vector_description_refuses_a_list_one_field_long
}

/// Contract: the lowest index whose value is not the kind the description
///   holds there → `Err(PossessionVectorFromFieldsErrorReturn::FieldKind {
///   index, expected })`.
/// Arrange:  `build_canonical_fields` with `values` overridden to the values
///   `possession_vector_description_rebuilds_the_record_from_its_fields`
///   arranges, with the values at index `2` and index `5` replaced by
///   `Unsigned256([0x03; 32])` and `Bytes(vec![0x06; 64])`, so two kinds are
///   wrong.
/// Act:      `fields_to_value` with `FromFieldsParams`.
/// Assert:   the error equals `PossessionVectorFromFieldsErrorReturn::
///   FieldKind { index: 2, expected: CanonicalFieldKind::Bytes }` by
///   `assert_eq!`.
/// Collaborators: none — the description is a leaf.
#[test]
fn possession_vector_description_refuses_the_lowest_field_of_the_wrong_kind() {
    // Arrange
    let fields = build_canonical_fields(CanonicalFieldsOverrides {
        values: Some(vec![
            CanonicalFieldValue::Unsigned256([0x01; 32]),
            CanonicalFieldValue::Unsigned256([0x02; 32]),
            CanonicalFieldValue::Unsigned256([0x03; 32]),
            CanonicalFieldValue::Bytes(vec![0x04; 128]),
            CanonicalFieldValue::Bytes(vec![0x05; 64]),
            CanonicalFieldValue::Bytes(vec![0x06; 64]),
            CanonicalFieldValue::Bytes(vec![0x07; 128]),
            CanonicalFieldValue::Unsigned256([0x08; 32]),
        ]),
    });
    let description = build_possession_vector_description(Default::default());

    // Act
    let error = description
        .fields_to_value(FromFieldsParams, fields)
        .err()
        .expect("a field of the wrong kind is refused");

    // Assert
    assert_eq!(
        error,
        PossessionVectorFromFieldsErrorReturn::FieldKind {
            index: 2,
            expected: CanonicalFieldKind::Bytes,
        }
    );
    // possession_vector_description_refuses_the_lowest_field_of_the_wrong_kind
}

/// Contract: `members()` → `Ok` of the members in member order, each admitted
///   name paired with the kind the description holds at its index.
/// Arrange:  `build_possession_vector_description(Default::default())`.
/// Act:      `members()`.
/// Assert:   the result equals `Ok` of `build_solidity_record_members` over
///   `x` and `y` of kind `CanonicalFieldKind::Unsigned256`, `pk1`, `pk2`, and
///   `r1` of kind `CanonicalFieldKind::Bytes`, `z1` of kind
///   `CanonicalFieldKind::Unsigned256`, `r2` of kind
///   `CanonicalFieldKind::Bytes`, and `z2` of kind
///   `CanonicalFieldKind::Unsigned256`, in that order, each member through
///   `build_solidity_record_member` with its name from
///   `build_solidity_member_name` stated as the literal text `x`, `y`,
///   `pk1`, `pk2`, `r1`, `z1`, `r2`, and `z2`.
/// Collaborators: none — the description is a leaf.
#[test]
fn possession_vector_description_declares_its_members() {
    // Arrange
    let description = build_possession_vector_description(Default::default());
    let expected = build_solidity_record_members(SolidityRecordMembersConstructorParamsOverrides {
        members: Some(vec![
            build_solidity_record_member(SolidityRecordMemberOverrides {
                name: Some(build_solidity_member_name(
                    SolidityMemberNameConstructorParamsOverrides {
                        text: Some("x".to_string()),
                    },
                )),
                kind: Some(CanonicalFieldKind::Unsigned256),
            }),
            build_solidity_record_member(SolidityRecordMemberOverrides {
                name: Some(build_solidity_member_name(
                    SolidityMemberNameConstructorParamsOverrides {
                        text: Some("y".to_string()),
                    },
                )),
                kind: Some(CanonicalFieldKind::Unsigned256),
            }),
            build_solidity_record_member(SolidityRecordMemberOverrides {
                name: Some(build_solidity_member_name(
                    SolidityMemberNameConstructorParamsOverrides {
                        text: Some("pk1".to_string()),
                    },
                )),
                kind: Some(CanonicalFieldKind::Bytes),
            }),
            build_solidity_record_member(SolidityRecordMemberOverrides {
                name: Some(build_solidity_member_name(
                    SolidityMemberNameConstructorParamsOverrides {
                        text: Some("pk2".to_string()),
                    },
                )),
                kind: Some(CanonicalFieldKind::Bytes),
            }),
            build_solidity_record_member(SolidityRecordMemberOverrides {
                name: Some(build_solidity_member_name(
                    SolidityMemberNameConstructorParamsOverrides {
                        text: Some("r1".to_string()),
                    },
                )),
                kind: Some(CanonicalFieldKind::Bytes),
            }),
            build_solidity_record_member(SolidityRecordMemberOverrides {
                name: Some(build_solidity_member_name(
                    SolidityMemberNameConstructorParamsOverrides {
                        text: Some("z1".to_string()),
                    },
                )),
                kind: Some(CanonicalFieldKind::Unsigned256),
            }),
            build_solidity_record_member(SolidityRecordMemberOverrides {
                name: Some(build_solidity_member_name(
                    SolidityMemberNameConstructorParamsOverrides {
                        text: Some("r2".to_string()),
                    },
                )),
                kind: Some(CanonicalFieldKind::Bytes),
            }),
            build_solidity_record_member(SolidityRecordMemberOverrides {
                name: Some(build_solidity_member_name(
                    SolidityMemberNameConstructorParamsOverrides {
                        text: Some("z2".to_string()),
                    },
                )),
                kind: Some(CanonicalFieldKind::Unsigned256),
            }),
        ]),
    });

    // Act
    let result = description.members();

    // Assert
    assert_eq!(result, Ok(expected));
    // possession_vector_description_declares_its_members
}
