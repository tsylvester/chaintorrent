# `main` — interaction spec

Branch contract for the `main` binary of `apps/harness-crypto`: `main` is the harness's composition root over the process — it builds the `ConfigDeps` of the five admission and layering functions, calls `config` with `SHIPPED_DEFAULTS` and the process's command line, `run` with the configuration `config` returned, and `output` with the outcome `run` returned and the destinations `config` admitted; it writes a configuration refusal or a refused write to standard error and exits with a success status only when the run succeeded and every destination was written. It declares no type and holds no literal of its own; it names no stream but standard error and reads no file, environment variable, or clock; every value it calls with is the process's command line, `SHIPPED_DEFAULTS`, or a callee's return. Each branch states condition, decision, dependency call, and the exact return outcome.

## `main`

`main() -> ExitCode`, the process entry; the conditions below are selected by what each callee returns, never by an input of its own.

### Configure

Dependency call `config(&ConfigDeps { layers, admit_selections, admit_statements, admit_generate, admit_output }, ConfigParams { arguments: std::env::args_os().collect() }, ConfigPayload { defaults: SHIPPED_DEFAULTS.to_owned() })`, once; condition it returns `Err(error)`; decision the `Result` match; outcome `error` is written to standard error as its `Debug` text, `run` and `output` are not called, and `main` returns `ExitCode::FAILURE`.

### Run

Condition `config` returns `Ok(ConfigSuccessReturn { configuration })`: the destinations `configuration.output.destinations` are cloned, then dependency call `run(&RunDeps, RunParams, RunPayload { configuration })`, once, whatever it returns; the `RunReturn` is held whole.

### Run outcome

Decision `outcome.is_ok()` read before the outcome is handed on; the boolean is held for the exit status.

### Output

Dependency call `output(&OutputDeps, OutputParams { destinations }, OutputPayload { outcome })`, once, with the outcome `run` returned, a success or a refusal; condition it returns `Err(error)`; outcome `error` is written to standard error as its `Debug` text and `main` returns `ExitCode::FAILURE`.

### Exit

Condition `output` returns `Ok(_)`; outcome `main` returns `ExitCode::SUCCESS` when the held boolean is true and `ExitCode::FAILURE` when it is false, a refused run being an outcome that was written and a failure of the process.

### Unreached branches

- Refused standard error: no input and no failure mode of a test process makes standard error refuse a write, so the arm of `eprintln!` has no block.

### Ordering

The configuration is obtained first; `run` is called only on a configuration; `output` is called only after `run` returned and is called whether `run` succeeded or refused; each callee is called at most once; the destinations are written in the order the configuration holds them by `output`; standard error is written only for a configuration refusal or a refused write; the same command line over the same filesystem and network always selects the same branch.

## Integration: own entries

- `a_configuration_refusal_is_written_to_standard_error_and_the_process_fails`: condition the binary invoked with a command line the configuration refuses; outcome a failure status, standard error holding the `Debug` text of the refusal and a line feed, standard output empty, `run` not called, and no destination written; variation an unknown flag, whose text begins `Layers(`, and `--set benchmark.iterations=0`, whose text is exactly `BenchmarkIterations`, so a binary that exits success on a refusal, writes the refusal to standard output, or runs after a refusal differs; edge a report or vector file present beneath the arranged directory, or standard output holding any text, differs.
- `a_refused_run_is_written_as_its_refusal_and_the_process_fails`: condition the binary invoked over the shipped defaults with `generate.foundry_root`, `report.destinations`, and `output.destinations` assigned beneath an arranged directory and the iteration counts assigned the lowest admitted value, the run reaching the verifier family's refusal for the address at which nothing is deployed; outcome a failure status, the text written to every configured destination being one document whose `outcome` is `"refused"` and whose `refusal` is a string beginning `PairingConsumer(`, the same text in every destination, standard error empty; variation the destinations standard output alone, a file alone, and both, so a binary that returns success for a refused run, converts the refusal, or writes a destination only on success differs; edge a destination holding text that differs from another destination's, or standard error holding text, differs.
- `a_refused_destination_write_is_written_to_standard_error_and_the_process_fails`: condition the binary invoked as the entry above states with `output.destinations` holding a file destination whose directory does not exist; outcome a failure status, standard error holding a line that begins `WriteFile { path: ` and is the `Debug` text of the refused write, and no destination after the refused one written; variation the refused destination first with a later file destination, and last with an earlier file destination, the first leaving the later file absent and the second leaving the earlier file holding the document; edge a later file present after a refused destination, an earlier file absent, or a success status differs.
- `a_successful_run_exits_with_success_after_every_destination_is_written`: condition the binary invoked with `--config ../../contracts/evm/deployments/base-sepolia.toml`, the layer `contracts/evm/deploy` emits, the iteration counts and the beneath-a-directory assignments as the entries above state, and `output.destinations` holding standard output and a file; outcome a success status, standard output and the file each holding one document whose `outcome` is `"succeeded"`, the two equal, standard error empty; variation the layer with standard output alone and with the file alone; edge a failure status after every destination was written, a document whose `outcome` is `"refused"`, or a destination left unwritten differs; end-to-end: proven by an end-to-end test run after `contracts/evm/deploy` executes, outside this node's integration elements and the ordinary test cycle, its runtime inputs the deployed verifiers and the layer file.

