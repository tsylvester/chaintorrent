use core::convert::Infallible;

pub struct OsRandomSource;

pub struct OsRandomSourceConstructorParams;

pub type OsRandomSourceTryNewReturn = Result<OsRandomSource, Infallible>;

pub enum OsRandomSourceFillBytesErrorReturn {
    OperatingSystem(getrandom::Error),
}
