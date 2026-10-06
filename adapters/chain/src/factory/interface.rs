use encoding::ICanonicalField;

pub const CHAIN_FORMS_INTERFACE_VERSION: u32 = 1;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ChainFormsIdentifier {
    EvmV1,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ChainFormsDeclaration {
    pub identifier: ChainFormsIdentifier,
    pub adapter_version: u32,
    pub interface_version: u32,
}

pub trait IChainForms {
    const DECLARATION: ChainFormsDeclaration;

    type Identity: ICanonicalField + Clone;
    type Entitlement: ICanonicalField + Clone;
    type Interval: ICanonicalField + Clone;
    type ChainIdentifier: ICanonicalField + Clone;
}
