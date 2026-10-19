# `admit_output` — interaction spec

Branch contract for the `admit_output` module of the `config` module of `apps/harness-crypto`: `admit_output`, the function the configuration document's output table is admitted by — each destination's kind text mapped to the `OutputDestination` variant it names, a file destination carrying the document's path, the destinations admitted in document order; a kind the vocabulary does not hold refused with its index and text, a `file` destination with no path and a `standard_output` destination with one each refused with its index, and a list naming no destination refused. Each branch states condition, decision, dependency call, and the exact return outcome.

## `admit_output`

`admit_output(deps: &AdmitOutputDeps, params: AdmitOutputParams, payload: AdmitOutputPayload<'_>) -> AdmitOutputReturn`: the payload is the document the deserializer proved the shape of, and this function proves its content; `deps` and `params` carry nothing and are not read. `output` below is `payload.output`.

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| empty | `output.destinations` is empty | none | none | `Err(AdmitOutputErrorReturn::Empty)` |
| destinations | each `(index, entry)` of `output.destinations` in order | a `match` on `(entry.kind.as_str(), &entry.path)`, the kind decided before the path | none | per destination, below |
| computed | every destination admitted | none | none | `Ok(AdmitOutputSuccessReturn { destinations })`, `destinations` the admitted destinations in document order |

Each destination's `(entry.kind.as_str(), &entry.path)`:

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| standard output | `("standard_output", None)` | the match arm | none | the admitted destination `OutputDestination::StandardOutput` |
| standard output with a path | `("standard_output", Some(_))` | the match arm | none | `Err(AdmitOutputErrorReturn::UnexpectedPath { index })` |
| file | `("file", Some(path))` | the match arm | none | the admitted destination `OutputDestination::File { path }`, `path` a clone of the document's path |
| file without a path | `("file", None)` | the match arm | none | `Err(AdmitOutputErrorReturn::MissingPath { index })` |
| unknown kind | any other kind text, whatever its path | the match arm | none | `Err(AdmitOutputErrorReturn::UnknownKind { index, text })`, `text` a clone of the kind's text |

### Ordering

The emptiness check, then each destination in document order; the first refusal returns; no side effect.
