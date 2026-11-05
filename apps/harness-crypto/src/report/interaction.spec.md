# `report` — interaction spec

Branch contract for the `report` module of `apps/harness-crypto`: `report` renders the release-evidence document of one curve's delivery measurement — a JSON object recording the measurement `measure` returned, the piece-group size and primary curve defaults in force, any adjustment the data supports, and the conditions the measurement was taken under — and writes it, as the same text, to each destination its params carry in the order the params carry them, selecting nothing against a threshold. It composes no measurement and decides no default. Each branch states condition, decision, dependency call, and the exact return outcome.

## `ReportText::try_new`

`ReportText::try_new(params: ReportTextConstructorParams) -> ReportTextTryNewReturn`, the constructor of the admitted text of a curve's label or of the conditions.

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| empty text | the text has no character other than whitespace | the emptiness check, `params.text.trim().is_empty()` | none | `Err(ReportTextTryNewErrorReturn::Empty)` |
| admitted text | the text has a non-whitespace character | none further | none | `Ok(ReportText { text })`, moved from the params |

## `ReportText::as_str`

`ReportText::as_str(&self) -> &str`: one branch; outcome the held text.

## `report`

`report(deps: &ReportDeps, params: ReportParams, payload: ReportPayload) -> ReportReturn`, the trusted form: every input is a typed value from the caller — the fieldless deps, the ordered destinations in `params`, and the measurement, the curve's label, the conditions, the defaults in force, and the adjustment in `payload`; the function computes, compares, and selects nothing and records each value as it was handed.

### Render

The first step: dependency call none; the document is the indented text of a `serde_json::Value` object of the shape

```text
{
    "curve": <curve text>,
    "conditions": <conditions text>,
    "iterations": <iterations>,
    "timings": {
        "prove_mint": { "seconds": <as_secs>, "subsec_nanos": <subsec_nanos> },
        "prove_transfer": { "seconds": <as_secs>, "subsec_nanos": <subsec_nanos> },
        "verify_mint": { "seconds": <as_secs>, "subsec_nanos": <subsec_nanos> },
        "verify_transfer": { "seconds": <as_secs>, "subsec_nanos": <subsec_nanos> },
        "decapsulate": { "seconds": <as_secs>, "subsec_nanos": <subsec_nanos> }
    },
    "sizes_bytes": {
        "capsule": <size>,
        "envelope": <size>,
        "mint_proof": <size>,
        "transfer_proof": <size>
    },
    "costs": {
        "mint": { "l2_execution_gas": <gas>, "l1_data_fee": <fee> },
        "transfer": { "l2_execution_gas": <gas>, "l1_data_fee": <fee> }
    },
    "defaults_in_force": {
        "piece_group_size_bytes": <size>,
        "primary_curve": <label>
    },
    "adjustment": <adjustment>
}
```

every value taken from the payload as it was handed and compared with nothing.

### Fee

Within render: condition `L1DataFee::Charged { wei }`; decision the exhaustive `match` on the fee; outcome `{ "state": "charged", "wei": <wei as decimal text> }`; condition `L1DataFee::NotApplicable`; outcome `{ "state": "not_applicable" }`.

### Adjustment

Within render: condition `payload.adjustment` is `None`; decision the `Option` match; outcome the JSON `null`; condition it is `Some`; outcome `{ "piece_group_size_bytes": <size or null>, "primary_curve": <label or null> }`, each field `null` where the adjustment holds `None`.

### Destination file

Per destination in order: condition the destination is `ReportDestination::File { path }`; decision the variant match; dependency call `std::fs::write(&path, document.as_bytes())`; outcome a failure returns `Err(ReportErrorReturn::WriteFile { path, error })`, the error unchanged, before any later destination is written.

### Destination standard output

Per destination in order: condition the destination is `ReportDestination::StandardOutput`; decision the variant match; dependency call `std::io::stdout().write_all(document.as_bytes())`; outcome a failure returns `Err(ReportErrorReturn::WriteStandardOutput(error))`, the error unchanged, before any later destination is written.

### No destination

Condition `params.destinations` is empty; decision the loop's bound; dependency call none; outcome the document is returned and nothing is written.

### Unreached branches

- Refused standard output: no input and no failure mode of the official mocks makes standard output refuse a write in a test process, so `Err(ReportErrorReturn::WriteStandardOutput(..))` has no unit block.

### Computed

