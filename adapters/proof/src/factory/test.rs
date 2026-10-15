#![allow(clippy::expect_used)]

use super::provides::{
    CreateDeliveryProofErrorReturn, CreateDeliveryProofParamsOverrides, CreateDeliveryProofReturn,
    DeliveryProofComponents, DeliveryProofConcrete, DeliveryProofShapeErrorReturn,
    DeliveryProofWireComponents, DeliveryProofWireComponentsOverrides, MockIDeliveryProofConsumer,
    build_create_delivery_proof_deps, build_create_delivery_proof_params,
    build_create_delivery_proof_payload, build_delivery_proof_wire_components,
    build_mint_responses, build_mint_second_group_first_messages, build_transfer_responses,
    build_transfer_second_group_first_messages, create_delivery_proof,
};
use crate::mint_statement::provides::{DELIVERY_STATEMENT_VERSION_ONE, DeliveryRelation};
use chain::{
    ConsumeChainFormsParams, ConsumeChainFormsPayload, CreateChainFormsReturn, IChainForms,
    IChainFormsConsumer, build_create_chain_forms_deps, build_create_chain_forms_params,
    build_create_chain_forms_payload, create_chain_forms,
};
use encoding::{
    ConsumeEncodingParams, ConsumeEncodingPayload, CreateEncodingReturn, IDecoderAdapter,
    IEncoderAdapter, IEncodingConsumer, build_create_encoding_deps, build_create_encoding_params,
    build_create_encoding_payload, create_encoding,
};
use hash_to_scalar::{
    build_create_hash_to_scalar_deps, build_create_hash_to_scalar_params,
    build_create_hash_to_scalar_payload, create_hash_to_scalar,
};
use pairing::{
    ConsumePairingParams, ConsumePairingPayload, CreatePairingPayload, EncodeG2Params,
    EncodeG2Payload, EncodeScalarParams, EncodeScalarPayload, IPairingArithmetic, IPairingConsumer,
    IPairingReference, VerifierGroupArithmetic, build_create_pairing_deps,
    build_create_pairing_params, create_pairing,
};
use random::{
    CreateRandomSourceDeps, CreateRandomSourcePayload, build_create_random_source_params,
    create_random_source,
};

/// Contract: wire conversion, admitted: both counts match → `Ok(components)`
///   in the variant of the wire's relation and form, the vectors named in the
///   relation's fields.
/// Arrange: the wire with `relation` `DeliveryRelation::Mint`, `verifier_form`
///   `BothGroups`, `responses` the scalars of `build_mint_responses` in field
///   order, distinct at every position, and no first messages.
/// Act:     `DeliveryProofComponents::try_from(wire)` inside the consumer's
///   method.
/// Assert:  in place, the result is `Ok` and matches
///   `DeliveryProofComponents::MintBothGroups { challenge, responses }`; the
///   encoding of `challenge` through the received pairing's `encode_scalar`
///   equals the encoding of the wire's challenge, and `alpha`, `r`, `rho`, and
///   `sigma` each equal the wire's response at the same position, so a swap of
///   two responses fails; outside, the factory result `is_ok()`.
/// Collaborators: the pairing family's mock concrete through `create_pairing`
///   under the default params.
#[test]
fn wire_components_of_the_mint_relation_in_the_both_groups_form_convert_to_the_matching_variant() {
    struct Consumer;

    impl IPairingConsumer for Consumer {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            let pairing = &payload.adapter;
            let encode_scalar = |scalar: P::Scalar| -> Vec<u8> {
                pairing
                    .encode_scalar(EncodeScalarParams, EncodeScalarPayload { scalar })
                    .expect("encoding a scalar succeeds")
                    .bytes
                    .expose()
                    .as_ref()
                    .to_vec()
            };

            // Arrange
            let responses = build_mint_responses(pairing, Default::default());
            let wire: DeliveryProofWireComponents<P::Scalar, P::G2> =
                build_delivery_proof_wire_components(
                    pairing,
                    DeliveryProofWireComponentsOverrides {
                        relation: Some(DeliveryRelation::Mint),
                        verifier_form: Some(VerifierGroupArithmetic::BothGroups),
                        responses: Some(vec![
                            responses.alpha.clone(),
                            responses.r.clone(),
                            responses.rho.clone(),
                            responses.sigma.clone(),
                        ]),
                        ..Default::default()
                    },
                );
            let expected_challenge = wire.challenge.clone();

            // Act
            let result: Result<
                DeliveryProofComponents<P::Scalar, P::G2>,
                DeliveryProofShapeErrorReturn,
            > = DeliveryProofComponents::try_from(wire);

            // Assert
            let Ok(DeliveryProofComponents::MintBothGroups {
                challenge,
                responses: converted,
            }) = result
            else {
                panic!("the wire converts to the mint both-groups components")
            };
            assert_eq!(encode_scalar(challenge), encode_scalar(expected_challenge));
            assert_eq!(
                encode_scalar(converted.alpha),
                encode_scalar(responses.alpha)
            );
            assert_eq!(encode_scalar(converted.r), encode_scalar(responses.r));
            assert_eq!(encode_scalar(converted.rho), encode_scalar(responses.rho));
            assert_eq!(
                encode_scalar(converted.sigma),
                encode_scalar(responses.sigma)
            );
        }
    }

    let created = create_pairing(
        &build_create_pairing_deps(Consumer, Default::default()),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    );

    assert!(created.is_ok());
}

