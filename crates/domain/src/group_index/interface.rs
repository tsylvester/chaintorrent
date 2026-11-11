use core::convert::Infallible;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GroupIndex {
    pub(super) value: u64,
}

pub struct GroupIndexConstructorParams {
    pub value: u64,
}

pub type GroupIndexTryNewReturn = Result<GroupIndex, Infallible>;
