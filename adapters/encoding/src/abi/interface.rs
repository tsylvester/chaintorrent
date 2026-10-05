use crate::factory::provides::CanonicalFieldKind;
use core::convert::Infallible;

pub struct AbiEncoding;

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
    NonCanonical,
}
