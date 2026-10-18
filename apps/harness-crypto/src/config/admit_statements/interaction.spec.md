# `admit_statements` — interaction spec

Branch contract for the `admit_statements` module of the `harness-crypto` crate: `admit_statements`, the function the configuration document's network profile and statements are admitted by — the chain identifier and the entitlement contract as canonical field values, each a kind and a value the document carries; the mint and transfer contexts, their suite identifier, asset identity hash, and parameter-set identifier through the domain constructors, their entitlements, intervals, seller, and buyer as canonical field values, their expiry, and their chain identifier and entitlement contract the network profile's; the mint and transfer purposes through their declared codes; and the identity bytes and the proof identity as `0x`-prefixed hex — every refusal carried with the field it was read from. Each branch states condition, decision, dependency call, and the exact return outcome.

## `admit_statements`

`admit_statements(deps: &AdmitStatementsDeps, params: AdmitStatementsParams, payload: AdmitStatementsPayload<'_>) -> AdmitStatementsReturn`: the payload is the document the deserializer proved the shape of, and this function proves its content; `deps` and `params` carry nothing and are not read.

### Hex

One local closure over a text and an optional width in bytes, returning the bytes or a `HexTextErrorReturn`.

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| missing prefix | the text does not begin with `0x` | the prefix check | none | `Err(HexTextErrorReturn::MissingPrefix)` |
| invalid digit | the lowest index `i` of the text past the prefix whose byte is outside `b'0'..=b'9'`, `b'a'..=b'f'`, and `b'A'..=b'F'` | the digit scan | none | `Err(HexTextErrorReturn::InvalidDigit { index: i, byte })`, `i` an index into the whole text |
| odd digit count | the digit count past the prefix is odd | the count check | none | `Err(HexTextErrorReturn::OddDigitCount { count })`, `count` the digits after the prefix |
| wrong width | a width is given and the byte count is not it | the width check | none | `Err(HexTextErrorReturn::WrongLength { expected, actual })`, each length in bytes |
| decoded | none of the above | the decode, each byte from two digits, high digit first | none | the bytes |

### Canonical field

One local closure over a `CanonicalFieldDocument`, returning a `CanonicalFieldValue` or a `CanonicalFieldAdmissionErrorReturn`, which every caller carries as `CanonicalField { field, error }`.

The kind is decided first, a `match` on `kind` as a `&str`; any other text is refused with outcome `Err(CanonicalFieldAdmissionErrorReturn::UnknownKind { text })`:

| Text | `CanonicalFieldKind` |
|---|---|
| `"fixed_bytes32"` | `FixedBytes32` |
| `"unsigned16"` | `Unsigned16` |
| `"unsigned32"` | `Unsigned32` |
| `"unsigned64"` | `Unsigned64` |
| `"text"` | `Text` |
| `"bytes"` | `Bytes` |
| `"fixed_bytes20"` | `FixedBytes20` |
| `"unsigned256"` | `Unsigned256` |

Then a `match` on the kind and the value:

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| unsigned integer | `Unsigned16`, `Unsigned32`, or `Unsigned64` with `Integer(value)` | the value's conversion into `u16`, `u32`, or `u64` | none | `CanonicalFieldValue` of that kind and the converted value; a failed conversion `Err(CanonicalFieldAdmissionErrorReturn::OutOfRange { kind, value })` |
| text for an unsigned kind | `Unsigned16`, `Unsigned32`, or `Unsigned64` with `Text` | the value's kind | none | `Err(CanonicalFieldAdmissionErrorReturn::ExpectedInteger { kind })` |
| 256-bit integer | `Unsigned256` with `Integer(value)` | the integer's big-endian encoding | none | `CanonicalFieldValue::Unsigned256` of the 32 bytes whose last eight are `value.to_be_bytes()` and the rest zero |
| 256-bit hex | `Unsigned256` with `Text` | the hex closure over the text at width 32 | the hex closure | `CanonicalFieldValue::Unsigned256` of the decoded bytes; a refusal `Err(CanonicalFieldAdmissionErrorReturn::Hex(error))` |
| fixed-width hex | `FixedBytes32` or `FixedBytes20` with `Text` | the hex closure at width 32 and 20 | the hex closure | `CanonicalFieldValue` of that kind and the decoded bytes; a refusal `Err(CanonicalFieldAdmissionErrorReturn::Hex(error))` |
| byte string | `Bytes` with `Text` | the hex closure at no width | the hex closure | `CanonicalFieldValue::Bytes` of the decoded bytes; a refusal `Err(CanonicalFieldAdmissionErrorReturn::Hex(error))` |
| text | `Text` with `Text(text)` | the value's kind | none | `CanonicalFieldValue::Text(text)`, the text as it is |
| integer for a text kind | `FixedBytes32`, `FixedBytes20`, `Bytes`, or `Text` with `Integer` | the value's kind | none | `Err(CanonicalFieldAdmissionErrorReturn::ExpectedText { kind })` |

