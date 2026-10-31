use super::create_chain_forms;
use super::interface::{
    ChainFormsConcrete, ConsumeCaseParams, ConsumeCasePayload, CreateChainFormsErrorReturn,
    CreateChainFormsParams, CreateChainFormsReturn, ICreateChainFormsCaseConsumer,
};
use super::mock::{
    CreateChainFormsCaseParamsOverrides, CreateChainFormsParamsOverrides, MockIChainFormsConsumer,
    build_create_chain_forms_case_deps, build_create_chain_forms_case_params,
    build_create_chain_forms_case_payload, build_create_chain_forms_deps,
    build_create_chain_forms_params, build_create_chain_forms_payload, create_chain_forms_case,
};
use encoding::{
    CreateEncodingParams, DECLARED_ENCODING_SELECTIONS, EncodingIdentifier, IEncoderAdapter,
};

/// Contract: `create_chain_forms`, admitted: `params.concrete` is `Evm`,
///   `params.identifier` is the identifier `EvmForms::DECLARATION` carries,
///   and the encoder declares `EncodingIdentifier::EthereumAbiV1` →
///   `Ok(CreateChainFormsSuccessReturn { output })`.
/// Arrange: the block's `CaseConsumer`, holding params from
///   `build_create_chain_forms_params` overriding `concrete` with
///   `ChainFormsConcrete::Evm`, the identifier at its default for that
///   concrete; case params from `build_create_chain_forms_case_params`
///   overriding `encoding` with the member of `DECLARED_ENCODING_SELECTIONS`,
///   which declares `EthereumAbiV1`; the `Evm` selection over the ABI
///   concrete is the variation, since the builder defaults select `Mock`
///   over the mock concrete. The case consumer calls `create_chain_forms`
///   once over `MockIChainFormsConsumer` and the encoder the encoding
///   level hands it, and returns the result whole.
/// Act:     `create_chain_forms_case(&deps, case_params, case_payload)`
///   per member of `DECLARED_ENCODING_SELECTIONS`.
/// Assert:  the case result is `Ok`, and the output it holds is an `Ok`.
#[test]
fn create_chain_forms_admits_and_constructs_the_evm_forms() {
    struct CaseConsumer {
        params: CreateChainFormsParams,
    }

    impl ICreateChainFormsCaseConsumer for CaseConsumer {
        type Output = CreateChainFormsReturn<()>;

        fn consume_case<E: IEncoderAdapter>(
            &self,
            _params: ConsumeCaseParams,
            payload: ConsumeCasePayload<'_, E>,
        ) -> Self::Output {
            create_chain_forms(
                &build_create_chain_forms_deps(
                    MockIChainFormsConsumer,
                    payload.encoder,
                    Default::default(),
                ),
                self.params,
                build_create_chain_forms_payload(Default::default()),
            )
        }
    }

    // Arrange
    let params = build_create_chain_forms_params(CreateChainFormsParamsOverrides {
        concrete: Some(ChainFormsConcrete::Evm),
        ..Default::default()
    });
    let deps = build_create_chain_forms_case_deps(CaseConsumer { params }, Default::default());
    for selection in DECLARED_ENCODING_SELECTIONS {
        assert_eq!(selection.identifier, EncodingIdentifier::EthereumAbiV1);
        let case_params =
            build_create_chain_forms_case_params(CreateChainFormsCaseParamsOverrides {
                encoding: Some(CreateEncodingParams {
                    concrete: selection.concrete,
                    identifier: selection.identifier,
                }),
            });
        let case_payload = build_create_chain_forms_case_payload(Default::default());

        // Act
        let result = create_chain_forms_case(&deps, case_params, case_payload);

        // Assert
        assert!(result.is_ok_and(|success| success.output.is_ok()));
    }
}
// create_chain_forms_admits_and_constructs_the_evm_forms

/// Contract: `create_chain_forms`, admitted: `params.concrete` is `Mock`,
///   `params.identifier` is the identifier `MockIChainForms::DECLARATION`
///   carries, and the encoder declares `EncodingIdentifier::Mock` →
///   `Ok(CreateChainFormsSuccessReturn { output })`.
/// Arrange: the block's `CaseConsumer`, holding params from
///   `build_create_chain_forms_params` at its defaults, which select `Mock`
///   and carry the identifier the mock concrete declares; case params from
///   `build_create_chain_forms_case_params` at its defaults, which select
///   the mock encoding concrete; the `Mock` selection is the variation.
/// Act:     `create_chain_forms_case(&deps, case_params, case_payload)`.
/// Assert:  the case result is `Ok`, and the output it holds is an `Ok`.
#[test]
fn create_chain_forms_admits_and_constructs_the_mock_concrete() {
    struct CaseConsumer {
        params: CreateChainFormsParams,
    }

    impl ICreateChainFormsCaseConsumer for CaseConsumer {
        type Output = CreateChainFormsReturn<()>;

        fn consume_case<E: IEncoderAdapter>(
            &self,
            _params: ConsumeCaseParams,
            payload: ConsumeCasePayload<'_, E>,
        ) -> Self::Output {
            create_chain_forms(
                &build_create_chain_forms_deps(
                    MockIChainFormsConsumer,
                    payload.encoder,
                    Default::default(),
                ),
                self.params,
                build_create_chain_forms_payload(Default::default()),
            )
        }
    }

    // Arrange
    let params = build_create_chain_forms_params(Default::default());
    let deps = build_create_chain_forms_case_deps(CaseConsumer { params }, Default::default());
    let case_params = build_create_chain_forms_case_params(Default::default());
    let case_payload = build_create_chain_forms_case_payload(Default::default());

    // Act
    let result = create_chain_forms_case(&deps, case_params, case_payload);

    // Assert
    assert!(result.is_ok_and(|success| success.output.is_ok()));
}
// create_chain_forms_admits_and_constructs_the_mock_concrete

