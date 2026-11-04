#![allow(clippy::expect_used)]

use chain::IChainForms;
use domain::Secret;
use encoding::{IDecoderAdapter, IEncoderAdapter};
use envelope::{
    CreateKeyAgreementParamsOverrides, GenerateKeysErrorReturn, IKeyAgreementAdapter,
    KeyAgreementConcrete, MockIKeyAgreementAdapterFailureMode,
    MockIKeyAgreementAdapterGenerateKeysErrorReturn, MockIKeyAgreementAdapterWrapToErrorReturn,
    WrapToErrorReturn, build_create_key_agreement_params,
};
use hash_to_scalar::{
    CreateHashToScalarParamsOverrides, HashToScalarConcrete, IHashToScalarAdapter,
    MockIHashToScalarAdapterFailureMode, build_create_hash_to_scalar_params,
    build_hash_to_scalar_params, build_hash_to_scalar_payload,
};
use hashing::{RootConstructorParamsOverrides, build_root};
use kdf::{
    CreateKeyDerivationParamsOverrides, DeriveKeyErrorReturn, IKeyDerivationAdapter, KdfConcrete,
    MockIKeyDerivationAdapterDeriveKeyErrorReturn, MockIKeyDerivationAdapterFailureMode,
    build_create_key_derivation_params,
};
use kem::{
    DeriveIdentityErrorReturn, ICredentialKemAdapter, SetupErrorReturn, build_encapsulate_params,
    build_encapsulate_payload, build_setup_params, build_setup_payload,
};
use pairing::{
    CreatePairingParamsOverrides, IPairingArithmetic, IPairingReference, ISampleUniformScalar,
    MockIPairingAdapterFailureMode, PairingConcrete, SampleUniformScalarParams,
    SampleUniformScalarPayload, build_create_pairing_params,
};
use proof::{
    CreateDeliveryProofParamsOverrides, DELIVERY_PURPOSE_MINT, DELIVERY_PURPOSE_TRANSFER,
    DeliveryContextOverrides, DeliveryProofConcrete, IDeliveryProofAdapter, MintPurpose,
    MockIDeliveryProofAdapterFailureMode, MockIDeliveryProofAdapterProveMintErrorReturn,
    ProveMintErrorReturn, TransferPurpose, build_create_delivery_proof_params,
    build_delivery_context,
};
use random::{
    CreateRandomSourceParamsOverrides, FillBytesParams, FillBytesPayloadOverrides,
    IRandomSourceAdapter, MockIRandomSourceAdapterFailureMode, RandomSourceKind,
    build_create_random_source_params, build_fill_bytes_payload,
};
use workflows::{WrapPieceGroupKeyErrorReturn, build_derivation_context};

use super::cross_holder::provides::CrossHolderErrorReturn;
use super::cross_set::provides::CrossSetErrorReturn;
use super::interface::{IVectorsCaseConsumer, VectorsErrorReturn};
use super::mock::{
    VectorsCaseParamsOverrides, VectorsPayloadOverrides, build_vectors_case_deps,
    build_vectors_case_params, build_vectors_case_payload, build_vectors_deps,
    build_vectors_params, build_vectors_payload, run_vectors_case,
};
use super::replay::provides::{NextSettlementField, ReplayErrorReturn};
use super::statement_mutations::provides::StatementMutationsErrorReturn;
use super::vectors;
use crate::delivery::provides::{AdmittedKemIdentity, MintErrorReturn};

const CANONICAL: &[u8] = b"pkg@1.0.0#1";
const OTHER_CANONICAL: &[u8] = b"pkg@2.0.0#7";

fn asset_root() -> hashing::Root {
    build_root(RootConstructorParamsOverrides {
        bytes: Some([0x44; 32]),
    })
}

fn other_asset_root() -> hashing::Root {
    build_root(RootConstructorParamsOverrides {
        bytes: Some([0x45; 32]),
    })
}

fn draw_uniform<P: IPairingArithmetic>(source: &dyn IRandomSourceAdapter) -> Secret<Vec<u8>> {
    source
        .fill_bytes(
            FillBytesParams,
            build_fill_bytes_payload(FillBytesPayloadOverrides {
                length: Some(<P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH),
            }),
        )
        .expect("the independent source fills")
        .bytes
}

