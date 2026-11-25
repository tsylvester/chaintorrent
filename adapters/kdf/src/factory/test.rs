#![allow(clippy::expect_used)]

use super::provides::{
    ConsumeKeyDerivationParams, ConsumeKeyDerivationPayload, CreateKeyDerivationParamsOverrides,
    DeriveKeyErrorReturn, DeriveKeyReturn, IKeyDerivationAdapter, IKeyDerivationConsumer,
    KdfConcrete, KdfDeclaration, KdfDeclarationOverrides,
    MockIKeyDerivationAdapterDeriveKeyErrorReturn, MockIKeyDerivationAdapterFailureMode,
    MockIKeyDerivationConsumer, build_create_key_derivation_deps,
    build_create_key_derivation_params, build_create_key_derivation_payload,
    build_derive_key_params, build_derive_key_payload, build_kdf_declaration,
    create_key_derivation,
};
use crate::lookup_kdf_declaration::provides::{
    LookupKdfDeclarationFn, LookupKdfDeclarationParams, LookupKdfDeclarationSuccessReturn,
};

/// Contract: `create_key_derivation` → `Ok(CreateKeyDerivationSuccessReturn
///   { output })` relaying the consumer's output, the factory handing the
///   constructed adapter and the declaration its `lookup` call returned for
///   `params.selection` to the consumer in `ConsumeKeyDerivationPayload`.
/// Collaborators: the family's mock concrete, reached through
///   `create_key_derivation` by `build_create_key_derivation_params` with the
///   `selection` override; `MockIKeyDerivationConsumer`, whose `Output` is the
///   `KdfDeclaration` it returns from `payload.declaration`, and a block-local
///   consumer whose `consume_key_derivation` calls `adapter.derive_key` on
///   the adapter it receives and returns the `DeriveKeyReturn` as its
///   `Output`, each placed in `deps` by `build_create_key_derivation_deps`; a
///   declaration lookup the block declares, of the production type
///   `LookupKdfDeclarationFn`; `build_create_key_derivation_payload` at its
///   default; no test names a concrete.
/// Arrange: `build_create_key_derivation_params` with the `selection`
///   override
///   `KdfConcrete::Mock(MockIKeyDerivationAdapterFailureMode::KeyMaterialLengthUnrepresentable)`,
///   so the failure mode differs from the `NoFailure` default and a factory
///   that drops the failure mode, or constructs another concrete, returns a
///   key instead of the error; the declared lookup returns
///   `Ok(LookupKdfDeclarationSuccessReturn { declaration })` whose
///   `declaration` `build_kdf_declaration` builds with `concrete` overridden
///   to the `concrete` the call's `LookupKdfDeclarationParams` carries, so
///   the assert proves the factory called the lookup with `params.selection`
///   and handed its return to the consumer; `build_derive_key_params` and
///   `build_derive_key_payload` with no override, whose key material is the
///   32-byte reference key material.
/// Act:     `create_key_derivation(&deps, params, payload)` once with
///   `MockIKeyDerivationConsumer` in `deps` and once with the block-local
///   consumer.
/// Assert:  each success arm is bound through `expect`; the first call's
///   `output` is the declaration whose `concrete` equals `params.selection`
///   by `assert_eq!`; the second call's `output` is the `DeriveKeyReturn`
///   the consumer observed, whose `err()` equals
///   `Some(DeriveKeyErrorReturn::Mock(MockIKeyDerivationAdapterDeriveKeyErrorReturn::KeyMaterialLengthUnrepresentable { length: 32 }))`
///   by `assert_eq!`, the length a literal and not read from the arrangement.
#[test]
fn create_key_derivation_constructs_the_selected_concrete_and_returns_the_consumers_output() {
    // Arrange
    struct DerivingConsumer;

    impl IKeyDerivationConsumer for DerivingConsumer {
        type Output = DeriveKeyReturn;

        fn consume_key_derivation<A: IKeyDerivationAdapter>(
            &self,
            _params: ConsumeKeyDerivationParams,
            payload: ConsumeKeyDerivationPayload<A>,
        ) -> Self::Output {
            payload.adapter.derive_key(
                build_derive_key_params(Default::default()),
                build_derive_key_payload(Default::default()),
            )
        }
    }

    let lookup: LookupKdfDeclarationFn =
        |_deps, lookup_params: LookupKdfDeclarationParams, _payload| {
            Ok(LookupKdfDeclarationSuccessReturn {
                declaration: build_kdf_declaration(KdfDeclarationOverrides {
                    concrete: Some(lookup_params.concrete),
                    ..Default::default()
                }),
            })
        };
    let deps = build_create_key_derivation_deps(
        MockIKeyDerivationConsumer,
        super::provides::CreateKeyDerivationDepsOverrides {
            lookup: Some(lookup),
        },
    );
    let deriving_deps = build_create_key_derivation_deps(
        DerivingConsumer,
        super::provides::CreateKeyDerivationDepsOverrides {
            lookup: Some(lookup),
        },
    );
    let params = build_create_key_derivation_params(CreateKeyDerivationParamsOverrides {
        selection: Some(KdfConcrete::Mock(
            MockIKeyDerivationAdapterFailureMode::KeyMaterialLengthUnrepresentable,
        )),
    });
    let payload = build_create_key_derivation_payload(Default::default());

    // Act
    let first = create_key_derivation(&deps, params, payload);
    let second = create_key_derivation(
        &deriving_deps,
        params,
        build_create_key_derivation_payload(Default::default()),
    );

    // Assert
    let first_success = first.expect("the selected concrete is constructed");
    let declaration: KdfDeclaration = first_success.output;
    assert_eq!(declaration.concrete, params.selection);
    let second_success = second.expect("the selected concrete is constructed");
    assert_eq!(
        second_success.output.err(),
        Some(DeriveKeyErrorReturn::Mock(
            MockIKeyDerivationAdapterDeriveKeyErrorReturn::KeyMaterialLengthUnrepresentable {
                length: 32
            }
        ))
    );
}
