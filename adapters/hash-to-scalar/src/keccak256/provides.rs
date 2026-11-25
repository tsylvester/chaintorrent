pub(crate) use super::interface::*;
#[cfg(any(test, feature = "mocks"))]
pub use super::mock::{
    BINDING_LONGER_TAG, BINDING_SHORTER_MESSAGE, EMPTY_MESSAGE, OTHER_TAG, REFERENCE_MESSAGE,
    REFERENCE_TAG, expected_uniform_bytes,
};