/// Contract: `create_chain_forms`, unsupported encoding identifier →
///   `Err(CreateChainFormsErrorReturn::UnsupportedEncodingIdentifier)`, the
///   consumer not called.
/// Arrange: the block's `CaseConsumer`, holding params selecting `Evm`
///   with its declared identifier; case params at their defaults, so the
///   encoder declares `EncodingIdentifier::Mock`, which the `Evm`
///   concrete does not require.
/// Act:     `create_chain_forms_case(&deps, case_params, case_payload)`.
/// Assert:  the case result is `Ok` and the output it holds is
///   `Err(CreateChainFormsErrorReturn::UnsupportedEncodingIdentifier)` by
///   `assert_eq!`.
#[test]
fn create_chain_forms_refuses_an_encoder_declaring_another_encoding_identifier_for_the_evm_forms() {
    struct CaseConsumer {
        params: CreateChainFormsParams,
    }

    impl ICreateChainFormsCaseConsumer for CaseConsumer {
        type Output = CreateChainFormsReturn<()>;

        fn consume_case<E: IEncoderAdapter>(
            &self,
            _params: ConsumeCaseParams,
            payload: ConsumeCasePayload<'_, E>,
        ) -> Self::Output {
            create_chain_forms(
                &build_create_chain_forms_deps(
                    MockIChainFormsConsumer,
                    payload.encoder,
                    Default::default(),
                ),
                self.params,
                build_create_chain_forms_payload(Default::default()),
            )
        }
    }

    // Arrange
    let params = build_create_chain_forms_params(CreateChainFormsParamsOverrides {
        concrete: Some(ChainFormsConcrete::Evm),
        ..Default::default()
    });
    let deps = build_create_chain_forms_case_deps(CaseConsumer { params }, Default::default());
    let case_params = build_create_chain_forms_case_params(Default::default());
    let case_payload = build_create_chain_forms_case_payload(Default::default());

    // Act
    let result = create_chain_forms_case(&deps, case_params, case_payload);

    // Assert
    assert_eq!(
        result
            .expect("the encoding level admits the selection")
            .output
            .err(),
        Some(CreateChainFormsErrorReturn::UnsupportedEncodingIdentifier)
    );
}
// create_chain_forms_refuses_an_encoder_declaring_another_encoding_identifier_for_the_evm_forms

/// Contract: `create_chain_forms`, unsupported encoding identifier under
///   the `Mock` selection →
///   `Err(CreateChainFormsErrorReturn::UnsupportedEncodingIdentifier)`.
/// Arrange: the block's `CaseConsumer`, holding params at their defaults,
///   the `Mock` selection; case params overriding `encoding` with the
///   member of `DECLARED_ENCODING_SELECTIONS`, so the encoder declares
///   `EthereumAbiV1`, which the mock concrete does not require.
/// Act:     `create_chain_forms_case(&deps, case_params, case_payload)`
///   per member of `DECLARED_ENCODING_SELECTIONS`.
/// Assert:  the case result is `Ok` and the output it holds is
///   `Err(CreateChainFormsErrorReturn::UnsupportedEncodingIdentifier)` by
///   `assert_eq!`.
#[test]
fn create_chain_forms_refuses_an_encoder_declaring_another_encoding_identifier_for_the_mock_concrete()
 {
    struct CaseConsumer {
        params: CreateChainFormsParams,
    }

    impl ICreateChainFormsCaseConsumer for CaseConsumer {
        type Output = CreateChainFormsReturn<()>;

        fn consume_case<E: IEncoderAdapter>(
            &self,
            _params: ConsumeCaseParams,
            payload: ConsumeCasePayload<'_, E>,
        ) -> Self::Output {
            create_chain_forms(
                &build_create_chain_forms_deps(
                    MockIChainFormsConsumer,
                    payload.encoder,
                    Default::default(),
                ),
                self.params,
                build_create_chain_forms_payload(Default::default()),
            )
        }
    }

    // Arrange
    let params = build_create_chain_forms_params(Default::default());
    let deps = build_create_chain_forms_case_deps(CaseConsumer { params }, Default::default());
    for selection in DECLARED_ENCODING_SELECTIONS {
        assert_eq!(selection.identifier, EncodingIdentifier::EthereumAbiV1);
        let case_params =
            build_create_chain_forms_case_params(CreateChainFormsCaseParamsOverrides {
                encoding: Some(CreateEncodingParams {
                    concrete: selection.concrete,
                    identifier: selection.identifier,
                }),
            });
        let case_payload = build_create_chain_forms_case_payload(Default::default());

        // Act
        let result = create_chain_forms_case(&deps, case_params, case_payload);

        // Assert
        assert_eq!(
            result
                .expect("the encoding level admits the selection")
                .output
                .err(),
            Some(CreateChainFormsErrorReturn::UnsupportedEncodingIdentifier)
        );
    }
}
// create_chain_forms_refuses_an_encoder_declaring_another_encoding_identifier_for_the_mock_concrete
