#![cfg(feature = "mocks")]
#![allow(clippy::expect_used)]

use domain::{
    GroupIndexConstructorParamsOverrides, PieceGeometryConstructorParamsOverrides,
    build_group_index, build_piece_geometry,
};
use encoding::{
    CreateEncodingParamsOverrides, DecodeErrorReturn, ENCODING_CONCRETES, EncodingConcrete,
    EncodingConcreteOverrides, EncodingIdentifier, LookupEncodingDeclarationParamsOverrides,
    build_create_encoding_deps, build_create_encoding_params, build_create_encoding_payload,
    build_encoding_concrete, build_lookup_encoding_declaration_deps,
    build_lookup_encoding_declaration_params, build_lookup_encoding_declaration_payload,
    build_mock_i_encoding_consumer, create_encoding, lookup_encoding_declaration,
};
use hex::decode;
use workflows::{
    DerivationContext, DerivationContextConstructorParamsOverrides,
    DerivationContextFromFieldsErrorReturn, DerivationContextTryNewErrorReturn,
    build_derivation_context, build_derivation_context_constructor_params,
    build_derivation_context_description,
};

const REFERENCE_VECTOR_HEX: &str = "010101010101010101010101010101010101010101010101010101010101010111111111111111111111111111111111111111111111111111111111111111112222222222222222222222222222222222222222222222222222222222222222000000000000000000000000000000000000000000000000000000000000000133333333333333333333333333333333333333333333333333333333333333330000000000000000000000000000000000000000000000000000000000000007000000000000000000000000000000000000000000000000000000000000400000000000000000000000000000000000000000000000000000000000000080000000000000000000000000000000000000000000000000000000000000100000";

const REFUSAL_VECTOR_HEX: &str = "010101010101010101010101010101010101010101010101010101010101010111111111111111111111111111111111111111111111111111111111111111112222222222222222222222222222222222222222222222222222222222222222000000000000000000000000000000000000000000000000000000000000000133333333333333333333333333333333333333333333333333333333333333330000000000000000000000000000000000000000000000000000000000000020000000000000000000000000000000000000000000000000000000000000400000000000000000000000000000000000000000000000000000000000000080000000000000000000000000000000000000000000000000000000000000100000";

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
    decode(REFERENCE_VECTOR_HEX).expect("the reference vector is hexadecimal")
}

fn refusal_vector() -> Vec<u8> {
    decode(REFUSAL_VECTOR_HEX).expect("the refusal vector is hexadecimal")
}

/// Contract: the own entry `every_concrete_decodes_the_derivation_context_it_encodes`:
///   the context a concrete decodes from the bytes it wrote for the reference
///   context equals the reference context.
/// Arrange: the consumer over the description, the reference context, and no
///   bytes; every member of `ENCODING_CONCRETES` in turn, each selection built
///   settings-free through `build_encoding_concrete` at defaults; the
///   reference context's nine values are distinct within each kind.
/// Act:     `create_encoding(&deps, params, payload)` per member.
/// Assert:  per member, the output is present, and its `round_trip` mapped to
///   its `described` equals `Ok` of the reference context by `assert_eq!`.
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
    let description = build_derivation_context_description(Default::default());
    for member in ENCODING_CONCRETES {
        let selection = build_encoding_concrete(*member, EncodingConcreteOverrides);
        let consumer =
            build_mock_i_encoding_consumer(&description, reference_context(), Vec::new());
        let deps = build_create_encoding_deps(consumer, Default::default());
        let params = build_create_encoding_params(CreateEncodingParamsOverrides {
            selection: Some(selection),
        });
        let payload = build_create_encoding_payload(Default::default());

        // Act
        let result = create_encoding(&deps, params, payload);

        // Assert
        let output = result
            .ok()
            .map(|success| success.output)
            .expect("the member's concrete is admitted under the selection it names");
        assert_eq!(
            output.round_trip.map(|success| success.described),
            Ok(reference_context())
        );
    }
}
// every_concrete_decodes_the_derivation_context_it_encodes

