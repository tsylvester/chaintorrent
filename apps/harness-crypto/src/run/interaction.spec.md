# `run` — interaction spec

Branch contract for the `run` module of `apps/harness-crypto`: `run` is the harness's one composition root — obtains the randomness source through `create_random_source`; measures each configured benchmark concrete through `create_pairing` with a `PairingBenchmark` over a source of its own; composes each configured curve through `create_pairing` with a `PairingConsumer` constructed at that curve's verifier concrete; records each curve's measurement through `report`; and returns the benchmark's measurement per concrete and, per curve, the files written, the measurement, and the report, or the first refusal unchanged.
It names no concrete, curve, kind, identifier, endpoint, address, path, count, or version; reads no file, environment variable, or clock; admits no selection, context, or identity; and every value it calls with is read from the configuration it is handed or from a callee's return.
Each branch states condition, decision, dependency call, and the exact return outcome.

## `run`

`run(deps: &RunDeps, params: RunParams, payload: RunPayload) -> RunReturn`, the trusted form; the deps and params carry nothing and are not read; every value the function calls with is read from `payload.configuration`.

### Source refused

Dependency call `create_random_source(&CreateRandomSourceDeps, CreateRandomSourceParams { kind: configuration.selections.random_source }, CreateRandomSourcePayload)`, once, the returned adapter held for the curves; condition it returns `Err(error)`; outcome `Err(RunErrorReturn::CreateRandomSource(error))`, `error` unchanged, and nothing after it runs.

### Benchmark source refused

Per entry of `configuration.selections.benchmark_concretes` in order: dependency call `create_random_source` with the same params, once for the entry, the adapter owned by the benchmark; condition it returns `Err(error)`; outcome `Err(RunErrorReturn::CreateRandomSource(error))`, `error` unchanged.

### Benchmark pairing refused

Dependency call `create_pairing(&CreatePairingDeps { consumer }, CreatePairingParams { concrete: pairing.concrete, supported_encodings: configuration.selections.precompile_encodings.clone(), target_group_encoding: pairing.target_group_encoding }, CreatePairingPayload)`, once for the entry, `consumer` the `PairingBenchmark` built by
`PairingBenchmark::try_new(PairingBenchmarkConstructorParams { random: the entry's adapter, iterations: configuration.benchmark_iterations })` through the irrefutable pattern `let Ok(consumer) = …;`;
condition the call returns `Err(error)`; outcome `Err(RunErrorReturn::CreatePairing(error))`, `error` unchanged.

### Benchmark refused

Condition the call returns `Ok(CreatePairingSuccessReturn { output })` and `output` is `Err(error)`; outcome `Err(RunErrorReturn::Benchmark(error))`, `error` unchanged.

### Benchmark measured

Condition `output` is `Ok(measurement)`; outcome the measurement is appended to the benchmarks in order and the next entry runs.

### Curve pairing refused

Per curve in order, each entry of `configuration.selections.curves` zipped with the entry of `configuration.verifiers` and the entry of `configuration.curve_labels` at its index:
dependency call `create_pairing(&CreatePairingDeps { consumer }, CreatePairingParams { concrete: curve.pairing.concrete, supported_encodings: configuration.selections.precompile_encodings.clone(), target_group_encoding: curve.pairing.target_group_encoding }, CreatePairingPayload)`, once for the curve, `consumer` the `PairingConsumer` built by
`PairingConsumer::try_new(PairingConsumerConstructorParams { random: the adapter borrowed as the trait object, statements: &configuration.statements, identities: &configuration.identities, hash_to_scalar: configuration.selections.hash_to_scalar, encoding: configuration.selections.encoding, chain_forms: configuration.selections.chain_forms, kem: configuration.selections.kem, key_agreement: configuration.selections.key_agreement, delivery_proof: configuration.selections.delivery_proof.concrete, generate: configuration.generate.concrete.clone(), kinds: configuration.generate.kinds.clone(), verifier: the curve's verifier concrete cloned, statement_version: configuration.selections.delivery_proof.statement_version, iterations: configuration.measurement.iterations })` through the irrefutable pattern `let Ok(consumer) = …;`;
condition the call returns `Err(error)`; outcome `Err(RunErrorReturn::CreatePairing(error))`, `error` unchanged.