### Network

The canonical field closure over `network.chain`, field `NetworkChain`, then over `network.entitlement_contract`, field `NetworkEntitlementContract`; the admitted pair is `NetworkProfile { chain, entitlement_contract }`.

### Contexts

The mint context with `relation` `DeliveryRelation::Mint` and then the transfer context with `relation` `DeliveryRelation::Transfer`, each field in this order:

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| suite identifier hex | `context.suite_identifier` | the hex closure at width 32 | the hex closure | the 32 identifier bytes; a refusal `Err(AdmitStatementsErrorReturn::Hex { field: StatementField::Context { relation, field: ContextField::SuiteIdentifier }, error })` |
| suite identifier | the decoded bytes and `context.suite_version` | the constructor | `SuiteIdentifier::try_new(SuiteIdentifierConstructorParams { identifier, version: suite_version })` | the `SuiteIdentifier`; a refusal `Err(AdmitStatementsErrorReturn::SuiteIdentifier { relation, error })` |
| asset identity hash | `context.asset_identity_hash` | the hex closure at width 32, then the constructor | the hex closure, `AssetIdentityHash::try_new(AssetIdentityHashConstructorParams { bytes })` | the `AssetIdentityHash`; a hex refusal `Err(AdmitStatementsErrorReturn::Hex { field: StatementField::Context { relation, field: ContextField::AssetIdentityHash }, error })`, a constructor refusal `Err(AdmitStatementsErrorReturn::AssetIdentityHash { relation, error })` |
| parameter-set identifier | `context.parameter_set_identifier` | the hex closure at width 32, then the constructor | the hex closure, `ParameterSetIdentifier::try_new(ParameterSetIdentifierConstructorParams { bytes })` | the `ParameterSetIdentifier`; a hex refusal `Err(AdmitStatementsErrorReturn::Hex { field: StatementField::Context { relation, field: ContextField::ParameterSetIdentifier }, error })`, a constructor refusal `Err(AdmitStatementsErrorReturn::ParameterSetIdentifier { relation, error })` |
| canonical context fields | `context.source_entitlement`, `context.target_entitlement`, `context.old_interval`, `context.new_interval`, `context.seller`, `context.buyer` | the canonical field closure over each in that order | the canonical field closure | each a `CanonicalFieldValue`; a refusal `Err(AdmitStatementsErrorReturn::CanonicalField { field: StatementField::Context { relation, field }, error })`, `field` the `ContextField` of its name |
| admitted context | every context field admitted | none | none | `CanonicalDeliveryContext { suite_identifier, chain, entitlement_contract, asset_identity_hash, parameter_set_identifier, source_entitlement, target_entitlement, old_interval, new_interval, seller, buyer, expiry }`, `chain` and `entitlement_contract` clones of the network profile's and `expiry` the document's |

Each 32-byte width is converted to its constructor's array by `<[u8; 32]>::try_from`, which the width check guarantees, so no input takes its failure, which returns the width refusal.

### Purposes

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| mint purpose | `statements.mint_purpose` | the purpose's own conversion | `MintPurpose::try_from(statements.mint_purpose)` | the `MintPurpose`; a refusal `Err(AdmitStatementsErrorReturn::MintPurpose(error))` |
| transfer purpose | `statements.transfer_purpose` | the purpose's own conversion | `TransferPurpose::try_from(statements.transfer_purpose)` | the `TransferPurpose`; a refusal `Err(AdmitStatementsErrorReturn::TransferPurpose(error))` |

### Identities

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| identities | each `(index, text)` of `statements.identities` in order | the hex closure at no width | the hex closure | the decoded bytes, collected in order; a refusal `Err(AdmitStatementsErrorReturn::Hex { field: StatementField::Identity { index }, error })` |
| proof identity | `statements.proof_identity` | the hex closure at no width | the hex closure | the decoded bytes; a refusal `Err(AdmitStatementsErrorReturn::Hex { field: StatementField::ProofIdentity, error })` |

### Computed

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| admitted | every field admitted | none | none | `Ok(AdmitStatementsSuccessReturn { network, statements: HarnessStatements { mint, transfer, mint_purpose, transfer_purpose, identities, proof_identity } })` |

### Ordering

The network's chain and contract, the mint context, the transfer context, the purposes, the identities, then the proof identity; within a context its fields in the order above; the first refusal returns; no side effect.