/// Contract: the own entry
///   `every_concrete_declaring_the_ethereum_abi_identifier_encodes_the_reference_context_to_the_reference_vector`:
///   the bytes such a concrete writes for the reference context equal the
///   reference vector.
/// Arrange: the consumer over the description, the reference context, and no
///   bytes; every member of `ENCODING_CONCRETES`, each selection built
///   settings-free through `build_encoding_concrete` at defaults; the
///   reference context's piece size and piece-group size differ.
/// Act:     `lookup_encoding_declaration` per member with its own built deps,
///   params, and payload, keeping each member whose returned declaration's
///   `identifier` equals `EncodingIdentifier::EthereumAbiV1`, then
///   `create_encoding(&deps, params, payload)` per kept member.
/// Assert:  at least one output is kept; each kept output's `encoded` mapped
///   to its `bytes` equals `Ok` of the reference vector by `assert_eq!`.
/// Boundary: the public `lookup_encoding_declaration` and `create_encoding`;
///   the real chain is `lookup_encoding_declaration`, ending at the member's
///   declaration constant, then `create_encoding`, the selected concrete's
///   constructor, `consume_encoding`, `encode`, and `to_fields`.
/// Mocked:  the consumer, replaced by `MockIEncodingConsumer`, which calls the
///   concrete's `encode` and relays its return; the lookup runs real and
///   constructs no adapter; the concretes and the description run real.
#[test]
fn every_concrete_declaring_the_ethereum_abi_identifier_encodes_the_reference_context_to_the_reference_vector()
 {
    // Arrange
    let description = build_derivation_context_description(Default::default());

    // Act
    let members = ENCODING_CONCRETES
        .iter()
        .map(|member| build_encoding_concrete(*member, EncodingConcreteOverrides))
        .filter(|selection| {
            let Ok(looked_up) = lookup_encoding_declaration(
                &build_lookup_encoding_declaration_deps(Default::default()),
                build_lookup_encoding_declaration_params(
                    LookupEncodingDeclarationParamsOverrides {
                        concrete: Some(*selection),
                    },
                ),
                build_lookup_encoding_declaration_payload(Default::default()),
            );
            looked_up.declaration.identifier == EncodingIdentifier::EthereumAbiV1
        })
        .collect::<Vec<EncodingConcrete>>();
    let outputs = members
        .into_iter()
        .map(|selection| {
            let consumer =
                build_mock_i_encoding_consumer(&description, reference_context(), Vec::new());
            let deps = build_create_encoding_deps(consumer, Default::default());
            let params = build_create_encoding_params(CreateEncodingParamsOverrides {
                selection: Some(selection),
            });
            let payload = build_create_encoding_payload(Default::default());

            create_encoding(&deps, params, payload)
                .ok()
                .map(|success| success.output)
        })
        .collect::<Vec<_>>();

    // Assert
    assert!(!outputs.is_empty());
    for output in outputs {
        let output =
            output.expect("the kept member's concrete is admitted under the selection it names");
        assert_eq!(
            output.encoded.map(|success| success.bytes),
            Ok(reference_vector())
        );
    }
}
// every_concrete_declaring_the_ethereum_abi_identifier_encodes_the_reference_context_to_the_reference_vector

/// Contract: the own entry
///   `every_concrete_declaring_the_ethereum_abi_identifier_decodes_the_reference_vector_to_the_reference_context`:
///   the context such a concrete decodes from the reference vector equals the
///   reference context.
/// Arrange: the consumer over the description, the reference context, and the
///   reference vector as its bytes; every member of `ENCODING_CONCRETES`, each
///   selection built settings-free through `build_encoding_concrete` at
///   defaults.
/// Act:     `lookup_encoding_declaration` per member with its own built deps,
///   params, and payload, keeping each member whose returned declaration's
///   `identifier` equals `EncodingIdentifier::EthereumAbiV1`, then
///   `create_encoding(&deps, params, payload)` per kept member.
/// Assert:  at least one output is kept; each kept output's `decoded` mapped
///   to its `described` equals `Ok` of the reference context by `assert_eq!`.
/// Boundary: the public `lookup_encoding_declaration` and `create_encoding`;
///   the real chain is `lookup_encoding_declaration`, ending at the member's
///   declaration constant, then `create_encoding`, the selected concrete's
///   constructor, `consume_encoding`, `decode`, `fields_to_value`, the
///   components' constructors, and `DerivationContext::try_new`.
/// Mocked:  the consumer, replaced by `MockIEncodingConsumer`, which calls the
///   concrete's `decode` and relays its return; the lookup runs real and
///   constructs no adapter; the concretes, the description, and the components
///   run real.
#[test]
fn every_concrete_declaring_the_ethereum_abi_identifier_decodes_the_reference_vector_to_the_reference_context()
 {
    // Arrange
    let description = build_derivation_context_description(Default::default());

    // Act
    let members = ENCODING_CONCRETES
        .iter()
        .map(|member| build_encoding_concrete(*member, EncodingConcreteOverrides))
        .filter(|selection| {
            let Ok(looked_up) = lookup_encoding_declaration(
                &build_lookup_encoding_declaration_deps(Default::default()),
                build_lookup_encoding_declaration_params(
                    LookupEncodingDeclarationParamsOverrides {
                        concrete: Some(*selection),
                    },
                ),
                build_lookup_encoding_declaration_payload(Default::default()),
            );
            looked_up.declaration.identifier == EncodingIdentifier::EthereumAbiV1
        })
        .collect::<Vec<EncodingConcrete>>();
    let outputs = members
        .into_iter()
        .map(|selection| {
            let consumer = build_mock_i_encoding_consumer(
                &description,
                reference_context(),
                reference_vector(),
            );
            let deps = build_create_encoding_deps(consumer, Default::default());
            let params = build_create_encoding_params(CreateEncodingParamsOverrides {
                selection: Some(selection),
            });
            let payload = build_create_encoding_payload(Default::default());

            create_encoding(&deps, params, payload)
                .ok()
                .map(|success| success.output)
        })
        .collect::<Vec<_>>();

    // Assert
    assert!(!outputs.is_empty());
    for output in outputs {
        let output =
            output.expect("the kept member's concrete is admitted under the selection it names");
        assert_eq!(
            output.decoded.map(|success| success.described),
            Ok(reference_context())
        );
    }
}
// every_concrete_declaring_the_ethereum_abi_identifier_decodes_the_reference_vector_to_the_reference_context

