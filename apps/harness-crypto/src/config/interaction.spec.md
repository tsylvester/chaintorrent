# `config` — interaction spec

Branch contract for the `config` module of `apps/harness-crypto`: `config` is the harness's composing function — it hands the command line and the shipped defaults text to `layers`, the document's tables to `admit_selections`, to `admit_statements` under the identity scope `admit_selections` admitted, to `admit_generate`, and to `admit_output`; it admits the benchmark's and the measurement's iteration counts, the report's defaults and adjustment, the curve labels, and the verifier's endpoint itself; it builds one verifier concrete per configured curve from the endpoint, that curve's deployment record, and the gas price oracle; and it returns the one typed configuration every factory call and every write in the harness takes, or the first refusal. Each branch states condition, decision, dependency call, and the exact return outcome.

## `config`

`config(deps: &ConfigDeps, params: ConfigParams, payload: ConfigPayload) -> ConfigReturn`, the trusted form: the command line arrives as handed arguments in `params`, the program name first; the defaults text arrives as handed text in `payload`; the function reads no file, no environment variable, and no clock itself — `layers` alone reads the files the arguments name; `deps` holds the five functions it calls.

### Layers

Dependency call `(deps.layers)(&LayersDeps, LayersParams { arguments: params.arguments }, LayersPayload { defaults: payload.defaults })`; a refusal returns `Err(ConfigErrorReturn::Layers(error))` unchanged; success continues with the `ConfigDocument` the call returns.

### Selections

Dependency call `(deps.admit_selections)(&AdmitSelectionsDeps, AdmitSelectionsParams, AdmitSelectionsPayload { selections: document.selections.clone(), precompile_encodings: document.network_profile.precompile_encodings.clone() })`; a refusal returns `Err(ConfigErrorReturn::Selections(error))` unchanged; success continues with the `AdmittedSelections` the call returns.

### Statements

Dependency call `(deps.admit_statements)(&AdmitStatementsDeps, AdmitStatementsParams, AdmitStatementsPayload { network_profile, statements, identities, curve_labels, scope })` over clones of the document's `network_profile`, `statements`, and `identities` tables, `curve_labels` the `label` of each entry of `admitted.curves` in order cloned, and `scope` the `scope` of `admitted.kem`; a refusal returns `Err(ConfigErrorReturn::Statements(error))` unchanged; success continues with the `AdmittedNetworkProfile`, `AdmittedStatements`, and `AdmittedIdentities` the call returns.

### Generate

Dependency call `(deps.admit_generate)(&AdmitGenerateDeps, AdmitGenerateParams, AdmitGeneratePayload { generate })` over a clone of the document's `generate` table; a refusal returns `Err(ConfigErrorReturn::Generate(error))` unchanged; success continues with the `AdmittedGenerate` the call returns.

### Output

Dependency call `(deps.admit_output)(&AdmitOutputDeps, AdmitOutputParams, AdmitOutputPayload { output, report })` over clones of the document's `output` and `report` tables; a refusal returns `Err(ConfigErrorReturn::Output(error))` unchanged; success continues with the `AdmittedOutput` the call returns.

### Benchmark iterations

Decision `NonZeroU32::new(document.benchmark.iterations)`; none returns `Err(ConfigErrorReturn::BenchmarkIterations)`; some continues with the count.

### Measurement

Decision `NonZeroU32::new(document.measurement.iterations)`; none returns `Err(ConfigErrorReturn::MeasurementIterations)`; some continues, then decision `ReportText::try_new` over `document.measurement.conditions`; a refusal returns `Err(ConfigErrorReturn::MeasurementConditions(error))` unchanged; success continues with the `MeasurementSettings` of both admitted values.

### Report defaults

Decision `NonZeroU32::new(document.report.piece_group_size)`; none returns `Err(ConfigErrorReturn::ReportPieceGroupSize)`; some continues, then decision `ReportText::try_new` over `document.report.primary_curve`; a refusal returns `Err(ConfigErrorReturn::ReportPrimaryCurve(error))` unchanged; success continues with the `ReportDefaults` of both admitted values.

### Report adjustment absent

Condition `document.report.adjustment` is `None`; outcome `None`, nothing read.

### Report adjustment present

Condition `document.report.adjustment` is `Some`: each present `piece_group_size` through `NonZeroU32::new`, none returning `Err(ConfigErrorReturn::AdjustmentPieceGroupSize)`; each present `primary_curve` through `ReportText::try_new`, a refusal returning `Err(ConfigErrorReturn::AdjustmentPrimaryCurve(error))` unchanged; an absent field stays `None` in the `ReportAdjustment`.

### Curve labels

Per entry of `admitted.curves` in order: decision `ReportText::try_new` over the entry's `label` cloned; a refusal returns `Err(ConfigErrorReturn::CurveLabel { index, error })` unchanged, `index` the entry's position; success continues with one `ReportText` per curve in the same order.

### Verifier endpoint

Decision `EvmVerifierEndpoint::try_new(EvmVerifierEndpointConstructorParams { text: document.network_profile.verifier_endpoint })`; a refusal returns `Err(ConfigErrorReturn::VerifierEndpoint(error))` unchanged; success continues with the endpoint.

