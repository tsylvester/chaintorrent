# `layers` — interaction spec

Branch contract for the `layers` module of the `harness-crypto` crate: `layers`, the function the configuration document is composed from — the command line parsed into its repeatable `--config <PATH>` files and `--set <KEY=VALUE>` assignments, each file read and every text parsed as TOML, the files and then the assignments merged in command-line order over the defaults text it is handed, and the merged table deserialized into the configuration document. Each branch states condition, decision, dependency call, and the exact return outcome.

## `layers`

`layers(deps: &LayersDeps, params: LayersParams, payload: LayersPayload<'_>) -> LayersReturn`, the validating form: the command line and every text arrive from outside the type system and are proven by parsing and deserialization; `deps` and `params` carry nothing and are not read.

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| arguments refused | `ConfigArguments::try_parse_from(payload.arguments)` returns `Err(error)` | the `Err` arm of the parse | `ConfigArguments::try_parse_from` | `Err(LayersErrorReturn::Arguments(error))`, the `clap` error unchanged, a help or version request among them |
| defaults not TOML | `payload.defaults.parse::<toml::Table>()` returns `Err(error)` | the `Err` arm of the parse | `FromStr` for `toml::Table` | `Err(LayersErrorReturn::ParseDefaults(error))`, the parser's error unchanged |
| defaults admitted | the defaults text parses | the `Ok` arm | the same call | the admitted table is the accumulated table |
| file unreadable | `std::fs::read_to_string(&path)` returns `Err(error)` for a `path` of `arguments.config`, taken in order | the `Err` arm of the read | `std::fs::read_to_string` | `Err(LayersErrorReturn::ReadFile { path, error })`, the read error unchanged |
| file not TOML | the read text's `parse::<toml::Table>()` returns `Err(error)` | the `Err` arm of the parse | `FromStr` for `toml::Table` | `Err(LayersErrorReturn::ParseFile { path, error })`, the parser's error unchanged |
| file merged | the file's text parses | the `Ok` arm | the same calls | the merge of that table over the accumulated table, then the next file |
| assignment not TOML | the assignment's `text.parse::<toml::Table>()` returns `Err(error)` at `(index, text)` of `arguments.set`, taken in order | the `Err` arm of the parse | `FromStr` for `toml::Table` | `Err(LayersErrorReturn::ParseAssignment { index, error })`, the parser's error unchanged |
| assignment not one key | the parsed assignment table's `len()` is not `1` | the length | none | `Err(LayersErrorReturn::AssignmentNotKeyValue { index, keys })`, `keys` the length |
| assignment merged | the assignment's table holds exactly one top-level key | the length | none | the merge of that table over the accumulated table, then the next assignment |
| document refused | `accumulated.try_into::<HarnessConfigDocument>()` returns `Err(error)`, a missing key, an undeclared key, and a value outside its field's type each among them | the `Err` arm of the deserialization | `Table::try_into` | `Err(LayersErrorReturn::Document(error))`, the deserializer's error unchanged |
| computed | the arguments parsed, the defaults and every layer merged, the document deserialized | none | the calls stated | `Ok(LayersSuccessReturn { document })` |

### Merge

A merge of an overlay table runs without recursion. A work list of pairs, each a key path and a table, begins with the empty path and the overlay; each pair taken from it yields, for each entry of its table in the table's order, a table step at the path extended by the entry's key, where the value is a `Value::Table`, the extended path and that table then added to the work list, and an insert step of the entry's value at the extended path otherwise.

Each step is applied in the order the walk yields it, through one local closure that takes the accumulated table by value and returns it: the accumulated table is descended along the step's path, the whole path for a table step and every key but the last for an insert step, each key's value removed and taken as the next table where it is a `Value::Table` and replaced by an empty table otherwise, every table passed through held with its key on a stack; an insert step then inserts its value under the path's last key, replacing whatever was there; the stack is then unwound, each held table receiving the table below it as a `Value::Table` under that table's key, and the unwound root is the accumulated table. No branch is unreachable, a missing or non-table value being replaced by an empty table at every step.

### Ordering

The arguments, the defaults, each file in order, each assignment in order, then the document; the first refusal returns; each file is read once; no side effect beyond the reads.
