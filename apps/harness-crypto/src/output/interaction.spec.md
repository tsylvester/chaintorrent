# `output` — interaction spec

Branch contract for the `output` module of `apps/harness-crypto`: `output` renders the run's outcome as one JSON document — a success as the benchmark's measurement of each configured concrete and, per curve, its label, the files written, its measurement, and its report document, a refusal as the refusal the run returned — and writes the text to each destination it is handed, standard output or a file at its path, in the order handed, returning the text or the first refused write with the destination it concerned. It computes, compares, and selects nothing and renders each value as it was handed. Each branch states condition, decision, dependency call, and the exact return outcome.

## `output`

`output(deps: &OutputDeps, params: OutputParams, payload: OutputPayload) -> OutputReturn`, the trusted form: every input is a typed value from the caller — the fieldless deps, the ordered destinations in `params`, and the outcome the run returned in `payload`; the deps carry nothing and are not read.

### Render

Private `render_outcome(outcome: &RunReturn) -> String`, the first step and a pure function: dependency call none; the document is a `serde_json::Value` built once from the outcome as handed and compared with nothing, and the text is its indented form `format!("{value:#}")`, a single JSON document with no trailing line feed.

### Render, success

Condition the outcome is `Ok(success)`; decision the `Result` match; outcome the object `{ "outcome": "succeeded", "benchmarks": [ … ], "curves": [ … ] }`.

### Render, benchmark

Per entry of `success.benchmarks` in order: outcome the object `{ "concrete": <concrete>, "iterations": <iterations>, "timings": { "mul_g1", "mul_g2", "msm_g1", "msm_g2", "pairing_product": each a duration } }`, `<concrete>` the `Debug` text of the entry's `concrete` as a JSON string and `<iterations>` the JSON number `iterations.get()`.

### Render, duration

Wherever a `Duration` is recorded: outcome the object `{ "seconds": <as_secs>, "subsec_nanos": <subsec_nanos> }`, so no precision is lost and no conversion can fail.

### Render, curve

Per entry of `success.curves` in order: outcome the object `{ "label": <label>, "files": [ … ], "measurement": { … }, "report": <document> }`, `<label>` the entry's `label.as_str()` and `<document>` the entry's `report.document` as one JSON string, verbatim.

### Render, file

Per entry of `files` in order: outcome the object `{ "kind": <kind>, "path": <path> }`; decision the `Option` match on the file's `kind`; `<kind>` the `Debug` text of the kind when it is `Some` and the JSON `null` when it is `None`; `<path>` the text `path.to_string_lossy().into_owned()`.

### Render, measurement

Within a curve: outcome the object `{ "iterations": <iterations>, "timings": { "prove_mint", "prove_transfer", "verify_mint", "verify_transfer", "decapsulate": each a duration }, "sizes_bytes": { "capsule", "envelope", "mint_proof", "transfer_proof": each the size }, "costs": { "mint", "transfer": each { "l2_execution_gas": <gas>, "l1_data_fee": <fee> } } }`, every value taken from the entry's `measurement` as handed.

### Render, fee

Within a measurement: condition `L1DataFee::Charged { wei }`; decision the exhaustive `match` on the fee; outcome `{ "state": "charged", "wei": <wei as decimal text> }`, since the wei is a `u128`; condition `L1DataFee::NotApplicable`; outcome `{ "state": "not_applicable" }`, never a zero.

### Render, refusal

Condition the outcome is `Err(error)`; decision the `Result` match; outcome the object `{ "outcome": "refused", "refusal": <refusal> }` with `<refusal>` the `Debug` text of `error`, whole, as a JSON string.

### Destination file

Per destination of `params.destinations` in order: condition the destination is `OutputDestination::File { path }`; decision the variant match; dependency call `std::fs::write(&path, text.as_bytes())`, which replaces the content of a file at the path and creates no directory; outcome a failure returns `Err(OutputErrorReturn::WriteFile { path, error })`, the error unchanged, before any later destination is written.

### Destination standard output

Per destination in order: condition the destination is `OutputDestination::StandardOutput`; decision the variant match; dependency call `std::io::stdout().write_all(text.as_bytes())`; outcome a failure returns `Err(OutputErrorReturn::WriteStandardOutput(error))`, the error unchanged, before any later destination is written.

### No destination

Condition `params.destinations` is empty; decision the loop's bound; dependency call none; outcome the text is returned and nothing is written.

### Unreached branches

- Refused standard output: no input and no failure mode of the official mocks makes standard output refuse a write in a test process, so `Err(OutputErrorReturn::WriteStandardOutput(..))` has no unit block.

### Computed

`Ok(OutputSuccessReturn { text })`, the text every destination received.

### Ordering

The text is rendered once; the destinations are written in the order the params carry them; the first failure returns and later destinations are not written; a refusal the run returned is rendered and written, not returned; no side effect beyond the writes; the same payload always renders the same text.

## Integration: callee dispositions

The entries of `harness-crypto/run` and `harness-crypto/config/admit_output` that this node's route reaches: each is carried — the property survives this node's boundary — restated where this node transforms it, with the chain of real functions it passes through ending at this node and the outer-edge collaborators that chain reaches.