### Verifier concrete

Decision an exhaustive `match` on `admitted.verifier`, per curve in curve order; the `VerifierConcreteKind::Evm` arm yields, for the curve at index `i`, `VerifierConcrete::Evm { settings: Box::new(EvmVerifierSettings { endpoint: endpoint.clone(), verifier_address: network_profile.deployment_records[i].clone(), gas_price_oracle: network_profile.gas_price_oracle.clone() }) }`, the record read being the one `admit_statements` returned for the curve at that index, so each curve's concrete holds its own deployment's address and no two curves share a record unless the document names the same address; the concretes are collected in curve order into `verifiers`.

### Success

Outcome `Ok(ConfigSuccessReturn { configuration })`, the `HarnessConfiguration` holding `selections`, `verifiers`, `curve_labels`, `network_profile`, `statements`, `identities`, `generate`, `output`, `benchmark_iterations`, `measurement`, and `report` — each field the called function's value unchanged or the value admitted above.

### Unreached branches

- Every refusal arm in unit tests: `mock_layers`, `mock_admit_selections`, `mock_admit_statements`, `mock_admit_generate`, and `mock_admit_output` return `Ok` alone, and the document `mock_layers` returns holds a nonzero count, a non-empty text, and a well-formed endpoint in each field this function admits, so no unit block reaches a refusal arm; every refusal arm is proven by an own entry below over the real functions.
- A `VerifierConcreteKind` the `match` has no arm for: the `match` has an arm for every member of the enum, so that branch does not exist and has no unit block.

### Ordering

Layers, selections, statements, generate, output, benchmark iterations, measurement iterations, measurement conditions, report piece-group size, report primary curve, report adjustment, each curve label, then the verifier endpoint; the first refusal returns with every later step unread; no side effect beyond `layers`' reads.

## Integration: own entries

- `the_shipped_defaults_configure_the_harness`: condition `config` over `SHIPPED_DEFAULTS` and the program name alone, every callee real; outcome `Ok` whose configuration holds, for each family, the declared member the default text names with the identifier, scope, and algebra that member carries, one curve and one curve label per `[[selections.curves]]` entry in order, one benchmark concrete per `[[selections.benchmark_concretes]]` entry in order, one `VerifierConcrete::Evm` per curve in curve order, each over the default endpoint, that curve's default deployment record, the address at which nothing is deployed, and the gas price oracle, the two statements' purposes `1` and `2`, the generate settings holding one kind per kind of `SOLIDITY_GENERATE_DECLARATION`, the output destinations the default holds, and the counts and report defaults the default holds; variation the default text alone; edge a key the text omits, a text that names no declared member, or a value the text declares that a callee refuses, differs from the configuration built in Arrange or refuses.
- `a_file_and_a_flag_each_replace_a_shipped_default`: condition `config` over `SHIPPED_DEFAULTS` and a command line naming a `--config` file, a `--set` assignment, or both in each order, each over a default the document declares as a scalar, as a table key, and as an array, every callee real and the filesystem real; outcome `Ok` whose configuration holds, for each key a later layer named, the value of the last layer that named it and, for each key no layer named, the shipped default; variation the interleavings file then flag, flag then file, and file alone, each over `benchmark.iterations`, `selections.identity_scope`, and `selections.curves`; edge a layer applied out of command-line order, an array extended instead of replaced, or a flag that reaches the document but not the admitted value, differs from the configuration built in Arrange.
- `an_unadmissible_value_is_refused_by_the_function_that_admits_it`: condition `config` over `SHIPPED_DEFAULTS` with one value overridden by a `--set` assignment, every callee real; outcome `Err` carrying the callee's refusal whole inside the variant of the function that returned it; variation an unknown flag for `Layers`, a key the document does not declare for `Layers`, the `Debug` text of a family's mock selection in `selections.kem` for `Selections`, an unknown kind in `network_profile.chain_identifier` for `Statements`, an unknown `generate.concrete` for `Generate`, and an empty `output.destinations` for `Output`; edge a refusal that is converted, summarized, or returned inside another function's variant differs.
- `a_value_this_function_admits_is_refused_with_its_own_variant`: condition `config` over `SHIPPED_DEFAULTS` with one value overridden by a `--set` assignment, every callee real; outcome `Err` of the variant that names the document path, `BenchmarkIterations`, `MeasurementIterations`, `MeasurementConditions`, `ReportPieceGroupSize`, `ReportPrimaryCurve`, `AdjustmentPieceGroupSize`, `AdjustmentPrimaryCurve`, `CurveLabel { index }`, and `VerifierEndpoint`; variation a zero for each count, a text of whitespace alone for each text, a curve label of whitespace alone at index one, and an endpoint text no URL parser admits; edge a zero admitted as `NonZeroU32::MIN`, a blank text admitted, or a refusal reported at another index or path differs.
- `the_verifier_is_selected_with_the_settings_the_document_holds`: condition `config` over `SHIPPED_DEFAULTS` with `network_profile.verifier_endpoint`, `network_profile.deployment_records.bn254`, `network_profile.deployment_records.bls12_381`, and `gas_price_oracle` overridden, every callee real; outcome `Ok` whose `verifiers` equals two `VerifierConcrete::Evm` in curve order, each over the endpoint, that curve's `verifier_address`, and the `gas_price_oracle` built in Arrange from the overridden values; variation each value overridden alone, the two records overridden together, and the layer of a single curve's record, which leaves the other curve's shipped default; edge a settings field read from another table, one curve's record read for the other curve, a single record shared by both curves, or the records and the oracle exchanged, differs.
- `the_first_refusal_is_returned_in_the_stated_order`: condition `config` over `SHIPPED_DEFAULTS` with a refused value in two or more steps, every callee real; outcome the earlier step's refusal; variation `Layers` before `Selections`, `Selections` before `Statements`, `Statements` before `Generate`, `Generate` before `Output`, `Output` before `BenchmarkIterations`, `BenchmarkIterations` before `MeasurementIterations`, and `MeasurementIterations` before `VerifierEndpoint`; edge a step run before its predecessor differs.

