pub const DOMAIN_TAG_MAXIMUM_LENGTH: usize = 255;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DomainTag {
    pub(super) bytes: Vec<u8>,
}

pub struct DomainTagConstructorParams {
    pub bytes: Vec<u8>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum DomainTagTryNewErrorReturn {
    Empty,
    TooLong { length: usize, maximum: usize },
    ByteOutsidePrintableAscii { index: usize, byte: u8 },
    SpaceAtEdge { index: usize },
}

pub type DomainTagTryNewReturn = Result<DomainTag, DomainTagTryNewErrorReturn>;
