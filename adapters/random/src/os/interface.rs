use crate::factory::provides::{
    RANDOM_SOURCE_INTERFACE_VERSION, RandomSourceDeclaration, RandomSourceKind,
};
use core::convert::Infallible;

pub struct OsRandomSource;

pub struct OsRandomSourceConstructorParams;

pub type OsRandomSourceTryNewReturn = Result<OsRandomSource, Infallible>;

#[derive(Debug, PartialEq, Eq)]
pub enum OsRandomSourceFillBytesErrorReturn {
    OperatingSystem(getrandom::Error),
}

impl OsRandomSource {
    pub const DECLARATION: RandomSourceDeclaration = RandomSourceDeclaration {
        source: RandomSourceKind::OperatingSystem,
        adapter_version: 1,
        interface_version: RANDOM_SOURCE_INTERFACE_VERSION,
    };
}
