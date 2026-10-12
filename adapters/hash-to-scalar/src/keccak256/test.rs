#![allow(clippy::expect_used)]

use super::Keccak256HashToScalar;
use super::interface::Keccak256HashToScalarErrorReturn;
use super::mock::{
    build_keccak256_hash_to_scalar, build_keccak256_hash_to_scalar_constructor_params,
};
use crate::factory::provides::{
    HashToScalarDeclaration, HashToScalarErrorReturn, HashToScalarIdentifier, IHashToScalarAdapter,
    build_hash_to_scalar_params, build_hash_to_scalar_payload,
};
use pairing::{
    ConsumePairingParams, ConsumePairingPayload, CreatePairingPayload, IPairingArithmetic,
    IPairingConsumer, IPairingReference, build_create_pairing_deps, build_create_pairing_params,
    create_pairing,
};

/// Contract: `try_new` over fieldless params yields `Ok(Keccak256HashToScalar)`.
/// Arrange: params from `build_keccak256_hash_to_scalar_constructor_params(Default::default())`.
/// Act:     `Keccak256HashToScalar::try_new` over those params.
/// Assert:  `result.is_ok()`.
#[test]
fn try_new_returns_the_adapter() {
    // Arrange
    let params = build_keccak256_hash_to_scalar_constructor_params(Default::default());

    // Act
    let result = Keccak256HashToScalar::try_new(params);

    // Assert
    assert!(result.is_ok());
}

/// Contract: `Keccak256HashToScalar::DECLARATION` is `HashToScalarDeclaration
///   { identifier: HashToScalarIdentifier::Keccak256V1, adapter_version: 1,
///   interface_version: HASH_TO_SCALAR_INTERFACE_VERSION }`.
/// Arrange: none; the constant is read as it stands.
/// Act:     read `Keccak256HashToScalar::DECLARATION`.
/// Assert:  `matches!(declaration.identifier, HashToScalarIdentifier::Keccak256V1)`,
///   `declaration.adapter_version` equals `1`, and `declaration.interface_version`
///   equals `1`, the literal being the interface version the objective states.
#[test]
fn declaration_constant_names_the_keccak256_identifier_and_versions() {
    // Arrange

    // Act
    let declaration = Keccak256HashToScalar::DECLARATION;

    // Assert
    assert!(matches!(
        declaration.identifier,
        HashToScalarIdentifier::Keccak256V1
    ));
    assert_eq!(declaration.adapter_version, 1);
    assert_eq!(declaration.interface_version, 1);
}

/// Contract: `declaration()` returns `Self::DECLARATION`.
/// Arrange: `Consumer` with `type Output = HashToScalarDeclaration;`, whose
///   `consume_pairing` builds the adapter with
///   `build_keccak256_hash_to_scalar(Default::default())` and returns
///   `IHashToScalarAdapter::<P::Scalar>::declaration(&adapter)` whole.
/// Act:     `create_pairing` runs the consumer, whose single call is `declaration()`.
/// Assert:  the output is `Some`, its identifier matches
///   `HashToScalarIdentifier::Keccak256V1`, its `adapter_version` equals `1`, and
///   its `interface_version` equals `1`, so the trait method and the constant are
///   proven separately.
/// Collaborators: the pairing family's mock concrete through `create_pairing`,
///   supplying `P::Scalar` only.
#[test]
fn declaration_reports_the_constant() {
    // Arrange
    struct Consumer;

    impl IPairingConsumer for Consumer {
        type Output = HashToScalarDeclaration;

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            _payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            let adapter = build_keccak256_hash_to_scalar(Default::default());
            IHashToScalarAdapter::<P::Scalar>::declaration(&adapter)
        }
    }

    // Act
    let created = create_pairing(
        &build_create_pairing_deps(Consumer, Default::default()),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    );

    // Assert
    let output = created.ok().map(|success| success.output);
    assert!(output.is_some());
    let declaration = output.expect("create_pairing runs the consumer");
    assert!(matches!(
        declaration.identifier,
        HashToScalarIdentifier::Keccak256V1
    ));
    assert_eq!(declaration.adapter_version, 1);
    assert_eq!(declaration.interface_version, 1);
}

/// Contract: when `S::UNIFORM_BYTES_LENGTH.checked_sub(KECCAK256_DIGEST_LENGTH)`
///   is `None`, `hash_to_scalar` yields `Err(HashToScalarErrorReturn::Keccak256(
///   Keccak256HashToScalarErrorReturn::UniformLengthBelowDigest
///   { uniform_length: S::UNIFORM_BYTES_LENGTH,
///   digest_length: KECCAK256_DIGEST_LENGTH }))`.
/// Arrange: `Consumer` with `type Output = ();`, whose `consume_pairing` builds
///   the adapter with `build_keccak256_hash_to_scalar(Default::default())`, takes
///   params from `build_hash_to_scalar_params(Default::default())` and payload
///   from `build_hash_to_scalar_payload(Default::default())`, calls
///   `IHashToScalarAdapter::<P::Scalar>::hash_to_scalar(&adapter, params, payload)`,
///   and asserts on the result in place; the received scalar is the pairing mock
///   concrete's, whose sampling input is 16 bytes, fewer than the 32 digest bytes.
/// Act:     `create_pairing` runs the consumer, whose single call is `hash_to_scalar`.
/// Assert:  in the consumer, `assert_eq!(result.err(),
///   Some(HashToScalarErrorReturn::Keccak256(
///   Keccak256HashToScalarErrorReturn::UniformLengthBelowDigest
///   { uniform_length: 16, digest_length: 32 })))`, the 16 being the pairing mock
///   concrete's sampling length and the 32 the Keccak-256 digest width; and outside
///   it, `assert!(created.is_ok())`, proving the consumer ran.
/// Collaborators: the pairing family's mock concrete through `create_pairing`,
///   supplying the received scalar; `sha3` is not reached, the length check
///   preceding the hasher.
#[test]
fn hash_to_scalar_refuses_a_sampling_input_shorter_than_the_digest() {
    // Arrange
    struct Consumer;

    impl IPairingConsumer for Consumer {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            _payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            let adapter = build_keccak256_hash_to_scalar(Default::default());
            let params = build_hash_to_scalar_params(Default::default());
            let payload = build_hash_to_scalar_payload(Default::default());
            let result =
                IHashToScalarAdapter::<P::Scalar>::hash_to_scalar(&adapter, params, payload);
            assert_eq!(
                result.err(),
                Some(HashToScalarErrorReturn::Keccak256(
                    Keccak256HashToScalarErrorReturn::UniformLengthBelowDigest {
                        uniform_length: 16,
                        digest_length: 32
                    }
                ))
            );
        }
    }

    // Act
    let created = create_pairing(
        &build_create_pairing_deps(Consumer, Default::default()),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    );

    // Assert
    assert!(created.is_ok());
}
