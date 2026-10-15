use super::create_chain_forms;
use super::interface::ChainFormsConcrete;
use super::mock::{
    CreateChainFormsParamsOverrides, MockIChainFormsConsumer, build_create_chain_forms_deps,
    build_create_chain_forms_params, build_create_chain_forms_payload,
};
use crate::factory::provides::ChainFormsIdentifier;

#[test]
fn create_chain_forms_admits_and_constructs_the_evm_forms() {
    let deps = build_create_chain_forms_deps(MockIChainFormsConsumer, Default::default());
    let params = build_create_chain_forms_params(CreateChainFormsParamsOverrides {
        concrete: Some(ChainFormsConcrete::Evm),
        identifier: Some(ChainFormsIdentifier::EvmV1),
    });
    let payload = build_create_chain_forms_payload(Default::default());
    let result = create_chain_forms(&deps, params, payload);
    assert!(result.is_ok());
}

#[test]
fn create_chain_forms_admits_and_constructs_the_mock_concrete() {
    let deps = build_create_chain_forms_deps(MockIChainFormsConsumer, Default::default());
    let params = build_create_chain_forms_params(Default::default());
    let payload = build_create_chain_forms_payload(Default::default());
    let result = create_chain_forms(&deps, params, payload);
    assert!(result.is_ok());
}
