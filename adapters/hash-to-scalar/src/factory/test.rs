#![allow(clippy::expect_used)]

use super::create_hash_to_scalar;
use super::provides::{
    ConsumeHashToScalarParams, ConsumeHashToScalarPayload, CreateHashToScalarDepsOverrides,
    CreateHashToScalarParamsOverrides, HashToScalarConcrete, HashToScalarDeclaration,
    HashToScalarDeclarationOverrides, HashToScalarErrorReturn, HashToScalarParamsOverrides,
    HashToScalarReturn, IHashToScalarAdapter, IHashToScalarConsumer,
    MockIHashToScalarAdapterFailureMode, MockIHashToScalarConsumer,
    build_create_hash_to_scalar_deps, build_create_hash_to_scalar_params,
    build_create_hash_to_scalar_payload, build_hash_to_scalar_declaration,
    build_hash_to_scalar_params, build_hash_to_scalar_payload,
};
use crate::domain_tag::provides::build_domain_tag;
use crate::lookup_hash_to_scalar_declaration::provides::{
    LookupHashToScalarDeclarationDeps, LookupHashToScalarDeclarationFn,
    LookupHashToScalarDeclarationParams, LookupHashToScalarDeclarationPayload,
    LookupHashToScalarDeclarationSuccessReturn,
};
use pairing::{
    ConsumePairingParams, ConsumePairingPayload, CreatePairingParamsOverrides,
    CreatePairingPayload, IPairingAdapter, IPairingConsumer, ISampleUniformScalar,
    SampleUniformScalarErrorReturn, build_admit_pairing_composition_deps,
    build_create_pairing_deps, build_create_pairing_params, create_pairing,
    mock_admit_pairing_composition, mock_lookup_pairing_declaration,
};

/// Contract: `create_hash_to_scalar` → `Ok(CreateHashToScalarSuccessReturn
///   { output })` relaying the consumer's output, the factory handing the
///   constructed adapter and the declaration its `lookup` call returned for
///   `params.concrete` to the consumer in `ConsumeHashToScalarPayload`.
/// Arrange: inside the pairing consumer, `MockIHashToScalarConsumer`, whose
///   `Output` is the `HashToScalarDeclaration` it returns from
///   `payload.declaration`, placed in deps by
///   `build_create_hash_to_scalar_deps(MockIHashToScalarConsumer, overrides)`;
///   a declaration lookup the block declares, of the production type
///   `LookupHashToScalarDeclarationFn`, that returns
///   `Ok(LookupHashToScalarDeclarationSuccessReturn { declaration })` whose
///   `declaration` `build_hash_to_scalar_declaration` builds with `concrete`
///   overridden to the `concrete` the call's `LookupHashToScalarDeclarationParams`
///   carries, so the assert proves the factory called the lookup with
///   `params.concrete` and handed its return to the consumer; per member of
///   `MockIHashToScalarAdapterFailureMode`, `Succeeds` and `SamplingRefused`,
///   params from `build_create_hash_to_scalar_params(CreateHashToScalarParamsOverrides
///   { concrete: Some(HashToScalarConcrete::Mock(mode)) })`, so an arm that
///   constructs for one failure mode alone fails.
/// Act:     `create_pairing` runs the pairing consumer, whose call per mode is
///   `create_hash_to_scalar::<P::Scalar, _>` over
///   `build_create_hash_to_scalar_deps(MockIHashToScalarConsumer,
///   CreateHashToScalarDepsOverrides { lookup: Some(that lookup),
///   ..Default::default() })`.
/// Assert:  in the consumer, per mode, the success arm is bound through
///   `expect`; `success.output.concrete` equals `params.concrete` by
///   `assert_eq!`; outside the consumer, `created.is_ok()`, proving the
///   consumer ran.
/// Collaborators: the family's mock concrete through the factory by the
///   selection; the declared lookup, handed in `deps`; the pairing family's
///   mock concrete through `create_pairing`, supplying `P::Scalar` only.
#[test]
fn create_hash_to_scalar_constructs_the_selected_member_and_returns_the_consumers_output() {
    struct Consumer;

    impl IPairingConsumer for Consumer {
        type Output = ();

        fn consume_pairing<P: IPairingAdapter>(
            &self,
            _params: ConsumePairingParams,
            _consumed: ConsumePairingPayload<P>,
        ) -> Self::Output {
            let declared_lookup: LookupHashToScalarDeclarationFn =
                |_deps: &LookupHashToScalarDeclarationDeps,
                 params: LookupHashToScalarDeclarationParams,
                 _payload: LookupHashToScalarDeclarationPayload| {
                    Ok(LookupHashToScalarDeclarationSuccessReturn {
                        declaration: build_hash_to_scalar_declaration(
                            HashToScalarDeclarationOverrides {
                                concrete: Some(params.concrete),
                                ..Default::default()
                            },
                        ),
                    })
                };
            for mode in [
                MockIHashToScalarAdapterFailureMode::Succeeds,
                MockIHashToScalarAdapterFailureMode::SamplingRefused,
            ] {
                let params =
                    build_create_hash_to_scalar_params(CreateHashToScalarParamsOverrides {
                        concrete: Some(HashToScalarConcrete::Mock(mode)),
                    });
                let deps = build_create_hash_to_scalar_deps(
                    MockIHashToScalarConsumer,
                    CreateHashToScalarDepsOverrides {
                        lookup: Some(declared_lookup),
                    },
                );
                let success = create_hash_to_scalar::<P::Scalar, _>(
                    &deps,
                    params,
                    build_create_hash_to_scalar_payload(Default::default()),
                )
                .expect("the mock selection constructs under every failure mode");
                let declaration: HashToScalarDeclaration = success.output;
                assert_eq!(declaration.concrete, params.concrete);
            }
        }
    }

    let created = create_pairing(
        &build_create_pairing_deps(
            Consumer,
            mock_lookup_pairing_declaration,
            mock_admit_pairing_composition,
            build_admit_pairing_composition_deps(
                mock_lookup_pairing_declaration,
                Default::default(),
            ),
            Default::default(),
        ),
        build_create_pairing_params(CreatePairingParamsOverrides::default()),
        CreatePairingPayload,
    );

    assert!(created.is_ok());
}