/// Contract: wire conversion, admitted: both counts match → `Ok(components)`
///   in the variant of the wire's relation and form, the first messages named
///   in order.
/// Arrange: the wire with `relation` `Mint`, `verifier_form` `FirstGroupOnly`,
///   the mint responses of the block above, and `second_group_first_messages`
///   the points of `build_mint_second_group_first_messages` in field order,
///   distinct at every position.
/// Act:     `DeliveryProofComponents::try_from(wire)` inside the consumer's
///   method.
/// Assert:  in place, the result matches `DeliveryProofComponents::
///   MintFirstGroupOnly { challenge, responses, first_messages }`, the
///   responses placed as above and `hpub`, `d1`, and `d2` of `first_messages`
///   each equal to the wire's message at the same position by `encode_g2`;
///   outside, the factory result `is_ok()`.
/// Collaborators: the pairing family's mock concrete through `create_pairing`
///   under the default params.
#[test]
fn wire_components_of_the_mint_relation_in_the_first_group_only_form_convert_to_the_matching_variant()
 {
    struct Consumer;

    impl IPairingConsumer for Consumer {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            let pairing = &payload.adapter;
            let encode_scalar = |scalar: P::Scalar| -> Vec<u8> {
                pairing
                    .encode_scalar(EncodeScalarParams, EncodeScalarPayload { scalar })
                    .expect("encoding a scalar succeeds")
                    .bytes
                    .expose()
                    .as_ref()
                    .to_vec()
            };
            let encode_g2 = |point: P::G2| -> Vec<u8> {
                pairing
                    .encode_g2(EncodeG2Params, EncodeG2Payload { point })
                    .expect("encoding a second-group point succeeds")
                    .bytes
                    .as_ref()
                    .to_vec()
            };

            // Arrange
            let responses = build_mint_responses(pairing, Default::default());
            let first_messages =
                build_mint_second_group_first_messages(pairing, Default::default());
            let wire: DeliveryProofWireComponents<P::Scalar, P::G2> =
                build_delivery_proof_wire_components(
                    pairing,
                    DeliveryProofWireComponentsOverrides {
                        relation: Some(DeliveryRelation::Mint),
                        verifier_form: Some(VerifierGroupArithmetic::FirstGroupOnly),
                        responses: Some(vec![
                            responses.alpha.clone(),
                            responses.r.clone(),
                            responses.rho.clone(),
                            responses.sigma.clone(),
                        ]),
                        second_group_first_messages: Some(vec![
                            first_messages.hpub.clone(),
                            first_messages.d1.clone(),
                            first_messages.d2.clone(),
                        ]),
                        ..Default::default()
                    },
                );
            let expected_challenge = wire.challenge.clone();

            // Act
            let result: Result<
                DeliveryProofComponents<P::Scalar, P::G2>,
                DeliveryProofShapeErrorReturn,
            > = DeliveryProofComponents::try_from(wire);

            // Assert
            let Ok(DeliveryProofComponents::MintFirstGroupOnly {
                challenge,
                responses: converted,
                first_messages: converted_messages,
            }) = result
            else {
                panic!("the wire converts to the mint first-group-only components")
            };
            assert_eq!(encode_scalar(challenge), encode_scalar(expected_challenge));
            assert_eq!(
                encode_scalar(converted.alpha),
                encode_scalar(responses.alpha)
            );
            assert_eq!(encode_scalar(converted.r), encode_scalar(responses.r));
            assert_eq!(encode_scalar(converted.rho), encode_scalar(responses.rho));
            assert_eq!(
                encode_scalar(converted.sigma),
                encode_scalar(responses.sigma)
            );
            assert_eq!(
                encode_g2(converted_messages.hpub),
                encode_g2(first_messages.hpub)
            );
            assert_eq!(
                encode_g2(converted_messages.d1),
                encode_g2(first_messages.d1)
            );
            assert_eq!(
                encode_g2(converted_messages.d2),
                encode_g2(first_messages.d2)
            );
        }
    }

    let created = create_pairing(
        &build_create_pairing_deps(Consumer, Default::default()),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    );

    assert!(created.is_ok());
}

/// Contract: wire conversion, admitted: both counts match → `Ok(components)`
///   in the variant of the wire's relation and form.
/// Arrange: the wire with `relation` `DeliveryRelation::Transfer`,
///   `verifier_form` `BothGroups`, `responses` the scalars of
///   `build_transfer_responses` in field order, distinct at every position,
///   and no first messages.
/// Act:     `DeliveryProofComponents::try_from(wire)` inside the consumer's
///   method.
/// Assert:  in place, the result matches `DeliveryProofComponents::
///   TransferBothGroups { challenge, responses }`, the challenge and each of
///   `x`, `y`, `s`, `rho`, and `sigma` equal to the wire's at the same
///   position by `encode_scalar`; outside, the factory result `is_ok()`.
/// Collaborators: the pairing family's mock concrete through `create_pairing`
///   under the default params.
#[test]
fn wire_components_of_the_transfer_relation_in_the_both_groups_form_convert_to_the_matching_variant()
 {
    struct Consumer;

    impl IPairingConsumer for Consumer {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            let pairing = &payload.adapter;
            let encode_scalar = |scalar: P::Scalar| -> Vec<u8> {
                pairing
                    .encode_scalar(EncodeScalarParams, EncodeScalarPayload { scalar })
                    .expect("encoding a scalar succeeds")
                    .bytes
                    .expose()
                    .as_ref()
                    .to_vec()
            };

            // Arrange
            let responses = build_transfer_responses(pairing, Default::default());
            let wire: DeliveryProofWireComponents<P::Scalar, P::G2> =
                build_delivery_proof_wire_components(
                    pairing,
                    DeliveryProofWireComponentsOverrides {
                        relation: Some(DeliveryRelation::Transfer),
                        verifier_form: Some(VerifierGroupArithmetic::BothGroups),
                        responses: Some(vec![
                            responses.x.clone(),
                            responses.y.clone(),
                            responses.s.clone(),
                            responses.rho.clone(),
                            responses.sigma.clone(),
                        ]),
                        ..Default::default()
                    },
                );
            let expected_challenge = wire.challenge.clone();

            // Act
            let result: Result<
                DeliveryProofComponents<P::Scalar, P::G2>,
                DeliveryProofShapeErrorReturn,
            > = DeliveryProofComponents::try_from(wire);

            // Assert
            let Ok(DeliveryProofComponents::TransferBothGroups {
                challenge,
                responses: converted,
            }) = result
            else {
                panic!("the wire converts to the transfer both-groups components")
            };
            assert_eq!(encode_scalar(challenge), encode_scalar(expected_challenge));
            assert_eq!(encode_scalar(converted.x), encode_scalar(responses.x));
            assert_eq!(encode_scalar(converted.y), encode_scalar(responses.y));
            assert_eq!(encode_scalar(converted.s), encode_scalar(responses.s));
            assert_eq!(encode_scalar(converted.rho), encode_scalar(responses.rho));
            assert_eq!(
                encode_scalar(converted.sigma),
                encode_scalar(responses.sigma)
            );
        }
    }

    let created = create_pairing(
        &build_create_pairing_deps(Consumer, Default::default()),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    );

    assert!(created.is_ok());
}

