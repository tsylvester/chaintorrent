#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::as_conversions
)]

use chain::MockIChainForms;
use domain::{SecretConstructorParamsOverrides, build_secret};
use encoding::{
    ConsumeEncodingParams, ConsumeEncodingPayload, CreateEncodingDeps, CreateEncodingPayload,
    IDecoderAdapter, IEncoderAdapter, IEncodingConsumer, build_create_encoding_params,
    create_encoding,
};
use envelope::EnvelopeAlgebra;
use hash_to_scalar::{
    CreateHashToScalarDeps, CreateHashToScalarPayload, IHashToScalarAdapter,
    build_create_hash_to_scalar_params, create_hash_to_scalar,
};
use pairing::{
    ConsumePairingParams, ConsumePairingPayload, CreatePairingDeps, CreatePairingParamsOverrides,
    CreatePairingPayload, IPairingAdapter, IPairingArithmetic, IPairingConsumer, IPairingReference,
    ISampleUniformScalar, PairingConcrete, SampleUniformScalarParams, SampleUniformScalarPayload,
    VerifierGroupArithmetic, build_create_pairing_params, create_pairing,
};
use random::{
    CreateRandomSourceDeps, CreateRandomSourcePayload, IRandomSourceAdapter,
    build_create_random_source_params, create_random_source,
};

use crate::mint_statement::provides::DELIVERY_STATEMENT_VERSION_ONE;
use crate::schnorr_fs::provides::SchnorrFsProofFromComponentsErrorReturn;

use super::create_delivery_proof;
use super::interface::{
    ConsumeDeliveryProofParams, ConsumeDeliveryProofPayload, CreateDeliveryProofDeps,
    CreateDeliveryProofErrorReturn, CreateDeliveryProofPayload, DELIVERY_PROOF_INTERFACE_VERSION,
    DeliveryProofComponents, DeliveryProofDeclaration, IDeliveryProofAdapter,
    IDeliveryProofConsumer, MintResponses, ProofFromComponentsErrorReturn,
    ProofFromComponentsParams, ProofFromComponentsPayload,
};
use super::mock::{CreateDeliveryProofParamsOverrides, build_create_delivery_proof_params};

fn scalar<P: IPairingAdapter>(fill: u8) -> P::Scalar {
    let Ok(sampled) = P::Scalar::sample_from_uniform_bytes(
        SampleUniformScalarParams,
        SampleUniformScalarPayload {
            uniform: build_secret(SecretConstructorParamsOverrides {
                value: Some(vec![fill; P::Scalar::UNIFORM_BYTES_LENGTH]),
            }),
        },
    ) else {
        panic!("sample_from_uniform_bytes refused a fixed uniform input")
    };
    sampled.scalar.expose().clone()
}

struct FactoryOutcome {
    declaration: Option<DeliveryProofDeclaration>,
    both_groups_fixture_refusal: Option<Option<ProofFromComponentsErrorReturn>>,
    factory_refusal: Option<CreateDeliveryProofErrorReturn>,
}

struct DeclarationProbe;

impl<P: IPairingAdapter> IDeliveryProofConsumer<P, MockIChainForms> for DeclarationProbe {
    type Output = FactoryOutcome;

    fn consume_delivery_proof<D: IDeliveryProofAdapter<Pairing = P, Forms = MockIChainForms>>(
        &self,
        _params: ConsumeDeliveryProofParams,
        payload: ConsumeDeliveryProofPayload<D>,
    ) -> Self::Output {
        let refusal = payload
            .adapter
            .proof_from_components(
                ProofFromComponentsParams,
                ProofFromComponentsPayload {
                    components: DeliveryProofComponents::MintBothGroups {
                        challenge: scalar::<P>(0x60),
                        responses: MintResponses {
                            alpha: scalar::<P>(0x61),
                            r: scalar::<P>(0x62),
                            rho: scalar::<P>(0x63),
                            sigma: scalar::<P>(0x64),
                        },
                    },
                },
            )
            .err();
        FactoryOutcome {
            declaration: Some(D::DECLARATION),
            both_groups_fixture_refusal: Some(refusal),
            factory_refusal: None,
        }
    }
}

struct FactoryStep<'a, P: IPairingArithmetic> {
    pairing: &'a P,
    hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
    random: &'a dyn IRandomSourceAdapter,
    verifier_form: VerifierGroupArithmetic,
    statement_version: u16,
}

