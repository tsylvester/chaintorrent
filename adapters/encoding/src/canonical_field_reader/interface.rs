use crate::factory::provides::{CanonicalFieldKind, CanonicalFieldValue, FromFieldErrorReturn};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CanonicalFieldReaderErrorReturn {
    Count {
        expected: usize,
        actual: usize,
    },
    Kind {
        index: usize,
        expected: CanonicalFieldKind,
        actual: CanonicalFieldKind,
    },
    Form {
        index: usize,
        error: FromFieldErrorReturn,
    },
}

pub struct CanonicalFieldReader {
    pub(super) values: Vec<CanonicalFieldValue>,
    pub(super) fields: &'static [CanonicalFieldKind],
    pub(super) index: usize,
}

pub type CanonicalFieldReaderTryNewReturn =
    Result<CanonicalFieldReader, CanonicalFieldReaderErrorReturn>;

pub type CanonicalFieldReaderReadReturn<T> = Result<T, CanonicalFieldReaderErrorReturn>;
