# `canonical_field_reader` — interaction spec

Branch contract for the `canonical_field_reader` module of the `encoding` crate: `CanonicalFieldReader`, the family-owned reader that checks a canonical field list against a description's field kinds once, in count and in kind, and hands each value back by its kind or as a form; and `CanonicalFieldValue::kind`, the one map from a value to its kind. Each branch states condition, decision, dependency call, and the exact return outcome.

## `CanonicalFieldValue::kind(&self) -> CanonicalFieldKind`

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| kind stated | any `CanonicalFieldValue` | a `match` on `self`, exhaustive with no wildcard arm, so a variant with no arm fails to compile | none | the like-named `CanonicalFieldKind` variant, `FixedBytes32(_)` to `FixedBytes32`, `Unsigned16(_)` to `Unsigned16`, `Unsigned32(_)` to `Unsigned32`, `Unsigned64(_)` to `Unsigned64`, `Text(_)` to `Text`, `Bytes(_)` to `Bytes`, `FixedBytes20(_)` to `FixedBytes20`, and `Unsigned256(_)` to `Unsigned256`; the function has no error arm |

## `CanonicalFieldReader::try_new(payload: CanonicalFields, fields: &'static [CanonicalFieldKind]) -> CanonicalFieldReaderTryNewReturn`

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| wrong count | `payload.values.len()` differs from `fields.len()` | length equality, the only check `try_new` makes; no kind is compared | none | `Err(CanonicalFieldReaderErrorReturn::Count { expected: fields.len(), actual: payload.values.len() })` |
| admitted | the lengths are equal | the check passes | none | `Ok(CanonicalFieldReader)` holding `payload.values`, `fields`, and `index` `0` |

## `read_value(&mut self) -> CanonicalFieldReaderReadReturn<CanonicalFieldValue>`

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| past the end | `values.get(index)` or `fields.get(index)` is `None` | `let … else` | none | `Err(CanonicalFieldReaderErrorReturn::Count { expected: fields.len(), actual: index + 1 })`, `index` unchanged |
| wrong kind | the value at `index` has a `kind()` other than `fields[index]` | equality of the two kinds, the one comparison of a value's kind with its field | none | `Err(CanonicalFieldReaderErrorReturn::Kind { index, expected: fields[index], actual: value.kind() })`, `index` unchanged |
| admitted | the kinds are equal | the check passes | none | `Ok` of a clone of the value at `index`, and `index` advanced by one |

## `read_fixed_bytes32`, `read_unsigned16`, `read_unsigned32`, `read_unsigned64`, `read_text`, `read_bytes`, `read_fixed_bytes20`, `read_unsigned256` — `&mut self`

`read_fixed_bytes32(&mut self) -> CanonicalFieldReaderReadReturn<[u8; 32]>`, `read_unsigned16 -> CanonicalFieldReaderReadReturn<u16>`, `read_unsigned32 -> CanonicalFieldReaderReadReturn<u32>`, `read_unsigned64 -> CanonicalFieldReaderReadReturn<u64>`, `read_text -> CanonicalFieldReaderReadReturn<String>`, `read_bytes -> CanonicalFieldReaderReadReturn<Vec<u8>>`, `read_fixed_bytes20 -> CanonicalFieldReaderReadReturn<[u8; 20]>`, and `read_unsigned256 -> CanonicalFieldReaderReadReturn<[u8; 32]>`; each shares the three branches below.

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| read refused | `self.read_value()` returns `Err` | none | `self.read_value()` once | its refusal unchanged |
| admitted | `read_value` returns the value and the value is the method's variant | `let CanonicalFieldValue::<Variant>(contents) = value else` | none | `Ok(contents)` |
| another variant | `read_value` returns a value that is not the method's variant, which only a description whose reads disagree with its own `FIELDS` reaches | the `else` of the variant check | none | `Err(CanonicalFieldReaderErrorReturn::Kind { index, expected: fields[index], actual: value.kind() })`, `index` the index the value was read at |

## `read_form<F: ICanonicalField>(&mut self) -> CanonicalFieldReaderReadReturn<F>`

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| past the end | `values.get(index)` is `None` | `let … else` | none | `Err(CanonicalFieldReaderErrorReturn::Count { expected: fields.len(), actual: index + 1 })`, `index` unchanged |
| refused | `F::from_field(FromFieldParams, value)` over a clone of the value at `index` returns `Err(error)` | `from_field`, the field's only kind check | `F::from_field` once | `Err(CanonicalFieldReaderErrorReturn::Form { index, error })`, `error` unchanged, `index` unchanged |
| admitted | `from_field` returns `Ok(FromFieldSuccessReturn { value })` | the check passes | none | `Ok(value)`, and `index` advanced by one |

## Ordering

`try_new` checks the count before any read compares a kind; reads take values in index order, so the first refusal a caller meets is at the lowest index whose value is refused.

## Integration: own entries and callee dispositions

No callee entry and no own entry: the reader's callees are the contract types this node authors and the `from_field` of the form its caller names, and every branch is proven by this node's unit tests.