fn default_contexts<F: IChainForms>() -> (
    proof::DeliveryContext<F, MintPurpose>,
    proof::DeliveryContext<F, TransferPurpose>,
    proof::DeliveryContext<F, TransferPurpose>,
) {
    let mint = build_delivery_context::<F, MintPurpose>(DELIVERY_PURPOSE_MINT, Default::default());
    let transfer =
        build_delivery_context::<F, TransferPurpose>(DELIVERY_PURPOSE_TRANSFER, Default::default());
    let next = build_delivery_context::<F, TransferPurpose>(
        DELIVERY_PURPOSE_TRANSFER,
        DeliveryContextOverrides {
            chain: Some(transfer.chain.clone()),
            entitlement_contract: Some(transfer.entitlement_contract.clone()),
            asset_root: Some(transfer.asset_root),
            parameter_set_identifier: Some(transfer.parameter_set_identifier.clone()),
            source_entitlement: Some(transfer.source_entitlement.clone()),
            target_entitlement: Some(transfer.target_entitlement.clone()),
            old_interval: Some(transfer.new_interval.clone()),
            seller: Some(transfer.buyer.clone()),
            ..Default::default()
        },
    );
    (mint, transfer, next)
}

/// Contract: Cross-holder set, `cross_holder` refuses →
///   `Err(VectorsErrorReturn::CrossHolder(error))`, `error` unchanged.
/// Arrange: `build_create_random_source_params` overriding `kind` with
///   `RandomSourceKind::Mock(MockIRandomSourceAdapterFailureMode::
///   FillBytesRefused)` for the subject's source and the independent source;
///   the expected `error` the refusal of the independent source's
///   `fill_bytes`.
/// Act:     `vectors`, once, in the case consumer's method.
/// Assert:  the result's `Err` equals `Some(VectorsErrorReturn::CrossHolder(
///   CrossHolderErrorReturn::FillBytes(error)))` whole; the outermost `Result`
///   `is_ok()`, proving the runner's levels ran.
/// Collaborators: every family's mock concrete, reached through its factory at
///   the defaults of its params builder, except the randomness selection,
///   which this block names.
#[test]
fn vectors_returns_the_draw_refusal_inside_the_cross_holder_variant() {
    struct Consumer;

    impl IVectorsCaseConsumer for Consumer {
        type Output = ();

        fn consume_vectors_case<P, H, E, F, K, A, D>(
            &self,
            pairing: &P,
            _hash_to_scalar: &H,
            encoder: &E,
            _forms: &F,
            kem: &K,
            key_agreement: &A,
            delivery_proof: &D,
            random: &dyn IRandomSourceAdapter,
            independent: &dyn IRandomSourceAdapter,
            kdf: &dyn IKeyDerivationAdapter,
            identity: AdmittedKemIdentity<'_>,
            other_identity: AdmittedKemIdentity<'_>,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            H: ?Sized + IHashToScalarAdapter<P::Scalar>,
            E: IEncoderAdapter + IDecoderAdapter,
            F: IChainForms,
            K: ICredentialKemAdapter<Pairing = P>,
            A: IKeyAgreementAdapter<Pairing = P>,
            D: IDeliveryProofAdapter<Pairing = P, Forms = F>,
        {
            // Arrange
            let expected = independent
                .fill_bytes(
                    FillBytesParams,
                    build_fill_bytes_payload(FillBytesPayloadOverrides {
                        length: Some(<P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH),
                    }),
                )
                .err()
                .expect("the independent source refuses under its named mode");
            let context = build_derivation_context(Default::default());
            let (mint, transfer, next) = default_contexts::<F>();
            // Act
            let returned = vectors::<P, K, A, D, E>(
                &build_vectors_deps(
                    pairing,
                    kem,
                    key_agreement,
                    delivery_proof,
                    random,
                    kdf,
                    encoder,
                    Default::default(),
                ),
                build_vectors_params(Default::default()),
                build_vectors_payload(
                    &context,
                    &mint,
                    &transfer,
                    &next,
                    VectorsPayloadOverrides {
                        identity: Some(identity),
                        other_identity: Some(other_identity),
                    },
                ),
            );
            // Assert
            assert_eq!(
                returned.err(),
                Some(VectorsErrorReturn::CrossHolder(
                    CrossHolderErrorReturn::FillBytes(expected)
                ))
            );
        }
    }
    let result = run_vectors_case(
        &build_vectors_case_deps(Consumer, Default::default()),
        build_vectors_case_params(VectorsCaseParamsOverrides {
            random_source: Some(build_create_random_source_params(
                CreateRandomSourceParamsOverrides {
                    kind: Some(RandomSourceKind::Mock(
                        MockIRandomSourceAdapterFailureMode::FillBytesRefused,
                    )),
                    ..Default::default()
                },
            )),
            ..Default::default()
        }),
        build_vectors_case_payload(
            CANONICAL,
            OTHER_CANONICAL,
            &asset_root(),
            &other_asset_root(),
            Default::default(),
        ),
    );
    assert!(result.is_ok());
}

/// Contract: Cross-holder set, `cross_holder`'s `setup` refuses its master
///   sampling → `Err(VectorsErrorReturn::CrossHolder(error))`, `error`
///   unchanged.
/// Arrange: `build_create_pairing_params` overriding `concrete` with
///   `PairingConcrete::Mock(MockIPairingAdapterFailureMode::
///   SampleScalarWrongLength)`; the expected `error` the `Err` of
///   `<P::Scalar as ISampleUniformScalar>::sample_from_uniform_bytes` over the
///   independent source's first draw.
/// Act:     `vectors`, once, in the case consumer's method.
/// Assert:  the result's `Err` equals `Some(VectorsErrorReturn::CrossHolder(
///   CrossHolderErrorReturn::Setup(SetupErrorReturn::MasterScalarSampling(
///   error))))` whole; the outermost `Result` `is_ok()`.
/// Collaborators: every family's mock concrete, reached through its factory at
///   the defaults of its params builder, except the pairing, which this block
///   names under `SampleScalarWrongLength`.
#[test]
fn vectors_returns_the_setup_sampling_refusal_inside_the_cross_holder_variant() {
    struct Consumer;

    impl IVectorsCaseConsumer for Consumer {
        type Output = ();

        fn consume_vectors_case<P, H, E, F, K, A, D>(
            &self,
            pairing: &P,
            _hash_to_scalar: &H,
            encoder: &E,
            _forms: &F,
            kem: &K,
            key_agreement: &A,
            delivery_proof: &D,
            random: &dyn IRandomSourceAdapter,
            independent: &dyn IRandomSourceAdapter,
            kdf: &dyn IKeyDerivationAdapter,
            identity: AdmittedKemIdentity<'_>,
            other_identity: AdmittedKemIdentity<'_>,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            H: ?Sized + IHashToScalarAdapter<P::Scalar>,
            E: IEncoderAdapter + IDecoderAdapter,
            F: IChainForms,
            K: ICredentialKemAdapter<Pairing = P>,
            A: IKeyAgreementAdapter<Pairing = P>,
            D: IDeliveryProofAdapter<Pairing = P, Forms = F>,
        {
            // Arrange
            let uniform = draw_uniform::<P>(independent);
            let expected = <P::Scalar as ISampleUniformScalar>::sample_from_uniform_bytes(
                SampleUniformScalarParams,
                SampleUniformScalarPayload { uniform },
            )
            .err()
            .expect("the pairing's scalar sampling refuses under its named mode");
            let context = build_derivation_context(Default::default());
            let (mint, transfer, next) = default_contexts::<F>();
            // Act
            let returned = vectors::<P, K, A, D, E>(
                &build_vectors_deps(
                    pairing,
                    kem,
                    key_agreement,
                    delivery_proof,
                    random,
                    kdf,
                    encoder,
                    Default::default(),
                ),
                build_vectors_params(Default::default()),
                build_vectors_payload(
                    &context,
                    &mint,
                    &transfer,
                    &next,
                    VectorsPayloadOverrides {
                        identity: Some(identity),
                        other_identity: Some(other_identity),
                    },
                ),
            );
            // Assert
            assert_eq!(
                returned.err(),
                Some(VectorsErrorReturn::CrossHolder(
                    CrossHolderErrorReturn::Setup(SetupErrorReturn::MasterScalarSampling(expected))
                ))
            );
        }
    }
    let result = run_vectors_case(
        &build_vectors_case_deps(Consumer, Default::default()),
        build_vectors_case_params(VectorsCaseParamsOverrides {
            pairing: Some(build_create_pairing_params(CreatePairingParamsOverrides {
                concrete: Some(PairingConcrete::Mock(
                    MockIPairingAdapterFailureMode::SampleScalarWrongLength,
                )),
                ..Default::default()
            })),
            ..Default::default()
        }),
        build_vectors_case_payload(
            CANONICAL,
            OTHER_CANONICAL,
            &asset_root(),
            &other_asset_root(),
            Default::default(),
        ),
    );
    assert!(result.is_ok());
}

/// Contract: Cross-holder set, `cross_holder`'s `derive_identity` refuses →
///   `Err(VectorsErrorReturn::CrossHolder(error))`, `error` unchanged.
/// Arrange: `build_create_hash_to_scalar_params` overriding `concrete` with
///   `HashToScalarConcrete::Mock(MockIHashToScalarAdapterFailureMode::
///   SamplingRefused)`; the expected refusal the `Err` of `hash_to_scalar`
///   over `build_hash_to_scalar_params` and `build_hash_to_scalar_payload` at
///   their defaults on an adapter built under the same mode.
/// Act:     `vectors`, once, in the case consumer's method.
/// Assert:  the result's `Err` equals `Some(VectorsErrorReturn::CrossHolder(
///   CrossHolderErrorReturn::DeriveIdentity(DeriveIdentityErrorReturn::
///   HashToScalar(error))))` whole; the outermost `Result` `is_ok()`.
/// Collaborators: every family's mock concrete, reached through its factory at
///   the defaults of its params builder, except the hash-to-scalar family,
///   which this block names under `SamplingRefused`.
#[test]
fn vectors_returns_the_hash_to_scalar_refusal_inside_the_cross_holder_variant() {
    struct Consumer;

    impl IVectorsCaseConsumer for Consumer {
        type Output = ();

        fn consume_vectors_case<P, H, E, F, K, A, D>(
            &self,
            pairing: &P,
            hash_to_scalar: &H,
            encoder: &E,
            _forms: &F,
            kem: &K,
            key_agreement: &A,
            delivery_proof: &D,
            random: &dyn IRandomSourceAdapter,
            _independent: &dyn IRandomSourceAdapter,
            kdf: &dyn IKeyDerivationAdapter,
            identity: AdmittedKemIdentity<'_>,
            other_identity: AdmittedKemIdentity<'_>,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            H: ?Sized + IHashToScalarAdapter<P::Scalar>,
            E: IEncoderAdapter + IDecoderAdapter,
            F: IChainForms,
            K: ICredentialKemAdapter<Pairing = P>,
            A: IKeyAgreementAdapter<Pairing = P>,
            D: IDeliveryProofAdapter<Pairing = P, Forms = F>,
        {
            // Arrange
            let expected = hash_to_scalar
                .hash_to_scalar(
                    build_hash_to_scalar_params(Default::default()),
                    build_hash_to_scalar_payload(Default::default()),
                )
                .err()
                .expect("the hash-to-scalar adapter refuses under its named mode");
            let context = build_derivation_context(Default::default());
            let (mint, transfer, next) = default_contexts::<F>();
            // Act
            let returned = vectors::<P, K, A, D, E>(
                &build_vectors_deps(
                    pairing,
                    kem,
                    key_agreement,
                    delivery_proof,
                    random,
                    kdf,
                    encoder,
                    Default::default(),
                ),
                build_vectors_params(Default::default()),
                build_vectors_payload(
                    &context,
                    &mint,
                    &transfer,
                    &next,
                    VectorsPayloadOverrides {
                        identity: Some(identity),
                        other_identity: Some(other_identity),
                    },
                ),
            );
            // Assert
            assert_eq!(
                returned.err(),
                Some(VectorsErrorReturn::CrossHolder(
                    CrossHolderErrorReturn::DeriveIdentity(
                        DeriveIdentityErrorReturn::HashToScalar(expected),
                    )
                ))
            );
        }
    }
    let result = run_vectors_case(
        &build_vectors_case_deps(Consumer, Default::default()),
        build_vectors_case_params(VectorsCaseParamsOverrides {
            hash_to_scalar: Some(build_create_hash_to_scalar_params(
                CreateHashToScalarParamsOverrides {
                    concrete: Some(HashToScalarConcrete::Mock(
                        MockIHashToScalarAdapterFailureMode::SamplingRefused,
                    )),
                    ..Default::default()
                },
            )),
            ..Default::default()
        }),
        build_vectors_case_payload(
            CANONICAL,
            OTHER_CANONICAL,
            &asset_root(),
            &other_asset_root(),
            Default::default(),
        ),
    );
    assert!(result.is_ok());
}

/// Contract: Cross-set set, `cross_set` refuses →
///   `Err(VectorsErrorReturn::CrossSet(error))`, `error` unchanged.
/// Arrange: `build_create_key_derivation_params` overriding `concrete` with
///   `KdfConcrete::Mock(MockIKeyDerivationAdapterFailureMode::
///   KeyMaterialLengthUnrepresentable)`; the expected `length` the length of
///   the `key_material()` of an `encapsulate` on the received KEM over its
///   builders' defaults.
/// Act:     `vectors`, once, in the case consumer's method.
/// Assert:  the result's `Err` equals `Some(VectorsErrorReturn::CrossSet(
///   CrossSetErrorReturn::Wrap(WrapPieceGroupKeyErrorReturn::KeyDerivation(
///   DeriveKeyErrorReturn::Mock(MockIKeyDerivationAdapterDeriveKeyErrorReturn::
///   KeyMaterialLengthUnrepresentable { length })))))` whole; the outermost
///   `Result` `is_ok()`.
/// Collaborators: every family's mock concrete at the defaults of its params
///   builder, except the key-derivation family's, which this block names
///   under `KeyMaterialLengthUnrepresentable`.
#[test]
fn vectors_returns_the_key_derivation_refusal_inside_the_cross_set_variant() {
    struct Consumer;

    impl IVectorsCaseConsumer for Consumer {
        type Output = ();

        fn consume_vectors_case<P, H, E, F, K, A, D>(
            &self,
            pairing: &P,
            _hash_to_scalar: &H,
            encoder: &E,
            _forms: &F,
            kem: &K,
            key_agreement: &A,
            delivery_proof: &D,
            random: &dyn IRandomSourceAdapter,
            _independent: &dyn IRandomSourceAdapter,
            kdf: &dyn IKeyDerivationAdapter,
            identity: AdmittedKemIdentity<'_>,
            other_identity: AdmittedKemIdentity<'_>,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            H: ?Sized + IHashToScalarAdapter<P::Scalar>,
            E: IEncoderAdapter + IDecoderAdapter,
            F: IChainForms,
            K: ICredentialKemAdapter<Pairing = P>,
            A: IKeyAgreementAdapter<Pairing = P>,
            D: IDeliveryProofAdapter<Pairing = P, Forms = F>,
        {
            // Arrange
            let setup = kem
                .setup(
                    build_setup_params(Default::default()),
                    build_setup_payload::<P::Scalar>(Default::default()),
                )
                .expect("the replay's setup succeeds");
            let encapsulated = kem
                .encapsulate(
                    build_encapsulate_params(Default::default()),
                    build_encapsulate_payload::<P::Scalar, _>(
                        &setup.parameter_set,
                        Default::default(),
                    ),
                )
                .expect("the replay's encapsulation succeeds");
            let length = encapsulated.encapsulated.key_material().expose().len();
            let context = build_derivation_context(Default::default());
            let (mint, transfer, next) = default_contexts::<F>();
            // Act
            let returned = vectors::<P, K, A, D, E>(
                &build_vectors_deps(
                    pairing,
                    kem,
                    key_agreement,
                    delivery_proof,
                    random,
                    kdf,
                    encoder,
                    Default::default(),
                ),
                build_vectors_params(Default::default()),
                build_vectors_payload(
                    &context,
                    &mint,
                    &transfer,
                    &next,
                    VectorsPayloadOverrides {
                        identity: Some(identity),
                        other_identity: Some(other_identity),
                    },
                ),
            );
            // Assert
            assert_eq!(
                returned.err(),
                Some(VectorsErrorReturn::CrossSet(CrossSetErrorReturn::Wrap(
                    WrapPieceGroupKeyErrorReturn::KeyDerivation(DeriveKeyErrorReturn::Mock(
                        MockIKeyDerivationAdapterDeriveKeyErrorReturn::KeyMaterialLengthUnrepresentable {
                            length,
                        },
                    ))
                )))
            );
        }
    }
    let result = run_vectors_case(
        &build_vectors_case_deps(Consumer, Default::default()),
        build_vectors_case_params(VectorsCaseParamsOverrides {
            key_derivation: Some(build_create_key_derivation_params(
                CreateKeyDerivationParamsOverrides {
                    concrete: Some(KdfConcrete::Mock(
                        MockIKeyDerivationAdapterFailureMode::KeyMaterialLengthUnrepresentable,
                    )),
                    ..Default::default()
                },
            )),
            ..Default::default()
        }),
        build_vectors_case_payload(
            CANONICAL,
            OTHER_CANONICAL,
            &asset_root(),
            &other_asset_root(),
            Default::default(),
        ),
    );
    assert!(result.is_ok());
}

/// Contract: Statement-mutation set, `statement_mutations`'s mint delivery
///   refuses → `Err(VectorsErrorReturn::StatementMutations(error))`, `error`
///   unchanged.
/// Arrange: `build_create_key_agreement_params` overriding `concrete` with
///   `KeyAgreementConcrete::Mock(MockIKeyAgreementAdapterFailureMode::
///   NonceDrawRefused)`.
/// Act:     `vectors`, once, in the case consumer's method.
/// Assert:  the result's `Err` equals `Some(VectorsErrorReturn::
///   StatementMutations(StatementMutationsErrorReturn::Mint(MintErrorReturn::
///   GenerateKeys(GenerateKeysErrorReturn::Mock(
///   MockIKeyAgreementAdapterGenerateKeysErrorReturn::NonceDrawRefused)))))`
///   whole; the outermost `Result` `is_ok()`.
/// Collaborators: every family's mock concrete at the defaults of its params
///   builder, except the key-agreement family's, which this block names under
///   `NonceDrawRefused`.
#[test]
fn vectors_returns_the_key_generation_refusal_inside_the_statement_mutations_variant() {
    struct Consumer;

    impl IVectorsCaseConsumer for Consumer {
        type Output = ();

        fn consume_vectors_case<P, H, E, F, K, A, D>(
            &self,
            pairing: &P,
            _hash_to_scalar: &H,
            encoder: &E,
            _forms: &F,
            kem: &K,
            key_agreement: &A,
            delivery_proof: &D,
            random: &dyn IRandomSourceAdapter,
            _independent: &dyn IRandomSourceAdapter,
            kdf: &dyn IKeyDerivationAdapter,
            identity: AdmittedKemIdentity<'_>,
            other_identity: AdmittedKemIdentity<'_>,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            H: ?Sized + IHashToScalarAdapter<P::Scalar>,
            E: IEncoderAdapter + IDecoderAdapter,
            F: IChainForms,
            K: ICredentialKemAdapter<Pairing = P>,
            A: IKeyAgreementAdapter<Pairing = P>,
            D: IDeliveryProofAdapter<Pairing = P, Forms = F>,
        {
            // Arrange
            let context = build_derivation_context(Default::default());
            let (mint, transfer, next) = default_contexts::<F>();
            // Act
            let returned = vectors::<P, K, A, D, E>(
                &build_vectors_deps(
                    pairing,
                    kem,
                    key_agreement,
                    delivery_proof,
                    random,
                    kdf,
                    encoder,
                    Default::default(),
                ),
                build_vectors_params(Default::default()),
                build_vectors_payload(
                    &context,
                    &mint,
                    &transfer,
                    &next,
                    VectorsPayloadOverrides {
                        identity: Some(identity),
                        other_identity: Some(other_identity),
                    },
                ),
            );
            // Assert
            assert_eq!(
                returned.err(),
                Some(VectorsErrorReturn::StatementMutations(
                    StatementMutationsErrorReturn::Mint(MintErrorReturn::GenerateKeys(
                        GenerateKeysErrorReturn::Mock(
                            MockIKeyAgreementAdapterGenerateKeysErrorReturn::NonceDrawRefused,
                        )
                    ))
                ))
            );
        }
    }
    let result = run_vectors_case(
        &build_vectors_case_deps(Consumer, Default::default()),
        build_vectors_case_params(VectorsCaseParamsOverrides {
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
        build_vectors_case_payload(
            CANONICAL,
            OTHER_CANONICAL,
            &asset_root(),
            &other_asset_root(),
            Default::default(),
        ),
    );
    assert!(result.is_ok());
}

/// Contract: Statement-mutation set, `statement_mutations`'s mint delivery's
///   wrap refuses → `Err(VectorsErrorReturn::StatementMutations(error))`,
///   `error` unchanged.
/// Arrange: `build_create_key_agreement_params` overriding `concrete` with
///   `KeyAgreementConcrete::Mock(MockIKeyAgreementAdapterFailureMode::
///   CoinDrawRefused)`.
/// Act:     `vectors`, once, in the case consumer's method.
/// Assert:  the result's `Err` equals `Some(VectorsErrorReturn::
///   StatementMutations(StatementMutationsErrorReturn::Mint(MintErrorReturn::
///   WrapTo(WrapToErrorReturn::Mock(
///   MockIKeyAgreementAdapterWrapToErrorReturn::CoinDrawRefused)))))` whole;
///   the outermost `Result` `is_ok()`.
/// Collaborators: every family's mock concrete at the defaults of its params
///   builder, except the key-agreement family's, which this block names under
///   `CoinDrawRefused`.
#[test]
fn vectors_returns_the_wrap_refusal_inside_the_statement_mutations_variant() {
    struct Consumer;

    impl IVectorsCaseConsumer for Consumer {
        type Output = ();

        fn consume_vectors_case<P, H, E, F, K, A, D>(
            &self,
            pairing: &P,
            _hash_to_scalar: &H,
            encoder: &E,
            _forms: &F,
            kem: &K,
            key_agreement: &A,
            delivery_proof: &D,
            random: &dyn IRandomSourceAdapter,
            _independent: &dyn IRandomSourceAdapter,
            kdf: &dyn IKeyDerivationAdapter,
            identity: AdmittedKemIdentity<'_>,
            other_identity: AdmittedKemIdentity<'_>,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            H: ?Sized + IHashToScalarAdapter<P::Scalar>,
            E: IEncoderAdapter + IDecoderAdapter,
            F: IChainForms,
            K: ICredentialKemAdapter<Pairing = P>,
            A: IKeyAgreementAdapter<Pairing = P>,
            D: IDeliveryProofAdapter<Pairing = P, Forms = F>,
        {
            // Arrange
            let context = build_derivation_context(Default::default());
            let (mint, transfer, next) = default_contexts::<F>();
            // Act
            let returned = vectors::<P, K, A, D, E>(
                &build_vectors_deps(
                    pairing,
                    kem,
                    key_agreement,
                    delivery_proof,
                    random,
                    kdf,
                    encoder,
                    Default::default(),
                ),
                build_vectors_params(Default::default()),
                build_vectors_payload(
                    &context,
                    &mint,
                    &transfer,
                    &next,
                    VectorsPayloadOverrides {
                        identity: Some(identity),
                        other_identity: Some(other_identity),
                    },
                ),
            );
            // Assert
            assert_eq!(
                returned.err(),
                Some(VectorsErrorReturn::StatementMutations(
                    StatementMutationsErrorReturn::Mint(MintErrorReturn::WrapTo(
                        WrapToErrorReturn::Mock(
                            MockIKeyAgreementAdapterWrapToErrorReturn::CoinDrawRefused,
                        )
                    ))
                ))
            );
        }
    }
    let result = run_vectors_case(
        &build_vectors_case_deps(Consumer, Default::default()),
        build_vectors_case_params(VectorsCaseParamsOverrides {
            key_agreement: Some(build_create_key_agreement_params(
                CreateKeyAgreementParamsOverrides {
                    concrete: Some(KeyAgreementConcrete::Mock(
                        MockIKeyAgreementAdapterFailureMode::CoinDrawRefused,
                    )),
                    ..Default::default()
                },
            )),
            ..Default::default()
        }),
        build_vectors_case_payload(
            CANONICAL,
            OTHER_CANONICAL,
            &asset_root(),
            &other_asset_root(),
            Default::default(),
        ),
    );
    assert!(result.is_ok());
}

/// Contract: Statement-mutation set, `statement_mutations`'s mint delivery's
///   prover refuses → `Err(VectorsErrorReturn::StatementMutations(error))`,
///   `error` unchanged.
/// Arrange: `build_create_delivery_proof_params` overriding `concrete` with
///   `DeliveryProofConcrete::Mock(MockIDeliveryProofAdapterFailureMode::
///   NonceDrawRefused)`.
/// Act:     `vectors`, once, in the case consumer's method.
/// Assert:  the result's `Err` equals `Some(VectorsErrorReturn::
///   StatementMutations(StatementMutationsErrorReturn::Mint(MintErrorReturn::
///   ProveMint(ProveMintErrorReturn::Mock(
///   MockIDeliveryProofAdapterProveMintErrorReturn::NonceDrawRefused)))))`
///   whole; the outermost `Result` `is_ok()`.
/// Collaborators: every family's mock concrete at the defaults of its params
///   builder, except the delivery-proof family's, which this block names
///   under `NonceDrawRefused`.
#[test]
fn vectors_returns_the_mint_nonce_draw_refusal_inside_the_statement_mutations_variant() {
    struct Consumer;

    impl IVectorsCaseConsumer for Consumer {
        type Output = ();

        fn consume_vectors_case<P, H, E, F, K, A, D>(
            &self,
            pairing: &P,
            _hash_to_scalar: &H,
            encoder: &E,
            _forms: &F,
            kem: &K,
            key_agreement: &A,
            delivery_proof: &D,
            random: &dyn IRandomSourceAdapter,
            _independent: &dyn IRandomSourceAdapter,
            kdf: &dyn IKeyDerivationAdapter,
            identity: AdmittedKemIdentity<'_>,
            other_identity: AdmittedKemIdentity<'_>,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            H: ?Sized + IHashToScalarAdapter<P::Scalar>,
            E: IEncoderAdapter + IDecoderAdapter,
            F: IChainForms,
            K: ICredentialKemAdapter<Pairing = P>,
            A: IKeyAgreementAdapter<Pairing = P>,
            D: IDeliveryProofAdapter<Pairing = P, Forms = F>,
        {
            // Arrange
            let context = build_derivation_context(Default::default());
            let (mint, transfer, next) = default_contexts::<F>();
            // Act
            let returned = vectors::<P, K, A, D, E>(
                &build_vectors_deps(
                    pairing,
                    kem,
                    key_agreement,
                    delivery_proof,
                    random,
                    kdf,
                    encoder,
                    Default::default(),
                ),
                build_vectors_params(Default::default()),
                build_vectors_payload(
                    &context,
                    &mint,
                    &transfer,
                    &next,
                    VectorsPayloadOverrides {
                        identity: Some(identity),
                        other_identity: Some(other_identity),
                    },
                ),
            );
            // Assert
            assert_eq!(
                returned.err(),
                Some(VectorsErrorReturn::StatementMutations(
                    StatementMutationsErrorReturn::Mint(MintErrorReturn::ProveMint(
                        ProveMintErrorReturn::Mock(
                            MockIDeliveryProofAdapterProveMintErrorReturn::NonceDrawRefused,
                        )
                    ))
                ))
            );
        }
    }
    let result = run_vectors_case(
        &build_vectors_case_deps(Consumer, Default::default()),
        build_vectors_case_params(VectorsCaseParamsOverrides {
            delivery_proof: Some(build_create_delivery_proof_params(
                CreateDeliveryProofParamsOverrides {
                    concrete: Some(DeliveryProofConcrete::Mock(
                        MockIDeliveryProofAdapterFailureMode::NonceDrawRefused,
                    )),
                    ..Default::default()
                },
            )),
            ..Default::default()
        }),
        build_vectors_case_payload(
            CANONICAL,
            OTHER_CANONICAL,
            &asset_root(),
            &other_asset_root(),
            Default::default(),
        ),
    );
    assert!(result.is_ok());
}

/// Contract: Replay set, `replay` refuses the next context →
///   `Err(VectorsErrorReturn::Replay(error))`, `error` unchanged.
/// Arrange: the next context overriding `old_interval` with a clone of the
///   transfer context's `old_interval`, the transfer's old interval and not
///   its new one, the default forms being seeded distinctly.
/// Act:     `vectors`, once, in the case consumer's method.
/// Assert:  the result's `Err` equals `Some(VectorsErrorReturn::Replay(
///   ReplayErrorReturn::NotTheNextSettlement { field: NextSettlementField::
///   OldInterval }))` whole; the outermost `Result` `is_ok()`.
/// Collaborators: every family's mock concrete, reached through its factory at
///   the defaults of its params builder.
#[test]
fn vectors_returns_the_next_settlement_refusal_inside_the_replay_variant() {
    struct Consumer;

    impl IVectorsCaseConsumer for Consumer {
        type Output = ();

        fn consume_vectors_case<P, H, E, F, K, A, D>(
            &self,
            pairing: &P,
            _hash_to_scalar: &H,
            encoder: &E,
            _forms: &F,
            kem: &K,
            key_agreement: &A,
            delivery_proof: &D,
            random: &dyn IRandomSourceAdapter,
            _independent: &dyn IRandomSourceAdapter,
            kdf: &dyn IKeyDerivationAdapter,
            identity: AdmittedKemIdentity<'_>,
            other_identity: AdmittedKemIdentity<'_>,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            H: ?Sized + IHashToScalarAdapter<P::Scalar>,
            E: IEncoderAdapter + IDecoderAdapter,
            F: IChainForms,
            K: ICredentialKemAdapter<Pairing = P>,
            A: IKeyAgreementAdapter<Pairing = P>,
            D: IDeliveryProofAdapter<Pairing = P, Forms = F>,
        {
            // Arrange
            let context = build_derivation_context(Default::default());
            let mint =
                build_delivery_context::<F, MintPurpose>(DELIVERY_PURPOSE_MINT, Default::default());
            let transfer = build_delivery_context::<F, TransferPurpose>(
                DELIVERY_PURPOSE_TRANSFER,
                Default::default(),
            );
            let next = build_delivery_context::<F, TransferPurpose>(
                DELIVERY_PURPOSE_TRANSFER,
                DeliveryContextOverrides {
                    chain: Some(transfer.chain.clone()),
                    entitlement_contract: Some(transfer.entitlement_contract.clone()),
                    asset_root: Some(transfer.asset_root),
                    parameter_set_identifier: Some(transfer.parameter_set_identifier.clone()),
                    source_entitlement: Some(transfer.source_entitlement.clone()),
                    target_entitlement: Some(transfer.target_entitlement.clone()),
                    old_interval: Some(transfer.old_interval.clone()),
                    seller: Some(transfer.buyer.clone()),
                    ..Default::default()
                },
            );
            // Act
            let returned = vectors::<P, K, A, D, E>(
                &build_vectors_deps(
                    pairing,
                    kem,
                    key_agreement,
                    delivery_proof,
                    random,
                    kdf,
                    encoder,
                    Default::default(),
                ),
                build_vectors_params(Default::default()),
                build_vectors_payload(
                    &context,
                    &mint,
                    &transfer,
                    &next,
                    VectorsPayloadOverrides {
                        identity: Some(identity),
                        other_identity: Some(other_identity),
                    },
                ),
            );
            // Assert
            assert_eq!(
                returned.err(),
                Some(VectorsErrorReturn::Replay(
                    ReplayErrorReturn::NotTheNextSettlement {
                        field: NextSettlementField::OldInterval,
                    }
                ))
            );
        }
    }
    let result = run_vectors_case(
        &build_vectors_case_deps(Consumer, Default::default()),
        build_vectors_case_params(Default::default()),
        build_vectors_case_payload(
            CANONICAL,
            OTHER_CANONICAL,
            &asset_root(),
            &other_asset_root(),
            Default::default(),
        ),
    );
    assert!(result.is_ok());
}
