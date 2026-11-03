#![allow(dead_code, clippy::expect_used)]

use super::interface::CanonicalFieldReader;
use crate::factory::provides::{CanonicalFieldKind, CanonicalFieldValue, CanonicalFields};

#[derive(Default)]
pub struct CanonicalFieldReaderOverrides {
    pub values: Option<Vec<CanonicalFieldValue>>,
}

pub fn build_canonical_field_reader(
    fields: &'static [CanonicalFieldKind],
    overrides: CanonicalFieldReaderOverrides,
) -> CanonicalFieldReader {
    let values = overrides.values.unwrap_or_else(|| {
        fields
            .iter()
            .map(|kind| match kind {
                CanonicalFieldKind::FixedBytes32 => CanonicalFieldValue::FixedBytes32([0x00; 32]),
                CanonicalFieldKind::Unsigned16 => CanonicalFieldValue::Unsigned16(0),
                CanonicalFieldKind::Unsigned32 => CanonicalFieldValue::Unsigned32(0),
                CanonicalFieldKind::Unsigned64 => CanonicalFieldValue::Unsigned64(0),
                CanonicalFieldKind::Text => CanonicalFieldValue::Text(String::new()),
                CanonicalFieldKind::Bytes => CanonicalFieldValue::Bytes(Vec::new()),
                CanonicalFieldKind::FixedBytes20 => CanonicalFieldValue::FixedBytes20([0x00; 20]),
                CanonicalFieldKind::Unsigned256 => CanonicalFieldValue::Unsigned256([0x00; 32]),
            })
            .collect()
    });
    CanonicalFieldReader::try_new(CanonicalFields { values }, fields)
        .expect("the built values match the field list in count")
}
