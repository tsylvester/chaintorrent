#![cfg(feature = "mocks")]
#![allow(clippy::expect_used)]

use domain::{
    GroupIndexConstructorParamsOverrides, PieceGeometryConstructorParamsOverrides,
    build_group_index, build_piece_geometry,
};
use encoding::{
    CreateEncodingParamsOverrides, DECLARED_ENCODING_SELECTIONS, DecodeErrorReturn,
    EncodingIdentifier, build_create_encoding_deps, build_create_encoding_params,
    build_create_encoding_payload, build_mock_i_encoding_consumer, create_encoding,
};
use workflows::{
    DerivationContext, DerivationContextConstructorParamsOverrides,
    DerivationContextFromFieldsErrorReturn, DerivationContextTryNewErrorReturn,
    build_derivation_context, build_derivation_context_constructor_params,
    build_derivation_context_description,
};

const REFERENCE_VECTOR_HEX: &str = concat!(
    "0101010101010101010101010101010101010101010101010101010101010101",
    "1111111111111111111111111111111111111111111111111111111111111111",
    "2222222222222222222222222222222222222222222222222222222222222222",
    "0000000000000000000000000000000000000000000000000000000000000001",
    "3333333333333333333333333333333333333333333333333333333333333333",
    "0000000000000000000000000000000000000000000000000000000000000007",
    "0000000000000000000000000000000000000000000000000000000000004000",
    "0000000000000000000000000000000000000000000000000000000000008000",
    "0000000000000000000000000000000000000000000000000000000000100000",
);

const REFUSAL_VECTOR_HEX: &str = concat!(
    "0101010101010101010101010101010101010101010101010101010101010101",
    "1111111111111111111111111111111111111111111111111111111111111111",
    "2222222222222222222222222222222222222222222222222222222222222222",
    "0000000000000000000000000000000000000000000000000000000000000001",
    "3333333333333333333333333333333333333333333333333333333333333333",
    "0000000000000000000000000000000000000000000000000000000000000020",
    "0000000000000000000000000000000000000000000000000000000000004000",
    "0000000000000000000000000000000000000000000000000000000000008000",
    "0000000000000000000000000000000000000000000000000000000000100000",
);

fn reference_context() -> DerivationContext {
    build_derivation_context(DerivationContextConstructorParamsOverrides {
        geometry: Some(build_piece_geometry(
            PieceGeometryConstructorParamsOverrides {
                piece_group_size: Some(32768),
                ..Default::default()
            },
        )),
        ..Default::default()
    })
}

fn reference_vector() -> Vec<u8> {
    hex::decode(REFERENCE_VECTOR_HEX).expect("the reference vector is hexadecimal")
}

fn refusal_vector() -> Vec<u8> {
    hex::decode(REFUSAL_VECTOR_HEX).expect("the refusal vector is hexadecimal")
}

/// Contract: the own entry `every_concrete_decodes_the_derivation_context_it_encodes`:
///   for each case of the declared set, the concrete `create_encoding` hands the
///   consumer encodes the reference context under `DerivationContextDescription`
///   and decodes the bytes it wrote → the decoded context equals the reference
///   context.
/// Arrange: the consumer over the description, the reference context, and no
///   bytes; every case of the declared set in turn; the reference context's nine
///   values are distinct within each kind.
/// Act:     `create_encoding(&deps, params, payload)` per case.
/// Assert:  per case, the output is present, and its `round_trip` mapped to its
///   `described` equals `Ok` of the reference context by `assert_eq!`.
/// Boundary: the public `create_encoding`; the real chain is `create_encoding`,
///   the selected concrete's constructor, `consume_encoding`, `encode`,
///   `to_fields`, `decode`, `fields_to_value`, the components' constructors, and
///   `DerivationContext::try_new`.
/// Mocked:  the consumer, replaced by `MockIEncodingConsumer`, which calls the
///   concrete's `encode` and `decode` and relays their returns; the concretes,
///   the description, and the components run real.
#[test]
fn every_concrete_decodes_the_derivation_context_it_encodes() {
    // Arrange
    for case in DECLARED_ENCODING_SELECTIONS {
        let description = build_derivation_context_description(Default::default());
        let consumer =
            build_mock_i_encoding_consumer(&description, reference_context(), Vec::new());
        let deps = build_create_encoding_deps(consumer);
        let params = build_create_encoding_params(CreateEncodingParamsOverrides {
            concrete: Some(case.concrete),
            identifier: Some(case.identifier),
        });
        let payload = build_create_encoding_payload(Default::default());

        // Act
        let result = create_encoding(&deps, params, payload);

        // Assert
        let output = result
            .ok()
            .map(|success| success.output)
            .expect("the case's concrete is admitted under the identifier it declares");
        assert_eq!(
            output.round_trip.map(|success| success.described),
            Ok(reference_context())
        );
    }
}
// every_concrete_decodes_the_derivation_context_it_encodes

