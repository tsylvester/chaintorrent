#![allow(clippy::expect_used)]

use super::create_encoding;
use super::provides::{
    CreateEncodingDepsOverrides, CreateEncodingParamsOverrides, EncodingConcrete,
    EncodingConcreteOverrides, EncodingDeclarationOverrides, MockIEncodingContract,
    build_canonical_fields, build_create_encoding_deps, build_create_encoding_params,
    build_create_encoding_payload, build_encoding_concrete, build_encoding_declaration,
    build_mock_i_encoding_consumer, mock_type_name,
};
use crate::lookup_encoding_declaration::provides::{
    LookupEncodingDeclarationDeps, LookupEncodingDeclarationFn, LookupEncodingDeclarationParams,
    LookupEncodingDeclarationPayload, LookupEncodingDeclarationReturn,
    LookupEncodingDeclarationSuccessReturnOverrides,
    build_lookup_encoding_declaration_success_return,
};

fn lookup_by_selection(
    _deps: &LookupEncodingDeclarationDeps,
    params: LookupEncodingDeclarationParams,
    _payload: LookupEncodingDeclarationPayload,
) -> LookupEncodingDeclarationReturn {
    let adapter_version = match params.concrete {
        EncodingConcrete::Abi => 7,
        EncodingConcrete::Mock => 9,
    };
    Ok(build_lookup_encoding_declaration_success_return(
        LookupEncodingDeclarationSuccessReturnOverrides {
            declaration: Some(build_encoding_declaration(
                mock_type_name,
                EncodingDeclarationOverrides {
                    concrete: Some(params.concrete),
                    adapter_version: Some(adapter_version),
                    ..Default::default()
                },
            )),
        },
    ))
}

const _: LookupEncodingDeclarationFn = lookup_by_selection;

/// Contract: `create_encoding`, the declaration read and the `Mock` arm:
///   `params.selection` is `EncodingConcrete::Mock` →
///   `Ok(CreateEncodingSuccessReturn { output })` holding the consumer's
///   output, whose `declaration` is the declaration the lookup returned for
///   `EncodingConcrete::Mock`.
/// Arrange: `build_create_encoding_deps` over the consumer, built by
///   `build_mock_i_encoding_consumer` over `MockIEncodingContract`,
///   `build_canonical_fields` at its defaults, and no bytes, overriding
///   `lookup` with `lookup_by_selection`; `build_create_encoding_params` at
///   its defaults, which select `EncodingConcrete::Mock`;
///   `build_create_encoding_payload`. The lookup returns `9` for the mock
///   selection and `7` for the other, so a factory that calls the lookup
///   with another selection, or hands another declaration, returns another
///   `adapter_version`.
/// Act:     `create_encoding(&deps, params, payload)`.
/// Assert:  the `declaration` of the `output` of the `Ok` arm has `concrete`
///   equal to `EncodingConcrete::Mock` and `adapter_version` equal to `9`,
///   each by `assert_eq!`, the literals the arrangement states only inside
///   the lookup the factory called.
#[test]
fn create_encoding_reads_the_declaration_for_the_mock_selection_and_hands_it_to_the_consumer() {
    // Arrange
    let consumer = build_mock_i_encoding_consumer(
        &MockIEncodingContract,
        build_canonical_fields(Default::default()),
        Vec::new(),
    );
    let deps = build_create_encoding_deps(
        consumer,
        CreateEncodingDepsOverrides {
            lookup: Some(lookup_by_selection),
        },
    );
    let params = build_create_encoding_params(Default::default());
    let payload = build_create_encoding_payload(Default::default());

    // Act
    let result = create_encoding(&deps, params, payload);

    // Assert
    let success = result.expect("the factory hands the concrete the selection names");
    assert_eq!(success.output.declaration.concrete, EncodingConcrete::Mock);
    assert_eq!(success.output.declaration.adapter_version, 9);
}
// create_encoding_reads_the_declaration_for_the_mock_selection_and_hands_it_to_the_consumer

/// Contract: `create_encoding`, the declaration read and the `Abi` arm:
///   `params.selection` is `EncodingConcrete::Abi` →
///   `Ok(CreateEncodingSuccessReturn { output })` holding the consumer's
///   output, whose `declaration` is the declaration the lookup returned for
///   `EncodingConcrete::Abi`.
/// Arrange: `build_create_encoding_deps` over the consumer, built by
///   `build_mock_i_encoding_consumer` over `MockIEncodingContract`,
///   `build_canonical_fields` at its defaults, and no bytes, overriding
///   `lookup` with `lookup_by_selection`; `build_create_encoding_params`
///   overriding `selection` with `build_encoding_concrete(
///   EncodingConcrete::Abi, EncodingConcreteOverrides::default())`;
///   `build_create_encoding_payload`. The selection differs from the
///   default `EncodingConcrete::Mock`, so a factory that hands the mock
///   arm's declaration for the `Abi` selection returns another
///   `adapter_version`.
/// Act:     `create_encoding(&deps, params, payload)`.
/// Assert:  the `declaration` of the `output` of the `Ok` arm has `concrete`
///   equal to `EncodingConcrete::Abi` and `adapter_version` equal to `7`,
///   each by `assert_eq!`.
#[test]
fn create_encoding_reads_the_declaration_for_the_abi_selection_and_hands_it_to_the_consumer() {
    // Arrange
    let consumer = build_mock_i_encoding_consumer(
        &MockIEncodingContract,
        build_canonical_fields(Default::default()),
        Vec::new(),
    );
    let deps = build_create_encoding_deps(
        consumer,
        CreateEncodingDepsOverrides {
            lookup: Some(lookup_by_selection),
        },
    );
    let params = build_create_encoding_params(CreateEncodingParamsOverrides {
        selection: Some(build_encoding_concrete(
            EncodingConcrete::Abi,
            EncodingConcreteOverrides,
        )),
    });
    let payload = build_create_encoding_payload(Default::default());

    // Act
    let result = create_encoding(&deps, params, payload);

    // Assert
    let success = result.expect("the factory hands the concrete the selection names");
    assert_eq!(success.output.declaration.concrete, EncodingConcrete::Abi);
    assert_eq!(success.output.declaration.adapter_version, 7);
}
// create_encoding_reads_the_declaration_for_the_abi_selection_and_hands_it_to_the_consumer