impl<P: IPairingArithmetic> IEncodingConsumer for FactoryStep<'_, P> {
    type Output = FactoryOutcome;

    fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
        &self,
        _params: ConsumeEncodingParams,
        payload: ConsumeEncodingPayload<E>,
    ) -> Self::Output {
        match create_delivery_proof(
            &CreateDeliveryProofDeps {
                pairing: self.pairing,
                hash_to_scalar: self.hash_to_scalar,
                encoder: &payload.adapter,
                random: self.random,
                consumer: DeclarationProbe,
            },
            build_create_delivery_proof_params(CreateDeliveryProofParamsOverrides {
                verifier_form: Some(self.verifier_form),
                statement_version: Some(self.statement_version),
                ..Default::default()
            }),
            CreateDeliveryProofPayload,
        ) {
            Ok(success) => success.output,
            Err(error) => FactoryOutcome {
                declaration: None,
                both_groups_fixture_refusal: None,
                factory_refusal: Some(error),
            },
        }
    }
}

struct FactoryProbe {
    verifier_form: VerifierGroupArithmetic,
    statement_version: u16,
}

impl IPairingConsumer for FactoryProbe {
    type Output = FactoryOutcome;

    fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
        &self,
        _params: ConsumePairingParams,
        payload: ConsumePairingPayload<P>,
    ) -> Self::Output {
        // Arrange
        let Ok(hashing) = create_hash_to_scalar::<P::Scalar>(
            &CreateHashToScalarDeps,
            build_create_hash_to_scalar_params(Default::default()),
            CreateHashToScalarPayload,
        ) else {
            panic!("the default hash-to-scalar concrete is constructed")
        };
        let Ok(source) = create_random_source(
            &CreateRandomSourceDeps,
            build_create_random_source_params(Default::default()),
            CreateRandomSourcePayload,
        );

        // Act
        let Ok(success) = create_encoding(
            &CreateEncodingDeps {
                consumer: FactoryStep {
                    pairing: &payload.adapter,
                    hash_to_scalar: hashing.adapter.as_ref(),
                    random: source.adapter.as_ref(),
                    verifier_form: self.verifier_form,
                    statement_version: self.statement_version,
                },
            },
            build_create_encoding_params(Default::default()),
            CreateEncodingPayload,
        ) else {
            panic!("the default encoding concrete is constructed and consumed")
        };

        success.output
    }
}

/// Contract: the Schnorr concrete, admitted for its algebra, form, and
///   statement version, reaches the consumer with its declaration.
/// Arrange: `FactoryProbe` over the both-groups form and statement version one.
/// Act:     `create_pairing` on `PairingConcrete::Bls12381Arkworks`, which
///   constructs the delivery proof through `create_delivery_proof` inside the
///   encoding consumer.
/// Assert:  `declaration` is `Some` with `algebras` equal to
///   `[EnvelopeAlgebra::PairingElGamal]`, `verifier_forms` equal to
///   `[VerifierGroupArithmetic::BothGroups, VerifierGroupArithmetic::FirstGroupOnly]`,
///   `statement_versions` equal to `[DELIVERY_STATEMENT_VERSION_ONE]`,
///   `challenge_tag` equal to `b"ChainTorrent-v1-proof-challenge"`,
///   `weight_tag` equal to `b"ChainTorrent-v1-proof-weight"`, `adapter_version`
///   `1`, and `interface_version` `DELIVERY_PROOF_INTERFACE_VERSION`.
#[test]
fn create_delivery_proof_hands_the_consumer_the_schnorr_fs_concrete_and_its_declaration() {
    // Arrange
    let consumer = FactoryProbe {
        verifier_form: VerifierGroupArithmetic::BothGroups,
        statement_version: DELIVERY_STATEMENT_VERSION_ONE,
    };
    let deps = CreatePairingDeps { consumer };
    let params = build_create_pairing_params(CreatePairingParamsOverrides {
        concrete: Some(PairingConcrete::Bls12381Arkworks),
        ..Default::default()
    });

    // Act
    let Ok(success) = create_pairing(&deps, params, CreatePairingPayload) else {
        panic!("the named concrete is constructed and consumed")
    };

    // Assert
    let Some(declaration) = success.output.declaration else {
        panic!("the admitted concrete reaches the consumer with its declaration")
    };
    assert_eq!(declaration.algebras, [EnvelopeAlgebra::PairingElGamal]);
    assert_eq!(
        declaration.verifier_forms,
        [
            VerifierGroupArithmetic::BothGroups,
            VerifierGroupArithmetic::FirstGroupOnly,
        ]
    );
    assert_eq!(
        declaration.statement_versions,
        [DELIVERY_STATEMENT_VERSION_ONE]
    );
    assert_eq!(
        declaration.challenge_tag,
        b"ChainTorrent-v1-proof-challenge".as_slice()
    );
    assert_eq!(
        declaration.weight_tag,
        b"ChainTorrent-v1-proof-weight".as_slice()
    );
    assert_eq!(declaration.adapter_version, 1);
    assert_eq!(
        declaration.interface_version,
        DELIVERY_PROOF_INTERFACE_VERSION
    );
}

