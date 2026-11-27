#![allow(clippy::expect_used)]

use super::create_key_agreement;
use super::provides::{
    ConsumeCreateKeyAgreementCaseParams, ConsumeCreateKeyAgreementCasePayload,
    ConsumeKeyAgreementParams, ConsumeKeyAgreementPayload, CreateKeyAgreementParamsOverrides,
    CreateKeyAgreementReturn, CreateKeyAgreementSuccessReturn, GenerateKeysErrorReturn,
    ICreateKeyAgreementCaseConsumer, IKeyAgreementAdapter, IKeyAgreementConsumer,
    KeyAgreementConcrete, MockIKeyAgreementAdapterFailureMode,
    MockIKeyAgreementAdapterGenerateKeysErrorReturn, PossessionComponentsParams,
    build_create_key_agreement_case_deps, build_create_key_agreement_case_params,
    build_create_key_agreement_case_payload, build_create_key_agreement_deps,
    build_create_key_agreement_params, build_create_key_agreement_payload,
    build_generate_keys_params, build_generate_keys_payload, build_key_agreement_concrete,
    build_possession_components_payload, run_create_key_agreement_case,
};
use encoding::{IDecoderAdapter, IEncoderAdapter};
use hash_to_scalar::IHashToScalarAdapter;
use kem::ICredentialKemAdapter;
use pairing::{EncodeG1Params, EncodeG1Payload, IPairingArithmetic, IPairingReference};
use random::IRandomSourceAdapter;

/// Contract: `create_key_agreement`, admitted: the `Mock` arm's `try_new`
///   returns `Ok(adapter)` → `deps.consumer.consume_key_agreement`, exactly
///   once, with the mock concrete under the failure mode the selection
///   carries → `Ok(CreateKeyAgreementSuccessReturn { output })`, the
///   consumer's output held whole.
/// Arrange: `Consumer` holding the mock selection under
///   `MockIKeyAgreementAdapterFailureMode::NonceDrawRefused`; the block's
///   `IKeyAgreementConsumer` returns whether `generate_keys` over draws that
///   differ from each other returned
///   `Err(GenerateKeysErrorReturn::Mock(MockIKeyAgreementAdapterGenerateKeysErrorReturn::NonceDrawRefused))`,
///   a return only the mock concrete under that failure mode produces.
/// Act:     `run_create_key_agreement_case` runs the case, whose consumer's
///   single call is `create_key_agreement`.
/// Assert:  the case success's `output` equals
///   `Ok(CreateKeyAgreementSuccessReturn { output: true })` whole, `true`
///   stated as an independent literal, so a factory that constructs the real
///   concrete or never calls the consumer fails.
/// Collaborators: the pairing `create_pairing` hands `IPairingConsumer`, the
///   encoder `create_encoding` hands `IEncodingConsumer`, the hash-to-scalar
///   concrete `create_hash_to_scalar` hands `IHashToScalarConsumer<P::Scalar>`
///   through `ConsumeHashToScalarPayload { adapter, declaration }`, and the
///   randomness source `create_random_source` hands `IRandomSourceConsumer`,
///   each reached through the case runner at the defaults of its official
///   params builder, which select that family's mock concrete; the `kem`
///   borrow is unused by the unit blocks.
#[test]
fn create_key_agreement_constructs_the_mock_member_for_the_mock_selection() {
    struct Inner<'a, P: IPairingArithmetic + IPairingReference> {
        pairing: &'a P,
    }

    impl<P: IPairingArithmetic + IPairingReference> IKeyAgreementConsumer<P> for Inner<'_, P> {
        type Output = bool;

        fn consume_key_agreement<K: IKeyAgreementAdapter<Pairing = P>>(
            &self,
            _params: ConsumeKeyAgreementParams,
            payload: ConsumeKeyAgreementPayload<K>,
        ) -> Self::Output {
            matches!(
                payload.adapter.generate_keys(
                    build_generate_keys_params(Default::default()),
                    build_generate_keys_payload::<P>(self.pairing, Default::default()),
                ),
                Err(GenerateKeysErrorReturn::Mock(
                    MockIKeyAgreementAdapterGenerateKeysErrorReturn::NonceDrawRefused,
                ))
            )
        }
    }

    struct Consumer {
        concrete: KeyAgreementConcrete,
    }

    impl ICreateKeyAgreementCaseConsumer for Consumer {
        type Output = CreateKeyAgreementReturn<bool>;

        fn consume_create_key_agreement_case<
            P: IPairingArithmetic + IPairingReference,
            E: IEncoderAdapter + IDecoderAdapter,
            H: IHashToScalarAdapter<P::Scalar>,
            R: IRandomSourceAdapter,
            K: ICredentialKemAdapter<Pairing = P>,
        >(
            &self,
            _params: ConsumeCreateKeyAgreementCaseParams,
            payload: ConsumeCreateKeyAgreementCasePayload<'_, P, E, H, R, K>,
        ) -> Self::Output {
            let _case = payload.case;
            create_key_agreement(
                &build_create_key_agreement_deps(
                    payload.pairing,
                    payload.hash_to_scalar,
                    payload.encoder,
                    payload.random,
                    Inner {
                        pairing: payload.pairing,
                    },
                    Default::default(),
                ),
                build_create_key_agreement_params(CreateKeyAgreementParamsOverrides {
                    concrete: Some(self.concrete),
                }),
                build_create_key_agreement_payload(Default::default()),
            )
        }
    }

    let created = run_create_key_agreement_case(
        &build_create_key_agreement_case_deps(
            Consumer {
                concrete: KeyAgreementConcrete::Mock(
                    MockIKeyAgreementAdapterFailureMode::NonceDrawRefused,
                ),
            },
            Default::default(),
        ),
        build_create_key_agreement_case_params(Default::default()),
        build_create_key_agreement_case_payload(Default::default()),
    );

    let Ok(success) = created else {
        panic!("the case ran")
    };
    assert!(matches!(
        success.output,
        Ok(CreateKeyAgreementSuccessReturn { output: true })
    ));
}