/// Contract: the own entry
///   `every_concrete_declaring_the_ethereum_abi_identifier_returns_the_context_refusal_whole`:
///   decoding the reference vector with the group-index word set to thirty-two
///   returns the context's refusal whole inside the family's `Description`
///   variant.
/// Arrange: the consumer over the description, the reference context, and as
///   its bytes `hex::decode` of the reference vector's literal with its
///   group-index word `0x00…0020`, bound through `expect`; every member of
///   `ENCODING_CONCRETES`, each selection built settings-free through
///   `build_encoding_concrete` at defaults; the index equals the reference
///   geometry's group count.
/// Act:     `lookup_encoding_declaration` per member with its own built deps,
///   params, and payload, keeping each member whose returned declaration's
///   `identifier` equals `EncodingIdentifier::EthereumAbiV1`, then
///   `create_encoding(&deps, params, payload)` per kept member.
/// Assert:  at least one output is kept; each kept output's `decoded` mapped
///   to its `described` equals the whole `Err` by `assert_eq!`.
/// Boundary: the public `lookup_encoding_declaration` and `create_encoding`;
///   the real chain is `lookup_encoding_declaration`, ending at the member's
///   declaration constant, then `create_encoding`, the selected concrete's
///   constructor, `consume_encoding`, `decode`, `fields_to_value`, the
///   components' constructors, and `DerivationContext::try_new`.
/// Mocked:  the consumer, replaced by `MockIEncodingConsumer`, which calls the
///   concrete's `decode` and relays its return; the lookup runs real and
///   constructs no adapter; the concretes, the description, and the components
///   run real.
#[test]
fn every_concrete_declaring_the_ethereum_abi_identifier_returns_the_context_refusal_whole() {
    // Arrange
    let description = build_derivation_context_description(Default::default());

    // Act
    let members = ENCODING_CONCRETES
        .iter()
        .map(|member| build_encoding_concrete(*member, EncodingConcreteOverrides))
        .filter(|selection| {
            let Ok(looked_up) = lookup_encoding_declaration(
                &build_lookup_encoding_declaration_deps(Default::default()),
                build_lookup_encoding_declaration_params(
                    LookupEncodingDeclarationParamsOverrides {
                        concrete: Some(*selection),
                    },
                ),
                build_lookup_encoding_declaration_payload(Default::default()),
            );
            looked_up.declaration.identifier == EncodingIdentifier::EthereumAbiV1
        })
        .collect::<Vec<EncodingConcrete>>();
    let outputs = members
        .into_iter()
        .map(|selection| {
            let consumer =
                build_mock_i_encoding_consumer(&description, reference_context(), refusal_vector());
            let deps = build_create_encoding_deps(consumer, Default::default());
            let params = build_create_encoding_params(CreateEncodingParamsOverrides {
                selection: Some(selection),
            });
            let payload = build_create_encoding_payload(Default::default());

            create_encoding(&deps, params, payload)
                .ok()
                .map(|success| success.output)
        })
        .collect::<Vec<_>>();

    // Assert
    assert!(!outputs.is_empty());
    for output in outputs {
        let output =
            output.expect("the kept member's concrete is admitted under the selection it names");
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