/// Contract: the handed concrete proves and verifies in the admitted form, so
///   a both-groups concrete rebuilds a proof carrying no second-group first
///   messages and a first-group-only concrete refuses one.
/// Arrange: `FactoryProbe` with `verifier_form` `BothGroups`, then
///   `FirstGroupOnly`, each with `statement_version` one.
/// Act:     `create_pairing` on `PairingConcrete::Bls12381Arkworks` for each.
/// Assert:  `both_groups_fixture_refusal` equals `Some(None)` for the
///   both-groups concrete and `Some(Some(ProofFromComponentsErrorReturn::SchnorrFs(
///   SchnorrFsProofFromComponentsErrorReturn::VerifierFormMismatch)))` for the
///   first-group-only concrete.
#[test]
fn create_delivery_proof_constructs_the_concrete_in_the_required_verifier_form() {
    // Arrange
    let both_groups = FactoryProbe {
        verifier_form: VerifierGroupArithmetic::BothGroups,
        statement_version: DELIVERY_STATEMENT_VERSION_ONE,
    };
    let first_group_only = FactoryProbe {
        verifier_form: VerifierGroupArithmetic::FirstGroupOnly,
        statement_version: DELIVERY_STATEMENT_VERSION_ONE,
    };
    let params = || {
        build_create_pairing_params(CreatePairingParamsOverrides {
            concrete: Some(PairingConcrete::Bls12381Arkworks),
            ..Default::default()
        })
    };

    // Act
    let Ok(both_groups_outcome) = create_pairing(
        &CreatePairingDeps {
            consumer: both_groups,
        },
        params(),
        CreatePairingPayload,
    ) else {
        panic!("the named concrete is constructed and consumed")
    };
    let Ok(first_group_only_outcome) = create_pairing(
        &CreatePairingDeps {
            consumer: first_group_only,
        },
        params(),
        CreatePairingPayload,
    ) else {
        panic!("the named concrete is constructed and consumed")
    };

    // Assert
    assert_eq!(
        both_groups_outcome.output.both_groups_fixture_refusal,
        Some(None)
    );
    assert_eq!(
        first_group_only_outcome.output.both_groups_fixture_refusal,
        Some(Some(ProofFromComponentsErrorReturn::SchnorrFs(
            SchnorrFsProofFromComponentsErrorReturn::VerifierFormMismatch,
        )))
    );
}

/// Contract: a concrete is admitted only for the version the hash-card names,
///   nothing constructed otherwise.
/// Arrange: `FactoryProbe` with `verifier_form` `BothGroups` and
///   `statement_version` `2`, which no concrete declares.
/// Act:     `create_pairing` on `PairingConcrete::Bls12381Arkworks`.
/// Assert:  `factory_refusal` equals
///   `Some(CreateDeliveryProofErrorReturn::UnsupportedStatementVersion)` and
///   `declaration` is `None`.
#[test]
fn create_delivery_proof_refuses_a_statement_version_the_concrete_does_not_declare() {
    // Arrange
    let consumer = FactoryProbe {
        verifier_form: VerifierGroupArithmetic::BothGroups,
        statement_version: 2,
    };
    let deps = CreatePairingDeps { consumer };
    let params = build_create_pairing_params(CreatePairingParamsOverrides {
        concrete: Some(PairingConcrete::Bls12381Arkworks),
        ..Default::default()
    });

    // Act
    let Ok(success) = create_pairing(&deps, params, CreatePairingPayload) else {
        panic!("the named concrete is constructed and consumed")
    };

    // Assert
    assert_eq!(
        success.output.factory_refusal,
        Some(CreateDeliveryProofErrorReturn::UnsupportedStatementVersion)
    );
    assert!(success.output.declaration.is_none());
}
