# Post-quantum credential KEM: the two derivations

The derivations behind candidate C12 of the [research notebook](cryptography-research-notebook.md), written out so a reader can check them. The notebook holds their provenance and the [critical path](cryptography-critical-path.md) their current standing; this file holds the mathematics.

## Standing of this document

Both derivations are one maintainer's, at sketch level. No source states either result, no proof here is complete, no parameter set is chosen, and neither has been independently re-derived. Each step that leans on a source names it by its evidence ID in the notebook; each step that is ours says so. The unchecked steps are listed at the end of each derivation.

The question they answer is the notebook's Q12: does a credential KEM exist that is secure against a quantum adversary and still meets the credential interface of [cryptography.md](cryptography.md), namely constant-size credentials across any number of sales, a capsule whose cost does not depend on the number of entitlements, a public validity check, authorship at sale without the master secret, non-convertibility under the entitlement scope, and holder authorship under the asset scope.

## Common setting

Integers `n`, `m`, a prime modulus `q`. A uniformly random matrix `A` over `Z_q` with `n` rows and `m` columns. The kernel lattice of `A` is `K(A) = { z in Z^m : A·z = 0 mod q }`. A short basis of `K(A)` is the master secret; with it the issuer samples short vectors in any coset of `K(A)` from a discrete Gaussian (E56).

The base scheme is the dual of Regev's encryption (E56), proved secure with a quantum-accessible identity oracle in E57:

- A **target** is a vector `u` in `Z_q^n`.
- A **credential** for `u` is a short vector `e` with `A·e = u mod q`.
- A **capsule** for `u` encapsulating a message `mu` is `(c0, c1) = (A^T·s + x, u^T·s + x' + encode(mu))` for a uniform `s` and short noise `x`, `x'`.
- **Decapsulation** computes `c1 − e^T·c0 = encode(mu) + (x' − e^T·x)`, and the bracket is small because `e`, `x` and `x'` are short.

Two facts are read straight off this shape and both derivations use them. Every short preimage of `u` recovers the same `mu` from the same capsule, so one capsule serves every holder. And whether a vector is a credential is checkable from public values: `A·e = u` and a norm bound.

## Derivation one: the asset scope

### Construction

One target `u` per parameter set, as the asset scope fixes one identity per parameter set.

- **Issuance.** The issuer samples a short preimage of `u` for each minted entitlement. This is the scheme of E55 read as a credential KEM: each user holds a distinct short vector in one shared lattice and one common ciphertext serves them all.
- **Published hints.** At setup the issuer samples short preimages `x_0, x_1, …, x_k` of `u`, publishes the differences `z_i = x_i − x_0` for `i = 1 … k` as part of the parameter set, and does not publish `x_0`. Each `z_i` is a short vector of `K(A)`. Write `L(Z)` for the lattice they generate.
- **Authorship at sale.** A holder of a credential `e` samples `e'` from a discrete Gaussian over the coset `e + L(Z)`, at a width fixed by the `z_i` alone, and delivers `e'`.

### What authorship at sale achieves