/// Contract: `create_hash_to_scalar`, `Mock` arm under
///   `MockIHashToScalarAdapterFailureMode::SamplingRefused` →
///   `Ok(CreateHashToScalarSuccessReturn { output })` whose `output` observes
///   `Err(HashToScalarErrorReturn::Sampling(SampleUniformScalarErrorReturn::WrongLength
///   { expected: P::Scalar::UNIFORM_BYTES_LENGTH, actual: 0 }))` from
///   `hash_to_scalar`.
/// Arrange: a block-local `HashConsumer` declared inside the pairing
///   consumer's block, implementing `IHashToScalarConsumer<S>` for every `S`
///   with `type Output = HashToScalarReturn<S>`, whose `consume_hash_to_scalar`
///   calls `payload.adapter.hash_to_scalar` over a tag `build_domain_tag`
///   builds at defaults and `build_hash_to_scalar_payload` at no override,
///   returning its return; params as above with the `concrete` override
///   `HashToScalarConcrete::Mock(MockIHashToScalarAdapterFailureMode::SamplingRefused)`,
///   so a factory that drops the failure mode returns a hash instead of the
///   refusal; deps by `build_create_hash_to_scalar_deps(HashConsumer,
///   overrides)` with `lookup` the declared lookup above.
/// Act:     `create_pairing` runs the pairing consumer, whose single call is
///   `create_hash_to_scalar::<P::Scalar, _>`.
/// Assert:  in the consumer, the success arm is bound through `expect`;
///   `success.output.err()` equals
///   `Some(HashToScalarErrorReturn::Sampling(SampleUniformScalarErrorReturn::WrongLength
///   { expected: P::Scalar::UNIFORM_BYTES_LENGTH, actual: 0 }))` by
///   `assert_eq!`, the bound read from the received scalar type; outside the
///   consumer, `created.is_ok()`.
/// Collaborators: the family's mock concrete through the factory by the
///   selection; the declared lookup, handed in `deps`; the pairing family's
///   mock concrete through `create_pairing`, supplying `P::Scalar` only.
#[test]
fn create_hash_to_scalar_constructs_the_mock_member_under_the_sampling_refused_mode() {
    struct HashConsumer;

    impl<S: ISampleUniformScalar + Clone> IHashToScalarConsumer<S> for HashConsumer {
        type Output = HashToScalarReturn<S>;

        fn consume_hash_to_scalar<K: IHashToScalarAdapter<S>>(
            &self,
            _params: ConsumeHashToScalarParams,
            payload: ConsumeHashToScalarPayload<S, K>,
        ) -> Self::Output {
            let tag = build_domain_tag(Default::default());
            payload.adapter.hash_to_scalar(
                build_hash_to_scalar_params(HashToScalarParamsOverrides { tag: Some(&tag) }),
                build_hash_to_scalar_payload(Default::default()),
            )
        }
    }

    struct Consumer;

    impl IPairingConsumer for Consumer {
        type Output = ();

        fn consume_pairing<P: IPairingAdapter>(
            &self,
            _params: ConsumePairingParams,
            _consumed: ConsumePairingPayload<P>,
        ) -> Self::Output {
            let declared_lookup: LookupHashToScalarDeclarationFn =
                |_deps: &LookupHashToScalarDeclarationDeps,
                 params: LookupHashToScalarDeclarationParams,
                 _payload: LookupHashToScalarDeclarationPayload| {
                    Ok(LookupHashToScalarDeclarationSuccessReturn {
                        declaration: build_hash_to_scalar_declaration(
                            HashToScalarDeclarationOverrides {
                                concrete: Some(params.concrete),
                                ..Default::default()
                            },
                        ),
                    })
                };
            let deps = build_create_hash_to_scalar_deps(
                HashConsumer,
                CreateHashToScalarDepsOverrides {
                    lookup: Some(declared_lookup),
                },
            );
            let success = create_hash_to_scalar::<P::Scalar, _>(
                &deps,
                build_create_hash_to_scalar_params(CreateHashToScalarParamsOverrides {
                    concrete: Some(HashToScalarConcrete::Mock(
                        MockIHashToScalarAdapterFailureMode::SamplingRefused,
                    )),
                }),
                build_create_hash_to_scalar_payload(Default::default()),
            )
            .expect("the mock selection constructs under the sampling-refused mode");
            assert_eq!(
                success.output.err(),
                Some(HashToScalarErrorReturn::Sampling(
                    SampleUniformScalarErrorReturn::WrongLength {
                        expected: P::Scalar::UNIFORM_BYTES_LENGTH,
                        actual: 0,
                    }
                ))
            );
        }
    }

    let created = create_pairing(
        &build_create_pairing_deps(
            Consumer,
            mock_lookup_pairing_declaration,
            mock_admit_pairing_composition,
            build_admit_pairing_composition_deps(
                mock_lookup_pairing_declaration,
                Default::default(),
            ),
            Default::default(),
        ),
        build_create_pairing_params(CreatePairingParamsOverrides::default()),
        CreatePairingPayload,
    );

    assert!(created.is_ok());
}
