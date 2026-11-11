# `layers` — interaction spec

Branch contract for the `layers` module of `apps/harness-crypto`: `layers` produces the configuration document from layered sources — the command line parsed through `clap` into its repeatable `--config <PATH>` and `--set <KEY=VALUE>` arguments, each named file read through `std::fs`, and the defaults text, each file's text, and each assignment parsed as TOML and merged over the defaults in command-line order, a table merging key by key and any other value replacing the earlier one whole — and deserializes the merged table into the configuration document, each of whose tables refuses a key it does not declare. Each branch states condition, decision, dependency call, and the exact return outcome.

## `layers`

`layers(deps: &LayersDeps, params: LayersParams, payload: LayersPayload) -> LayersReturn`, the trusted form: the command line arrives as handed arguments in `params`, the program name first; the defaults arrive as handed text in `payload`; the function reads the filesystem only for the files the arguments name, and reads no environment variable and no file the arguments do not name.

### Arguments refused

Condition `ConfigArguments::command().try_get_matches_from(params.arguments)` returns `Err(error)`, an unknown flag, a flag without its value, and the help and version requests among them; decision the `Result`; dependency call `clap`; outcome `Err(LayersErrorReturn::Arguments(error))`, the error unchanged, before the defaults are parsed.

### Arguments admitted

Condition the matches are `Ok`; dependency call `ConfigArguments::from_arg_matches(&matches)`; outcome the `ConfigArguments`, whose `config` and `set` hold their values in the order they appeared; a refusal of `from_arg_matches` returns `Err(LayersErrorReturn::Arguments(error))` unchanged.

### Command-line order

The layer list pairs each value of `config` with the position `matches.indices_of("config")` returns for it and each value of `set` with the position `matches.indices_of("set")` returns for it, and sorts the pairs by position, so the layers apply in the order the flags appeared across both flags.

### Defaults refused

Condition `toml::from_str::<toml::Table>(&payload.defaults)` returns `Err(error)`; outcome `Err(LayersErrorReturn::ParseDefaults(error))`, the error unchanged, before any file is read.

### Defaults admitted

Condition the text parses; outcome the table the merge starts from.

### File layer

Per layer in order: condition the layer is a `--config` path; dependency call `std::fs::read_to_string(&path)`; outcome a failure returns `Err(LayersErrorReturn::ReadFile { path, error })`, the error unchanged, before any later layer is read.

### File text refused

Condition the file's text does not parse as a TOML table; dependency call `toml::from_str::<toml::Table>`; outcome `Err(LayersErrorReturn::ParseFile { path, error })`, the error unchanged.

### Assignment malformed

Condition the assignment has no `=` or its text before the first `=` is empty once trimmed; decision `split_once('=')` and the trimmed key; dependency call none; outcome `Err(LayersErrorReturn::MalformedAssignment { assignment })`.

### Assignment text refused

Condition the assignment is a key and a value and the whole assignment text does not parse as a TOML table of one assignment; dependency call `toml::from_str::<toml::Table>(&assignment)`; outcome `Err(LayersErrorReturn::ParseAssignment { assignment, error })`, the error unchanged.

### Merge

Per layer: condition the layer's table parsed; decision for each key of the layer, both values are `Value::Table`; outcome the shared key's merge recurses into the two tables; decision either value is not a table or the key is absent from the table so far; outcome the layer's value replaces the earlier value whole, or is inserted.

### Document refused

Condition `toml::Value::Table(merged).try_into::<ConfigDocument>()` returns `Err(error)`, an undeclared key in any table, an absent required key, or a value of the wrong kind; outcome `Err(LayersErrorReturn::Document(error))`, the error unchanged.

### Document admitted

Outcome `Ok(LayersSuccessReturn { document })`.

### Unreached branches

- Refused arguments after matching: no input makes `ConfigArguments::from_arg_matches` refuse matches the command it was derived from accepted, so that `Err` has no unit block.

### Ordering

Arguments, then defaults, then each layer in command-line order, then the document; the first failure returns; each file is read when its layer is reached; no side effect beyond the reads.

## Integration: own entries

- `layered_sources_merge_in_command_line_order`: condition `layers` over a command line naming two files and two assignments interleaved, every collaborator real, the filesystem real, over a defaults text declaring every table; outcome `Ok` whose document holds, for each key a later layer named, the value of the last layer that named it, and, for each key no layer named, the defaults' value; variation the interleavings file then assignment, assignment then file, and two files, each over a key the defaults declare as a scalar, as a table key, and as an array; edge a layer applied in flag order within its own flag instead of across both, a table replaced whole instead of merged key by key, or an array merged instead of replaced, differs from the document built in Arrange.

## Integration: private surface

The chain of real functions `layers`, `clap`'s command and matches, `std::fs::read_to_string`, `toml::from_str`, the merge, and `toml::Value::try_into`, all running for real; the outer-edge collaborator is the filesystem, a directory beneath the operating system's temporary directory removed before each act; the observable result is the `LayersReturn`; the entry proven is `layered_sources_merge_in_command_line_order`.

## Integration: public surface

An outside caller invokes `layers` through the crate root with a `LayersDeps`, a `LayersParams` holding a command line, and a `LayersPayload` holding a defaults text, and observes the `LayersReturn` and the `ConfigDocument` in its success; the entry proven is `layered_sources_merge_in_command_line_order`; the outer-edge collaborator is the filesystem.
