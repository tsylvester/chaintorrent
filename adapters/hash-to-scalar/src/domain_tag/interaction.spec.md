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

## Integration: own entries

Each entry is named and proven by the public integration block of the same name.

- `a_tag_of_the_maximum_length_reads_back_through_the_crate_root`: condition a caller outside the crate names `DomainTag`, `DomainTagConstructorParams`, and `DOMAIN_TAG_MAXIMUM_LENGTH` through the crate root and passes `try_new` a tag of exactly `DOMAIN_TAG_MAXIMUM_LENGTH` printable bytes with an interior space and a visible byte at each edge; outcome `Ok(tag)` whose `as_bytes()` yields exactly the bytes passed; variation the maximum length with an interior space, which a read that truncates, trims, or re-encodes fails; edge that must survive: the bytes an outside caller reads are the bytes the tag was constructed from, reached through the accessor alone.
- `a_tag_one_byte_over_the_maximum_is_refused_whole_through_the_crate_root`: condition a caller outside the crate passes `try_new` a tag of `DOMAIN_TAG_MAXIMUM_LENGTH + 1` printable bytes; outcome `Err(DomainTagTryNewErrorReturn::TooLong { length: DOMAIN_TAG_MAXIMUM_LENGTH + 1, maximum: DOMAIN_TAG_MAXIMUM_LENGTH })`, matched through the crate-root error type, and no `DomainTag` exists to read; variation one byte over the length the read-back entry admits; edge that must survive: the refusal carries both values it failed on.

## Integration: public surface

An outside caller invokes `DomainTag::try_new` and `DomainTag::as_bytes` through the crate root, building params with `build_domain_tag_constructor_params` under the `mocks` feature, and observes the `Ok` tag through `as_bytes` and the `Err` through `DomainTagTryNewErrorReturn`. The entries proven are `a_tag_of_the_maximum_length_reads_back_through_the_crate_root` and `a_tag_one_byte_over_the_maximum_is_refused_whole_through_the_crate_root` above. The route of each is the crate barrel `lib.rs`, then `domain_tag::provides`, then `DomainTag::try_new`, and for the read-back entry `DomainTag::as_bytes`; the outer-edge collaborators on that route are none.

