# Errors and Returns

A function returns a member of its `SuccessReturn | ErrorReturn` union, and every error is surfaced intact. This topic owns error **handling** and return **consumption**; [composition](composition.md) owns the union's shape.

Cited by: construction view (workplan node `interface` and `implementation` elements) and implementation view (interface prompt, implement prompt, unitTest prompt). Governed by all Process topics.

## Errors are typed

- Errors are explicitly typed. You know which errors a function can produce — do not guess the error kind with a ternary.
- Each error is a single, specific failure.
- Each error explains exactly what went wrong and exactly where.

## Errors are inspectable and comparable

An error states exactly what went wrong, so it can be printed and it can be compared by value. Every type in an error arm — the `ErrorReturn`, every flavor, and every value a variant carries — is printable and comparable. In Rust each derives `Debug`, `PartialEq`, and `Eq`.

- An error carries the facts that locate the failure — indexes, lengths, kinds, and a callee's error intact — and never a secret-typed value, so nothing in an error blocks printing it.
- A callee's error carried intact brings its derives with it. A callee error that lacks one is a defect in the callee's interface: report it, propose the node that adds the derives in its owning interface, and halt (see [discovery-halt](discovery-halt.md)). The carrying error never drops a derive to accommodate it.
- A vendor error that a concrete propagates unchanged is the one value the repo cannot derive on. The error type holding it omits only the trait the vendor type lacks, and that omission carries up through every error that holds it. No other omission exists.

## Errors are surfaced, never altered

- Every error is surfaced, every single time.
- Errors are never swallowed, never stored, never converted, coerced, or modified.
- You get an error, you pass the error along — unchanged. Surface it so it can be fixed; do not hide it or reshape it.

## Which error does a function return?

When a function hits a failure, classify it:

1. A dependency or callee already returned a **typed error** → propagate it unchanged. Never convert, coerce, or re-wrap it. In Rust, `?` propagates a callee's error only where this function's error arm carries it intact in a variant through `From`; a `map_err` that reshapes it is the forbidden conversion.
2. The failure is **this function's own** — a validation failure, a violated precondition → return a **new, specific typed error this function owns**, naming exactly what failed and where.

There is no third option. You never convert one error type into another, and you never invent an untyped or generic error to stand in for a specific one.

## A family's errors