`Ok(ReportSuccessReturn { document })`, the document the text every destination received.

### Ordering

The document is rendered once; the destinations are written in the order the params carry them; the first failure returns; no side effect beyond the writes; the same payload always renders the same document.

## Integration: callee dispositions

The entries of `harness-crypto/measure` that this node's route reaches: each is carried — the property survives this node's boundary — restated where this node transforms it, with the chain of real functions it passes through ending at this node and the outer-edge collaborators that chain reaches.

- `every_operation_is_timed_on_every_concrete`: carried; restated where this node transforms it — each of the five timings the document records is greater than zero seconds or greater than zero subsecond nanoseconds; route `create_random_source`, `create_pairing`, the arm's concrete through its `try_new`, `create_hash_to_scalar`, `create_encoding`, `create_chain_forms`, `create_kem`, `create_key_agreement`, `create_delivery_proof`, `create_verifier`, `measure`, `report`; outer-edge collaborators the operating system's generator and monotonic clock running real, the verifier family's mock concrete, and the filesystem.
- `the_measurement_carries_the_iteration_count_its_means_were_computed_from`: carried; restated where this node transforms it — the document's `iterations` equals the count the measurement was taken with; route and outer-edge collaborators as the entry above.
- `every_object_is_sized_by_the_encodings_of_its_elements`: carried; restated where this node transforms it — the document's `sizes_bytes` equal the encoded lengths of the object's elements, each computed in Arrange through the received pairing's encoders over its generators and a scalar its sampling bound samples; route and outer-edge collaborators as the first entry above.
- `every_accepted_delivery_reports_the_cost_the_verifier_returned`: carried; restated where this node transforms it — the document's `costs` equal the cost the verifier family's mock concrete returns for each submission, its gas `100_000` and its fee charged at the wei `"1000"`; route and outer-edge collaborators as the first entry above.

## Integration: own entries

- `the_document_records_every_value_a_measurement_holds`: condition `report` over the measurement the real `measure` returned for a case with an iteration count of one, every adapter real, the operating-system source real, and the verifier family's mock concrete under `NoFailure`, and one file destination; outcome `Ok`, the file read back equal to the returned document, and the document parsed to a JSON value whose `iterations`, five `timings`, four `sizes_bytes`, and two `costs` equal the measurement's values, each timing as the whole seconds and subsecond nanoseconds of its `Duration`; variation every member of `PAIRING_CONCRETES` crossed with the selections the other families declare and with the verifier form each pairing declares, so both curves and both verifier forms run; edge a timing or a size read from another field, a cost read from the other relation, a fee recorded as a zero, or a field omitted, differs from the values the measurement holds.

## Integration: private surface

The chain of real functions `create_random_source` for the operating-system kind, `create_pairing`, the arm's concrete through its `try_new`, `create_hash_to_scalar`, `create_encoding`, `create_chain_forms`, `create_kem`, `create_key_agreement`, `create_delivery_proof`, and `create_verifier`, each selected concrete through its `try_new`, `measure`, `mint`, `transfer`, every adapter method those functions call, `report`, and the `Secret` of `domain`, all running for real; the cases are those `harness-crypto/measure`'s private surface lists, the verifier the family's mock concrete under `NoFailure`; the outer-edge collaborators are the operating system's generator and monotonic clock running real, the verifier family's mock concrete, and the filesystem, a directory beneath the operating system's temporary directory removed before each act; the observable result is the `ReportReturn`, the file read back, and the document parsed; the entry proven is `the_document_records_every_value_a_measurement_holds`.

## Integration: public surface

An outside caller invokes `report` through the crate root with a `ReportDeps`, a `ReportParams` holding a file destination, and a `ReportPayload` over the measurement `measure` returned through the crate root, a curve label, conditions, defaults, and an adjustment, and observes the `ReportReturn`, the document, and the file written, with the official mocks selectable through each family's params under the `mocks` feature; the entries proven are `the_document_records_every_value_a_measurement_holds` and the carried `measure` entries `every_operation_is_timed_on_every_concrete`, `the_measurement_carries_the_iteration_count_its_means_were_computed_from`, `every_object_is_sized_by_the_encodings_of_its_elements`, and `every_accepted_delivery_reports_the_cost_the_verifier_returned`; the outer-edge collaborators are the operating system's generator and monotonic clock behind `create_random_source`, running real, the verifier family's mock concrete selected by configuration, and the filesystem.