/// Contract: the own entry
///   `every_concrete_declaring_the_ethereum_abi_identifier_encodes_the_reference_context_to_the_reference_vector`:
///   for each case of the declared set whose concrete declares
///   `EncodingIdentifier::EthereumAbiV1`, the concrete encodes the reference
///   context under `DerivationContextDescription` → the bytes equal the
///   reference vector, nine 32-byte words.
/// Arrange: the consumer over the description, the reference context, and no
///   bytes; every case of the declared set runs and the Ethereum ABI outputs
///   are kept; the reference context's piece size and piece-group size differ.
/// Act:     `create_encoding(&deps, params, payload)` per case.
/// Assert:  at least one output is kept; each kept output's `encoded` mapped
///   to its `bytes` equals `Ok` of the reference vector by `assert_eq!`.
/// Boundary: the public `create_encoding`; the real chain is `create_encoding`,
///   the selected concrete's constructor, `consume_encoding`, `encode`, and
///   `to_fields`.
/// Mocked:  the consumer, replaced by `MockIEncodingConsumer`, which calls the
///   concrete's `encode` and relays its return; the concretes and the
///   description run real.
#[test]
fn every_concrete_declaring_the_ethereum_abi_identifier_encodes_the_reference_context_to_the_reference_vector()
 {
    // Arrange
    let outputs = DECLARED_ENCODING_SELECTIONS
        .iter()
        .map(|case| {
            let description = build_derivation_context_description(Default::default());
            let consumer =
                build_mock_i_encoding_consumer(&description, reference_context(), Vec::new());
            let deps = build_create_encoding_deps(consumer);
            let params = build_create_encoding_params(CreateEncodingParamsOverrides {
                concrete: Some(case.concrete),
                identifier: Some(case.identifier),
            });
            let payload = build_create_encoding_payload(Default::default());

            // Act
            create_encoding(&deps, params, payload)
                .ok()
                .map(|success| success.output)
        })
        .collect::<Vec<_>>();

    // Assert
    let kept = outputs
        .into_iter()
        .flatten()
        .filter(|output| output.declaration.identifier == EncodingIdentifier::EthereumAbiV1)
        .collect::<Vec<_>>();
    assert!(!kept.is_empty());
    for output in kept {
        assert_eq!(
            output.encoded.map(|success| success.bytes),
            Ok(reference_vector())
        );
    }
}
// every_concrete_declaring_the_ethereum_abi_identifier_encodes_the_reference_context_to_the_reference_vector

/// Contract: the own entry
///   `every_concrete_declaring_the_ethereum_abi_identifier_decodes_the_reference_vector_to_the_reference_context`:
///   for each case of the declared set whose concrete declares
///   `EncodingIdentifier::EthereumAbiV1`, the concrete decodes the reference
///   vector under `DerivationContextDescription` → the decoded context equals
///   the reference context.
/// Arrange: the consumer over the description, the reference context, and the
///   reference vector as its bytes; every case runs and the Ethereum ABI
///   outputs are kept.
/// Act:     `create_encoding(&deps, params, payload)` per case.
/// Assert:  at least one output is kept; each kept output's `decoded` mapped
///   to its `described` equals `Ok` of the reference context by `assert_eq!`.
/// Boundary: the public `create_encoding`; the real chain is `create_encoding`,
///   the selected concrete's constructor, `consume_encoding`, `decode`,
///   `fields_to_value`, the components' constructors, and
///   `DerivationContext::try_new`.
/// Mocked:  the consumer, replaced by `MockIEncodingConsumer`, which calls the
///   concrete's `decode` and relays its return; the concretes, the description,
///   and the components run real.
#[test]
fn every_concrete_declaring_the_ethereum_abi_identifier_decodes_the_reference_vector_to_the_reference_context()
 {
    // Arrange
    let outputs = DECLARED_ENCODING_SELECTIONS
        .iter()
        .map(|case| {
            let description = build_derivation_context_description(Default::default());
            let consumer = build_mock_i_encoding_consumer(
                &description,
                reference_context(),
                reference_vector(),
            );
            let deps = build_create_encoding_deps(consumer);
            let params = build_create_encoding_params(CreateEncodingParamsOverrides {
                concrete: Some(case.concrete),
                identifier: Some(case.identifier),
            });
            let payload = build_create_encoding_payload(Default::default());

            // Act
            create_encoding(&deps, params, payload)
                .ok()
                .map(|success| success.output)
        })
        .collect::<Vec<_>>();

    // Assert
    let kept = outputs
        .into_iter()
        .flatten()
        .filter(|output| output.declaration.identifier == EncodingIdentifier::EthereumAbiV1)
        .collect::<Vec<_>>();
    assert!(!kept.is_empty());
    for output in kept {
        assert_eq!(
            output.decoded.map(|success| success.described),
            Ok(reference_context())
        );
    }
}
// every_concrete_declaring_the_ethereum_abi_identifier_decodes_the_reference_vector_to_the_reference_context