/// Contract: wire conversion, admitted: both counts match → `Ok(components)`
///   in the variant of the wire's relation and form, the first messages named
///   in order.
/// Arrange: the wire with `relation` `Transfer`, `verifier_form`
///   `FirstGroupOnly`, the transfer responses of the block above, and
///   `second_group_first_messages` the points of
///   `build_transfer_second_group_first_messages` in field order, distinct at
///   every position.
/// Act:     `DeliveryProofComponents::try_from(wire)` inside the consumer's
///   method.
/// Assert:  in place, the result matches `DeliveryProofComponents::
///   TransferFirstGroupOnly { challenge, responses, first_messages }`, the
///   responses placed as above and each of the three fields of
///   `first_messages` equal to the wire's message at the same position by
///   `encode_g2`; outside, the factory result `is_ok()`.
/// Collaborators: the pairing family's mock concrete through `create_pairing`
///   under the default params.
#[test]
fn wire_components_of_the_transfer_relation_in_the_first_group_only_form_convert_to_the_matching_variant()
 {
    struct Consumer;

    impl IPairingConsumer for Consumer {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            let pairing = &payload.adapter;
            let encode_scalar = |scalar: P::Scalar| -> Vec<u8> {
                pairing
                    .encode_scalar(EncodeScalarParams, EncodeScalarPayload { scalar })
                    .expect("encoding a scalar succeeds")
                    .bytes
                    .expose()
                    .as_ref()
                    .to_vec()
            };
            let encode_g2 = |point: P::G2| -> Vec<u8> {
                pairing
                    .encode_g2(EncodeG2Params, EncodeG2Payload { point })
                    .expect("encoding a second-group point succeeds")
                    .bytes
                    .as_ref()
                    .to_vec()
            };

            // Arrange
            let responses = build_transfer_responses(pairing, Default::default());
            let first_messages =
                build_transfer_second_group_first_messages(pairing, Default::default());
            let wire: DeliveryProofWireComponents<P::Scalar, P::G2> =
                build_delivery_proof_wire_components(
                    pairing,
                    DeliveryProofWireComponentsOverrides {
                        relation: Some(DeliveryRelation::Transfer),
                        verifier_form: Some(VerifierGroupArithmetic::FirstGroupOnly),
                        responses: Some(vec![
                            responses.x.clone(),
                            responses.y.clone(),
                            responses.s.clone(),
                            responses.rho.clone(),
                            responses.sigma.clone(),
                        ]),
                        second_group_first_messages: Some(vec![
                            first_messages.pk2.clone(),
                            first_messages.d1.clone(),
                            first_messages.d2.clone(),
                        ]),
                        ..Default::default()
                    },
                );
            let expected_challenge = wire.challenge.clone();

            // Act
            let result: Result<
                DeliveryProofComponents<P::Scalar, P::G2>,
                DeliveryProofShapeErrorReturn,
            > = DeliveryProofComponents::try_from(wire);

            // Assert
            let Ok(DeliveryProofComponents::TransferFirstGroupOnly {
                challenge,
                responses: converted,
                first_messages: converted_messages,
            }) = result
            else {
                panic!("the wire converts to the transfer first-group-only components")
            };
            assert_eq!(encode_scalar(challenge), encode_scalar(expected_challenge));
            assert_eq!(encode_scalar(converted.x), encode_scalar(responses.x));
            assert_eq!(encode_scalar(converted.y), encode_scalar(responses.y));
            assert_eq!(encode_scalar(converted.s), encode_scalar(responses.s));
            assert_eq!(encode_scalar(converted.rho), encode_scalar(responses.rho));
            assert_eq!(
                encode_scalar(converted.sigma),
                encode_scalar(responses.sigma)
            );
            assert_eq!(
                encode_g2(converted_messages.pk2),
                encode_g2(first_messages.pk2)
            );
            assert_eq!(
                encode_g2(converted_messages.d1),
                encode_g2(first_messages.d1)
            );
            assert_eq!(
                encode_g2(converted_messages.d2),
                encode_g2(first_messages.d2)
            );
        }
    }

    let created = create_pairing(
        &build_create_pairing_deps(Consumer, Default::default()),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    );

    assert!(created.is_ok());
}