- `the_run_returns_a_measurement_per_benchmark_concrete_and_an_outcome_per_curve_in_configuration_order` of `harness-crypto/run`: carried; restated where this node transforms it — the document's `benchmarks` array holds an object for each configured benchmark concrete in configuration order, each naming its concrete, and its `curves` array holds an object for each curve in curve order, each holding its own label, its own files, and its own measurement.
  - Route: `create_random_source`, `create_pairing`, the selected pairing concrete's `try_new`, `PairingBenchmark::try_new`, `PairingBenchmark::consume_pairing`, `PairingConsumer::try_new`, `consume_pairing`, `report`, `run`, then `output`, ending at `output`.
  - Outer-edge collaborators: the filesystem, a directory beneath the operating system's temporary directory, the operating system's generator and monotonic clock, all real, and the verifier family's mock concrete.
  - Proven by the block of the same name of the public integration test element of this node.
- `each_curve_is_reported_under_its_own_label_with_the_configured_defaults` of `harness-crypto/run`: carried; restated where this node transforms it — the `report` member of each curve's object holds that curve's report document verbatim as one string, the document the file written to the configured report destination holds; route as the entry above states, ending at `output`; outer-edge collaborators as the entry above states; proven by the block of the same name of the public integration test element.
- `the_first_refusal_is_returned_in_the_stated_order` of `harness-crypto/run`: carried; restated where this node transforms it — the refusal the run returned, whole inside the variant of the stage that refused, is the `refusal` member's `Debug` text, a benchmark stage's refusal rendered in place of a curve consumer's when both would refuse; route `create_random_source`, `create_pairing`, the pairing family's mock concrete, `PairingBenchmark::consume_pairing`, `run`, then `output`, ending at `output`; outer-edge collaborators the filesystem, a directory beneath the operating system's temporary directory, the operating system's generator and monotonic clock, all real, the pairing family's mock concrete and the verifier family's mock concrete; proven by the block of the same name of the public integration test element.
- `every_declared_destination_kind_is_admitted_to_its_own_variant` of `harness-crypto/config/admit_output`: carried; restated where this node transforms it — a destination admitted for each member of `DESTINATION_KINDS` is a destination the text reaches, a file holding the path its entry carried and standard output the process's standard output; route `admit_output`, then `output`, ending at `output`; outer-edge collaborators the filesystem, a directory beneath the operating system's temporary directory, real; proven by the block of the same name of the public integration test element.
- `a_destination_list_keeps_the_order_the_document_holds` of `harness-crypto/config/admit_output`: carried; restated where this node transforms it — the destinations are written in the order the admitted list holds them, observed by a refused destination stopping the writes after it and leaving the writes before it; route and outer-edge collaborators as the entry above states; proven by the block of the same name of the public integration test element.

The other entries of `harness-crypto/run` and `harness-crypto/config/admit_output` end at those nodes' tests and are not observed through `output`'s text or writes, which hold the rendered outcome and no deployed verifier, selection, or document table of their own; none of them is dispositioned here.

## Integration: own entries

- `a_successful_runs_outcome_is_written_with_every_value_it_holds`: condition `output` over the `RunReturn` of the real `run` over the configuration the own entry `the_run_returns_a_measurement_per_benchmark_concrete_and_an_outcome_per_curve_in_configuration_order` of `harness-crypto/run` arranges, and destinations that are files beneath a directory under the operating system's temporary directory, every collaborator real and the verifier family's mock concrete under `NoFailure`; outcome `Ok` whose text parsed with `serde_json::from_str::<Value>` equals the `json!` literal built in Arrange from the outcome's values field by field, and each file read back equal to the returned text.
  - Variation: one file destination and two, over the benchmark concretes in a configured order and in the reverse order, so a render that sorts, drops a benchmark, or reads one curve's file, measurement, or report document under another curve's label fails.
  - Edge: a curve's report document altered, a file or measurement omitted, a timing or the iteration count read from another field, or a destination left unwritten differs from the literal built in Arrange.
- `a_refused_runs_outcome_is_written_as_its_refusal`: condition `output` over the `RunReturn` of the real `run` over a configuration whose benchmark concrete is `PairingConcrete::Mock(MockIPairingAdapterFailureMode::SampleScalarWrongLength)`, and file destinations as the entry above states, every collaborator real; outcome `Ok` whose text parsed equals `json!({ "outcome": "refused", "refusal": <text> })` with `<text>` the `Debug` text of the refusal the run returned, taken in Arrange before the outcome is handed, and each file read back equal to the returned text; variation one file destination and two; edge a document holding the success members, a refusal summarized or converted, or an `Err` returned for the run's refusal differs.

## Integration: private surface

The chain of real functions `admit_output`, `create_random_source`, `create_pairing`, the selected pairing concretes' `try_new`, `PairingBenchmark::try_new`, `PairingBenchmark::consume_pairing`, `PairingConsumer::try_new`, `consume_pairing`, the consumers and factories beneath it that `harness-crypto/run`'s route lists, `report`, `run`, `render_outcome`, and `output`, and the adapter methods they call; the outer-edge collaborators are the filesystem, a directory beneath the operating system's temporary directory removed before each act, the operating system's generator and monotonic clock running real, the verifier family's mock concrete under `NoFailure`, and, for the refusal, the pairing family's mock concrete; the observable result is the `OutputReturn` and the files read back; the entries proven are `a_successful_runs_outcome_is_written_with_every_value_it_holds` and `a_refused_runs_outcome_is_written_as_its_refusal`.

## Integration: public surface

An outside caller invokes `output` through the crate root with an `OutputDeps`, an `OutputParams` holding the destinations `admit_output` admitted, and an `OutputPayload` holding the `RunReturn` that `run` returned through the crate root, and observes the `OutputReturn`, the text, and the files written; the entries proven are every own entry above and every carried entry the dispositions list; the outer-edge collaborators are the filesystem, the operating system's generator and monotonic clock, and the families' mock concretes selected by configuration where an entry names a failure mode.
