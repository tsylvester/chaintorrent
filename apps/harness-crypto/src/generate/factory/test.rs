#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use chain::IChainForms;
use encoding::{IDecoderAdapter, IEncoderAdapter};
use envelope::IKeyAgreementAdapter;
use hash_to_scalar::IHashToScalarAdapter;
use kem::ICredentialKemAdapter;
use pairing::{IPairingArithmetic, IPairingReference};
use proof::IDeliveryProofAdapter;
use random::IRandomSourceAdapter;

use super::super::solidity::provides::build_msm_term_count;
use super::create_generate;
use super::interface::{
    CreateGenerateErrorReturn, CreateGenerateParams, CreateGenerateReturn,
    ICreateGenerateCaseConsumer, VectorKind, VectorKindQuantity,
};
use super::mock::{
    CreateGenerateParamsOverrides, MockIGenerateConsumer, VectorKindConfigurationOverrides,
    VectorKindQuantitiesOverrides, build_create_generate_case_deps,
    build_create_generate_case_params, build_create_generate_case_payload,
    build_create_generate_deps, build_create_generate_params, build_create_generate_payload,
    build_vector_kind_configuration, build_vector_kind_quantities, run_create_generate_case,
};
use crate::delivery::provides::AdmittedKemIdentity;

/// Contract: every declared kind named exactly once by the entries, in any
///   order, is admitted, the concrete constructed, and the consumer's output
///   returned.
/// Arrange: `kinds` the configuration of every kind the mock concrete's
///   declaration names, in the reverse of the declared order.
/// Act:    `create_generate`, the single call inside the block's case
///   consumer, run once by `run_create_generate_case`.
/// Assert: `result`'s `output` mapped to its `output` equals `Ok(())` by
///   `assert_eq!`.
#[test]
fn create_generate_returns_the_consumers_output_for_settings_naming_every_declared_kind_in_reverse_order()
 {
    // Arrange
    struct Consumer {
        params: CreateGenerateParams,
    }

    impl ICreateGenerateCaseConsumer for Consumer {
        type Output = CreateGenerateReturn<()>;

        fn consume_create_generate_case<P, E, F, K, A, D>(
            &self,
            pairing: &P,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            encoder: &E,
            _decoder: &E,
            kem: &K,
            key_agreement: &A,
            delivery_proof: &D,
            random: &dyn IRandomSourceAdapter,
            _identities: Vec<AdmittedKemIdentity<'_>>,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            E: IEncoderAdapter + IDecoderAdapter,
            F: IChainForms,
            K: ICredentialKemAdapter<Pairing = P>,
            A: IKeyAgreementAdapter<Pairing = P>,
            D: IDeliveryProofAdapter<Pairing = P, Forms = F>,
        {
            create_generate(
                &build_create_generate_deps(
                    pairing,
                    hash_to_scalar,
                    encoder,
                    random,
                    kem,
                    key_agreement,
                    delivery_proof,
                    MockIGenerateConsumer,
                    Default::default(),
                ),
                self.params.clone(),
                build_create_generate_payload(Default::default()),
            )
        }
    }

    let mut entries = build_create_generate_params(Default::default()).kinds;
    entries.reverse();
    let consumer = Consumer {
        params: build_create_generate_params(CreateGenerateParamsOverrides {
            kinds: Some(entries),
            ..Default::default()
        }),
    };

    // Act
    let result = run_create_generate_case(
        &build_create_generate_case_deps(consumer, Default::default()),
        build_create_generate_case_params(Default::default()),
        build_create_generate_case_payload(&[], &[], Default::default()),
    );

    // Assert
    assert_eq!(result.unwrap().output.map(|s| s.output), Ok(()));
}
// create_generate_returns_the_consumers_output_for_settings_naming_every_declared_kind_in_reverse_order

/// Contract: an entry whose `kind` no `VectorKindDeclaration` of the selected
///   concrete's `kinds` names is refused `UndeclaredKind { kind }`.
/// Arrange: `kinds` the configuration of every kind the mock concrete's
///   declaration names, followed by an entry for `VectorKind::Possession`.
/// Act:    `create_generate`, the single call inside the block's case
///   consumer, run once by `run_create_generate_case`.
/// Assert: `result`'s `output` equals `Err(CreateGenerateErrorReturn::
///   UndeclaredKind { kind: VectorKind::Possession })` by `assert_eq!`.
#[test]
fn create_generate_refuses_a_kind_the_declaration_does_not_name() {
    // Arrange
    struct Consumer {
        params: CreateGenerateParams,
    }

    impl ICreateGenerateCaseConsumer for Consumer {
        type Output = CreateGenerateReturn<()>;

        fn consume_create_generate_case<P, E, F, K, A, D>(
            &self,
            pairing: &P,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            encoder: &E,
            _decoder: &E,
            kem: &K,
            key_agreement: &A,
            delivery_proof: &D,
            random: &dyn IRandomSourceAdapter,
            _identities: Vec<AdmittedKemIdentity<'_>>,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            E: IEncoderAdapter + IDecoderAdapter,
            F: IChainForms,
            K: ICredentialKemAdapter<Pairing = P>,
            A: IKeyAgreementAdapter<Pairing = P>,
            D: IDeliveryProofAdapter<Pairing = P, Forms = F>,
        {
            create_generate(
                &build_create_generate_deps(
                    pairing,
                    hash_to_scalar,
                    encoder,
                    random,
                    kem,
                    key_agreement,
                    delivery_proof,
                    MockIGenerateConsumer,
                    Default::default(),
                ),
                self.params.clone(),
                build_create_generate_payload(Default::default()),
            )
        }
    }

    let mut entries = build_create_generate_params(Default::default()).kinds;
    entries.push(build_vector_kind_configuration(
        VectorKindConfigurationOverrides {
            kind: Some(VectorKind::Possession),
            ..Default::default()
        },
    ));
    let consumer = Consumer {
        params: build_create_generate_params(CreateGenerateParamsOverrides {
            kinds: Some(entries),
            ..Default::default()
        }),
    };

    // Act
    let result = run_create_generate_case(
        &build_create_generate_case_deps(consumer, Default::default()),
        build_create_generate_case_params(Default::default()),
        build_create_generate_case_payload(&[], &[], Default::default()),
    );

    // Assert
    assert_eq!(
        result.unwrap().output.map(|_| ()),
        Err(CreateGenerateErrorReturn::UndeclaredKind {
            kind: VectorKind::Possession,
        })
    );
}
// create_generate_refuses_a_kind_the_declaration_does_not_name

/// Contract: an undeclared kind is refused `UndeclaredKind` before its repeat
///   could be refused `DuplicateKind`.
/// Arrange: `kinds` the configuration of every declared kind followed by two
///   entries for `VectorKind::Possession`.
/// Act:    `create_generate`, the single call inside the block's case
///   consumer, run once by `run_create_generate_case`.
/// Assert: `result`'s `output` equals `Err(CreateGenerateErrorReturn::
///   UndeclaredKind { kind: VectorKind::Possession })` by `assert_eq!`.
#[test]
fn create_generate_refuses_an_undeclared_kind_before_its_repeat() {
    // Arrange
    struct Consumer {
        params: CreateGenerateParams,
    }

    impl ICreateGenerateCaseConsumer for Consumer {
        type Output = CreateGenerateReturn<()>;

        fn consume_create_generate_case<P, E, F, K, A, D>(
            &self,
            pairing: &P,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            encoder: &E,
            _decoder: &E,
            kem: &K,
            key_agreement: &A,
            delivery_proof: &D,
            random: &dyn IRandomSourceAdapter,
            _identities: Vec<AdmittedKemIdentity<'_>>,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            E: IEncoderAdapter + IDecoderAdapter,
            F: IChainForms,
            K: ICredentialKemAdapter<Pairing = P>,
            A: IKeyAgreementAdapter<Pairing = P>,
            D: IDeliveryProofAdapter<Pairing = P, Forms = F>,
        {
            create_generate(
                &build_create_generate_deps(
                    pairing,
                    hash_to_scalar,
                    encoder,
                    random,
                    kem,
                    key_agreement,
                    delivery_proof,
                    MockIGenerateConsumer,
                    Default::default(),
                ),
                self.params.clone(),
                build_create_generate_payload(Default::default()),
            )
        }
    }

    let mut entries = build_create_generate_params(Default::default()).kinds;
    entries.push(build_vector_kind_configuration(
        VectorKindConfigurationOverrides {
            kind: Some(VectorKind::Possession),
            ..Default::default()
        },
    ));
    entries.push(build_vector_kind_configuration(
        VectorKindConfigurationOverrides {
            kind: Some(VectorKind::Possession),
            ..Default::default()
        },
    ));
    let consumer = Consumer {
        params: build_create_generate_params(CreateGenerateParamsOverrides {
            kinds: Some(entries),
            ..Default::default()
        }),
    };

    // Act
    let result = run_create_generate_case(
        &build_create_generate_case_deps(consumer, Default::default()),
        build_create_generate_case_params(Default::default()),
        build_create_generate_case_payload(&[], &[], Default::default()),
    );

    // Assert
    assert_eq!(
        result.unwrap().output.map(|_| ()),
        Err(CreateGenerateErrorReturn::UndeclaredKind {
            kind: VectorKind::Possession,
        })
    );
}
// create_generate_refuses_an_undeclared_kind_before_its_repeat

/// Contract: an entry whose `kind` an earlier entry already names is refused
///   `DuplicateKind { kind }`.
/// Arrange: `kinds` the configuration of every declared kind followed by a
///   second entry for `VectorKind::G1Addition`.
/// Act:    `create_generate`, the single call inside the block's case
///   consumer, run once by `run_create_generate_case`.
/// Assert: `result`'s `output` equals `Err(CreateGenerateErrorReturn::
///   DuplicateKind { kind: VectorKind::G1Addition })` by `assert_eq!`.
#[test]
fn create_generate_refuses_a_kind_named_twice() {
    // Arrange
    struct Consumer {
        params: CreateGenerateParams,
    }

    impl ICreateGenerateCaseConsumer for Consumer {
        type Output = CreateGenerateReturn<()>;

        fn consume_create_generate_case<P, E, F, K, A, D>(
            &self,
            pairing: &P,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            encoder: &E,
            _decoder: &E,
            kem: &K,
            key_agreement: &A,
            delivery_proof: &D,
            random: &dyn IRandomSourceAdapter,
            _identities: Vec<AdmittedKemIdentity<'_>>,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            E: IEncoderAdapter + IDecoderAdapter,
            F: IChainForms,
            K: ICredentialKemAdapter<Pairing = P>,
            A: IKeyAgreementAdapter<Pairing = P>,
            D: IDeliveryProofAdapter<Pairing = P, Forms = F>,
        {
            create_generate(
                &build_create_generate_deps(
                    pairing,
                    hash_to_scalar,
                    encoder,
                    random,
                    kem,
                    key_agreement,
                    delivery_proof,
                    MockIGenerateConsumer,
                    Default::default(),
                ),
                self.params.clone(),
                build_create_generate_payload(Default::default()),
            )
        }
    }

    let mut entries = build_create_generate_params(Default::default()).kinds;
    entries.push(build_vector_kind_configuration(
        VectorKindConfigurationOverrides {
            kind: Some(VectorKind::G1Addition),
            ..Default::default()
        },
    ));
    let consumer = Consumer {
        params: build_create_generate_params(CreateGenerateParamsOverrides {
            kinds: Some(entries),
            ..Default::default()
        }),
    };

    // Act
    let result = run_create_generate_case(
        &build_create_generate_case_deps(consumer, Default::default()),
        build_create_generate_case_params(Default::default()),
        build_create_generate_case_payload(&[], &[], Default::default()),
    );

    // Assert
    assert_eq!(
        result.unwrap().output.map(|_| ()),
        Err(CreateGenerateErrorReturn::DuplicateKind {
            kind: VectorKind::G1Addition,
        })
    );
}
// create_generate_refuses_a_kind_named_twice

/// Contract: an entry for a `Drawn` kind holding no `count` is refused
///   `MissingQuantity { kind, quantity: Count }`.
/// Arrange: `kinds` the configuration of every declared kind with the
///   `G1Addition` entry's `count` absent.
/// Act:    `create_generate`, the single call inside the block's case
///   consumer, run once by `run_create_generate_case`.
/// Assert: `result`'s `output` equals `Err(CreateGenerateErrorReturn::
///   MissingQuantity { kind: VectorKind::G1Addition, quantity:
///   VectorKindQuantity::Count })` by `assert_eq!`.
#[test]
fn create_generate_refuses_a_drawn_kind_missing_its_count() {
    // Arrange
    struct Consumer {
        params: CreateGenerateParams,
    }

    impl ICreateGenerateCaseConsumer for Consumer {
        type Output = CreateGenerateReturn<()>;

        fn consume_create_generate_case<P, E, F, K, A, D>(
            &self,
            pairing: &P,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            encoder: &E,
            _decoder: &E,
            kem: &K,
            key_agreement: &A,
            delivery_proof: &D,
            random: &dyn IRandomSourceAdapter,
            _identities: Vec<AdmittedKemIdentity<'_>>,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            E: IEncoderAdapter + IDecoderAdapter,
            F: IChainForms,
            K: ICredentialKemAdapter<Pairing = P>,
            A: IKeyAgreementAdapter<Pairing = P>,
            D: IDeliveryProofAdapter<Pairing = P, Forms = F>,
        {
            create_generate(
                &build_create_generate_deps(
                    pairing,
                    hash_to_scalar,
                    encoder,
                    random,
                    kem,
                    key_agreement,
                    delivery_proof,
                    MockIGenerateConsumer,
                    Default::default(),
                ),
                self.params.clone(),
                build_create_generate_payload(Default::default()),
            )
        }
    }

    let mut entries = build_create_generate_params(Default::default()).kinds;
    entries[0].quantities = build_vector_kind_quantities(VectorKindQuantitiesOverrides {
        count: Some(None),
        ..Default::default()
    });
    let consumer = Consumer {
        params: build_create_generate_params(CreateGenerateParamsOverrides {
            kinds: Some(entries),
            ..Default::default()
        }),
    };

    // Act
    let result = run_create_generate_case(
        &build_create_generate_case_deps(consumer, Default::default()),
        build_create_generate_case_params(Default::default()),
        build_create_generate_case_payload(&[], &[], Default::default()),
    );

    // Assert
    assert_eq!(
        result.unwrap().output.map(|_| ()),
        Err(CreateGenerateErrorReturn::MissingQuantity {
            kind: VectorKind::G1Addition,
            quantity: VectorKindQuantity::Count,
        })
    );
}
// create_generate_refuses_a_drawn_kind_missing_its_count

/// Contract: an entry for a `DrawnTerms` kind holding no `terms` is refused
///   `MissingQuantity { kind, quantity: Terms }`.
/// Arrange: `kinds` the configuration of every declared kind with the
///   `G1MultiScalarMultiplication` entry's `terms` absent.
/// Act:    `create_generate`, the single call inside the block's case
///   consumer, run once by `run_create_generate_case`.
/// Assert: `result`'s `output` equals `Err(CreateGenerateErrorReturn::
///   MissingQuantity { kind: VectorKind::G1MultiScalarMultiplication,
///   quantity: VectorKindQuantity::Terms })` by `assert_eq!`.
#[test]
fn create_generate_refuses_a_kind_drawn_with_terms_missing_its_terms() {
    // Arrange
    struct Consumer {
        params: CreateGenerateParams,
    }

    impl ICreateGenerateCaseConsumer for Consumer {
        type Output = CreateGenerateReturn<()>;

        fn consume_create_generate_case<P, E, F, K, A, D>(
            &self,
            pairing: &P,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            encoder: &E,
            _decoder: &E,
            kem: &K,
            key_agreement: &A,
            delivery_proof: &D,
            random: &dyn IRandomSourceAdapter,
            _identities: Vec<AdmittedKemIdentity<'_>>,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            E: IEncoderAdapter + IDecoderAdapter,
            F: IChainForms,
            K: ICredentialKemAdapter<Pairing = P>,
            A: IKeyAgreementAdapter<Pairing = P>,
            D: IDeliveryProofAdapter<Pairing = P, Forms = F>,
        {
            create_generate(
                &build_create_generate_deps(
                    pairing,
                    hash_to_scalar,
                    encoder,
                    random,
                    kem,
                    key_agreement,
                    delivery_proof,
                    MockIGenerateConsumer,
                    Default::default(),
                ),
                self.params.clone(),
                build_create_generate_payload(Default::default()),
            )
        }
    }

    let mut entries = build_create_generate_params(Default::default()).kinds;
    entries[1].quantities = build_vector_kind_quantities(Default::default());
    let consumer = Consumer {
        params: build_create_generate_params(CreateGenerateParamsOverrides {
            kinds: Some(entries),
            ..Default::default()
        }),
    };

    // Act
    let result = run_create_generate_case(
        &build_create_generate_case_deps(consumer, Default::default()),
        build_create_generate_case_params(Default::default()),
        build_create_generate_case_payload(&[], &[], Default::default()),
    );

    // Assert
    assert_eq!(
        result.unwrap().output.map(|_| ()),
        Err(CreateGenerateErrorReturn::MissingQuantity {
            kind: VectorKind::G1MultiScalarMultiplication,
            quantity: VectorKindQuantity::Terms,
        })
    );
}
// create_generate_refuses_a_kind_drawn_with_terms_missing_its_terms

/// Contract: an entry for a `DrawnMessages` kind holding no `message_length`
///   is refused `MissingQuantity { kind, quantity: MessageLength }`.
/// Arrange: `kinds` the configuration of every declared kind with the
///   `HashToScalar` entry's `message_length` absent.
/// Act:    `create_generate`, the single call inside the block's case
///   consumer, run once by `run_create_generate_case`.
/// Assert: `result`'s `output` equals `Err(CreateGenerateErrorReturn::
///   MissingQuantity { kind: VectorKind::HashToScalar, quantity:
///   VectorKindQuantity::MessageLength })` by `assert_eq!`.
#[test]
fn create_generate_refuses_a_kind_drawn_with_messages_missing_its_message_length() {
    // Arrange
    struct Consumer {
        params: CreateGenerateParams,
    }

    impl ICreateGenerateCaseConsumer for Consumer {
        type Output = CreateGenerateReturn<()>;

        fn consume_create_generate_case<P, E, F, K, A, D>(
            &self,
            pairing: &P,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            encoder: &E,
            _decoder: &E,
            kem: &K,
            key_agreement: &A,
            delivery_proof: &D,
            random: &dyn IRandomSourceAdapter,
            _identities: Vec<AdmittedKemIdentity<'_>>,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            E: IEncoderAdapter + IDecoderAdapter,
            F: IChainForms,
            K: ICredentialKemAdapter<Pairing = P>,
            A: IKeyAgreementAdapter<Pairing = P>,
            D: IDeliveryProofAdapter<Pairing = P, Forms = F>,
        {
            create_generate(
                &build_create_generate_deps(
                    pairing,
                    hash_to_scalar,
                    encoder,
                    random,
                    kem,
                    key_agreement,
                    delivery_proof,
                    MockIGenerateConsumer,
                    Default::default(),
                ),
                self.params.clone(),
                build_create_generate_payload(Default::default()),
            )
        }
    }

    let mut entries = build_create_generate_params(Default::default()).kinds;
    entries[2].quantities = build_vector_kind_quantities(Default::default());
    let consumer = Consumer {
        params: build_create_generate_params(CreateGenerateParamsOverrides {
            kinds: Some(entries),
            ..Default::default()
        }),
    };

    // Act
    let result = run_create_generate_case(
        &build_create_generate_case_deps(consumer, Default::default()),
        build_create_generate_case_params(Default::default()),
        build_create_generate_case_payload(&[], &[], Default::default()),
    );

    // Assert
    assert_eq!(
        result.unwrap().output.map(|_| ()),
        Err(CreateGenerateErrorReturn::MissingQuantity {
            kind: VectorKind::HashToScalar,
            quantity: VectorKindQuantity::MessageLength,
        })
    );
}
// create_generate_refuses_a_kind_drawn_with_messages_missing_its_message_length

/// Contract: an entry for an `Enumerated` kind holding a `count` is refused
///   `UnexpectedQuantity { kind, quantity: Count }`.
/// Arrange: `kinds` the configuration of every declared kind with the
///   `DecodeRejection` entry holding a `count`.
/// Act:    `create_generate`, the single call inside the block's case
///   consumer, run once by `run_create_generate_case`.
/// Assert: `result`'s `output` equals `Err(CreateGenerateErrorReturn::
///   UnexpectedQuantity { kind: VectorKind::DecodeRejection, quantity:
///   VectorKindQuantity::Count })` by `assert_eq!`.
#[test]
fn create_generate_refuses_a_count_on_an_enumerated_kind() {
    // Arrange
    struct Consumer {
        params: CreateGenerateParams,
    }

    impl ICreateGenerateCaseConsumer for Consumer {
        type Output = CreateGenerateReturn<()>;

        fn consume_create_generate_case<P, E, F, K, A, D>(
            &self,
            pairing: &P,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            encoder: &E,
            _decoder: &E,
            kem: &K,
            key_agreement: &A,
            delivery_proof: &D,
            random: &dyn IRandomSourceAdapter,
            _identities: Vec<AdmittedKemIdentity<'_>>,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            E: IEncoderAdapter + IDecoderAdapter,
            F: IChainForms,
            K: ICredentialKemAdapter<Pairing = P>,
            A: IKeyAgreementAdapter<Pairing = P>,
            D: IDeliveryProofAdapter<Pairing = P, Forms = F>,
        {
            create_generate(
                &build_create_generate_deps(
                    pairing,
                    hash_to_scalar,
                    encoder,
                    random,
                    kem,
                    key_agreement,
                    delivery_proof,
                    MockIGenerateConsumer,
                    Default::default(),
                ),
                self.params.clone(),
                build_create_generate_payload(Default::default()),
            )
        }
    }

    let mut entries = build_create_generate_params(Default::default()).kinds;
    entries[3].quantities = build_vector_kind_quantities(Default::default());
    let consumer = Consumer {
        params: build_create_generate_params(CreateGenerateParamsOverrides {
            kinds: Some(entries),
            ..Default::default()
        }),
    };

    // Act
    let result = run_create_generate_case(
        &build_create_generate_case_deps(consumer, Default::default()),
        build_create_generate_case_params(Default::default()),
        build_create_generate_case_payload(&[], &[], Default::default()),
    );

    // Assert
    assert_eq!(
        result.unwrap().output.map(|_| ()),
        Err(CreateGenerateErrorReturn::UnexpectedQuantity {
            kind: VectorKind::DecodeRejection,
            quantity: VectorKindQuantity::Count,
        })
    );
}
// create_generate_refuses_a_count_on_an_enumerated_kind

/// Contract: an entry for an `OverConfiguredInputs` kind holding `terms` is
///   refused `UnexpectedQuantity { kind, quantity: Terms }`.
/// Arrange: `kinds` the configuration of every declared kind with the
///   `IdentityMapping` entry holding `terms` and no `count`.
/// Act:    `create_generate`, the single call inside the block's case
///   consumer, run once by `run_create_generate_case`.
/// Assert: `result`'s `output` equals `Err(CreateGenerateErrorReturn::
///   UnexpectedQuantity { kind: VectorKind::IdentityMapping, quantity:
///   VectorKindQuantity::Terms })` by `assert_eq!`.
#[test]
fn create_generate_refuses_terms_on_a_kind_over_configured_inputs() {
    // Arrange
    struct Consumer {
        params: CreateGenerateParams,
    }

    impl ICreateGenerateCaseConsumer for Consumer {
        type Output = CreateGenerateReturn<()>;

        fn consume_create_generate_case<P, E, F, K, A, D>(
            &self,
            pairing: &P,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            encoder: &E,
            _decoder: &E,
            kem: &K,
            key_agreement: &A,
            delivery_proof: &D,
            random: &dyn IRandomSourceAdapter,
            _identities: Vec<AdmittedKemIdentity<'_>>,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            E: IEncoderAdapter + IDecoderAdapter,
            F: IChainForms,
            K: ICredentialKemAdapter<Pairing = P>,
            A: IKeyAgreementAdapter<Pairing = P>,
            D: IDeliveryProofAdapter<Pairing = P, Forms = F>,
        {
            create_generate(
                &build_create_generate_deps(
                    pairing,
                    hash_to_scalar,
                    encoder,
                    random,
                    kem,
                    key_agreement,
                    delivery_proof,
                    MockIGenerateConsumer,
                    Default::default(),
                ),
                self.params.clone(),
                build_create_generate_payload(Default::default()),
            )
        }
    }

    let mut entries = build_create_generate_params(Default::default()).kinds;
    entries[4].quantities = build_vector_kind_quantities(VectorKindQuantitiesOverrides {
        count: Some(None),
        terms: Some(Some(build_msm_term_count(Default::default()))),
        ..Default::default()
    });
    let consumer = Consumer {
        params: build_create_generate_params(CreateGenerateParamsOverrides {
            kinds: Some(entries),
            ..Default::default()
        }),
    };

    // Act
    let result = run_create_generate_case(
        &build_create_generate_case_deps(consumer, Default::default()),
        build_create_generate_case_params(Default::default()),
        build_create_generate_case_payload(&[], &[], Default::default()),
    );

    // Assert
    assert_eq!(
        result.unwrap().output.map(|_| ()),
        Err(CreateGenerateErrorReturn::UnexpectedQuantity {
            kind: VectorKind::IdentityMapping,
            quantity: VectorKindQuantity::Terms,
        })
    );
}
// create_generate_refuses_terms_on_a_kind_over_configured_inputs

/// Contract: an entry for a `Drawn` kind holding a `message_length` is refused
///   `UnexpectedQuantity { kind, quantity: MessageLength }`.
/// Arrange: `kinds` the configuration of every declared kind with the
///   `G1Addition` entry holding a `message_length`.
/// Act:    `create_generate`, the single call inside the block's case
///   consumer, run once by `run_create_generate_case`.
/// Assert: `result`'s `output` equals `Err(CreateGenerateErrorReturn::
///   UnexpectedQuantity { kind: VectorKind::G1Addition, quantity:
///   VectorKindQuantity::MessageLength })` by `assert_eq!`.
#[test]
fn create_generate_refuses_a_message_length_on_a_drawn_kind() {
    // Arrange
    struct Consumer {
        params: CreateGenerateParams,
    }

    impl ICreateGenerateCaseConsumer for Consumer {
        type Output = CreateGenerateReturn<()>;

        fn consume_create_generate_case<P, E, F, K, A, D>(
            &self,
            pairing: &P,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            encoder: &E,
            _decoder: &E,
            kem: &K,
            key_agreement: &A,
            delivery_proof: &D,
            random: &dyn IRandomSourceAdapter,
            _identities: Vec<AdmittedKemIdentity<'_>>,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            E: IEncoderAdapter + IDecoderAdapter,
            F: IChainForms,
            K: ICredentialKemAdapter<Pairing = P>,
            A: IKeyAgreementAdapter<Pairing = P>,
            D: IDeliveryProofAdapter<Pairing = P, Forms = F>,
        {
            create_generate(
                &build_create_generate_deps(
                    pairing,
                    hash_to_scalar,
                    encoder,
                    random,
                    kem,
                    key_agreement,
                    delivery_proof,
                    MockIGenerateConsumer,
                    Default::default(),
                ),
                self.params.clone(),
                build_create_generate_payload(Default::default()),
            )
        }
    }

    let mut entries = build_create_generate_params(Default::default()).kinds;
    entries[0].quantities = build_vector_kind_quantities(VectorKindQuantitiesOverrides {
        message_length: Some(Some(32)),
        ..Default::default()
    });
    let consumer = Consumer {
        params: build_create_generate_params(CreateGenerateParamsOverrides {
            kinds: Some(entries),
            ..Default::default()
        }),
    };

    // Act
    let result = run_create_generate_case(
        &build_create_generate_case_deps(consumer, Default::default()),
        build_create_generate_case_params(Default::default()),
        build_create_generate_case_payload(&[], &[], Default::default()),
    );

    // Assert
    assert_eq!(
        result.unwrap().output.map(|_| ()),
        Err(CreateGenerateErrorReturn::UnexpectedQuantity {
            kind: VectorKind::G1Addition,
            quantity: VectorKindQuantity::MessageLength,
        })
    );
}
// create_generate_refuses_a_message_length_on_a_drawn_kind

/// Contract: an entry's `count` is checked before its `terms` — a
///   `DrawnTerms` kind holding neither is refused `MissingQuantity` on
///   `count`.
/// Arrange: `kinds` the configuration of every declared kind with the
///   `G1MultiScalarMultiplication` entry holding neither `count` nor `terms`.
/// Act:    `create_generate`, the single call inside the block's case
///   consumer, run once by `run_create_generate_case`.
/// Assert: `result`'s `output` equals `Err(CreateGenerateErrorReturn::
///   MissingQuantity { kind: VectorKind::G1MultiScalarMultiplication,
///   quantity: VectorKindQuantity::Count })` by `assert_eq!`.
#[test]
fn create_generate_checks_a_kinds_count_before_its_terms() {
    // Arrange
    struct Consumer {
        params: CreateGenerateParams,
    }

    impl ICreateGenerateCaseConsumer for Consumer {
        type Output = CreateGenerateReturn<()>;

        fn consume_create_generate_case<P, E, F, K, A, D>(
            &self,
            pairing: &P,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            encoder: &E,
            _decoder: &E,
            kem: &K,
            key_agreement: &A,
            delivery_proof: &D,
            random: &dyn IRandomSourceAdapter,
            _identities: Vec<AdmittedKemIdentity<'_>>,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            E: IEncoderAdapter + IDecoderAdapter,
            F: IChainForms,
            K: ICredentialKemAdapter<Pairing = P>,
            A: IKeyAgreementAdapter<Pairing = P>,
            D: IDeliveryProofAdapter<Pairing = P, Forms = F>,
        {
            create_generate(
                &build_create_generate_deps(
                    pairing,
                    hash_to_scalar,
                    encoder,
                    random,
                    kem,
                    key_agreement,
                    delivery_proof,
                    MockIGenerateConsumer,
                    Default::default(),
                ),
                self.params.clone(),
                build_create_generate_payload(Default::default()),
            )
        }
    }

    let mut entries = build_create_generate_params(Default::default()).kinds;
    entries[1].quantities = build_vector_kind_quantities(VectorKindQuantitiesOverrides {
        count: Some(None),
        ..Default::default()
    });
    let consumer = Consumer {
        params: build_create_generate_params(CreateGenerateParamsOverrides {
            kinds: Some(entries),
            ..Default::default()
        }),
    };

    // Act
    let result = run_create_generate_case(
        &build_create_generate_case_deps(consumer, Default::default()),
        build_create_generate_case_params(Default::default()),
        build_create_generate_case_payload(&[], &[], Default::default()),
    );

    // Assert
    assert_eq!(
        result.unwrap().output.map(|_| ()),
        Err(CreateGenerateErrorReturn::MissingQuantity {
            kind: VectorKind::G1MultiScalarMultiplication,
            quantity: VectorKindQuantity::Count,
        })
    );
}
// create_generate_checks_a_kinds_count_before_its_terms

/// Contract: an entry's `terms` is checked before its `message_length` — a
///   `Drawn` kind holding both is refused `UnexpectedQuantity` on `terms`.
/// Arrange: `kinds` the configuration of every declared kind with the
///   `G1Addition` entry holding `terms` and `message_length`.
/// Act:    `create_generate`, the single call inside the block's case
///   consumer, run once by `run_create_generate_case`.
/// Assert: `result`'s `output` equals `Err(CreateGenerateErrorReturn::
///   UnexpectedQuantity { kind: VectorKind::G1Addition, quantity:
///   VectorKindQuantity::Terms })` by `assert_eq!`.
#[test]
fn create_generate_checks_a_kinds_terms_before_its_message_length() {
    // Arrange
    struct Consumer {
        params: CreateGenerateParams,
    }

    impl ICreateGenerateCaseConsumer for Consumer {
        type Output = CreateGenerateReturn<()>;

        fn consume_create_generate_case<P, E, F, K, A, D>(
            &self,
            pairing: &P,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            encoder: &E,
            _decoder: &E,
            kem: &K,
            key_agreement: &A,
            delivery_proof: &D,
            random: &dyn IRandomSourceAdapter,
            _identities: Vec<AdmittedKemIdentity<'_>>,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            E: IEncoderAdapter + IDecoderAdapter,
            F: IChainForms,
            K: ICredentialKemAdapter<Pairing = P>,
            A: IKeyAgreementAdapter<Pairing = P>,
            D: IDeliveryProofAdapter<Pairing = P, Forms = F>,
        {
            create_generate(
                &build_create_generate_deps(
                    pairing,
                    hash_to_scalar,
                    encoder,
                    random,
                    kem,
                    key_agreement,
                    delivery_proof,
                    MockIGenerateConsumer,
                    Default::default(),
                ),
                self.params.clone(),
                build_create_generate_payload(Default::default()),
            )
        }
    }

    let mut entries = build_create_generate_params(Default::default()).kinds;
    entries[0].quantities = build_vector_kind_quantities(VectorKindQuantitiesOverrides {
        terms: Some(Some(build_msm_term_count(Default::default()))),
        message_length: Some(Some(32)),
        ..Default::default()
    });
    let consumer = Consumer {
        params: build_create_generate_params(CreateGenerateParamsOverrides {
            kinds: Some(entries),
            ..Default::default()
        }),
    };

    // Act
    let result = run_create_generate_case(
        &build_create_generate_case_deps(consumer, Default::default()),
        build_create_generate_case_params(Default::default()),
        build_create_generate_case_payload(&[], &[], Default::default()),
    );

    // Assert
    assert_eq!(
        result.unwrap().output.map(|_| ()),
        Err(CreateGenerateErrorReturn::UnexpectedQuantity {
            kind: VectorKind::G1Addition,
            quantity: VectorKindQuantity::Terms,
        })
    );
}
// create_generate_checks_a_kinds_terms_before_its_message_length

/// Contract: the first failing entry is refused before a later entry is
///   checked.
/// Arrange: `kinds` a `G1Addition` entry holding no `count` followed by an
///   entry for `VectorKind::Possession`.
/// Act:    `create_generate`, the single call inside the block's case
///   consumer, run once by `run_create_generate_case`.
/// Assert: `result`'s `output` equals `Err(CreateGenerateErrorReturn::
///   MissingQuantity { kind: VectorKind::G1Addition, quantity:
///   VectorKindQuantity::Count })` by `assert_eq!`.
#[test]
fn create_generate_refuses_the_first_failing_entry_before_a_later_one() {
    // Arrange
    struct Consumer {
        params: CreateGenerateParams,
    }

    impl ICreateGenerateCaseConsumer for Consumer {
        type Output = CreateGenerateReturn<()>;

        fn consume_create_generate_case<P, E, F, K, A, D>(
            &self,
            pairing: &P,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            encoder: &E,
            _decoder: &E,
            kem: &K,
            key_agreement: &A,
            delivery_proof: &D,
            random: &dyn IRandomSourceAdapter,
            _identities: Vec<AdmittedKemIdentity<'_>>,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            E: IEncoderAdapter + IDecoderAdapter,
            F: IChainForms,
            K: ICredentialKemAdapter<Pairing = P>,
            A: IKeyAgreementAdapter<Pairing = P>,
            D: IDeliveryProofAdapter<Pairing = P, Forms = F>,
        {
            create_generate(
                &build_create_generate_deps(
                    pairing,
                    hash_to_scalar,
                    encoder,
                    random,
                    kem,
                    key_agreement,
                    delivery_proof,
                    MockIGenerateConsumer,
                    Default::default(),
                ),
                self.params.clone(),
                build_create_generate_payload(Default::default()),
            )
        }
    }

    let entries = vec![
        build_vector_kind_configuration(VectorKindConfigurationOverrides {
            kind: Some(VectorKind::G1Addition),
            quantities: Some(build_vector_kind_quantities(
                VectorKindQuantitiesOverrides {
                    count: Some(None),
                    ..Default::default()
                },
            )),
        }),
        build_vector_kind_configuration(VectorKindConfigurationOverrides {
            kind: Some(VectorKind::Possession),
            ..Default::default()
        }),
    ];
    let consumer = Consumer {
        params: build_create_generate_params(CreateGenerateParamsOverrides {
            kinds: Some(entries),
            ..Default::default()
        }),
    };

    // Act
    let result = run_create_generate_case(
        &build_create_generate_case_deps(consumer, Default::default()),
        build_create_generate_case_params(Default::default()),
        build_create_generate_case_payload(&[], &[], Default::default()),
    );

    // Assert
    assert_eq!(
        result.unwrap().output.map(|_| ()),
        Err(CreateGenerateErrorReturn::MissingQuantity {
            kind: VectorKind::G1Addition,
            quantity: VectorKindQuantity::Count,
        })
    );
}
// create_generate_refuses_the_first_failing_entry_before_a_later_one

/// Contract: after every entry is admitted, the first declared kind no entry
///   names, in declared order, is refused `MissingKind { kind }`.
/// Arrange: `kinds` the entries of `IdentityMapping`, `HashToScalar`, and
///   `G1Addition` in that order, `G1MultiScalarMultiplication` and
///   `DecodeRejection` having no entry.
/// Act:    `create_generate`, the single call inside the block's case
///   consumer, run once by `run_create_generate_case`.
/// Assert: `result`'s `output` equals `Err(CreateGenerateErrorReturn::
///   MissingKind { kind: VectorKind::G1MultiScalarMultiplication })` by
///   `assert_eq!`.
#[test]
fn create_generate_refuses_the_first_declared_kind_no_entry_names() {
    // Arrange
    struct Consumer {
        params: CreateGenerateParams,
    }

    impl ICreateGenerateCaseConsumer for Consumer {
        type Output = CreateGenerateReturn<()>;

        fn consume_create_generate_case<P, E, F, K, A, D>(
            &self,
            pairing: &P,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            encoder: &E,
            _decoder: &E,
            kem: &K,
            key_agreement: &A,
            delivery_proof: &D,
            random: &dyn IRandomSourceAdapter,
            _identities: Vec<AdmittedKemIdentity<'_>>,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            E: IEncoderAdapter + IDecoderAdapter,
            F: IChainForms,
            K: ICredentialKemAdapter<Pairing = P>,
            A: IKeyAgreementAdapter<Pairing = P>,
            D: IDeliveryProofAdapter<Pairing = P, Forms = F>,
        {
            create_generate(
                &build_create_generate_deps(
                    pairing,
                    hash_to_scalar,
                    encoder,
                    random,
                    kem,
                    key_agreement,
                    delivery_proof,
                    MockIGenerateConsumer,
                    Default::default(),
                ),
                self.params.clone(),
                build_create_generate_payload(Default::default()),
            )
        }
    }

    let entries = vec![
        build_vector_kind_configuration(VectorKindConfigurationOverrides {
            kind: Some(VectorKind::IdentityMapping),
            quantities: Some(build_vector_kind_quantities(
                VectorKindQuantitiesOverrides {
                    count: Some(None),
                    ..Default::default()
                },
            )),
        }),
        build_vector_kind_configuration(VectorKindConfigurationOverrides {
            kind: Some(VectorKind::HashToScalar),
            quantities: Some(build_vector_kind_quantities(
                VectorKindQuantitiesOverrides {
                    message_length: Some(Some(32)),
                    ..Default::default()
                },
            )),
        }),
        build_vector_kind_configuration(VectorKindConfigurationOverrides {
            kind: Some(VectorKind::G1Addition),
            ..Default::default()
        }),
    ];
    let consumer = Consumer {
        params: build_create_generate_params(CreateGenerateParamsOverrides {
            kinds: Some(entries),
            ..Default::default()
        }),
    };

    // Act
    let result = run_create_generate_case(
        &build_create_generate_case_deps(consumer, Default::default()),
        build_create_generate_case_params(Default::default()),
        build_create_generate_case_payload(&[], &[], Default::default()),
    );

    // Assert
    assert_eq!(
        result.unwrap().output.map(|_| ()),
        Err(CreateGenerateErrorReturn::MissingKind {
            kind: VectorKind::G1MultiScalarMultiplication,
        })
    );
}
// create_generate_refuses_the_first_declared_kind_no_entry_names

/// Contract: an entry's failing quantities are refused before the missing
///   declared kinds are checked.
/// Arrange: `kinds` only a `G1Addition` entry holding no `count`.
/// Act:    `create_generate`, the single call inside the block's case
///   consumer, run once by `run_create_generate_case`.
/// Assert: `result`'s `output` equals `Err(CreateGenerateErrorReturn::
///   MissingQuantity { kind: VectorKind::G1Addition, quantity:
///   VectorKindQuantity::Count })` by `assert_eq!`.
#[test]
fn create_generate_refuses_an_entrys_quantity_before_a_missing_kind() {
    // Arrange
    struct Consumer {
        params: CreateGenerateParams,
    }

    impl ICreateGenerateCaseConsumer for Consumer {
        type Output = CreateGenerateReturn<()>;

        fn consume_create_generate_case<P, E, F, K, A, D>(
            &self,
            pairing: &P,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            encoder: &E,
            _decoder: &E,
            kem: &K,
            key_agreement: &A,
            delivery_proof: &D,
            random: &dyn IRandomSourceAdapter,
            _identities: Vec<AdmittedKemIdentity<'_>>,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            E: IEncoderAdapter + IDecoderAdapter,
            F: IChainForms,
            K: ICredentialKemAdapter<Pairing = P>,
            A: IKeyAgreementAdapter<Pairing = P>,
            D: IDeliveryProofAdapter<Pairing = P, Forms = F>,
        {
            create_generate(
                &build_create_generate_deps(
                    pairing,
                    hash_to_scalar,
                    encoder,
                    random,
                    kem,
                    key_agreement,
                    delivery_proof,
                    MockIGenerateConsumer,
                    Default::default(),
                ),
                self.params.clone(),
                build_create_generate_payload(Default::default()),
            )
        }
    }

    let entries = vec![build_vector_kind_configuration(
        VectorKindConfigurationOverrides {
            kind: Some(VectorKind::G1Addition),
            quantities: Some(build_vector_kind_quantities(
                VectorKindQuantitiesOverrides {
                    count: Some(None),
                    ..Default::default()
                },
            )),
        },
    )];
    let consumer = Consumer {
        params: build_create_generate_params(CreateGenerateParamsOverrides {
            kinds: Some(entries),
            ..Default::default()
        }),
    };

    // Act
    let result = run_create_generate_case(
        &build_create_generate_case_deps(consumer, Default::default()),
        build_create_generate_case_params(Default::default()),
        build_create_generate_case_payload(&[], &[], Default::default()),
    );

    // Assert
    assert_eq!(
        result.unwrap().output.map(|_| ()),
        Err(CreateGenerateErrorReturn::MissingQuantity {
            kind: VectorKind::G1Addition,
            quantity: VectorKindQuantity::Count,
        })
    );
}
// create_generate_refuses_an_entrys_quantity_before_a_missing_kind
