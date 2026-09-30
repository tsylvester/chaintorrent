use core::convert::Infallible;
use pairing::SampleUniformScalarErrorReturn;

pub const KECCAK256_DIGEST_LENGTH: usize = 32;

pub struct Keccak256HashToScalar;

pub struct Keccak256HashToScalarConstructorParams;

pub type Keccak256HashToScalarTryNewReturn = Result<Keccak256HashToScalar, Infallible>;

pub enum Keccak256HashToScalarErrorReturn {
    TagLengthExceedsPrefix {
        length: usize,
    },
    UniformLengthBelowDigest {
        uniform_length: usize,
        digest_length: usize,
    },
    Sampling(SampleUniformScalarErrorReturn),
}
