#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use encoding::{
    CanonicalFieldReaderErrorReturn, CanonicalFieldValue, CanonicalFieldsOverrides,
    FromFieldsParams, IDecoderAdapter, IEncoderAdapter, IEncodingContract, ToFieldsParams,
    build_canonical_fields,
};
use envelope::{
    CreateKeyAgreementParamsOverrides, GenerateKeysErrorReturn, IKeyAgreementAdapter,
    KeyAgreementConcrete, MockIKeyAgreementAdapterFailureMode,
    MockIKeyAgreementAdapterGenerateKeysErrorReturn, build_create_key_agreement_params,
};
use hash_to_scalar::IHashToScalarAdapter;
use pairing::{IPairingArithmetic, IPairingReference};
use random::{
    CreateRandomSourceParamsOverrides, FillBytesErrorReturn, IRandomSourceAdapter,
    MockIRandomSourceAdapterFailureMode, RandomSourceKind, build_create_random_source_params,
};

use super::super::group_vectors::provides::{
    VectorCountConstructorParamsOverrides, build_vector_count,
};
use super::super::render::provides::{
    SolidityMemberNameConstructorParamsOverrides, build_solidity_member_name,
};
use super::interface::{
    IPossessionVectorsCaseConsumer, POSSESSION_VECTOR_FIELD_KINDS, PossessionVectorDescription,
    PossessionVectorFromFieldsErrorReturn, PossessionVectorsErrorReturn, PossessionVectorsReturn,
};
use super::mock::{
    PossessionVectorsCaseParamsOverrides, PossessionVectorsParamsOverrides,
    build_possession_vector, build_possession_vector_description,
    build_possession_vector_description_constructor_params, build_possession_vectors_case_deps,
    build_possession_vectors_case_params, build_possession_vectors_case_payload,
    build_possession_vectors_deps, build_possession_vectors_params,
    build_possession_vectors_payload, run_possession_vectors_case,
};
use super::possession_vectors;