/// Contract: wire conversion, response count refused: `responses.len()`
///   differs from the expected → `Err(DeliveryProofShapeErrorReturn::
///   ResponseCount { expected, actual })`.
/// Arrange: the wire with `relation` `Mint`, `verifier_form` `BothGroups`, and
///   `responses` one scalar shorter than the mint relation's.
/// Act:     `DeliveryProofComponents::try_from(wire)` inside the consumer's
///   method.
/// Assert:  in place, the whole error equals `ResponseCount { expected: 4,
///   actual: 3 }` by `assert_eq!`; outside, the factory result `is_ok()`.
/// Collaborators: the pairing family's mock concrete through `create_pairing`
///   under the default params.
#[test]
fn wire_components_refuse_a_response_count_other_than_the_relations() {
    struct Consumer;

    impl IPairingConsumer for Consumer {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            let pairing = &payload.adapter;

            // Arrange
            let responses = build_mint_responses(pairing, Default::default());
            let wire: DeliveryProofWireComponents<P::Scalar, P::G2> =
                build_delivery_proof_wire_components(
                    pairing,
                    DeliveryProofWireComponentsOverrides {
                        relation: Some(DeliveryRelation::Mint),
                        verifier_form: Some(VerifierGroupArithmetic::BothGroups),
                        responses: Some(vec![responses.alpha, responses.r, responses.rho]),
                        ..Default::default()
                    },
                );

            // Act
            let result: Result<
                DeliveryProofComponents<P::Scalar, P::G2>,
                DeliveryProofShapeErrorReturn,
            > = DeliveryProofComponents::try_from(wire);

            // Assert
            assert_eq!(
                result.err(),
                Some(DeliveryProofShapeErrorReturn::ResponseCount {
                    expected: 4,
                    actual: 3
                })
            );
        }
    }

    let created = create_pairing(
        &build_create_pairing_deps(Consumer, Default::default()),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    );

    assert!(created.is_ok());
}

/// Contract: wire conversion, response count refused: the transfer relation's
///   expected count is five → `Err(DeliveryProofShapeErrorReturn::
///   ResponseCount { expected, actual })`.
/// Arrange: the wire with `relation` `Transfer`, `verifier_form` `BothGroups`,
///   and `responses` the mint relation's four scalars, so a conversion that
///   expects one count for both relations fails.
/// Act:     `DeliveryProofComponents::try_from(wire)` inside the consumer's
///   method.
/// Assert:  in place, the whole error equals `ResponseCount { expected: 5,
///   actual: 4 }` by `assert_eq!`; outside, the factory result `is_ok()`.
/// Collaborators: the pairing family's mock concrete through `create_pairing`
///   under the default params.
#[test]
fn wire_components_refuse_a_transfer_response_count_other_than_the_relations() {
    struct Consumer;

    impl IPairingConsumer for Consumer {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            let pairing = &payload.adapter;

            // Arrange
            let responses = build_mint_responses(pairing, Default::default());
            let wire: DeliveryProofWireComponents<P::Scalar, P::G2> =
                build_delivery_proof_wire_components(
                    pairing,
                    DeliveryProofWireComponentsOverrides {
                        relation: Some(DeliveryRelation::Transfer),
                        verifier_form: Some(VerifierGroupArithmetic::BothGroups),
                        responses: Some(vec![
                            responses.alpha,
                            responses.r,
                            responses.rho,
                            responses.sigma,
                        ]),
                        ..Default::default()
                    },
                );

            // Act
            let result: Result<
                DeliveryProofComponents<P::Scalar, P::G2>,
                DeliveryProofShapeErrorReturn,
            > = DeliveryProofComponents::try_from(wire);

            // Assert
            assert_eq!(
                result.err(),
                Some(DeliveryProofShapeErrorReturn::ResponseCount {
                    expected: 5,
                    actual: 4
                })
            );
        }
    }

    let created = create_pairing(
        &build_create_pairing_deps(Consumer, Default::default()),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    );

    assert!(created.is_ok());
}

/// Contract: wire conversion, first-message count refused: the response count
///   matches and the both-groups form expects none →
///   `Err(DeliveryProofShapeErrorReturn::SecondGroupFirstMessageCount
///   { expected, actual })`.
/// Arrange: the wire with `relation` `Mint`, `verifier_form` `BothGroups`, the
///   mint responses, and the three points of
///   `build_mint_second_group_first_messages`.
/// Act:     `DeliveryProofComponents::try_from(wire)` inside the consumer's
///   method.
/// Assert:  in place, the whole error equals
///   `SecondGroupFirstMessageCount { expected: 0, actual: 3 }` by `assert_eq!`;
///   outside, the factory result `is_ok()`.
/// Collaborators: the pairing family's mock concrete through `create_pairing`
///   under the default params.
#[test]
fn wire_components_refuse_first_messages_the_both_groups_form_does_not_carry() {
    struct Consumer;

    impl IPairingConsumer for Consumer {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            let pairing = &payload.adapter;

            // Arrange
            let responses = build_mint_responses(pairing, Default::default());
            let first_messages =
                build_mint_second_group_first_messages(pairing, Default::default());
            let wire: DeliveryProofWireComponents<P::Scalar, P::G2> =
                build_delivery_proof_wire_components(
                    pairing,
                    DeliveryProofWireComponentsOverrides {
                        relation: Some(DeliveryRelation::Mint),
                        verifier_form: Some(VerifierGroupArithmetic::BothGroups),
                        responses: Some(vec![
                            responses.alpha,
                            responses.r,
                            responses.rho,
                            responses.sigma,
                        ]),
                        second_group_first_messages: Some(vec![
                            first_messages.hpub,
                            first_messages.d1,
                            first_messages.d2,
                        ]),
                        ..Default::default()
                    },
                );

            // Act
            let result: Result<
                DeliveryProofComponents<P::Scalar, P::G2>,
                DeliveryProofShapeErrorReturn,
            > = DeliveryProofComponents::try_from(wire);

            // Assert
            assert_eq!(
                result.err(),
                Some(
                    DeliveryProofShapeErrorReturn::SecondGroupFirstMessageCount {
                        expected: 0,
                        actual: 3
                    }
                )
            );
        }
    }

    let created = create_pairing(
        &build_create_pairing_deps(Consumer, Default::default()),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    );

    assert!(created.is_ok());
}

