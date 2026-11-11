# `admit_output` — interaction spec

Branch contract for the `admit_output` module of `apps/harness-crypto`: `admit_output` turns the document's `output` and `report` tables into the destination types the output and the report take — each entry's `kind` text mapped to the member of `DESTINATION_KINDS` it names, `StandardOutput` admitted to the variant of that name and `File` to a file holding the path the entry carries — and refuses an empty list, a kind no member names, or a file with no path, each with the document path that held it. Each branch states condition, decision, dependency call, and the exact return outcome.

## `admit_output`

`admit_output(deps: &AdmitOutputDeps, params: AdmitOutputParams, payload: AdmitOutputPayload) -> AdmitOutputReturn`, the trusted form: the document's `output` and `report` tables arrive as data in `payload`; the function reads no file, no environment variable, and no clock; `deps` and `params` carry nothing and are not read.

### Naming

Private `kind_named`: given a text; decision the first member of `DESTINATION_KINDS` in declared order for which `format!("{:?}", &member)` equals the text, compared whole and case-sensitively; outcome that `DestinationKind`, or none; dependency call none.

### Output list empty

Condition `payload.output.destinations` is empty; decision the emptiness check; dependency call none; outcome `Err(AdmitOutputErrorReturn::OutputDestinationsEmpty)`.

### Output destination kind refused

Per entry of `payload.output.destinations` in order: decision `kind_named` over the entry's `kind`; no member returns `Err(AdmitOutputErrorReturn::OutputDestinationKind { index, text })`, `index` the entry's position and `text` the entry's `kind`; a member continues with that `DestinationKind`.

### Output destination admitted or refused

Per entry: decision an exhaustive `match` on the admitted `DestinationKind`; the kind `StandardOutput` yields `OutputDestination::StandardOutput`, the entry's `path` unread; the kind `File` with a `path` of `Some(path)` yields `OutputDestination::File { path }`, the path cloned from the entry; the kind `File` with a `path` of `None` returns `Err(AdmitOutputErrorReturn::OutputDestinationPath { index })`, `index` the entry's position.

### Report list empty

Condition `payload.report.destinations` is empty; outcome `Err(AdmitOutputErrorReturn::ReportDestinationsEmpty)`.

### Report destination kind refused

Per entry of `payload.report.destinations` in order: the same decision as the output entries; no member returns `Err(AdmitOutputErrorReturn::ReportDestinationKind { index, text })`, `index` the entry's position and `text` the entry's `kind`.

### Report destination admitted or refused

Per entry: the same exhaustive `match`, yielding `ReportDestination::StandardOutput` for the kind `StandardOutput`, the entry's `path` unread, and `ReportDestination::File { path }` for the kind `File` with a `path` of `Some(path)`, the path cloned from the entry; the kind `File` with a `path` of `None` returns `Err(AdmitOutputErrorReturn::ReportDestinationPath { index })`, `index` the entry's position.

### Success

Outcome `Ok(AdmitOutputSuccessReturn { admitted: AdmittedOutput { destinations, report_destinations } })`, `destinations` the admitted `OutputDestination`s and `report_destinations` the admitted `ReportDestination`s, each in the order of the document's entries.

### Ordering

The output list's emptiness, then each output entry's kind and path in order, then the report list's emptiness, then each report entry's kind and path in order; the first refusal returns and every later value is unread; `deps` and `params` carry nothing and are not read; no side effect.

### Unreached branches

- An entry whose kind is a member of `DESTINATION_KINDS` that the exhaustive `match` has no arm for: the `match` has an arm for every member, so that branch does not exist and has no unit block.

## Integration: own entries

- `every_declared_destination_kind_is_admitted_to_its_own_variant`: condition `admit_output` over a document whose `output.destinations` and `report.destinations` each hold one entry whose `kind` is the `Debug` text of the member under test, its `path` chosen by an exhaustive `match` on the member (absent for `StandardOutput`, a distinct path in each list for `File`), every collaborator real, the declared set and the report's destination type; outcome `Ok` whose `destinations` holds the `OutputDestination` the member names and whose `report_destinations` holds the `ReportDestination` the member names, a file holding the path its own list's entry carried; variation every member of `DESTINATION_KINDS` in turn; edge a text matched to a neighbouring member, a path dropped or placed on the other variant, or the two lists' paths exchanged differs from the destinations built in Arrange.
- `a_destination_list_keeps_the_order_the_document_holds`: condition `admit_output` over a document whose two lists each hold every member of `DESTINATION_KINDS`, once in declared order and once in reverse, every collaborator real; outcome `Ok` whose lists hold the admitted destinations in the order the entries were held, each file with its own path; variation the two orders; edge a list sorted by kind, deduplicated, or reversed differs from the destinations built in Arrange.
- `an_unadmissible_destination_list_is_never_admitted`: condition `admit_output` over a document whose `output.destinations` and then whose `report.destinations`, the other list admissible, is empty, holds an entry whose `kind` is no member's `Debug` text, or holds a `File` entry with no path, every collaborator real; outcome `Err` carrying the list's variant with the entry's index and, for a kind, its text; variation the three refusals in turn for each list; edge a list that admitted an empty list, a kind matched case-insensitively, or a file without a path would differ from the refusal built in Arrange.

## Integration: callee dispositions

No entry of a direct dependency passes through `admit_output` or ends at its tests, because it reads `layers`' tables as data and `report`'s destination type as a value and calls neither of their functions, and no entry of `harness-crypto/config/layers` or `harness-crypto/report` is proven through the lookup or the admission.

## Integration: private surface

The chain of real functions `admit_output`, `kind_named`, and the `Debug` text of each declared member, all running for real over the real declared set; no outer-edge collaborator; the observable result is the `AdmitOutputReturn`; the entries proven are `every_declared_destination_kind_is_admitted_to_its_own_variant`, `a_destination_list_keeps_the_order_the_document_holds`, and `an_unadmissible_destination_list_is_never_admitted`.

## Integration: public surface

An outside caller invokes `admit_output` through the crate root with an `AdmitOutputDeps`, an `AdmitOutputParams`, and an `AdmitOutputPayload` holding an `OutputTable` and a `ReportTable`, and observes the `AdmitOutputReturn` and the `AdmittedOutput` in its success; the entries proven are `every_declared_destination_kind_is_admitted_to_its_own_variant`, `a_destination_list_keeps_the_order_the_document_holds`, and `an_unadmissible_destination_list_is_never_admitted`; there is no outer-edge collaborator.
