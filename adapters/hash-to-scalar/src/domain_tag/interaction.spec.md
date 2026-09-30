# `domain_tag` — interaction spec

Branch contract for the `domain_tag` module of the `hash-to-scalar` crate: the family-owned domain-tag value type. Each branch states condition, decision, dependency call, and the exact return outcome.

## `DomainTag::try_new(params: DomainTagConstructorParams) -> DomainTagTryNewReturn`

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| empty | `params.bytes.is_empty()` | the emptiness check | none | `Err(DomainTagTryNewErrorReturn::Empty)` |
| too long | the tag is non-empty and `params.bytes.len() > DOMAIN_TAG_MAXIMUM_LENGTH` | the comparison | none | `Err(DomainTagTryNewErrorReturn::TooLong { length, maximum: DOMAIN_TAG_MAXIMUM_LENGTH })`, `length` the tag's length |
| byte outside printable ASCII | the length passes and some byte is neither `b' '` nor `is_ascii_graphic` | the first such byte by index, `iter().enumerate().find(…)` | none | `Err(DomainTagTryNewErrorReturn::ByteOutsidePrintableAscii { index, byte })` for the lowest such index |
| leading space | every byte is printable ASCII and `params.bytes.first()` is `Some(&b' ')` | the comparison | none | `Err(DomainTagTryNewErrorReturn::SpaceAtEdge { index: 0 })` |
| trailing space | the leading byte is not a space and `params.bytes.last()` is `Some(&b' ')` | the comparison | none | `Err(DomainTagTryNewErrorReturn::SpaceAtEdge { index })`, `index` the tag's length less one |
| admitted | every check passes | all checks pass | none | `Ok(DomainTag { bytes })`, the vector moved from the params without copy |

## `DomainTag::as_bytes(&self) -> &[u8]`

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| read | any instance | none | none | a shared reference to the held bytes, no copy, no side effect |

## Ordering and invariants

- The checks run in order: emptiness, then length, then the byte scan, then the leading edge, then the trailing edge. The same params always yield the same outcome.
- Every `DomainTag` holds between 1 and 255 bytes, each printable ASCII, its first and last byte visible ASCII; its only producer is `try_new`.
