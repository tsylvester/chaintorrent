#![allow(dead_code, clippy::too_many_arguments)]

use super::interface::{OsRandomSource, OsRandomSourceConstructorParams};

pub fn build_os_random_source() -> OsRandomSource {
    let Ok(source) = OsRandomSource::try_new(OsRandomSourceConstructorParams);
    source
}