/// Contract: wire conversion, first-message count refused: the response count
///   matches and the first-group-only form expects three →
///   `Err(DeliveryProofShapeErrorReturn::SecondGroupFirstMessageCount
///   { expected, actual })`.
/// Arrange: the wire with `relation` `Transfer`, `verifier_form`
///   `FirstGroupOnly`, the transfer responses, and no first messages.
/// Act:     `DeliveryProofComponents::try_from(wire)` inside the consumer's
///   method.
/// Assert:  in place, the whole error equals
///   `SecondGroupFirstMessageCount { expected: 3, actual: 0 }` by `assert_eq!`;
///   outside, the factory result `is_ok()`.
/// Collaborators: the pairing family's mock concrete through `create_pairing`
///   under the default params.
#[test]
fn wire_components_refuse_missing_first_messages_in_the_first_group_only_form() {
    struct Consumer;

    impl IPairingConsumer for Consumer {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            let pairing = &payload.adapter;

            // Arrange
            let responses = build_transfer_responses(pairing, Default::default());
            let wire: DeliveryProofWireComponents<P::Scalar, P::G2> =
                build_delivery_proof_wire_components(
                    pairing,
                    DeliveryProofWireComponentsOverrides {
                        relation: Some(DeliveryRelation::Transfer),
                        verifier_form: Some(VerifierGroupArithmetic::FirstGroupOnly),
                        responses: Some(vec![
                            responses.x,
                            responses.y,
                            responses.s,
                            responses.rho,
                            responses.sigma,
                        ]),
                        second_group_first_messages: Some(Vec::new()),
                        ..Default::default()
                    },
                );

            // Act
            let result: Result<
                DeliveryProofComponents<P::Scalar, P::G2>,
                DeliveryProofShapeErrorReturn,
            > = DeliveryProofComponents::try_from(wire);

            // Assert
            assert_eq!(
                result.err(),
                Some(
                    DeliveryProofShapeErrorReturn::SecondGroupFirstMessageCount {
                        expected: 3,
                        actual: 0
                    }
                )
            );
        }
    }

    let created = create_pairing(
        &build_create_pairing_deps(Consumer, Default::default()),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    );

    assert!(created.is_ok());
}

/// Contract: wire conversion, ordering: the response count is checked before
///   the first-message count → `Err(DeliveryProofShapeErrorReturn::
///   ResponseCount { expected, actual })`.
/// Arrange: the wire with `relation` `Mint`, `verifier_form` `BothGroups`, one
///   scalar fewer than the mint responses, and the three first messages the
///   form does not carry.
/// Act:     `DeliveryProofComponents::try_from(wire)` inside the consumer's
///   method.
/// Assert:  in place, the whole error equals `ResponseCount { expected: 4,
///   actual: 3 }` by `assert_eq!`; outside, the factory result `is_ok()`.
/// Collaborators: the pairing family's mock concrete through `create_pairing`
///   under the default params.
#[test]
fn wire_components_check_the_response_count_before_the_first_message_count() {
    struct Consumer;

    impl IPairingConsumer for Consumer {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            let pairing = &payload.adapter;

            // Arrange
            let responses = build_mint_responses(pairing, Default::default());
            let first_messages =
                build_mint_second_group_first_messages(pairing, Default::default());
            let wire: DeliveryProofWireComponents<P::Scalar, P::G2> =
                build_delivery_proof_wire_components(
                    pairing,
                    DeliveryProofWireComponentsOverrides {
                        relation: Some(DeliveryRelation::Mint),
                        verifier_form: Some(VerifierGroupArithmetic::BothGroups),
                        responses: Some(vec![responses.alpha, responses.r, responses.rho]),
                        second_group_first_messages: Some(vec![
                            first_messages.hpub,
                            first_messages.d1,
                            first_messages.d2,
                        ]),
                        ..Default::default()
                    },
                );

            // Act
            let result: Result<
                DeliveryProofComponents<P::Scalar, P::G2>,
                DeliveryProofShapeErrorReturn,
            > = DeliveryProofComponents::try_from(wire);

            // Assert
            assert_eq!(
                result.err(),
                Some(DeliveryProofShapeErrorReturn::ResponseCount {
                    expected: 4,
                    actual: 3
                })
            );
        }
    }

    let created = create_pairing(
        &build_create_pairing_deps(Consumer, Default::default()),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    );

    assert!(created.is_ok());
}