A family's trait states its failures in the contract entries of its factory module's interaction spec (see [workplan-structure](workplan-structure.md#the-interaction-spec-carries-the-integration-elements)). Each failure a contract entry states is a flavor of that method's error union, declared in the factory module's interface. Every concrete that meets the failure, the mock concrete included, decides it from its inputs and returns that flavor, and the flavor names no concrete. A callee error the entry states surfacing is carried intact in the flavor.

A concrete's own variant in the union carries only what that concrete alone produces — its vendor's error, propagated intact — and appears only in the unions of the methods that produce it. The mock concrete's own variant carries only the errors of its failure modes that stand for a concrete's own error (see [mocks](mocks.md#families--the-mock-is-a-concrete-of-the-family-rust)).

### What reaches the failure decides where it is declared

- **An input every concrete can check.** The failure is the family's flavor, declared in the factory module's interface. Every concrete decides it from its inputs before its vendor runs, and the flavor names no concrete.
- **An input only one concrete's vendor detects.** The failure is that concrete's own error, propagated intact in that concrete's variant. The contract entry states it as each concrete's own, never as the family's.
- **No input at all, an outer-edge failure.** A concrete that can meet it states it as its own typed error. The mock concrete's own variant carries one only where a failure mode stands for it.
- **A conversion that cannot fail on a supported target.** A concrete's production code states it as its own typed error, because production code carries no `unwrap` or `expect` (see [types](types.md)). The mock concrete makes the same conversion with `expect` and carries no variant for it.

The mock concrete's own variant is never an arm an input reaches. Where the mock concrete refuses an input, the refusal is a family flavor and the contract entry states it for every concrete.

```rust
// interface.rs — the error union of one method of a family
#[derive(Debug, PartialEq, Eq)]
pub enum MyMethodErrorReturn {
    Truncated { index: usize },                           // the family's flavor, decided from inputs by every concrete
    MyVendor(MyVendorMyMethodErrorReturn),                // one concrete's own error, carried intact
    #[cfg(any(test, feature = "mocks"))]
    Mock(MockIMyAdapterMyMethodErrorReturn),              // the mock concrete's own error, failure modes only
}

#[cfg(any(test, feature = "mocks"))]
#[derive(Debug, PartialEq, Eq)]
pub enum MockIMyAdapterMyMethodErrorReturn {
    LengthUnrepresentable { length: usize },              // stands for a concrete's own error; no input reaches it
}
```

## The return is always the union

- The function returns a member of `SuccessReturn | ErrorReturn`. "This function only ever succeeds" is not the author's call — the error arm exists on purpose.
- The payload is the data the function operates on; the return is always the Success-or-Error union. The two are distinct and both are always present in the contract.
- Callers narrow the return by its discriminant and handle both arms. A caller that reads only the success arm has dropped an error path.

## Either arm may be a union of flavors

The top-level return always has **exactly two arms**: `SuccessReturn | ErrorReturn`. Never three, never one. But either arm may itself be a union of discrete members when that outcome has more than one discrete flavor. The flavors nest **inside** the arm; they are members of `SuccessReturn` (or `ErrorReturn`), never siblings of it.

```ts
MyFunctionReturn = MyFunctionSuccessReturn | MyFunctionErrorReturn   // always two arms

MyFunctionSuccessReturn =
  | ArtifactFoundReturn   // success: an existing artifact was returned
  | EnqueuedReturn        // success: no artifact found, a job was enqueued

MyFunctionErrorReturn =
  | ValidationError
  | EnqueueError
```

The Rust form: `Result` is the two arms, and each arm is a named enum of flavors.

```rust
pub type MyFunctionReturn = Result<MyFunctionSuccessReturn, MyFunctionErrorReturn>;   // always two arms

pub enum MyFunctionSuccessReturn {
    ArtifactFound(ArtifactFoundReturn),   // success: an existing artifact was returned
    Enqueued(EnqueuedReturn),             // success: no artifact found, a job was enqueued
}

pub enum MyFunctionErrorReturn {
    Validation(ValidationError),
    Enqueue(EnqueueError),
}
```

Both `ArtifactFoundReturn` and `EnqueuedReturn` are successes — the operation did what it should. They differ only in *which* successful outcome occurred, so they are members of `MyFunctionSuccessReturn`. (The enqueued case is one illustration; any success or error condition with several discrete outcomes takes the same shape.)

```ts
// FORBIDDEN — hoisting a success flavor to sit beside Success and Error
MyFunctionReturn = MyFunctionSuccessReturn | EnqueuedReturn | MyFunctionErrorReturn
```

`EnqueuedReturn` is a member of `MyFunctionSuccessReturn`, not a peer of it. Hoisting it breaks the two-arm invariant: a caller must be able to answer "did this succeed?" by discriminating the two arms **without** enumerating every flavor, then discriminate the flavor *within* the arm. A three-plus-arm top-level union forces every consumer to know every flavor just to decide success from failure.

Membership is transitive — `EnqueuedReturn` is assignable to `MyFunctionSuccessReturn` is assignable to `MyFunctionReturn` — and is proven by typed assignment in the interface test (see [tests](tests.md#interface)).

## Precedence

This topic outranks the workplan. A node step that returns only the success type, hoists a success or error flavor into the top-level union (making it more than two arms), swallows or rewrites an error, converts an error to a different type, declares an error type that cannot be printed or compared by value, or leaves an error arm unhandled is defective — comply with this topic and report the discrepancy.