### Curve consumer refused

Condition the call returns `Ok(CreatePairingSuccessReturn { output })` and `output` is `Err(error)`; outcome `Err(RunErrorReturn::PairingConsumer(error))`, `error` unchanged.

### Curve reported

Condition `output` is `Ok(success)`:
dependency call `report(&ReportDeps, ReportParams { destinations: configuration.output.report_destinations.clone() }, ReportPayload { measurement: success.measurement, curve: the curve's label cloned, conditions: configuration.measurement.conditions.clone(), defaults: configuration.report.defaults.clone(), adjustment: configuration.report.adjustment.clone() })`, once for the curve;
condition it returns `Err(error)`; outcome `Err(RunErrorReturn::Report(error))`, `error` unchanged.

### Curve outcome

Condition the report returns `Ok(report)`; outcome `RunCurveOutcome { label, files: success.files, measurement: success.measurement, report }` is appended to the curves in order and the next curve runs.

### Computed

Outcome `Ok(RunSuccessReturn { benchmarks, curves })`, each value the callee's own, moved or cloned and altered by nothing.

### Ordering

The source for the curves is obtained first; the benchmark entries run in configuration order, each after the previous entry's measurement; then the curves run in curve order, each curve's report written before the next curve's pairing is created; the first refusal returns and nothing after it runs; the verifier concrete of curve `i` is the entry of `verifiers` at `i` and no other, and the label of curve `i` is the entry of `curve_labels` at `i`.

### Unreached, refused source

Every constructor error of the random family's concretes is uninhabited, so `Err(RunErrorReturn::CreateRandomSource(..))` has no unit block.

## Integration: own entries

- `the_run_returns_a_measurement_per_benchmark_concrete_and_an_outcome_per_curve_in_configuration_order`: condition `run` over a configuration whose `selections` name the real concretes of every family, the real members of `PAIRING_CONCRETES` as `benchmark_concretes` and two curves of distinct pairing concretes, whose `verifiers` are `VerifierConcrete::Mock(MockIVerifierAdapterFailureMode::NoFailure)`, whose `generate` is a real concrete rooted beneath a directory under the operating system's temporary directory, and whose `output.report_destinations` is a file beneath that directory per run;
  outcome `Ok` whose `benchmarks` hold the measurement of each configured benchmark concrete in configuration order, each `concrete` equal to its entry, and whose `curves` hold, in curve order, each curve's label, the files its consumer wrote, a measurement whose iteration count is the configured one, and the report;
  variation the benchmark concretes in a configured order and in the reverse order, and two curves whose labels and pairing concretes differ, so a run that sorts, reuses one curve's outcome, or reads another index fails;
  edge a benchmark entry skipped, a curve's label read from another index, or a curve's files or measurement taken from another curve differs from the configuration built in Arrange.
- `each_curve_is_composed_with_the_verifier_concrete_of_its_own_index`: condition `run` over a configuration of two curves whose `verifiers` are `VerifierConcrete::Mock(MockIVerifierAdapterFailureMode::NoFailure)` at one index and `VerifierConcrete::Mock(MockIVerifierAdapterFailureMode::RpcRefused)` at the other, every other family's concrete real or the family's mock concrete as the case names;
  outcome `Ok` when no index holds the refusing concrete, and otherwise `Err(RunErrorReturn::PairingConsumer(error))` whose `error` is the refusal the block `consume_pairing_returns_the_verifiers_refusal_after_the_generate_step_succeeds` of `harness-crypto/run/pairing_consumer` asserts, with the report of every curve before the refusing index written and no report written for the refusing index or after it;
  variation the refusing concrete at the first index and at the second, so a run that reads one verifier for every curve, reads another curve's index, or reports before the consumer returns fails;
  edge a report file present for the refusing curve, or absent for a curve before it, differs.