/// Contract: `create_key_agreement`, admitted: the `PairingElGamal` arm's
///   `try_new` returns `Ok(adapter)` → `deps.consumer.consume_key_agreement`,
///   exactly once, with the real concrete →
///   `Ok(CreateKeyAgreementSuccessReturn { output })`, the consumer's output
///   held whole.
/// Arrange: `Consumer` holding the selection `build_key_agreement_concrete`
///   returns for `KeyAgreementConcrete::PairingElGamal`; the block's
///   `IKeyAgreementConsumer` calls `generate_keys` twice over the same draws
///   and returns whether the two possessions' `r1` encodings, read through
///   `possession_components` and `encode_g1`, differ, a difference only the
///   real concrete produces because it draws each nonce from the randomness
///   source, whose counter advances between the calls, while the mock
///   concrete forms its proof from constant nonces and returns equal
///   encodings.
/// Act:     `run_create_key_agreement_case` runs the case, whose consumer's
///   single call is `create_key_agreement`.
/// Assert:  the case success's `output` equals
///   `Ok(CreateKeyAgreementSuccessReturn { output: true })` whole, `true`
///   stated as an independent literal, so a factory that constructs the mock
///   concrete or never calls the consumer fails.
/// Collaborators: as
///   `create_key_agreement_constructs_the_mock_member_for_the_mock_selection`;
///   the key agreement is the real `pairing_elgamal` concrete the real
///   selection constructs over those collaborators.
#[test]
fn create_key_agreement_constructs_the_pairing_elgamal_member_for_the_pairing_elgamal_selection() {
    struct Inner<'a, P: IPairingArithmetic + IPairingReference> {
        pairing: &'a P,
    }

    impl<P: IPairingArithmetic + IPairingReference> IKeyAgreementConsumer<P> for Inner<'_, P> {
        type Output = bool;

        fn consume_key_agreement<K: IKeyAgreementAdapter<Pairing = P>>(
            &self,
            _params: ConsumeKeyAgreementParams,
            payload: ConsumeKeyAgreementPayload<K>,
        ) -> Self::Output {
            let encoded_r1 = |possession: &K::Possession| {
                payload
                    .adapter
                    .possession_components(
                        PossessionComponentsParams,
                        build_possession_components_payload(possession, Default::default()),
                    )
                    .ok()
                    .and_then(|success| {
                        self.pairing
                            .encode_g1(
                                EncodeG1Params,
                                EncodeG1Payload {
                                    point: success.components.r1,
                                },
                            )
                            .ok()
                            .map(|encoded| encoded.bytes)
                    })
            };
            let first = payload.adapter.generate_keys(
                build_generate_keys_params(Default::default()),
                build_generate_keys_payload::<P>(self.pairing, Default::default()),
            );
            let second = payload.adapter.generate_keys(
                build_generate_keys_params(Default::default()),
                build_generate_keys_payload::<P>(self.pairing, Default::default()),
            );
            match (first, second) {
                (Ok(first), Ok(second)) => {
                    encoded_r1(&first.possession) != encoded_r1(&second.possession)
                }
                _ => false,
            }
        }
    }

    struct Consumer {
        concrete: KeyAgreementConcrete,
    }

    impl ICreateKeyAgreementCaseConsumer for Consumer {
        type Output = CreateKeyAgreementReturn<bool>;

        fn consume_create_key_agreement_case<
            P: IPairingArithmetic + IPairingReference,
            E: IEncoderAdapter + IDecoderAdapter,
            H: IHashToScalarAdapter<P::Scalar>,
            R: IRandomSourceAdapter,
            K: ICredentialKemAdapter<Pairing = P>,
        >(
            &self,
            _params: ConsumeCreateKeyAgreementCaseParams,
            payload: ConsumeCreateKeyAgreementCasePayload<'_, P, E, H, R, K>,
        ) -> Self::Output {
            let _case = payload.case;
            create_key_agreement(
                &build_create_key_agreement_deps(
                    payload.pairing,
                    payload.hash_to_scalar,
                    payload.encoder,
                    payload.random,
                    Inner {
                        pairing: payload.pairing,
                    },
                    Default::default(),
                ),
                build_create_key_agreement_params(CreateKeyAgreementParamsOverrides {
                    concrete: Some(self.concrete),
                }),
                build_create_key_agreement_payload(Default::default()),
            )
        }
    }

    let created = run_create_key_agreement_case(
        &build_create_key_agreement_case_deps(
            Consumer {
                concrete: build_key_agreement_concrete(
                    KeyAgreementConcrete::PairingElGamal,
                    Default::default(),
                ),
            },
            Default::default(),
        ),
        build_create_key_agreement_case_params(Default::default()),
        build_create_key_agreement_case_payload(Default::default()),
    );

    let Ok(success) = created else {
        panic!("the case ran")
    };
    assert!(matches!(
        success.output,
        Ok(CreateKeyAgreementSuccessReturn { output: true })
    ));
}