`e'` is a short preimage of `u`, since it differs from `e` by a kernel vector. The Gaussian over a coset depends on the coset and not on the representative the holder started from (E56's sampler), so the width never grows: after any number of sales a credential is one short vector of the same norm bound. No master secret is used. This is the property C11 has through rerandomization along the public direction `(F_I, g2)`; here the public directions are the `z_i`.

Two limits against C11, both ours. A rerandomized credential is fresh only inside its coset of the rank-`k` lattice `L(Z)`, so its entropy is bounded by `k` and the width. And two credentials derived from one another are publicly linkable, because their difference lies in `L(Z)`; C11's tuples of one identity are likewise linkable through the pairing.

### The claim

A party that holds `A`, `u` and `z_1 … z_k`, and no preimage of `u`, cannot distinguish the message of a capsule.

### The problem it reduces to

E55's Definition 15, with its centre matrix set to zero. Given `A`, `u` and `k'` short preimages of the target as hints, distinguish a noisy LWE sample under the augmented matrix, the row `u` stacked on `A`, from a noisy uniform point of the orthogonal complement of the span of the augmented hints `(1 ∥ x_i)`. The complement replaces the full space because each hint does decrypt: the inner product of an augmented hint with a real sample is small. E55's Theorem 18 reduces LWE to this problem, with example parameters `k' = m/10`, `σ = Θ(n)`, `σ′ = Θ(n^5)`, `q = Θ(n^5)`, `m = Θ(n log n)`.

Note the orientation: E55 writes its preimages for the target's negation and its matrices transposed relative to this file. Nothing below depends on the convention.

### Sketch of the argument

Write `W_X` for the orthogonal complement of the span of all `k + 1` augmented hints `(1 ∥ x_0), …, (1 ∥ x_k)`, and `W_Z` for the orthogonal complement of the span of the augmented differences `(0 ∥ z_1), …, (0 ∥ z_k)`. Since the differences lie in the span of the hints, `W_X` is contained in `W_Z`, and `W_X` is `W_Z` cut by the single further equation that `(1 ∥ x_0)` imposes.

**Step 1, computational.** Take the E55 problem with `k' = k + 1` hints. Its hardness says a capsule is indistinguishable from a noisy uniform point of `W_X`, even to a party that holds `x_0 … x_k`. A reduction that holds the hints computes the differences and gives the adversary `A`, `u` and the `z_i`, which is less than the reduction holds. So to the adversary of the claim, a capsule is indistinguishable from a noisy uniform point of `W_X` with `encode(mu)` added to its first coordinate.

**Step 2, statistical.** Write a point as `(y_0 ∥ y)`. Membership of `W_Z` constrains only `y`: it requires `z_i^T·y = 0` for every `i`, and leaves `y_0` free. Membership of `W_X` adds `y_0 = −x_0^T·y`. So a uniform point of `W_X` is a uniform `y` of the constrained set together with `y_0 = −x_0^T·y`, and a uniform point of `W_Z` is the same `y` together with an independent uniform `y_0`.

The adversary does not hold `x_0`. For a fixed `x_0`, the map `y ↦ x_0^T·y` over the constrained set depends on `x_0` only modulo the span of the `z_i`, and for two inputs that differ outside that span the outputs collide with probability `1/q` over a uniform `y`. That is a universal hash family keyed by `y`. By the leftover hash lemma, if `x_0` retains enough min-entropy modulo the span of the `z_i`, conditioned on everything the adversary sees, then `x_0^T·y` is statistically close to uniform given `y`, and the two distributions of Step 2 are statistically close.

**Step 3.** In a noisy uniform point of `W_Z` the first coordinate is uniform and independent of the rest, so adding `encode(mu)` to it leaves a distribution that does not depend on `mu`.

### The entropy condition

Step 2 is the lemma's price. The adversary sees `A`, the syndrome `u = A·x_0`, and the differences. Conditioned on the differences, `x_0` is distributed as a Gaussian over its coset of `K(A)` narrowed by a factor of about `√(k+1)`, because each difference is a noisy observation of `−x_0`. Its entropy is then about `m·log(σ/√(k+1)) − n·log q`. Reducing modulo the span of the `z_i` costs at most about `k·log q` bits. The lemma needs what remains to exceed `log q` plus a statistical margin:

`m·log(σ/√(k+1)) ≥ (n + k + 1)·log q + margin`

and it needs the narrowed width `σ/√(k+1)` to stay above the smoothing parameter of `K(A)`, so that the narrowed Gaussian is well behaved.

On E55's example parameters with `k = m/10` this is borderline by our arithmetic: the left side is about `(m/2)·log n` and the right side about `(m/2)·log n + 5n·log n`, before counting the wider coordinates E55's hint covariance provides. So `k` is a smaller constant fraction of `m`, or the width is larger. The condition is a parameter constraint, not an obstruction.

### What follows

Under the asset scope the construction meets the interface at sketch level: constant-size credential across any number of sales, capsule cost independent of the entitlement count, public credential validity, authorship at sale without the master secret, and holder authorship. Non-holder confidentiality rests on E55's hardness result and the entropy condition. Non-convertibility is not a property the asset scope has.

A coalition of holders does accumulate kernel vectors beyond `L(Z)` when the issuer samples each credential freshly, and about `m` of them recover a short basis of `K(A)`. Under the asset scope this is not a loss: any single holder is already entitled to author credentials.

### Unchecked in derivation one

- The exact conditional distribution of `x_0` given the differences, and the entropy it retains.
- The form of the orthogonal complement over the torus E55 works in, and that Step 2's hash argument carries through its noise.
- That the joint law of the differences does not depend on which target the preimages were drawn for, which is needed if the hints are generated once and reused.
- Whether E55's Theorem 18 at `k + 1` hints with zero centres is exactly the statement Step 1 needs; E55 proves its reduction for unit-vector centres and gives a separate reduction to zero centres for small centres.
- Any parameter set, and any extension to structured lattices.
- A capsule well-formedness check. An LWE capsule is not publicly checkable, and E55 records as open a chosen-ciphertext-secure lattice scheme in which independent keys serve one public key.

## Derivation two: the entitlement scope

### What the entitlement scope requires

Each entitlement has its own cryptographic identity `I`, a leaked credential attributes to its entitlement, and a credential for one entitlement cannot be converted into one for another. At the same time one common capsule must be opened by every entitlement of the asset. In C11a this is the all-wildcard pattern: the capsule `(U, V, W)` names no identity, and a holder combines `V` and `W` with its own `I`.

### The lattice form of the wildcard pattern

In the shape of the standard identity-matrix lattice constructions, take public matrices `A`, `B_0`, `B_1` and a target `u`, and let the identity `I` be small, so that multiplying by it does not destroy shortness.

- A **credential** for `I` is a short pair `(e1, e2)` with `A·e1 + (B_0 + I·B_1)·e2 = u`.
- A **capsule** carries LWE samples under each matrix separately: `c0 = A^T·s + x0`, `cB0 = B_0^T·s + x1`, `cB1 = B_1^T·s + x2`, and `c1 = u^T·s + x3 + encode(mu)`.
- **Decapsulation** by the holder of `I` computes `c1 − e1^T·c0 − e2^T·(cB0 + I·cB1)`.

The capsule must carry `cB0` and `cB1` separately because it cannot name `I`; that is what makes it a wildcard.

### The observation

Rewrite the credential equation as `A·e1 + B_0·e2 + B_1·(I·e2) = u`. So `(e1, e2, I·e2)` is a short preimage of the one target `u` under the one wide matrix `M = [A | B_0 | B_1]`, for every entitlement of the asset, whatever its identity. And decapsulation is the inner product of that long vector with `(c0, cB0, cB1)`.

Three consequences.

**Unattributable material exists.** Any short preimage `(f1, f2, f3)` of `u` under `M` opens every capsule, by the same inner product, whether or not `f3` is a multiple of `f2`. Such a vector passes no credential validity check for any identity and opens capsules all the same.

**Coalitions reach it.** Two credentials for different entitlements give two short preimages of `u` under `M`, and their difference is a short vector of the kernel lattice of `M`. A coalition of about as many holders as that lattice has dimensions holds a full-rank set of short kernel vectors, converts it to a short basis by the standard procedure, and can then sample short preimages of `u` under `M` at will. So master hiding and attribution hold only against coalitions below about the dimension of the wide lattice.

**Rerandomization hints make it worse.** Hints that let the holder of `I` rerandomize are short kernel vectors of `[A | B_0 + I·B_1]`, and each lifts to a short kernel vector of `M` in the same way. Published per entitlement, they accumulate across all entitlements and bound the total number that may ever be minted before the public material alone yields a short basis, which contradicts unbounded issuance. Held privately and passed along each ownership chain, they accumulate across a coalition and lower the bound of the previous paragraph by a factor of the hints per entitlement.

### Why C11a does not have this limit

In C11a a credential is `(g1^alpha · F_I^r, g2^r)`. The credentials of different entitlements also satisfy linear relations, but in the exponents, with coefficients that are discrete logarithms nobody knows; a coalition cannot do the linear algebra, and recovering the master element is a co-CDH instance for every coalition size. In the lattice scheme the linear relations hold over the integers in the clear. The only barrier is shortness, and enough samples defeat it.

### Structured lattices sharpen the limit

Over the ring-structured lattices that give kilobyte keys and capsules (E58), a short vector multiplied by the ring's monomials yields as many independent short vectors as the ring has dimensions. One credential difference then contributes a whole block of kernel vectors, and the coalition needed falls from about the integer dimension to about the module's rank, which is a handful of holders. Compactness and coalition resistance trade directly against each other in this family.

### What follows

Under the entitlement scope, non-convertibility and master hiding against every coalition are not available in this family as derived. A bounded-coalition form is available, with the bound set by the wide lattice's dimension, as in E55's own bound of `m/c` traitors. This agrees with the reason E55 gives in its open problems: each user owns part of one shared secret.

What would bend to carry the entitlement scope across is its attribution: either to a published coalition bound, or by using the asset scope for every suite. The notebook records non-convertibility as a stronger and non-mandatory property (S15), and the choice is held for its Q16.

### Unchecked in derivation two

- That every lattice realization of a wildcard capsule has this form. The argument is about schemes whose decapsulation is linear in the credential and whose credentials for different identities share one linear system; a construction outside that description is not covered.
- The exact coalition size, which depends on the quality of the basis a coalition obtains and on the parameters.
- Whether a short basis of the wide kernel lattice also lets a coalition forge a credential of valid form for a chosen identity, which is a stronger break than unattributable material. Not established either way.
- The structured-lattice statement, which is argued from the monomial action and not from a specific scheme's analysis.

## Sources

By evidence ID in the notebook's register, each read at the passages named there and none in full.

- E55, Ling, Phan, Stehlé, Steinfeld, Hardness of k-LWE and Applications in Traitor Tracing: Definition 15, Theorem 18, the introduction and open problems.
- E56, Gentry, Peikert, Vaikuntanathan, Trapdoors for Hard Lattices and New Cryptographic Constructions: the introduction.
- E57, Zhandry, Secure Identity-Based Encryption in the Quantum Random Oracle Model: Theorem 5.1 at statement level.
- E58, Ducas, Lyubashevsky, Prest, Efficient Identity-Based Encryption over NTRU Lattices: abstract.
- E59, Cash, Hofheinz, Kiltz, Peikert, Bonsai Trees, or How to Delegate a Lattice Basis: section 3.4 and Lemma 3.3, for the excluded delegation route.