- `each_curve_is_reported_under_its_own_label_with_the_configured_defaults`: condition `run` over the configuration of the first own entry and a file report destination;
  outcome `Ok`, the file read back equal to each curve's `report.document` as the final report written, and the document parsed to a JSON value whose `curve` equals the curve's label, whose `conditions` equals the configured conditions, whose `iterations` equals the measurement's iteration count, whose `defaults_in_force` equals the configured defaults, and whose `adjustment` equals the configured adjustment or `null`;
  variation an absent adjustment and an adjustment holding a piece-group size alone, over two curves of distinct labels, so a run that reports one curve's label for another, drops the adjustment, or substitutes a default differs;
  edge a label, conditions text, or default read from another field differs from the values the configuration holds.
- `the_first_refusal_is_returned_in_the_stated_order`: condition `run` over a configuration with a refusal in two stages;
  outcome the earlier stage's refusal whole, a benchmark stage's `Err(RunErrorReturn::Benchmark(error))` before a curve consumer's `Err(RunErrorReturn::PairingConsumer(error))`, and a curve consumer's before a report's `Err(RunErrorReturn::Report(error))`;
  variation a benchmark concrete selected as `PairingConcrete::Mock(MockIPairingAdapterFailureMode::SampleScalarWrongLength)` with a verifier refusing at any index, and a verifier refusing at the first curve with a report destination no directory holds;
  edge a stage run before its predecessor, or a later stage's refusal returned in place of the earlier, differs.
- `the_shipped_defaults_reach_no_deployed_verifier`: condition `config` over `SHIPPED_DEFAULTS` with `generate.foundry_root` and `report.destinations` set by `--set` assignments to a directory beneath the operating system's temporary directory, every callee real, then `run` over the configuration it returns;
  outcome `Err(RunErrorReturn::PairingConsumer(error))` whose `error` ends at the verifier family's refusal for the address at which nothing is deployed, the generate step having written the vector files before it;
  variation the shipped defaults alone, whose curve records are the address at which nothing is deployed;
  edge a run that accepts, or a refusal returned from a step before the verifier, differs.
- `a_deployment_record_layer_and_a_flag_each_replace_a_shipped_default`: condition `config` over `SHIPPED_DEFAULTS` and a command line naming `--config ../../contracts/evm/deployments/base-sepolia.toml`, the layer `contracts/evm/deploy` emits, and `--set benchmark.iterations`, `--set measurement.iterations`, `generate.foundry_root`, and `report.destinations` assignments, then `run` over the configuration it returns;
  outcome `Ok` whose every curve's measurement carries its costs as L2 execution gas and L1 data fee, no cost value asserted since a cost is a measured value with no target, whose benchmark measurements carry the `benchmark.iterations` the flag names, and whose curve measurements carry the `measurement.iterations` the flag names;
  variation the layer alone and the layer with each flag, and the layer of a single curve's record, which leaves the other curve at its shipped default and returns that curve's verifier refusal;
  edge a curve reached at the other curve's record, or a flag that reaches the document and not the run, differs;
  runtime inputs, per the dependency map's Runtime sequence: the deployed verifiers and the layer file, neither ordering this node.

## Integration: callee dispositions

- `the_files_and_the_measurement_reach_the_caller_of_create_pairing_whole` of `harness-crypto/run/pairing_consumer`: carried; restated where this node transforms it: the files and the measurement the consumer returns for a curve are the `files` and `measurement` of that curve's `RunCurveOutcome`, no path twice, in the order written;
  route `create_random_source`, `create_pairing`, the selected pairing concrete's `try_new`, `PairingConsumer::try_new`, `consume_pairing`, then the route the pairing consumer's entry states through `create_hash_to_scalar`, `create_encoding`, `create_chain_forms`, `admit_contexts`, `create_kem`, `create_key_agreement`, `create_delivery_proof`, `create_generate`, `emit`, `create_verifier`, `consume_verifier`, and `measure`, then `report`, ending at `run`;
  outer-edge collaborators the filesystem, a directory beneath the operating system's temporary directory, the operating system's generator and monotonic clock, all real, and the verifier family's mock concrete;
  proven by the block of the same name of the public integration test element of this node.
