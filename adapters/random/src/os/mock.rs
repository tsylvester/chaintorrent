use super::interface::{OsRandomSource, OsRandomSourceConstructorParams};

pub fn build_os_random_source() -> OsRandomSource {
    let Ok(source) = OsRandomSource::try_new(OsRandomSourceConstructorParams);
    source
}