/// Contract: `create_delivery_proof`, admitted: every requirement is
///   declared and `try_new` returns `Ok(adapter)` → the consumer is called
///   once with the adapter and the subject returns
///   `Ok(CreateDeliveryProofSuccessReturn { output })` holding the
///   consumer's output.
/// Arrange: params from `build_create_delivery_proof_params` overridden to
///   `concrete` `DeliveryProofConcrete::SchnorrFs`, the other fields at the
///   defaults the concrete's declaration lists.
/// Act:     `create_delivery_proof`, called in the innermost consumer's
///   method.
/// Assert:  `assert!(result.is_ok())`; the output, the unit value of
///   `MockIDeliveryProofConsumer`, is not asserted.
/// Collaborators: the pairing, encoding, chain-forms, hash-to-scalar, and
///   randomness families' mock concretes through `create_pairing`,
///   `create_encoding`, `create_chain_forms`, `create_hash_to_scalar`, and
///   `create_random_source`, each at the defaults of its params builder;
///   `MockIDeliveryProofConsumer` in the deps.
#[test]
fn create_delivery_proof_returns_the_consumers_output_for_the_schnorr_fs_selection() {
    struct OuterConsumer;

    impl IPairingConsumer for OuterConsumer {
        type Output = CreateEncodingReturn<CreateChainFormsReturn<CreateDeliveryProofReturn<()>>>;

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            struct MiddleConsumer<'a, P: IPairingArithmetic> {
                pairing: &'a P,
            }

            impl<P: IPairingArithmetic> IEncodingConsumer for MiddleConsumer<'_, P> {
                type Output = CreateChainFormsReturn<CreateDeliveryProofReturn<()>>;

                fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
                    &self,
                    _params: ConsumeEncodingParams,
                    consumed: ConsumeEncodingPayload<E>,
                ) -> Self::Output {
                    struct InnerConsumer<'a, P: IPairingArithmetic, E: IEncoderAdapter> {
                        pairing: &'a P,
                        encoder: &'a E,
                    }

                    impl<P: IPairingArithmetic, E: IEncoderAdapter> IChainFormsConsumer for InnerConsumer<'_, P, E> {
                        type Output = CreateDeliveryProofReturn<()>;

                        fn consume_chain_forms<F: IChainForms>(
                            &self,
                            _params: ConsumeChainFormsParams,
                            _payload: ConsumeChainFormsPayload<F>,
                        ) -> Self::Output {
                            // Arrange
                            let hash_to_scalar = create_hash_to_scalar::<P::Scalar>(
                                &build_create_hash_to_scalar_deps(Default::default()),
                                build_create_hash_to_scalar_params(Default::default()),
                                build_create_hash_to_scalar_payload(Default::default()),
                            )
                            .expect(
                                "the default selection constructs the hash-to-scalar mock concrete",
                            )
                            .adapter;
                            let random = create_random_source(
                                &CreateRandomSourceDeps,
                                build_create_random_source_params(Default::default()),
                                CreateRandomSourcePayload,
                            )
                            .expect(
                                "the default selection constructs the random-source mock concrete",
                            )
                            .adapter;

                            // Act
                            create_delivery_proof::<P, E, F, MockIDeliveryProofConsumer>(
                                &build_create_delivery_proof_deps(
                                    self.pairing,
                                    &*hash_to_scalar,
                                    self.encoder,
                                    &*random,
                                    MockIDeliveryProofConsumer,
                                    Default::default(),
                                ),
                                build_create_delivery_proof_params(
                                    CreateDeliveryProofParamsOverrides {
                                        concrete: Some(DeliveryProofConcrete::SchnorrFs),
                                        ..Default::default()
                                    },
                                ),
                                build_create_delivery_proof_payload(Default::default()),
                            )
                        }
                    }

                    create_chain_forms(
                        &build_create_chain_forms_deps(
                            InnerConsumer {
                                pairing: self.pairing,
                                encoder: &consumed.adapter,
                            },
                            Default::default(),
                        ),
                        build_create_chain_forms_params(Default::default()),
                        build_create_chain_forms_payload(Default::default()),
                    )
                }
            }

            create_encoding(
                &build_create_encoding_deps(MiddleConsumer {
                    pairing: &payload.adapter,
                }),
                build_create_encoding_params(Default::default()),
                build_create_encoding_payload(Default::default()),
            )
        }
    }

    let result = create_pairing(
        &build_create_pairing_deps(OuterConsumer, Default::default()),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    )
    .expect("the default selection constructs the pairing mock concrete")
    .output
    .expect("the default selection constructs the encoding mock concrete")
    .output
    .expect("the default selection constructs the chain-forms mock concrete")
    .output;

    // Assert
    assert!(result.is_ok());
}