## Integration: callee dispositions

- `an_unadmissible_value_is_refused_by_the_function_that_admits_it`, `a_value_this_function_admits_is_refused_with_its_own_variant`, and `the_first_refusal_is_returned_in_the_stated_order` of `harness-crypto/config`: carried; restated where this node transforms them: a refusal reaches standard error whole inside the variant of the function that returned it, the earliest of two refused values first, and the process fails; route `layers` or the admission function that refuses, then `config`, then `main`, ending at `main`; outer-edge collaborators the filesystem, a real directory; proven by the blocks of the same names of the public integration test element of this node.
- `a_file_and_a_flag_each_replace_a_shipped_default` of `harness-crypto/config`: carried; restated where this node transforms it: the destinations the last layer that named `output.destinations` holds are the destinations written, observed by which of standard output and a file holds the outcome; route `layers`, the admission functions, `config`, `run`, `output`, then `main`, ending at `main`; outer-edge collaborators the filesystem, a real directory, and the verifier family's refusal for the address at which nothing is deployed; proven by the block of the same name of the public integration test element.
- `the_shipped_defaults_reach_no_deployed_verifier` of `harness-crypto/run`: carried; restated where this node transforms it: a refused run the process reports as a failure status and writes as its refusal, the vector files its generate step wrote existing beneath the arranged directory; route `config`, `run`, `output`, then `main`, ending at `main`; outer-edge collaborators as the entry above states; proven by the block of the same name of the public integration test element. `a_deployment_record_layer_and_a_flag_each_replace_a_shipped_default` is not carried: an accepted run is observable through the process only against the deployed verifiers; the entry is end-to-end.
- `a_refused_runs_outcome_is_written_as_its_refusal` of `harness-crypto/output`: carried; restated where this node transforms it: the document reaches every configured destination as the text of the run's refusal; route `run`, `render_outcome`, `output`, then `main`, ending at `main`; outer-edge collaborators as the entries above state; proven by the block of the same name of the public integration test element. `a_successful_runs_outcome_is_written_with_every_value_it_holds` is not carried: a success document is observable at the process boundary only against the deployed verifiers; the entry stays proven at `harness-crypto/output`'s own tests.
- `every_declared_destination_kind_is_admitted_to_its_own_variant` and `a_destination_list_keeps_the_order_the_document_holds` of `harness-crypto/config/admit_output`: carried; restated where this node transforms them: a standard output destination and a file destination each receive the document, and a destination that is refused stops the writes after it and leaves the writes before it, ending in a failure status and a line on standard error; route `layers`, `admit_output`, `config`, `output`, then `main`, ending at `main`; outer-edge collaborators the filesystem, a real directory; proven by the blocks of the same names of the public integration test element.
- The other entries of `harness-crypto/config`, `harness-crypto/run`, `harness-crypto/output`, and the nodes beneath them end at those nodes' tests and are not observed through this binary's exit status, standard output, standard error, or files, which hold no admitted selection, pairing, scalar, encoded value, or per-curve record of their own; none of them is dispositioned here.

## Integration: private surface

None; this node has no in-crate chain, and every entry crosses the process boundary.

## Integration: public surface

An outside caller invokes the built binary `harness-crypto` as a process with a command line and a working directory and observes the exit status, standard output, standard error, and the files the configuration names; the entries proven are every own entry above but `a_successful_run_exits_with_success_after_every_destination_is_written`, and every carried entry the dispositions list; the outer-edge collaborators are the filesystem, a directory beneath the operating system's temporary directory, the operating system's generator and monotonic clock, and the network the verifier family's endpoint names; the entries needing the deployed verifiers — the own entry `a_successful_run_exits_with_success_after_every_destination_is_written` and the carried `a_deployment_record_layer_and_a_flag_each_replace_a_shipped_default` — are end-to-end, and the carried `a_successful_runs_outcome_is_written_with_every_value_it_holds` stays proven at `harness-crypto/output`'s own tests, none of them proven by this element.
