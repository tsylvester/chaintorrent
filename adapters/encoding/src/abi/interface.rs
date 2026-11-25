use crate::factory::provides::{
    CanonicalFieldKind, ENCODING_INTERFACE_VERSION, EncodingConcrete, EncodingDeclaration,
    EncodingIdentifier,
};
use core::convert::Infallible;

pub struct AbiEncoding;

impl AbiEncoding {
    pub const DECLARATION: EncodingDeclaration = EncodingDeclaration {
        concrete: EncodingConcrete::Abi,
        identifier: EncodingIdentifier::EthereumAbiV1,
        adapter_version: 1,
        interface_version: ENCODING_INTERFACE_VERSION,
        type_name: AbiEncoding::type_name,
    };
}

pub struct AbiEncodingConstructorParams;

pub type AbiEncodingTryNewReturn = Result<AbiEncoding, Infallible>;

#[derive(Debug, PartialEq)]
pub enum AbiDecoderErrorReturn {
    Malformed(alloy::dyn_abi::Error),
    DecodedShapeMismatch {
        index: usize,
    },
    ValueOutOfRange {
        index: usize,
        kind: CanonicalFieldKind,
    },
}
