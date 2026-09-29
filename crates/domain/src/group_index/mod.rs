mod interface;
#[cfg(any(test, feature = "mocks"))]
mod mock;
pub mod provides;
#[cfg(test)]
mod test;

use interface::{GroupIndex, GroupIndexConstructorParams, GroupIndexTryNewReturn};

impl GroupIndex {
    pub fn try_new(params: GroupIndexConstructorParams) -> GroupIndexTryNewReturn {
        Ok(GroupIndex {
            value: params.value,
        })
    }

    pub fn value(&self) -> u64 {
        self.value
    }
}
