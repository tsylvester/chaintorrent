mod interface;
#[cfg(any(test, feature = "mocks"))]
mod mock;
pub mod provides;
#[cfg(test)]
mod test;

use interface::{
    DOMAIN_TAG_MAXIMUM_LENGTH, DomainTag, DomainTagConstructorParams, DomainTagTryNewErrorReturn,
    DomainTagTryNewReturn,
};

impl DomainTag {
    pub fn try_new(params: DomainTagConstructorParams) -> DomainTagTryNewReturn {
        if params.bytes.is_empty() {
            return Err(DomainTagTryNewErrorReturn::Empty);
        }
        if params.bytes.len() > DOMAIN_TAG_MAXIMUM_LENGTH {
            return Err(DomainTagTryNewErrorReturn::TooLong {
                length: params.bytes.len(),
                maximum: DOMAIN_TAG_MAXIMUM_LENGTH,
            });
        }
        if let Some((index, byte)) = params
            .bytes
            .iter()
            .enumerate()
            .find(|(_, byte)| **byte != b' ' && !byte.is_ascii_graphic())
        {
            return Err(DomainTagTryNewErrorReturn::ByteOutsidePrintableAscii {
                index,
                byte: *byte,
            });
        }
        if params.bytes.first() == Some(&b' ') {
            return Err(DomainTagTryNewErrorReturn::SpaceAtEdge { index: 0 });
        }
        if params.bytes.last() == Some(&b' ') {
            return Err(DomainTagTryNewErrorReturn::SpaceAtEdge {
                index: params.bytes.len() - 1,
            });
        }
        Ok(DomainTag {
            bytes: params.bytes,
        })
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes
    }
}