/// Contract: `create_delivery_proof`, unsupported statement version: the
///   algebra and form are declared and `DECLARATION.statement_versions` does
///   not contain `params.statement_version` →
///   `Err(CreateDeliveryProofErrorReturn::UnsupportedStatementVersion)`,
///   nothing constructed and the consumer not called.
/// Arrange: params from `build_create_delivery_proof_params` overridden to
///   `concrete` `DeliveryProofConcrete::SchnorrFs` and `statement_version`
///   `DELIVERY_STATEMENT_VERSION_ONE + 1`, a version no entry of the
///   declaration's slice holds, the algebra and form at the defaults the
///   concrete's declaration lists.
/// Act:     `create_delivery_proof`, called in the innermost consumer's
///   method.
/// Assert:  `assert_eq!(result.err(),
///   Some(CreateDeliveryProofErrorReturn::UnsupportedStatementVersion))`.
/// Collaborators: as
///   `create_delivery_proof_returns_the_consumers_output_for_the_schnorr_fs_selection`;
///   the refusal is decided from the selected concrete's declaration
///   constant, before any construction, so no real concrete's function runs.
#[test]
fn create_delivery_proof_refuses_a_statement_version_the_schnorr_fs_concrete_does_not_declare() {
    struct OuterConsumer;

    impl IPairingConsumer for OuterConsumer {
        type Output = CreateEncodingReturn<CreateChainFormsReturn<CreateDeliveryProofReturn<()>>>;

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            struct MiddleConsumer<'a, P: IPairingArithmetic> {
                pairing: &'a P,
            }

            impl<P: IPairingArithmetic> IEncodingConsumer for MiddleConsumer<'_, P> {
                type Output = CreateChainFormsReturn<CreateDeliveryProofReturn<()>>;

                fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
                    &self,
                    _params: ConsumeEncodingParams,
                    consumed: ConsumeEncodingPayload<E>,
                ) -> Self::Output {
                    struct InnerConsumer<'a, P: IPairingArithmetic, E: IEncoderAdapter> {
                        pairing: &'a P,
                        encoder: &'a E,
                    }

                    impl<P: IPairingArithmetic, E: IEncoderAdapter> IChainFormsConsumer for InnerConsumer<'_, P, E> {
                        type Output = CreateDeliveryProofReturn<()>;

                        fn consume_chain_forms<F: IChainForms>(
                            &self,
                            _params: ConsumeChainFormsParams,
                            _payload: ConsumeChainFormsPayload<F>,
                        ) -> Self::Output {
                            // Arrange
                            let hash_to_scalar = create_hash_to_scalar::<P::Scalar>(
                                &build_create_hash_to_scalar_deps(Default::default()),
                                build_create_hash_to_scalar_params(Default::default()),
                                build_create_hash_to_scalar_payload(Default::default()),
                            )
                            .expect(
                                "the default selection constructs the hash-to-scalar mock concrete",
                            )
                            .adapter;
                            let random = create_random_source(
                                &CreateRandomSourceDeps,
                                build_create_random_source_params(Default::default()),
                                CreateRandomSourcePayload,
                            )
                            .expect(
                                "the default selection constructs the random-source mock concrete",
                            )
                            .adapter;

                            // Act
                            create_delivery_proof::<P, E, F, MockIDeliveryProofConsumer>(
                                &build_create_delivery_proof_deps(
                                    self.pairing,
                                    &*hash_to_scalar,
                                    self.encoder,
                                    &*random,
                                    MockIDeliveryProofConsumer,
                                    Default::default(),
                                ),
                                build_create_delivery_proof_params(
                                    CreateDeliveryProofParamsOverrides {
                                        concrete: Some(DeliveryProofConcrete::SchnorrFs),
                                        statement_version: Some(DELIVERY_STATEMENT_VERSION_ONE + 1),
                                        ..Default::default()
                                    },
                                ),
                                build_create_delivery_proof_payload(Default::default()),
                            )
                        }
                    }

                    create_chain_forms(
                        &build_create_chain_forms_deps(
                            InnerConsumer {
                                pairing: self.pairing,
                                encoder: &consumed.adapter,
                            },
                            Default::default(),
                        ),
                        build_create_chain_forms_params(Default::default()),
                        build_create_chain_forms_payload(Default::default()),
                    )
                }
            }

            create_encoding(
                &build_create_encoding_deps(MiddleConsumer {
                    pairing: &payload.adapter,
                }),
                build_create_encoding_params(Default::default()),
                build_create_encoding_payload(Default::default()),
            )
        }
    }

    let result = create_pairing(
        &build_create_pairing_deps(OuterConsumer, Default::default()),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    )
    .expect("the default selection constructs the pairing mock concrete")
    .output
    .expect("the default selection constructs the encoding mock concrete")
    .output
    .expect("the default selection constructs the chain-forms mock concrete")
    .output;

    // Assert
    assert_eq!(
        result.err(),
        Some(CreateDeliveryProofErrorReturn::UnsupportedStatementVersion)
    );
}

/// Contract: `create_delivery_proof`, mock selection, admitted:
///   `params.concrete` is `Mock` and every requirement is declared → the
///   consumer is called once with the mock concrete and the subject returns
///   `Ok(CreateDeliveryProofSuccessReturn { output })` holding the
///   consumer's output.
/// Arrange: params from `build_create_delivery_proof_params` at the
///   defaults, which select the mock concrete under `NoFailure` and admit it.
/// Act:     `create_delivery_proof`, called in the innermost consumer's
///   method.
/// Assert:  `assert!(result.is_ok())`; the output, the unit value of
///   `MockIDeliveryProofConsumer`, is not asserted.
/// Collaborators: as
///   `create_delivery_proof_returns_the_consumers_output_for_the_schnorr_fs_selection`;
///   the concrete is the family's mock concrete the `Mock` selection
///   constructs.
#[test]
fn create_delivery_proof_returns_the_consumers_output_for_the_mock_selection() {
    struct OuterConsumer;

    impl IPairingConsumer for OuterConsumer {
        type Output = CreateEncodingReturn<CreateChainFormsReturn<CreateDeliveryProofReturn<()>>>;

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            struct MiddleConsumer<'a, P: IPairingArithmetic> {
                pairing: &'a P,
            }

            impl<P: IPairingArithmetic> IEncodingConsumer for MiddleConsumer<'_, P> {
                type Output = CreateChainFormsReturn<CreateDeliveryProofReturn<()>>;

                fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
                    &self,
                    _params: ConsumeEncodingParams,
                    consumed: ConsumeEncodingPayload<E>,
                ) -> Self::Output {
                    struct InnerConsumer<'a, P: IPairingArithmetic, E: IEncoderAdapter> {
                        pairing: &'a P,
                        encoder: &'a E,
                    }

                    impl<P: IPairingArithmetic, E: IEncoderAdapter> IChainFormsConsumer for InnerConsumer<'_, P, E> {
                        type Output = CreateDeliveryProofReturn<()>;

                        fn consume_chain_forms<F: IChainForms>(
                            &self,
                            _params: ConsumeChainFormsParams,
                            _payload: ConsumeChainFormsPayload<F>,
                        ) -> Self::Output {
                            // Arrange
                            let hash_to_scalar = create_hash_to_scalar::<P::Scalar>(
                                &build_create_hash_to_scalar_deps(Default::default()),
                                build_create_hash_to_scalar_params(Default::default()),
                                build_create_hash_to_scalar_payload(Default::default()),
                            )
                            .expect(
                                "the default selection constructs the hash-to-scalar mock concrete",
                            )
                            .adapter;
                            let random = create_random_source(
                                &CreateRandomSourceDeps,
                                build_create_random_source_params(Default::default()),
                                CreateRandomSourcePayload,
                            )
                            .expect(
                                "the default selection constructs the random-source mock concrete",
                            )
                            .adapter;

                            // Act
                            create_delivery_proof::<P, E, F, MockIDeliveryProofConsumer>(
                                &build_create_delivery_proof_deps(
                                    self.pairing,
                                    &*hash_to_scalar,
                                    self.encoder,
                                    &*random,
                                    MockIDeliveryProofConsumer,
                                    Default::default(),
                                ),
                                build_create_delivery_proof_params(Default::default()),
                                build_create_delivery_proof_payload(Default::default()),
                            )
                        }
                    }

                    create_chain_forms(
                        &build_create_chain_forms_deps(
                            InnerConsumer {
                                pairing: self.pairing,
                                encoder: &consumed.adapter,
                            },
                            Default::default(),
                        ),
                        build_create_chain_forms_params(Default::default()),
                        build_create_chain_forms_payload(Default::default()),
                    )
                }
            }

            create_encoding(
                &build_create_encoding_deps(MiddleConsumer {
                    pairing: &payload.adapter,
                }),
                build_create_encoding_params(Default::default()),
                build_create_encoding_payload(Default::default()),
            )
        }
    }

    let result = create_pairing(
        &build_create_pairing_deps(OuterConsumer, Default::default()),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    )
    .expect("the default selection constructs the pairing mock concrete")
    .output
    .expect("the default selection constructs the encoding mock concrete")
    .output
    .expect("the default selection constructs the chain-forms mock concrete")
    .output;

    // Assert
    assert!(result.is_ok());
}

