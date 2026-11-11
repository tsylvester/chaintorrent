# harness-crypto

The ChainTorrent cryptographic test harness. It composes the selected adapters per configured curve, emits test vectors and generated Solidity libraries, benchmarks the pairing concretes, measures the delivery statements against an on-chain verifier, and writes a JSON report.

The crate is a library plus one binary, `harness-crypto`. The process is a single pass: `config` builds the configuration from the command line and the shipped defaults, `run` executes the pipeline, `output` writes the outcome to every configured destination.

## Build

From the repository root:

```
cargo build -p harness-crypto
```

The debug binary lands at `target/debug/harness-crypto` (`target\debug\harness-crypto.exe` on Windows). For an optimized build:

```
cargo build --release -p harness-crypto
```

`cargo check -p harness-crypto` type-checks without producing a binary.

## Run

```
cargo run -p harness-crypto -- [OPTIONS]
```

or invoke the built binary directly:

```
target\debug\harness-crypto.exe [OPTIONS]
```

Arguments after `--` go to the harness. Without the separator, cargo interprets `--config` as its own dotted-key flag and the harness never sees it.

The binary has no subcommands. It exits `0` only when the run succeeded and every output destination was written; it exits `1` on a configuration refusal (written to standard error as `Debug` text) or on a refused run (written to the configured destinations as an `"outcome": "refused"` document).

## Configuration

Configuration is layered. The shipped defaults are compiled into the binary from `harness.toml` in this directory (`SHIPPED_DEFAULTS`), so the harness runs with no arguments. Two repeatable flags add layers on top of the defaults:

- `--config <PATH>` — read a TOML file and merge it over the defaults.
- `--set <KEY=VALUE>` — merge a single TOML assignment.

Both flags may be given any number of times and apply in command-line order: a later layer wins over an earlier one for the same key. TOML tables merge key by key; any other value (scalar, array) replaces the earlier value whole. `KEY=VALUE` values are TOML, so strings must be quoted:

```
cargo run -p harness-crypto -- --config contracts/evm/deployments/base-sepolia.toml --set "benchmark.iterations = 10" --set "report.primary_curve = \"bn254\""
```

Every table refuses a key it does not declare, and the first refused layer stops configuration.

Do not include any secret values in the committed `harness.toml`. 

Put secret-bearing values — a `verifier_endpoint` with an embedded API key, for example — in a `*.local.toml` file (ignored by git) and layer it with `--config`, e.g. `--config harness.local.toml`. The committed defaults stay usable alone; the local file supplies only the keys it names.

### Tables

- `[selections]` — the concrete each adapter family uses (random source, encoding, hash-to-scalar, chain forms, KEM, key agreement, delivery proof, verifier, identity scope, envelope algebra, statement version).
- `[[selections.curves]]` — one entry per measured curve: a label, the pairing concrete, and the target-group encoding.
- `[[selections.benchmark_concretes]]` — pairing concretes timed by the benchmark stage, with their target-group encodings.
- `[network_profile]` — `verifier_endpoint` (the chain's RPC URL), `chain_identifier`, `entitlement_contract`, `gas_price_oracle`, and `precompile_encodings`.
- `[network_profile.deployment_records]` — the deployed verifier address per curve label. The shipped records are the zero address.
- `[statements.mint]` and `[statements.transfer]` — the statement fields each delivery proof is generated and measured over.
- `[identities]` — the proof and mapping identity values.
- `[generate]` — the generate concrete (`Solidity`), `foundry_root`, `generated_directory`, compiler version, license, and per-curve library names and directories.
- `[generate.kinds.<KIND>]` — per vector kind: record count, output file name, and the Solidity struct, decode function, and path-constant names.
- `[benchmark]` — `iterations` per benchmark concrete.
- `[measurement]` — `iterations` per curve and a free-text `conditions` note carried into the report.
- `[report]` — report `destinations`, `piece_group_size`, `primary_curve`, and an optional `adjustment` overriding either for the report only.
- `[output]` — the outcome document's `destinations` (e.g. `{ kind = "StandardOutput" }` or `{ kind = "File", path = "…" }`).

## What a run does

1. **Config** — parses the command line, merges every layer over the shipped defaults, admits each selection and statement value, and builds one verifier concrete per configured curve from the endpoint, that curve's deployment record, and the gas price oracle.
2. **Benchmark** — creates an OS random source per `benchmark_concretes` entry and times that concrete's pairing operations over `benchmark.iterations`.
3. **Per curve, in order** — composes the pairing consumer: hash-to-scalar, encoding, chain forms, KEM, key agreement, delivery proof, then generate, emit, verify, measure. Vector files land beneath `generate.foundry_root`/`generated_directory`/`curve_directories.<label>`; the curve's report is written to `report.destinations` before the next curve starts.
4. **Output** — writes the outcome document to every `output.destinations` entry, in order.

The first refusal at any stage returns immediately; no later stage runs.

## On-chain dependency

Measurement calls the deployed verifier contract through `verifier_endpoint`. The shipped `deployment_records` are the zero address, so a run with no layer generates the first curve's vector files and then refuses:

```
"outcome": "refused"  …  NothingDeployed { address: [0, 0, …, 0] }
```

For an end-to-end success, supply real deployment records — either a deployments layer file (the `contracts/evm/deploy` step emits `contracts/evm/deployments/base-sepolia.toml`) passed via `--config`, or per-curve `--set` assignments such as:

```
--set 'network_profile.deployment_records.bn254 = { kind = "FixedBytes20", value = "0x…" }'
```

## Tests

Unit, interface, guard, and integration tests live beside each module:

```
cargo test -p harness-crypto --all-targets --all-features
```

The repository-wide check script `run-checks.ps1` runs `cargo check`, `cargo clippy`, `cargo fmt --check`, `cargo deny check`, and `cargo test` over the whole workspace and logs to `rust-checks.log`. The entries that need live deployed verifiers are end-to-end and run only after `contracts/evm/deploy` has produced the deployments layer — outside the ordinary test cycle.