- `the_hash_to_scalar_selection_the_consumer_carries_is_the_concrete_the_factory_constructs`, `the_encoding_selection_the_consumer_carries_is_the_concrete_create_encoding_constructs`, and `the_hash_to_scalar_refusal_precedes_the_encoding_refusal` of `harness-crypto/run/pairing_consumer`: carried; restated where this node transforms them: the selections the configuration holds are the selections the consumer is constructed with, field by field, and a refusal arrives as `Err(RunErrorReturn::PairingConsumer(PairingConsumerErrorReturn::CreateHashToScalar(..)))` or `Err(RunErrorReturn::PairingConsumer(PairingConsumerErrorReturn::CreateEncoding(..)))` with the variant the entry names, for the first curve, no report written;
  route as the entry above states, ending at `run`;
  outer-edge collaborators none;
  proven by the blocks of the same names of the public integration test element of this node.
- `the_files_are_encoded_under_the_encoding_selection_the_consumer_holds` of `harness-crypto/run/pairing_consumer`: carried; restated where this node transforms it: the lines of the file tagged `Some(VectorKind::Challenge)` in a curve's `files` decode under the decoder of the configuration's encoding selection and are refused by the decoder of the other;
  route and outer-edge collaborators as the first carried entry states;
  proven by the block of the same name of the public integration test element.
- `the_chain_forms_selection_the_consumer_carries_is_the_concrete_create_chain_forms_constructs` and `an_encoder_declaring_another_encoding_identifier_is_refused_before_the_chain_forms_consumer_runs` of `harness-crypto/run/pairing_consumer`: carried; restated where this node transforms them: the configuration's chain forms and encoding selections reach the consumer unaltered and the refusal arrives as `Err(RunErrorReturn::PairingConsumer(PairingConsumerErrorReturn::Encoding(EncodingConsumerErrorReturn::CreateChainForms(..))))` with the variant the entry names, no file written and no report written;
  route and outer-edge collaborators as the entry above states;
  proven by the blocks of the same names of the public integration test element.
- `the_contexts_and_the_identities_the_consumer_carries_reach_the_delivery_proof_consumer_as_the_records_the_files_hold` and `a_kinds_configured_count_reaches_the_concrete_as_the_records_its_file_holds` of `harness-crypto/run/pairing_consumer`: carried; restated where this node transforms them: the configuration's statements, identities, and per-kind configuration reach the consumer, so the files in a curve's outcome hold the records the entries state, the line counts the configuration names;
  route and outer-edge collaborators as the first carried entry states;
  proven by the blocks of the same names of the public integration test element.
- `create_generate_refuses_an_encoder_declaring_another_encoding_identifier`, `a_hash_to_scalar_refusal_reaches_the_caller_whole`, and `a_nonce_draw_refusal_reaches_the_caller_whole` of `harness-crypto/run/pairing_consumer`: carried; restated where this node transforms them: each refusal arrives whole inside `Err(RunErrorReturn::PairingConsumer(..))` through the variants the consumer entries state, unconverted, no report written;
  route and outer-edge collaborators as the entries above state;
  proven by the blocks of the same names of the public integration test element.
- `the_consumer_receives_the_concrete_the_params_name`, `a_sampled_scalar_encodes_through_secret`, `a_sampling_refusal_reaches_the_caller_unchanged`, and `every_operation_is_timed_on_every_concrete` of `harness-crypto/benchmark`: carried; restated where this node transforms them: each entry of `benchmarks` has the `concrete` of the configured entry it was run for, the output is `Ok` for every member of `PAIRING_CONCRETES`, each of its five timings is greater than `Duration::ZERO`, and a sampling refusal arrives as `Err(RunErrorReturn::Benchmark(PairingBenchmarkErrorReturn::SampleScalar(error)))`, the error whole;
  route `create_random_source`, `create_pairing`, the entry's concrete through its `try_new`, `PairingBenchmark::try_new`, `PairingBenchmark::consume_pairing`, `OsRandomSource::fill_bytes`, `sample_from_uniform_bytes`, and the concrete's group operations, ending at `run`;
  outer-edge collaborators the operating system's generator and monotonic clock, real, and, for the refusal, the pairing family's mock concrete;
  proven by the blocks of the same names of the public integration test element.