/// Contract: `deps.random.fill_bytes` returns `Err(error)` on a draw →
///   `Err(PossessionVectorsErrorReturn::FillBytes(error))`, the error unchanged.
/// Arrange: the case's `random_source` selection `build_create_random_source_params`
///   with `kind` `RandomSourceKind::Mock(MockIRandomSourceAdapterFailureMode::
///   FillBytesRefused)`, which differs from the builder's default of `Succeeds`,
///   every other selection at its default.
/// Act:    `possession_vectors` in the case consumer's method.
/// Assert: the result equals `Err(PossessionVectorsErrorReturn::FillBytes(
///   FillBytesErrorReturn::MockIRandomSourceAdapter))` by `assert_eq!`.
#[test]
fn possession_vectors_return_the_draw_refusal_unchanged() {
    // Arrange
    struct Consumer;

    impl IPossessionVectorsCaseConsumer for Consumer {
        type Output = PossessionVectorsReturn;

        fn consume_possession_vectors_case<P, H, E, K>(
            &self,
            pairing: &P,
            _hash_to_scalar: &H,
            _encoder: &E,
            _decoder: &E,
            key_agreement: &K,
            random: &dyn IRandomSourceAdapter,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            H: ?Sized + IHashToScalarAdapter<P::Scalar>,
            E: IEncoderAdapter + IDecoderAdapter,
            K: IKeyAgreementAdapter<Pairing = P>,
        {
            possession_vectors::<P, K>(
                &build_possession_vectors_deps(pairing, key_agreement, random, Default::default()),
                build_possession_vectors_params(Default::default()),
                build_possession_vectors_payload(Default::default()),
            )
        }
    }

    // Act
    let output = run_possession_vectors_case(
        &build_possession_vectors_case_deps(Consumer, Default::default()),
        build_possession_vectors_case_params(PossessionVectorsCaseParamsOverrides {
            random_source: Some(build_create_random_source_params(
                CreateRandomSourceParamsOverrides {
                    kind: Some(RandomSourceKind::Mock(
                        MockIRandomSourceAdapterFailureMode::FillBytesRefused,
                    )),
                },
            )),
            ..Default::default()
        }),
        build_possession_vectors_case_payload(Default::default()),
    )
    .expect("the case runs")
    .output;

    // Assert
    assert_eq!(
        output,
        Err(PossessionVectorsErrorReturn::FillBytes(
            FillBytesErrorReturn::MockIRandomSourceAdapter
        ))
    );
}
// possession_vectors_return_the_draw_refusal_unchanged

/// Contract: `deps.key_agreement.generate_keys` returns `Err(error)` on a record →
///   `Err(PossessionVectorsErrorReturn::GenerateKeys(error))`, the error unchanged.
/// Arrange: the case's `key_agreement` selection `build_create_key_agreement_params`
///   with `concrete` `KeyAgreementConcrete::Mock(MockIKeyAgreementAdapterFailureMode::
///   NonceDrawRefused)`, which differs from the builder's default of `NoFailure`,
///   the random selection at its default so both draws succeed.
/// Act:    `possession_vectors` in the case consumer's method.
/// Assert: the result equals `Err(PossessionVectorsErrorReturn::GenerateKeys(
///   GenerateKeysErrorReturn::Mock(MockIKeyAgreementAdapterGenerateKeysErrorReturn::
///   NonceDrawRefused)))` by `assert_eq!`.
#[test]
fn possession_vectors_return_the_generation_refusal_unchanged() {
    // Arrange
    struct Consumer;

    impl IPossessionVectorsCaseConsumer for Consumer {
        type Output = PossessionVectorsReturn;

        fn consume_possession_vectors_case<P, H, E, K>(
            &self,
            pairing: &P,
            _hash_to_scalar: &H,
            _encoder: &E,
            _decoder: &E,
            key_agreement: &K,
            random: &dyn IRandomSourceAdapter,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            H: ?Sized + IHashToScalarAdapter<P::Scalar>,
            E: IEncoderAdapter + IDecoderAdapter,
            K: IKeyAgreementAdapter<Pairing = P>,
        {
            possession_vectors::<P, K>(
                &build_possession_vectors_deps(pairing, key_agreement, random, Default::default()),
                build_possession_vectors_params(Default::default()),
                build_possession_vectors_payload(Default::default()),
            )
        }
    }

    // Act
    let output = run_possession_vectors_case(
        &build_possession_vectors_case_deps(Consumer, Default::default()),
        build_possession_vectors_case_params(PossessionVectorsCaseParamsOverrides {
            key_agreement: Some(build_create_key_agreement_params(
                CreateKeyAgreementParamsOverrides {
                    concrete: Some(KeyAgreementConcrete::Mock(
                        MockIKeyAgreementAdapterFailureMode::NonceDrawRefused,
                    )),
                    ..Default::default()
                },
            )),
            ..Default::default()
        }),
        build_possession_vectors_case_payload(Default::default()),
    )
    .expect("the case runs")
    .output;

    // Assert
    assert_eq!(
        output,
        Err(PossessionVectorsErrorReturn::GenerateKeys(
            GenerateKeysErrorReturn::Mock(
                MockIKeyAgreementAdapterGenerateKeysErrorReturn::NonceDrawRefused
            )
        ))
    );
}
// possession_vectors_return_the_generation_refusal_unchanged

/// Contract: every record generated → `Ok` whose `records` hold the count the
///   params carry.
/// Arrange: `count` of `3` through `build_vector_count`, a count other than the
///   params builder's default `2`, every selection at its default.
/// Act:    `possession_vectors` in the case consumer's method.
/// Assert: the success's `records` length equals `3` by `assert_eq!`, so a
///   function that emits the default count or one record fails.
#[test]
fn possession_vectors_emit_the_configured_count_of_records() {
    // Arrange
    struct Consumer {
        params_overrides: PossessionVectorsParamsOverrides,
    }

    impl IPossessionVectorsCaseConsumer for Consumer {
        type Output = PossessionVectorsReturn;

        fn consume_possession_vectors_case<P, H, E, K>(
            &self,
            pairing: &P,
            _hash_to_scalar: &H,
            _encoder: &E,
            _decoder: &E,
            key_agreement: &K,
            random: &dyn IRandomSourceAdapter,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            H: ?Sized + IHashToScalarAdapter<P::Scalar>,
            E: IEncoderAdapter + IDecoderAdapter,
            K: IKeyAgreementAdapter<Pairing = P>,
        {
            possession_vectors::<P, K>(
                &build_possession_vectors_deps(pairing, key_agreement, random, Default::default()),
                build_possession_vectors_params(PossessionVectorsParamsOverrides {
                    count: self.params_overrides.count,
                }),
                build_possession_vectors_payload(Default::default()),
            )
        }
    }

    let consumer = Consumer {
        params_overrides: PossessionVectorsParamsOverrides {
            count: Some(build_vector_count(VectorCountConstructorParamsOverrides {
                count: Some(3),
            })),
        },
    };

    // Act
    let output = run_possession_vectors_case(
        &build_possession_vectors_case_deps(consumer, Default::default()),
        build_possession_vectors_case_params(Default::default()),
        build_possession_vectors_case_payload(Default::default()),
    )
    .expect("the case runs")
    .output;

    // Assert
    let success = output.expect("the generation succeeds");
    assert_eq!(success.records.len(), 3);
}
// possession_vectors_emit_the_configured_count_of_records

/// Contract: each record's two draws are the next two draws of the random
///   source → no two records share an `x`, no two share a `y`, and a record's
///   `x` differs from its `y`.
/// Arrange: `count` of `3`, the random selection at its default, the family's
///   mock concrete, whose draws differ from call to call.
/// Act:    `possession_vectors` in the case consumer's method.
/// Assert: every pair of records differs in `x` and in `y`, and each record's
///   `x` differs from its `y`, so a function that reuses a draw or a record
///   fails.
#[test]
fn possession_vectors_give_each_record_its_own_secrets() {
    // Arrange
    struct Consumer {
        params_overrides: PossessionVectorsParamsOverrides,
    }

    impl IPossessionVectorsCaseConsumer for Consumer {
        type Output = PossessionVectorsReturn;

        fn consume_possession_vectors_case<P, H, E, K>(
            &self,
            pairing: &P,
            _hash_to_scalar: &H,
            _encoder: &E,
            _decoder: &E,
            key_agreement: &K,
            random: &dyn IRandomSourceAdapter,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            H: ?Sized + IHashToScalarAdapter<P::Scalar>,
            E: IEncoderAdapter + IDecoderAdapter,
            K: IKeyAgreementAdapter<Pairing = P>,
        {
            possession_vectors::<P, K>(
                &build_possession_vectors_deps(pairing, key_agreement, random, Default::default()),
                build_possession_vectors_params(PossessionVectorsParamsOverrides {
                    count: self.params_overrides.count,
                }),
                build_possession_vectors_payload(Default::default()),
            )
        }
    }

    let consumer = Consumer {
        params_overrides: PossessionVectorsParamsOverrides {
            count: Some(build_vector_count(VectorCountConstructorParamsOverrides {
                count: Some(3),
            })),
        },
    };

    // Act
    let output = run_possession_vectors_case(
        &build_possession_vectors_case_deps(consumer, Default::default()),
        build_possession_vectors_case_params(Default::default()),
        build_possession_vectors_case_payload(Default::default()),
    )
    .expect("the case runs")
    .output;

    // Assert
    let success = output.expect("the generation succeeds");
    for first in 0..success.records.len() {
        for second in (first + 1)..success.records.len() {
            assert_ne!(success.records[first].x, success.records[second].x);
            assert_ne!(success.records[first].y, success.records[second].y);
        }
        assert_ne!(success.records[first].x, success.records[first].y);
    }
}
// possession_vectors_give_each_record_its_own_secrets

/// Contract: `x`, `y`, `z1`, and `z2` are the scalar encodings of the key
///   pair's secrets and the possession's responses → each is the scalar's 32
///   big-endian bytes.
/// Arrange: `count` of `3`, every selection at its default, the pairing the
///   family's mock concrete, whose scalars are residues below `65521` encoded
///   as 32 big-endian bytes with the first twenty-four zero.
/// Act:    `possession_vectors` in the case consumer's method.
/// Assert: for every record each of `x`, `y`, `z1`, and `z2` has its first
///   twenty-four bytes zero and its last eight bytes, read as a big-endian
///   `u64`, below the literal `65521`, so a little-endian, truncated, or
///   padded-on-the-wrong-side encoding fails.
#[test]
fn possession_vectors_carry_their_scalar_fields_as_32_byte_encodings() {
    // Arrange
    struct Consumer {
        params_overrides: PossessionVectorsParamsOverrides,
    }

    impl IPossessionVectorsCaseConsumer for Consumer {
        type Output = PossessionVectorsReturn;

        fn consume_possession_vectors_case<P, H, E, K>(
            &self,
            pairing: &P,
            _hash_to_scalar: &H,
            _encoder: &E,
            _decoder: &E,
            key_agreement: &K,
            random: &dyn IRandomSourceAdapter,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            H: ?Sized + IHashToScalarAdapter<P::Scalar>,
            E: IEncoderAdapter + IDecoderAdapter,
            K: IKeyAgreementAdapter<Pairing = P>,
        {
            possession_vectors::<P, K>(
                &build_possession_vectors_deps(pairing, key_agreement, random, Default::default()),
                build_possession_vectors_params(PossessionVectorsParamsOverrides {
                    count: self.params_overrides.count,
                }),
                build_possession_vectors_payload(Default::default()),
            )
        }
    }

    let consumer = Consumer {
        params_overrides: PossessionVectorsParamsOverrides {
            count: Some(build_vector_count(VectorCountConstructorParamsOverrides {
                count: Some(3),
            })),
        },
    };

    // Act
    let output = run_possession_vectors_case(
        &build_possession_vectors_case_deps(consumer, Default::default()),
        build_possession_vectors_case_params(Default::default()),
        build_possession_vectors_case_payload(Default::default()),
    )
    .expect("the case runs")
    .output;

    // Assert
    let success = output.expect("the generation succeeds");
    for record in &success.records {
        for scalar in [record.x, record.y, record.z1, record.z2] {
            assert!(scalar[..24].iter().all(|byte| *byte == 0));
            let tail: [u8; 8] = scalar[24..].try_into().expect("eight tail bytes");
            assert!(u64::from_be_bytes(tail) < 65521);
        }
    }
}
// possession_vectors_carry_their_scalar_fields_as_32_byte_encodings

/// Contract: `pk1` is the `encode_g1` bytes of the first-group generator
///   times the key pair's `x` → the record's `pk1` read as a big-endian
///   `u64` equals its `x` read as a big-endian `u64`, the mock concrete's
///   first-group generator being residue `1` and its multiplication a product
///   modulo `65521`.
/// Arrange: `count` of `3`, every selection at its default.
/// Act:    `possession_vectors` in the case consumer's method.
/// Assert: for every record `pk1` is eight bytes and its `u64` equals the
///   `u64` of the last eight bytes of `x`, so a `pk1` formed from `y`, or
///   swapped with `pk2`, fails.
#[test]
fn possession_vectors_carry_the_first_group_public_key_the_secret_forms() {
    // Arrange
    struct Consumer {
        params_overrides: PossessionVectorsParamsOverrides,
    }

    impl IPossessionVectorsCaseConsumer for Consumer {
        type Output = PossessionVectorsReturn;

        fn consume_possession_vectors_case<P, H, E, K>(
            &self,
            pairing: &P,
            _hash_to_scalar: &H,
            _encoder: &E,
            _decoder: &E,
            key_agreement: &K,
            random: &dyn IRandomSourceAdapter,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            H: ?Sized + IHashToScalarAdapter<P::Scalar>,
            E: IEncoderAdapter + IDecoderAdapter,
            K: IKeyAgreementAdapter<Pairing = P>,
        {
            possession_vectors::<P, K>(
                &build_possession_vectors_deps(pairing, key_agreement, random, Default::default()),
                build_possession_vectors_params(PossessionVectorsParamsOverrides {
                    count: self.params_overrides.count,
                }),
                build_possession_vectors_payload(Default::default()),
            )
        }
    }

    let consumer = Consumer {
        params_overrides: PossessionVectorsParamsOverrides {
            count: Some(build_vector_count(VectorCountConstructorParamsOverrides {
                count: Some(3),
            })),
        },
    };

    // Act
    let output = run_possession_vectors_case(
        &build_possession_vectors_case_deps(consumer, Default::default()),
        build_possession_vectors_case_params(Default::default()),
        build_possession_vectors_case_payload(Default::default()),
    )
    .expect("the case runs")
    .output;

    // Assert
    let success = output.expect("the generation succeeds");
    for record in &success.records {
        let pk1: [u8; 8] = record.pk1[..].try_into().expect("pk1 is eight bytes");
        let x: [u8; 8] = record.x[24..].try_into().expect("eight tail bytes");
        assert_eq!(u64::from_be_bytes(pk1), u64::from_be_bytes(x));
    }
}
// possession_vectors_carry_the_first_group_public_key_the_secret_forms

/// Contract: `pk2` is the `encode_g2` bytes of the second-group generator
///   times the key pair's `y` → the record's `pk2` read as a big-endian
///   `u64` equals twice its `y` read as a big-endian `u64`, the mock
///   concrete's second-group generator being residue `2` and its
///   multiplication a product modulo `2 * 65521`.
/// Arrange: `count` of `3`, every selection at its default.
/// Act:    `possession_vectors` in the case consumer's method.
/// Assert: for every record `pk2` is eight bytes and its `u64` equals twice
///   the `u64` of the last eight bytes of `y`, so a `pk2` formed from `x`, or
///   swapped with `pk1`, fails.
#[test]
fn possession_vectors_carry_the_second_group_public_key_the_secret_forms() {
    // Arrange
    struct Consumer {
        params_overrides: PossessionVectorsParamsOverrides,
    }

    impl IPossessionVectorsCaseConsumer for Consumer {
        type Output = PossessionVectorsReturn;

        fn consume_possession_vectors_case<P, H, E, K>(
            &self,
            pairing: &P,
            _hash_to_scalar: &H,
            _encoder: &E,
            _decoder: &E,
            key_agreement: &K,
            random: &dyn IRandomSourceAdapter,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            H: ?Sized + IHashToScalarAdapter<P::Scalar>,
            E: IEncoderAdapter + IDecoderAdapter,
            K: IKeyAgreementAdapter<Pairing = P>,
        {
            possession_vectors::<P, K>(
                &build_possession_vectors_deps(pairing, key_agreement, random, Default::default()),
                build_possession_vectors_params(PossessionVectorsParamsOverrides {
                    count: self.params_overrides.count,
                }),
                build_possession_vectors_payload(Default::default()),
            )
        }
    }

    let consumer = Consumer {
        params_overrides: PossessionVectorsParamsOverrides {
            count: Some(build_vector_count(VectorCountConstructorParamsOverrides {
                count: Some(3),
            })),
        },
    };

    // Act
    let output = run_possession_vectors_case(
        &build_possession_vectors_case_deps(consumer, Default::default()),
        build_possession_vectors_case_params(Default::default()),
        build_possession_vectors_case_payload(Default::default()),
    )
    .expect("the case runs")
    .output;

    // Assert
    let success = output.expect("the generation succeeds");
    for record in &success.records {
        let pk2: [u8; 8] = record.pk2[..].try_into().expect("pk2 is eight bytes");
        let y: [u8; 8] = record.y[24..].try_into().expect("eight tail bytes");
        assert_eq!(u64::from_be_bytes(pk2), 2 * u64::from_be_bytes(y));
    }
}
// possession_vectors_carry_the_second_group_public_key_the_secret_forms

/// Contract: `r1` is the `encode_g1` bytes and `r2` the `encode_g2` bytes of
///   the proofs' commitments → each is the eight big-endian bytes of a
///   residue of its group.
/// Arrange: `count` of `3`, every selection at its default.
/// Act:    `possession_vectors` in the case consumer's method.
/// Assert: for every record `r1` is eight bytes whose `u64` is below the
///   literal `65521` and `r2` is eight bytes whose `u64` is even and below
///   the literal `131042`, the second group's subgroup being the even
///   residues, so a commitment encoded in the other group fails.
#[test]
fn possession_vectors_carry_each_commitment_in_its_groups_encoding() {
    // Arrange
    struct Consumer {
        params_overrides: PossessionVectorsParamsOverrides,
    }

    impl IPossessionVectorsCaseConsumer for Consumer {
        type Output = PossessionVectorsReturn;

        fn consume_possession_vectors_case<P, H, E, K>(
            &self,
            pairing: &P,
            _hash_to_scalar: &H,
            _encoder: &E,
            _decoder: &E,
            key_agreement: &K,
            random: &dyn IRandomSourceAdapter,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            H: ?Sized + IHashToScalarAdapter<P::Scalar>,
            E: IEncoderAdapter + IDecoderAdapter,
            K: IKeyAgreementAdapter<Pairing = P>,
        {
            possession_vectors::<P, K>(
                &build_possession_vectors_deps(pairing, key_agreement, random, Default::default()),
                build_possession_vectors_params(PossessionVectorsParamsOverrides {
                    count: self.params_overrides.count,
                }),
                build_possession_vectors_payload(Default::default()),
            )
        }
    }

    let consumer = Consumer {
        params_overrides: PossessionVectorsParamsOverrides {
            count: Some(build_vector_count(VectorCountConstructorParamsOverrides {
                count: Some(3),
            })),
        },
    };

    // Act
    let output = run_possession_vectors_case(
        &build_possession_vectors_case_deps(consumer, Default::default()),
        build_possession_vectors_case_params(Default::default()),
        build_possession_vectors_case_payload(Default::default()),
    )
    .expect("the case runs")
    .output;

    // Assert
    let success = output.expect("the generation succeeds");
    for record in &success.records {
        let r1: [u8; 8] = record.r1[..].try_into().expect("r1 is eight bytes");
        assert!(u64::from_be_bytes(r1) < 65521);
        let r2: [u8; 8] = record.r2[..].try_into().expect("r2 is eight bytes");
        let commitment = u64::from_be_bytes(r2);
        assert!(commitment % 2 == 0);
        assert!(commitment < 131042);
    }
}
// possession_vectors_carry_each_commitment_in_its_groups_encoding

/// Contract: `try_new` over the fieldless params → `Ok(PossessionVectorDescription)`.
/// Arrange: `build_possession_vector_description_constructor_params(Default::default())`.
/// Act:    `PossessionVectorDescription::try_new`.
/// Assert: `matches!(result, Ok(PossessionVectorDescription))`.
#[test]
fn possession_vector_description_admits_its_fieldless_constructor_params() {
    // Arrange

    // Act
    let result = PossessionVectorDescription::try_new(
        build_possession_vector_description_constructor_params(Default::default()),
    );

    // Assert
    assert!(matches!(result, Ok(PossessionVectorDescription)));
}
// possession_vector_description_admits_its_fieldless_constructor_params

/// Contract: `to_fields` → `Ok(ToFieldsSuccessReturn { fields })` whose `values` are,
///   in member order, `Unsigned256` of `x` and of `y`, `Bytes` of `pk1`, `pk2`,
///   and `r1`, `Unsigned256` of `z1`, `Bytes` of `r2`, and `Unsigned256` of `z2`.
/// Arrange: `build_possession_vector(Default::default())` and
///   `build_possession_vector_description(Default::default())`.
/// Act:    `to_fields` over `ToFieldsParams` and the record.
/// Assert: `fields` equals `build_canonical_fields` with `values`
///   `Unsigned256([0x01; 32])`, `Unsigned256([0x02; 32])`,
///   `Bytes(vec![0x03; 64])`, `Bytes(vec![0x04; 128])`, `Bytes(vec![0x05; 64])`,
///   `Unsigned256([0x06; 32])`, `Bytes(vec![0x07; 128])`, and
///   `Unsigned256([0x08; 32])` in that order, the record's fields differing
///   from each other so a swap of two fields fails.
#[test]
fn possession_vector_description_maps_secrets_keys_and_proof_components_in_member_order() {
    // Arrange
    let vector = build_possession_vector(Default::default());
    let description = build_possession_vector_description(Default::default());

    // Act
    let fields = description
        .to_fields(ToFieldsParams, &vector)
        .expect("the description maps the record")
        .fields;

    // Assert
    assert_eq!(
        fields,
        build_canonical_fields(CanonicalFieldsOverrides {
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
        })
    );
}
// possession_vector_description_maps_secrets_keys_and_proof_components_in_member_order

/// Contract: the reader admits every field → `Ok(FromFieldsSuccessReturn {
///   described: PossessionVector { x, y, pk1, pk2, r1, z1, r2, z2 } })`, each
///   the value its read returned.
/// Arrange: `build_canonical_fields` with `values` `Unsigned256([0x01; 32])`,
///   `Unsigned256([0x02; 32])`, `Bytes(vec![0x03; 64])`, `Bytes(vec![0x04; 128])`,
///   `Bytes(vec![0x05; 64])`, `Unsigned256([0x06; 32])`, `Bytes(vec![0x07; 128])`,
///   and `Unsigned256([0x08; 32])`, and
///   `build_possession_vector_description(Default::default())`.
/// Act:    `fields_to_value` over `FromFieldsParams`.
/// Assert: `described` equals `build_possession_vector(Default::default())`.
#[test]
fn possession_vector_description_rebuilds_the_record_from_its_fields() {
    // Arrange
    let payload = build_canonical_fields(CanonicalFieldsOverrides {
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
    let described = description
        .fields_to_value(FromFieldsParams, payload)
        .expect("the fields rebuild the record")
        .described;

    // Assert
    assert_eq!(described, build_possession_vector(Default::default()));
}
// possession_vector_description_rebuilds_the_record_from_its_fields

/// Contract: a refusal of `CanonicalFieldReader::try_new` or of any read →
///   `Err(PossessionVectorFromFieldsErrorReturn::Fields(error))`, the error
///   unchanged.
/// Arrange: `build_canonical_fields` with `values` the values
///   `possession_vector_description_rebuilds_the_record_from_its_fields`
///   arranges, less the last.
/// Act:    `fields_to_value`.
/// Assert: the error equals `PossessionVectorFromFieldsErrorReturn::Fields(
///   CanonicalFieldReaderErrorReturn::Count { expected: 8, actual: 7 })` by
///   `assert_eq!`.
#[test]
fn possession_vector_description_carries_the_readers_refusal() {
    // Arrange
    let payload = build_canonical_fields(CanonicalFieldsOverrides {
        values: Some(vec![
            CanonicalFieldValue::Unsigned256([0x01; 32]),
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
    let result = description.fields_to_value(FromFieldsParams, payload);

    // Assert
    assert_eq!(
        result.err(),
        Some(PossessionVectorFromFieldsErrorReturn::Fields(
            CanonicalFieldReaderErrorReturn::Count {
                expected: 8,
                actual: 7,
            }
        ))
    );
}
// possession_vector_description_carries_the_readers_refusal

/// Contract: `member_names()` → `Ok` of the admitted names in member order.
/// Arrange: `build_possession_vector_description(Default::default())`.
/// Act:    `member_names()`.
/// Assert: the result equals `Ok` of the names `x`, `y`, `pk1`, `pk2`, `r1`,
///   `z1`, `r2`, and `z2`, in that order, each through
///   `build_solidity_member_name` stated as the literal text, and its length
///   equals the length of `POSSESSION_VECTOR_FIELD_KINDS`.
#[test]
fn possession_vector_description_declares_its_member_names() {
    // Arrange
    let description = build_possession_vector_description(Default::default());

    // Act
    let result = description.member_names();

    // Assert
    assert_eq!(
        result,
        Ok(vec![
            build_solidity_member_name(SolidityMemberNameConstructorParamsOverrides {
                text: Some("x".to_string()),
            }),
            build_solidity_member_name(SolidityMemberNameConstructorParamsOverrides {
                text: Some("y".to_string()),
            }),
            build_solidity_member_name(SolidityMemberNameConstructorParamsOverrides {
                text: Some("pk1".to_string()),
            }),
            build_solidity_member_name(SolidityMemberNameConstructorParamsOverrides {
                text: Some("pk2".to_string()),
            }),
            build_solidity_member_name(SolidityMemberNameConstructorParamsOverrides {
                text: Some("r1".to_string()),
            }),
            build_solidity_member_name(SolidityMemberNameConstructorParamsOverrides {
                text: Some("z1".to_string()),
            }),
            build_solidity_member_name(SolidityMemberNameConstructorParamsOverrides {
                text: Some("r2".to_string()),
            }),
            build_solidity_member_name(SolidityMemberNameConstructorParamsOverrides {
                text: Some("z2".to_string()),
            }),
        ])
    );
    let names = result.expect("the names are admitted");
    assert_eq!(names.len(), POSSESSION_VECTOR_FIELD_KINDS.len());
}
// possession_vector_description_declares_its_member_names
