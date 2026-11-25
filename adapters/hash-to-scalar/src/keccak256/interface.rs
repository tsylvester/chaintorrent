use crate::factory::provides::{
    HASH_TO_SCALAR_INTERFACE_VERSION, HashToScalarConcrete, HashToScalarDeclaration,
    HashToScalarIdentifier,
};
use core::convert::Infallible;

pub const KECCAK256_DIGEST_LENGTH: usize = 32;

pub struct Keccak256HashToScalar;

pub struct Keccak256HashToScalarConstructorParams;

pub type Keccak256HashToScalarTryNewReturn = Result<Keccak256HashToScalar, Infallible>;

#[derive(Debug, PartialEq, Eq)]
pub enum Keccak256HashToScalarErrorReturn {
    TagLengthExceedsPrefix {
        length: usize,
    },
    UniformLengthBelowDigest {
        uniform_length: usize,
        digest_length: usize,
    },
}

impl Keccak256HashToScalar {
    pub const DECLARATION: HashToScalarDeclaration = HashToScalarDeclaration {
        concrete: HashToScalarConcrete::Keccak256,
        identifier: HashToScalarIdentifier::Keccak256V1,
        adapter_version: 1,
        interface_version: HASH_TO_SCALAR_INTERFACE_VERSION,
    };
}