## Integration: callee dispositions

- `layers` entry `layered_sources_merge_in_command_line_order`: carried; restated where this node transforms it: `config` hands the command line to `layers` unchanged and every admission reads the merged document, so each admitted value is the value of the last layer that named its key; route `layers`, then `admit_selections`, `admit_statements`, `admit_generate`, and `admit_output` over the merged document, then `config`; outer-edge collaborator the filesystem; proven by `a_file_and_a_flag_each_replace_a_shipped_default` of the public integration test element of this node.
- `admit_selections` entries `every_member_of_every_declared_set_is_admitted_by_its_own_name` and `a_mock_selections_name_is_never_admitted`: carried; restated where this node transforms them: the admitted selections reach the configuration unchanged and a refusal reaches the caller inside `ConfigErrorReturn::Selections`; route `layers`, `admit_selections`, `member_named`, then `config`; outer-edge collaborator none; proven by the blocks of the same names of the public integration test element.
- `admit_statements` entries `every_declared_kind_admits_its_value_text_to_its_own_variant`, `a_kind_outside_the_declared_sets_is_never_admitted`, `every_context_carries_the_network_profiles_chain_identifier_and_entitlement_contract`, `identities_are_admitted_under_the_scope_the_payload_names`, `every_curve_has_its_own_deployment_record_in_curve_order`, and `a_curve_without_a_record_and_a_record_without_a_curve_are_each_refused_with_the_label`: carried; restated where this node transforms them: the admitted profile, statements, and identities reach the configuration unchanged, each curve's record reaches `verifiers` at that curve's index, the curve labels and the scope handed are the ones `admit_selections` admitted, and a refusal reaches the caller inside `ConfigErrorReturn::Statements`; route `layers`, `admit_selections`, `admit_statements`, its private functions, then `config`; outer-edge collaborator none; proven by the blocks of the same names of the public integration test element.
- `admit_generate` entries `every_declared_kind_is_admitted_to_its_own_settings_and_configuration`, `every_declared_curve_directory_reaches_the_field_its_curve_names`, and `an_undeclared_name_is_never_admitted`: carried; restated where this node transforms them: the admitted generate settings reach the configuration unchanged and a refusal reaches the caller inside `ConfigErrorReturn::Generate`; route `layers`, `admit_generate`, then `config`; outer-edge collaborator none; proven by the blocks of the same names of the public integration test element.
- `admit_output` entries `every_declared_destination_kind_is_admitted_to_its_own_variant`, `a_destination_list_keeps_the_order_the_document_holds`, and `an_unadmissible_destination_list_is_never_admitted`: carried; restated where this node transforms them: the admitted destinations reach the configuration unchanged in order and a refusal reaches the caller inside `ConfigErrorReturn::Output`; route `layers`, `admit_output`, then `config`; outer-edge collaborator none; proven by the blocks of the same names of the public integration test element.

## Integration: private surface

The chain of real functions `config`, `layers`, `admit_selections`, `admit_statements`, `admit_generate`, `admit_output`, `ReportText::try_new`, and `EvmVerifierEndpoint::try_new`, all running for real over `SHIPPED_DEFAULTS`; the outer-edge collaborator is the filesystem, a directory beneath the operating system's temporary directory removed before each act; the observable result is the `ConfigReturn`; the entries proven are `the_shipped_defaults_configure_the_harness`, `a_file_and_a_flag_each_replace_a_shipped_default`, `an_unadmissible_value_is_refused_by_the_function_that_admits_it`, `a_value_this_function_admits_is_refused_with_its_own_variant`, `the_verifier_is_selected_with_the_settings_the_document_holds`, and `the_first_refusal_is_returned_in_the_stated_order`.

## Integration: public surface

An outside caller invokes `config` through the crate root with a `ConfigDeps` holding the five functions, a `ConfigParams` holding a command line, and a `ConfigPayload` holding `SHIPPED_DEFAULTS`, and observes the `ConfigReturn` and the `HarnessConfiguration` in its success; the entries proven are the own entries above and the carried entries the dispositions above list; the outer-edge collaborator is the filesystem.