/// Contract: `create_delivery_proof`, mock selection, unsupported statement
///   version: `params.concrete` is `Mock` and `DECLARATION.statement_versions`
///   does not contain `params.statement_version` →
///   `Err(CreateDeliveryProofErrorReturn::UnsupportedStatementVersion)`,
///   nothing constructed and the consumer not called.
/// Arrange: params at the defaults with `statement_version` overridden to
///   `DELIVERY_STATEMENT_VERSION_ONE + 1`.
/// Act:     `create_delivery_proof`, called in the innermost consumer's
///   method.
/// Assert:  `assert_eq!(result.err(),
///   Some(CreateDeliveryProofErrorReturn::UnsupportedStatementVersion))`.
/// Collaborators: as
///   `create_delivery_proof_returns_the_consumers_output_for_the_schnorr_fs_selection`;
///   the refusal is decided from the mock concrete's declaration constant,
///   before any construction.
#[test]
fn create_delivery_proof_refuses_a_statement_version_the_mock_concrete_does_not_declare() {
    struct OuterConsumer;

    impl IPairingConsumer for OuterConsumer {
        type Output = CreateEncodingReturn<CreateChainFormsReturn<CreateDeliveryProofReturn<()>>>;

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            struct MiddleConsumer<'a, P: IPairingArithmetic> {
                pairing: &'a P,
            }

            impl<P: IPairingArithmetic> IEncodingConsumer for MiddleConsumer<'_, P> {
                type Output = CreateChainFormsReturn<CreateDeliveryProofReturn<()>>;

                fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
                    &self,
                    _params: ConsumeEncodingParams,
                    consumed: ConsumeEncodingPayload<E>,
                ) -> Self::Output {
                    struct InnerConsumer<'a, P: IPairingArithmetic, E: IEncoderAdapter> {
                        pairing: &'a P,
                        encoder: &'a E,
                    }

                    impl<P: IPairingArithmetic, E: IEncoderAdapter> IChainFormsConsumer for InnerConsumer<'_, P, E> {
                        type Output = CreateDeliveryProofReturn<()>;

                        fn consume_chain_forms<F: IChainForms>(
                            &self,
                            _params: ConsumeChainFormsParams,
                            _payload: ConsumeChainFormsPayload<F>,
                        ) -> Self::Output {
                            // Arrange
                            let hash_to_scalar = create_hash_to_scalar::<P::Scalar>(
                                &build_create_hash_to_scalar_deps(Default::default()),
                                build_create_hash_to_scalar_params(Default::default()),
                                build_create_hash_to_scalar_payload(Default::default()),
                            )
                            .expect(
                                "the default selection constructs the hash-to-scalar mock concrete",
                            )
                            .adapter;
                            let random = create_random_source(
                                &CreateRandomSourceDeps,
                                build_create_random_source_params(Default::default()),
                                CreateRandomSourcePayload,
                            )
                            .expect(
                                "the default selection constructs the random-source mock concrete",
                            )
                            .adapter;

                            // Act
                            create_delivery_proof::<P, E, F, MockIDeliveryProofConsumer>(
                                &build_create_delivery_proof_deps(
                                    self.pairing,
                                    &*hash_to_scalar,
                                    self.encoder,
                                    &*random,
                                    MockIDeliveryProofConsumer,
                                    Default::default(),
                                ),
                                build_create_delivery_proof_params(
                                    CreateDeliveryProofParamsOverrides {
                                        statement_version: Some(DELIVERY_STATEMENT_VERSION_ONE + 1),
                                        ..Default::default()
                                    },
                                ),
                                build_create_delivery_proof_payload(Default::default()),
                            )
                        }
                    }

                    create_chain_forms(
                        &build_create_chain_forms_deps(
                            InnerConsumer {
                                pairing: self.pairing,
                                encoder: &consumed.adapter,
                            },
                            Default::default(),
                        ),
                        build_create_chain_forms_params(Default::default()),
                        build_create_chain_forms_payload(Default::default()),
                    )
                }
            }

            create_encoding(
                &build_create_encoding_deps(MiddleConsumer {
                    pairing: &payload.adapter,
                }),
                build_create_encoding_params(Default::default()),
                build_create_encoding_payload(Default::default()),
            )
        }
    }

    let result = create_pairing(
        &build_create_pairing_deps(OuterConsumer, Default::default()),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    )
    .expect("the default selection constructs the pairing mock concrete")
    .output
    .expect("the default selection constructs the encoding mock concrete")
    .output
    .expect("the default selection constructs the chain-forms mock concrete")
    .output;

    // Assert
    assert_eq!(
        result.err(),
        Some(CreateDeliveryProofErrorReturn::UnsupportedStatementVersion)
    );
}