- `the_document_records_every_value_a_measurement_holds`, `every_operation_is_timed_on_every_concrete`, `the_measurement_carries_the_iteration_count_its_means_were_computed_from`, `every_object_is_sized_by_the_encodings_of_its_elements`, and `every_accepted_delivery_reports_the_cost_the_verifier_returned` of `harness-crypto/report`: carried; restated where this node transforms them: each curve's `report.document` and the file written to the configured report destination record the measurement that curve's consumer returned, its timings, iteration count, sizes, and costs, under that curve's label;
  route the pairing consumer entry's route above followed by `report`, ending at `run`;
  outer-edge collaborators as the first carried entry states;
  proven by `each_curve_is_reported_under_its_own_label_with_the_configured_defaults` and the blocks of the same names of the public integration test element.
- `the_verifier_is_selected_with_the_settings_the_document_holds`, `a_file_and_a_flag_each_replace_a_shipped_default`, and `the_shipped_defaults_configure_the_harness` of `harness-crypto/config`: carried; restated where this node transforms them: the verifier concrete at curve `i` reaches the consumer of curve `i`, a layer's record replaces one curve's default alone, and a flag replaces the default the run then runs with;
  route `config` over the shipped defaults, then `run`;
  outer-edge collaborators the filesystem, a real directory, and the deployed verifiers;
  proven by `each_curve_is_composed_with_the_verifier_concrete_of_its_own_index`, `a_deployment_record_layer_and_a_flag_each_replace_a_shipped_default`, and `the_shipped_defaults_reach_no_deployed_verifier` in the public integration test element.
- `the_consumer_receives_the_concrete_the_params_name` of `pairing/factory`: carried through the entry of `harness-crypto/benchmark` and the entry of `harness-crypto/run/pairing_consumer` dispositioned above, whose routes and blocks state it.
- The other entries of `pairing/factory`, the entries of `harness-crypto/config` that its own dispositions placed in `harness-crypto/config/layers`, `harness-crypto/config/admit_selections`, `harness-crypto/config/admit_statements`, `harness-crypto/config/admit_generate`, and `harness-crypto/config/admit_output`, and the entries of `harness-crypto/run/pairing_consumer` that its own dispositions placed in the consumers and factories beneath it end at those nodes' tests and are not observed through `run`'s outcome, which holds measurements, labels, files, and reports and no pairing, scalar, encoded value, or admitted selection of its own; none of them is dispositioned here.

## Integration: private surface

The chain of real functions `run`, `create_random_source`, `create_pairing`, the selected pairing concretes' `try_new`, `PairingBenchmark::try_new`, `PairingBenchmark::consume_pairing`, `PairingConsumer::try_new`, `consume_pairing`, the consumers and factories beneath it that the pairing consumer entry's route lists, `report`, and the adapter methods they call;
the cases are the configurations the own entries name over the real members of `PAIRING_CONCRETES`, the real members of every other family's declared set, and the pairing family's mock concrete where an entry names its failure mode;
the outer-edge collaborators are the filesystem, a directory beneath the operating system's temporary directory removed before each act, the operating system's generator and monotonic clock running real, the verifier family's mock concrete under the failure mode an entry names, and the randomness family's mock concrete where an entry names it;
the observable result is the `RunReturn`, the report files read back, and the vector files named in the curves' outcomes; the entries proven are `the_run_returns_a_measurement_per_benchmark_concrete_and_an_outcome_per_curve_in_configuration_order`, `each_curve_is_composed_with_the_verifier_concrete_of_its_own_index`, `each_curve_is_reported_under_its_own_label_with_the_configured_defaults`, `the_first_refusal_is_returned_in_the_stated_order`, and the carried entries that name the verifier family's mock concrete or the pairing family's mock concrete.

## Integration: public surface

An outside caller invokes `run` through the crate root with a `RunDeps`, a `RunParams`, and a `RunPayload` holding the configuration `config` returned through the crate root, and observes the `RunReturn`, the report files, and the vector files;
the entries proven are every own entry above and every carried entry the dispositions list;
the outer-edge collaborators are the filesystem, the operating system's generator and monotonic clock, the families' mock concretes selected by configuration where an entry names a failure mode, and, for the two deployed-verifier entries, the deployed verifiers the layer names.