/// Contract: the own entry
///   `every_concrete_declaring_the_ethereum_abi_identifier_returns_the_context_refusal_whole`:
///   for each case of the declared set whose concrete declares
///   `EncodingIdentifier::EthereumAbiV1`, the concrete decodes the reference
///   vector with its group-index word replaced by `0x00…0020`, thirty-two, the
///   reference geometry's group count →
///   `Err(DecodeErrorReturn::Description(DerivationContextFromFieldsErrorReturn
///   ::Context(DerivationContextTryNewErrorReturn::GroupIndexOutOfRange
///   { group_index: 32, group_count: 32 })))`.
/// Arrange: the consumer over the description, the reference context, and as
///   its bytes the reference vector's literal with its group-index word
///   `0x00…0020`; every case runs and the Ethereum ABI outputs are kept; the
///   index equals the reference geometry's group count.
/// Act:     `create_encoding(&deps, params, payload)` per case.
/// Assert:  at least one output is kept; each kept output's `decoded` mapped
///   to its `described` equals the whole `Err` by `assert_eq!`.
/// Boundary: the public `create_encoding`; the real chain is `create_encoding`,
///   the selected concrete's constructor, `consume_encoding`, `decode`,
///   `fields_to_value`, the components' constructors, and
///   `DerivationContext::try_new`.
/// Mocked:  the consumer, replaced by `MockIEncodingConsumer`, which calls the
///   concrete's `decode` and relays its return; the concretes, the description,
///   and the components run real.
#[test]
fn every_concrete_declaring_the_ethereum_abi_identifier_returns_the_context_refusal_whole() {
    // Arrange
    let outputs = DECLARED_ENCODING_SELECTIONS
        .iter()
        .map(|case| {
            let description = build_derivation_context_description(Default::default());
            let consumer =
                build_mock_i_encoding_consumer(&description, reference_context(), refusal_vector());
            let deps = build_create_encoding_deps(consumer);
            let params = build_create_encoding_params(CreateEncodingParamsOverrides {
                concrete: Some(case.concrete),
                identifier: Some(case.identifier),
            });
            let payload = build_create_encoding_payload(Default::default());

            // Act
            create_encoding(&deps, params, payload)
                .ok()
                .map(|success| success.output)
        })
        .collect::<Vec<_>>();

    // Assert
    let kept = outputs
        .into_iter()
        .flatten()
        .filter(|output| output.declaration.identifier == EncodingIdentifier::EthereumAbiV1)
        .collect::<Vec<_>>();
    assert!(!kept.is_empty());
    for output in kept {
        assert_eq!(
            output.decoded.map(|success| success.described),
            Err(DecodeErrorReturn::Description(
                DerivationContextFromFieldsErrorReturn::Context(
                    DerivationContextTryNewErrorReturn::GroupIndexOutOfRange {
                        group_index: 32,
                        group_count: 32
                    }
                )
            ))
        );
    }
}
// every_concrete_declaring_the_ethereum_abi_identifier_returns_the_context_refusal_whole

/// Contract: the own entry
///   `the_crate_root_refuses_a_group_index_outside_the_geometry_whole`:
///   `DerivationContext::try_new` called through the crate barrel with the
///   index equal to the reference geometry's group count →
///   `Err(DerivationContextTryNewErrorReturn::GroupIndexOutOfRange
///   { group_index: 32, group_count: 32 })`, and no context.
/// Arrange: `build_derivation_context_constructor_params` with the reference
///   geometry, thirty-two groups, and `build_group_index` at `32`, the lowest
///   index the reference geometry refuses; the block runs once and reaches no
///   family.
/// Act:     `DerivationContext::try_new(params)`.
/// Assert:  `result.err()` equals `Some` of the whole error by `assert_eq!`.
/// Boundary: the public call `DerivationContext::try_new`; the route `lib.rs`,
///   `sidecar::derivation_context::provides`, then `DerivationContext::try_new`.
/// Mocked:  none.
#[test]
fn the_crate_root_refuses_a_group_index_outside_the_geometry_whole() {
    // Arrange
    let params =
        build_derivation_context_constructor_params(DerivationContextConstructorParamsOverrides {
            geometry: Some(build_piece_geometry(
                PieceGeometryConstructorParamsOverrides {
                    piece_group_size: Some(32768),
                    ..Default::default()
                },
            )),
            group_index: Some(build_group_index(GroupIndexConstructorParamsOverrides {
                value: Some(32),
            })),
            ..Default::default()
        });

    // Act
    let result = DerivationContext::try_new(params);

    // Assert
    assert_eq!(
        result.err(),
        Some(DerivationContextTryNewErrorReturn::GroupIndexOutOfRange {
            group_index: 32,
            group_count: 32
        })
    );
}
// the_crate_root_refuses_a_group_index_outside_the_geometry_whole
