# Families

Every external touchpoint, every vendor, and every behavior with more than one possible implementation is an adapter family: a factory that owns the family's interface, concretes beneath it that implement that interface, and consumers that know only the interface. This topic owns the family rule for production code: what a family is and owns, how a concrete is selected, configured, admitted, and reached, and what nothing outside the family may name.

Cited by: construction view (workplan node `objective`, `deps`, `context_slice`, `interface`, `mock`, `implementation`, and `provides` elements of every factory, concrete, family-owned function, family consumer, configuration admission, and composition root) and implementation view (interface prompt, mock prompt, implement prompt, provide prompt), and review view (validateNode prompt). Governed by all Process topics.

The names in this topic's examples — `MyFamily`, `IMyAdapter`, `MyConcrete`, `OtherConcrete`, `OtherFamily`, `MyMethod`, `MyOutcome` — are placeholders for any family, concrete, method, and result of the same form. A case binds to this topic when it has the form, whatever its domain, vendor, language, or name.

## The rule

A caller reaches a concrete only through its family's factory, selected from configuration, admitted by its declaration, and received as the family's interface. Outside the concrete's own module and its family's modules, no module, document, or workplan node names a concrete, its types, its constants, its constructors, its settings, or its vocabulary.

A family exists wherever a concrete exists. A family with a single concrete is still a family; a further concrete is not the occasion that creates it.

The measure of the rule is what a further concrete changes. Adding a concrete is its own module; its variant in the family's identifier, in the selection, and in each error union its own errors reach; its entry in the family's declared set; and its branch in the factory, the selection admission, the declaration lookup, and the declared-selection builders. These are the only locations that name a concrete, and nothing outside the family changes. A node states this measure as a non-functional constraint in its `objective`, and a review tests a node by asking what else a further concrete would change.

## What a family owns

A family is a module holding its factory module, its concretes, and its family-owned functions.

- **The factory module** owns the family's interface (the trait in Rust, the interface type in TypeScript), the family's declaration type, the identifier type naming each member, the selection, the declared set, each method's error union, the family's configuration that every concrete takes (see [composition](composition.md#configuration-follows-ownership-rust)), the consumer surface through which a caller receives a concrete, the family's description types, and the family's mock (see [mocks](mocks.md#families--the-mock-is-a-concrete-of-the-family-rust), [mocks](mocks.md#families--the-typescript-form)). Its one function is the factory.
- **A concrete** owns its implementation, its own settings type, its settings-document type and the guard that narrows to it, its declaration constant, its own error type, and the admission of its own settings. It is a child module of the family.
- **A family-owned function** is a function the family provides beside its factory: the selection admission, the declaration lookup, the composition admission, and any other function whose subject is the family rather than one concrete. Each is its own module and its own node under the family module, in the four-slot form (see [composition](composition.md#the-signature)).

The Rust form of the factory module's family-level types:

```rust
// my_family/factory/interface.rs
pub enum MyFamilyIdentifier {
    MyConcrete,
    OtherConcrete,
}

pub enum MyFamilySelection {
    MyConcrete { settings: MyConcreteSettings },       // the concrete's own settings type, declared in its interface
    OtherConcrete { settings: OtherConcreteSettings },
    #[cfg(any(test, feature = "mocks"))]
    Mock { failure_mode: MockIMyAdapterFailureMode },
}

pub struct MyFamilyDeclaration {
    pub identifier: MyFamilyIdentifier,
    pub name: &'static str,                            // the name configuration uses and the settings table is keyed by
    pub requires: MyFamilyRequirements,
}

pub const DECLARED_MY_FAMILY_IDENTIFIERS: &[MyFamilyIdentifier] = &[
    MyFamilyIdentifier::MyConcrete,
    MyFamilyIdentifier::OtherConcrete,
];
```

The TypeScript form, each union a named type declared in the owning interface (see [types](types.md#inline-unions-are-inline-type-definitions--always-prohibited)):

```ts
// myFamily/factory/factory.interface.ts
export type MyFamilyIdentifier = "myConcrete" | "otherConcrete";

export interface MyConcreteSelection {
  identifier: "myConcrete";
  settings: MyConcreteSettings;            // declared in the concrete's interface
}

export interface OtherConcreteSelection {
  identifier: "otherConcrete";
  settings: OtherConcreteSettings;
}

export type MyFamilySelection = MyConcreteSelection | OtherConcreteSelection;

export interface MyFamilyDeclaration {
  identifier: MyFamilyIdentifier;
  name: string;
  requires: MyFamilyRequirements;
}

export const DECLARED_MY_FAMILY_IDENTIFIERS: ReadonlyArray<MyFamilyIdentifier> = ["myConcrete", "otherConcrete"];
```

A family's mock and its absence from the declared set are owned by [mocks](mocks.md#families--the-mock-is-a-concrete-of-the-family-rust): in Rust the mock concrete is a member of the selection, and in TypeScript the family's interface is mocked as an injected interface and the selection carries no mock member (see [mocks](mocks.md#families--the-typescript-form)).

## The declaration

A concrete declares itself in enough detail that every caller routes from the declaration and never from knowledge of the concrete. Its declaration is a constant of the family's declaration type, held in the concrete's interface and reached through the family's declaration lookup. It states:

- **Its identity.** The identifier and the name configuration uses for it. Every name that reaches a document, a report, a path, or a log comes from a declaration.
- **What it provides.** Every capability, form, version, or value a caller would otherwise know from the concrete's name, so that the caller reads it.
- **What it requires.** For every family it is composed with, the identifiers of that family's members, or the declared values of those members, it can work with. A concrete that works with any member of a family states no requirement of it.
- **Its settings.** For each field of its own settings type, whether the concrete fixes it or permits it to be set, the bounds within which it may be set, and its default. A setting is never a literal outside the declaration.

## Selecting a concrete from configuration

Configuration names a member; the family turns that name and the member's settings into the selection. The family's **selection admission** is a family-owned function in the validating form (see [composition](composition.md#parameter-jurisdiction--what-is-trusted-vs-proven)): its params carry the identifier admitted from configuration and nothing else, its payload is the untrusted settings document, and it returns the settings-carrying selection.

It is an exhaustive branch on the identifier. Each branch narrows the document on entry to that concrete's settings-document type through the guard the concrete owns (see [guards](guards.md#guard-on-entry--validating-functions-take-unknown)), then calls the concrete's settings admission in the trusted form with the narrowed document, building that admission's deps and params from what the family admission received.

Each callee's error is carried intact in a variant named by the concrete (see [errors-and-returns](errors-and-returns.md#naming-a-variant-that-carries-a-callees-error)). No concrete's own value is carried in the family admission's params or deps (see [composition](composition.md#configuration-follows-ownership-rust)).

It is the production counterpart of the declared-selection builders (see [mocks](mocks.md#families--the-mock-is-a-concrete-of-the-family-rust), [mocks](mocks.md#families--the-typescript-form)), and it is one of the locations the measure lists (see [The rule](#the-rule)).

```rust
// my_family/admit_my_family_selection/mod.rs
pub fn admit_my_family_selection(
    _deps: &AdmitMyFamilySelectionDeps,
    params: AdmitMyFamilySelectionParams,
    payload: serde_json::Value,
) -> AdmitMyFamilySelectionReturn {
    match params.identifier {
        MyFamilyIdentifier::MyConcrete => {
            let document = match MyConcreteSettingsDocument::try_from(payload) {
                Ok(document) => document,
                Err(error) => return Err(AdmitMyFamilySelectionErrorReturn::MyConcreteDocument(error)),
            };
            let admitted = admit_my_concrete_settings(&AdmitMyConcreteSettingsDeps {}, AdmitMyConcreteSettingsParams {}, document)?;
            Ok(AdmitMyFamilySelectionSuccessReturn { selection: MyFamilySelection::MyConcrete { settings: admitted.settings } })
        }
        MyFamilyIdentifier::OtherConcrete => { … }
    }
}
```

```ts
// myFamily/admitMyFamilySelection/admitMyFamilySelection.ts
export const admitMyFamilySelection: AdmitMyFamilySelectionFn = (deps, params, payload: unknown) => {
  switch (params.identifier) {
    case "myConcrete": {
      if (!isMyConcreteSettingsDocument(payload)) return { ok: false, error: { kind: "myConcreteDocument" } };
      const admitted = admitMyConcreteSettings({}, {}, payload);   // trusted form, narrowed document
      if (!admitted.ok) return { ok: false, error: { kind: "myConcrete", error: admitted.error } };
      return { ok: true, selection: { identifier: "myConcrete", settings: admitted.settings } };
    }
    case "otherConcrete": { … }
  }
};
```

A concrete's settings-document type is its settings type with every field optional; the concrete's guard narrows to it, and the concrete's settings admission takes it in the trusted form. The settings admission reads its declaration. It refuses a document value for a setting the declaration fixes, refuses a settable value outside its declared bounds, and takes the declared default where the document supplies none. Each refusal is the concrete's own typed error, naming the setting.

### Where a concrete's settings live in a document

A concrete's settings sit in a table the concrete owns, keyed by the name its declaration supplies and nested beneath the family's table. The configuration layers declare the family's table alone; the concrete's settings admission owns every key beneath its own table. A further concrete adds a table and no key to any shared table, and no configuration function, layer, or composition root names a concrete's setting.

### A list of members defaults to the declared set

Where configuration selects a list of members, an absent list is the family's declared set, and a present list is admitted name by name against the declarations' names and narrows the set. A name no declaration supplies is the admission's own error. The admission reads the names from the declarations; it never holds them as literals.

### Configuration naming a member

A configuration file is configuration: it names a member by the name the member's declaration supplies, and that naming conforms. A node that authors a configuration file states, for each value, the declaration or default it comes from.

## Reading a declaration before construction

A value a member's declaration fixes is known before the member is constructed. The family provides a **declaration lookup**, a family-owned function from an identifier to its member's declaration, so a level that holds an identifier reads what the member declares without constructing it and hands it inward (see [composition](composition.md#nested-calls-build-their-inputs-from-what-they-receive-rust)). A value only a constructed concrete produces is read where the concrete is received: in Rust inside the family's consumer method (see [composition](composition.md#a-return-that-carries-the-callers-type-rust)), in TypeScript from the adapter the factory returns. A caller never restates a member's declared value from its own knowledge of the member.

## Compatibility is admitted before construction and before effort

Whether members of different families can be composed is determinate from their declarations. The family provides a **composition admission**, a family-owned function whose params carry the selection's identifier and whose payload carries the declarations of the members it is to be composed with; it compares what the selected member's declaration requires with what each other member declares, and returns the family's `Incompatible` flavor, naming the identifiers and declared values that disagree and no concrete, where they do not match. The factory calls it before constructing anything.

A composition whose members perform side effects — writing files, sending requests, spending resources — calls the composition admission of every member, through the declaration lookups, before its first side effect. A refusal never follows work the refused composition caused. A missing runtime input is not an incompatibility: it is the failure of the step that reads it.

## A family's surface carries no concrete's model

The family's interface, its method names, params, payloads, success flavors, and error flavors use the family's vocabulary: the capability every member provides, named for what it does, never for how one member does it. A field, a flavor, or a word only one member's model has does not appear on the family's surface.

- **A failure.** Where the failure is decided and where it is declared is owned by [errors-and-returns](errors-and-returns.md#a-familys-errors). A concrete's own variant is the one place a family's error union names a concrete; a consumer carries the family's union intact and never branches on a concrete's variant.
- **A result only one member's model shapes.** A result whose shape differs per member crosses the consumer chain as the family's **description**: a type the factory module declares, whose entries carry names and units from the producing member's declaration and values of the family's own types. Consumers carry, compare, and render the description without naming any entry. In Rust, the family's trait declares the member's own result as an associated type and a method that describes it, and the consumer method produces the description where the member's types are in scope, since a consumer chain's return cannot be generic in the member. In TypeScript, the interface method returns the description, and the member's own result type never leaves its module.

```rust
// my_family/factory/interface.rs
pub trait IMyAdapter {
    type MyOutcome;

    fn my_method(&self, params: MyMethodParams, payload: MyMethodPayload) -> MyMethodReturn<Self::MyOutcome>;
    fn describe_my_outcome(&self, params: DescribeMyOutcomeParams, payload: DescribeMyOutcomePayload<'_, Self::MyOutcome>) -> DescribeMyOutcomeReturn;
}

pub struct MyOutcomeDescription {
    pub entries: Vec<MyOutcomeEntry>,
}

pub struct MyOutcomeEntry {
    pub name: &'static str,     // from the producing member's declaration
    pub unit: &'static str,     // from the producing member's declaration
    pub quantity: MyQuantity,   // a family type
}
```

```ts
// myFamily/factory/factory.interface.ts
export interface IMyAdapter {
  myMethod: IMyAdapterMyMethodFn;   // returns MyOutcomeDescription in its success arm
}

export interface MyOutcomeEntry {
  name: string;
  unit: string;
  quantity: MyQuantity;
}

export interface MyOutcomeDescription {
  entries: ReadonlyArray<MyOutcomeEntry>;
}
```

## Concretes are unreachable outside their family

A family module exposes the family's surface and never a concrete's.

The Rust form. The family module's `mod.rs` declares each concrete module private and its `provides` re-exports the factory module's `provides` and each family-owned function's `provides`, never a concrete's. The factory and the family-owned functions reach a concrete through the family module's private path, which no module outside the family can name. This holds identically for a family that is its own crate, where `lib.rs` re-exports the family's `provides`, and for a family that is a module of a larger crate, where the crate boundary does not hide the concrete and the module's privacy does.

```rust
// my_family/mod.rs
mod admit_my_family_selection;
mod factory;
mod my_concrete;
mod other_concrete;
pub mod provides;

// my_family/provides.rs
pub use super::admit_my_family_selection::provides::*;
pub use super::factory::provides::*;
```

The TypeScript form. The family folder's `provides.ts` re-exports the factory's and each family-owned function's `provides` and never a concrete's. A concrete's `provides` is imported only by the factory and the family-owned functions in its own family folder. TypeScript gives no compiler boundary here, so the import is the boundary: a module outside the family folder that imports from a concrete's folder is defective.

```ts
// myFamily/myFamily.provides.ts
export * from "./factory/factory.provides.ts";
export * from "./admitMyFamilySelection/admitMyFamilySelection.provides.ts";
```

## Counterparts in another language or runtime

A concrete whose behavior includes a component outside the codebase's language — a program deployed to another runtime, a schema a remote service holds, source a generator emits — owns that component under the concrete's declared identity. The component's location, its build and deployment outputs, and every record of where it is deployed are keyed by that identity, so a further concrete's component is placed beside it and never over it. A generator concrete declares the identity it emits for, and a client concrete declares the identity it requires; the composition admission compares them. The component's own language and tooling are that concrete's vocabulary, named in its own nodes and nowhere else.

## Tests

A test reaches a family's members as production code does, names no concrete but its own subject, and runs over the declared set; how is owned by [mocks](mocks.md#families--the-mock-is-a-concrete-of-the-family-rust), [mocks](mocks.md#families--the-typescript-form), and [integrationTest](integrationTest.md#configuration-over-the-declared-set-rust).

## A further concrete extends its family

A family location is complete when it covers every concrete that exists. Each concrete's node is followed, in dependency order, by an increment node for each family location the measure lists, which adds that concrete to the location. A family location never names a concrete before that concrete's node, and is never left without the increment that adds it. Every file other than a family location is written once, complete, in its one node.

## Ownership at the first consumer

A family, a family-owned function, and every value a family owns are [authored once, complete](#a-further-concrete-extends-its-family), in the node for their first consumer (see [workplan-structure](workplan-structure.md#node-constructions--wrong-vs-right)). Where that consumer precedes the family's concretes in the work breakdown, the family's nodes are inserted ahead of it. No module holds a family's value for a later node to take over.

## Canary

A node's `deps` element states each family the node consumes by the family's name and the surface it consumes it through: the factory, the consumer surface, the declaration lookup, the selection admission, or the composition admission. A concrete's type, constant, constructor, or settings name appearing in any node, element, or file outside that concrete's family trips the canary, and the node or file is discarded (see [traceability](traceability.md#canaries)).

## Halt triggers

Each of these is a discovery: report, propose the node, and halt (see [discovery-halt](discovery-halt.md)).

- The work needs to name a concrete, its types, its constants, its constructors, or its settings outside its family.
- The family's surface needs a field, flavor, or word only one member's model has.
- A declaration lacks what a caller would otherwise know from a concrete's name, or lacks a requirement a composition admission must compare.
- A side effect of a composition precedes the admission of one of its members.
- A family's value has no owner at the node of its first consumer.

## Precedence

This topic outranks the workplan. A node step that names a concrete outside its family, names a concrete outside its family in place of a family location's increment, builds a selection outside the family's selection admission, carries a concrete's settings in a shared table or as a literal outside its declaration, restates a member's declared value from knowledge of the member, takes a value a member's declaration fixes from configuration, puts a member's own model on the family's surface, branches on a concrete's error variant outside the family, constructs a member or causes a side effect before the composition admits it, or re-exports a concrete beyond its family is defective — comply with this topic and report the discrepancy.
