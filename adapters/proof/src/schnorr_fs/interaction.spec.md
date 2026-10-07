# `schnorr_fs` — interaction spec

Branch contract for the `SchnorrFsDeliveryProof` adapter of the `proof` crate: the generalized Fiat–Shamir Schnorr delivery proof over any supported pairing concrete, in both verifier forms `BothGroups` and `FirstGroupOnly`. A mint proof proves knowledge of `(α, r, ρ, σ)` for the credential and envelope equations; a transfer proof proves knowledge of `(x_s, y_s, s, ρ, σ)` from the seller's fresh decryption and the total offset. Each branch states condition, decision, dependency call, and the exact return outcome.

The adapter borrows its collaborators (`pairing`, `hash_to_scalar`, `encoder`, `random`), so the caller that resolved the pairing keeps it for the credential KEM and the key agreement, which compute in the same groups.

The trusted form: the prover payloads carry typed `AlgebraicMintStatement` / `AlgebraicTransferStatement` values whose `DeliveryContext<F, MintPurpose>` / `DeliveryContext<F, TransferPurpose>` guarantees the purpose relation, and the proofs carry fixed named response and first-message sets, so no entry guard exists and no trusted component contains a shape-ambiguous vector. Vectors exist only in `DeliveryProofWireComponents`, the untrusted decoding boundary.

## Shared steps

The steps each method names once:

- **Nonce.** `self.random.fill_bytes(FillBytesParams, FillBytesPayload { length: <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH })`; an `Err(error)` returns in the prover's `NonceDraw` variant unchanged. Then `P::Scalar::sample_from_uniform_bytes(SampleUniformScalarParams, SampleUniformScalarPayload { uniform })` with the drawn bytes moved in; an `Err(error)` returns in the prover's `NonceSampling` variant unchanged. The drawn length is the sampling bound's, so the sampling refuses no drawn bytes, and neither refusal has a unit test, the operating system's generator being the outer edge.
- **Response.** `add_scalar(k, mul_scalar(c, w))` for a nonce `k`, the challenge `c`, and a witness `w`, each operand a clone of the held or exposed value.
- **Transcript.** Build `MintStatement<P, F>` or `TransferStatement<P, F>` from context clones and typed key, envelope, and named first-message components; only their descriptions flatten group elements to precompile byte strings.
- **Challenge.** `challenge(&ChallengeDeps { pairing: self.pairing, encoder: self.encoder, hash_to_scalar: self.hash_to_scalar, tag: &self.challenge_tag }, ChallengeParams, ChallengePayload { transcript: ChallengeTranscript::Mint(&transcript) })`, or `ChallengeTranscript::Transfer(&transcript)`; an `Err(error)` returns in the method's `Challenge` variant unchanged. The encoding refusal propagates through `ChallengeErrorReturn::Encoding` and has a rejecting-encoder test.
- **Scalar equality.** Two scalars are equal when the exposed bytes of their `encode_scalar` encodings are equal.

## `SchnorrFsDeliveryProof::try_new(params: SchnorrFsDeliveryProofConstructorParams<'_, P, E>) -> SchnorrFsDeliveryProofTryNewReturn<'_, P, E, F>`

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| challenge tag refused | `DomainTag::try_new(SCHNORR_FS_CHALLENGE_TAG)` returns `Err(error)` | none | `DomainTag::try_new(SCHNORR_FS_CHALLENGE_TAG)`, once | `Err(SchnorrFsDeliveryProofTryNewErrorReturn::ChallengeTag(error))`, the refusal unchanged |
| weight tag refused | the challenge tag admits and `DomainTag::try_new(SCHNORR_FS_WEIGHT_TAG)` returns `Err(error)` | none | `DomainTag::try_new(SCHNORR_FS_WEIGHT_TAG)`, once, after the challenge tag | `Err(SchnorrFsDeliveryProofTryNewErrorReturn::WeightTag(error))`, the refusal unchanged |
| admitted | both tags admit | none | as above | `Ok(SchnorrFsDeliveryProof { pairing, hash_to_scalar, encoder, random, verifier_form, challenge_tag, weight_tag, forms: PhantomData })`, holding the params' borrows and form |

## `SchnorrFsDeliveryProof::<'_, P, E, F>::DECLARATION`

The inherent constant

```
DeliveryProofDeclaration {
    algebras: &[EnvelopeAlgebra::PairingElGamal],
    verifier_forms: &[VerifierGroupArithmetic::BothGroups, VerifierGroupArithmetic::FirstGroupOnly],
    statement_versions: &[DELIVERY_STATEMENT_VERSION_ONE],
    challenge_tag: SCHNORR_FS_CHALLENGE_TAG,
    weight_tag: SCHNORR_FS_WEIGHT_TAG,
    adapter_version: 1,
    interface_version: DELIVERY_PROOF_INTERFACE_VERSION,
}
```

its tags the constants `challenge` and `try_new` construct their `DomainTag`s from; the generate family mirrors `weight_tag` to Solidity from this declaration.

## `prove_mint(params: ProveMintParams, payload: ProveMintPayload<'_, F, P::Scalar, P::G1, P::G2>) -> ProveMintReturn<SchnorrFsProof<P>>`

`payload.statement.context` is `DeliveryContext<F, MintPurpose>`; the purpose relation is guaranteed by the type, with no runtime relation refusal.

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| nonce refused | the nonce step for `k_α`, `k_r`, `k_ρ`, then `k_σ` refuses | none | the nonce step per witness, in order | `Err(ProveMintErrorReturn::SchnorrFs(…))` carrying that refusal |
| proved | all nonces drawn | none | with `g1` and `g2` the generators, `F` the statement's `identity_element`, and `pk1`, `pk2` its buyer keys: `T_hpub = mul_g2(g2, k_α)`; `T_C1 = mul_g1(g1, k_ρ)`; `T_C2 = msm_g1` over `(g1, k_α)`, `(F, k_r)`, `(pk1, k_ρ)`; `T_D1 = mul_g2(g2, k_σ)`; `T_D2 = msm_g2` over `(g2, k_r)`, `(pk2, k_σ)`; the transcript step over the mint transcript with first messages `T_hpub`, `T_C1`, `T_C2`, `T_D1`, `T_D2`; the challenge step yielding `c`; the response step for `(k_α, α)`, `(k_r, r)`, `(k_ρ, ρ)`, `(k_σ, σ)`, `α` from `master_scalar.value`, `r` from `credential_randomness`, and `ρ`, `σ` from `coins` | `Ok(ProveMintSuccessReturn { proof })` with `proof` `SchnorrFsProof::MintBothGroups { challenge: c, responses: MintResponses { alpha, r, rho, sigma } }` when `self.verifier_form` is `BothGroups`, and `SchnorrFsProof::MintFirstGroupOnly { challenge: c, responses: MintResponses { alpha, r, rho, sigma }, first_messages: MintSecondGroupFirstMessages { hpub: T_hpub, d1: T_D1, d2: T_D2 } }` when it is `FirstGroupOnly`; the witnesses, moved in, and the nonces drop and zeroize |

## `prove_transfer(params: ProveTransferParams, payload: ProveTransferPayload<'_, F, P::Scalar, P::G1, P::G2>) -> ProveTransferReturn<SchnorrFsProof<P>>`

`payload.statement.context` is `DeliveryContext<F, TransferPurpose>`; the purpose relation is guaranteed by the type, with no runtime relation refusal.

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| nonce refused | the nonce step for `k_x`, `k_y`, `k_s`, `k_ρ`, then `k_σ` refuses | none | the nonce step per witness, in order | `Err(ProveTransferErrorReturn::SchnorrFs(…))` carrying that refusal |
| proved | all nonces drawn | none | with `F` the statement's `identity_element`, `pk1_s`, `pk2_s` its seller keys, `pk1_b`, `pk2_b` its buyer keys, and `C1_o`, `D1_o` its old envelope's: `T_pk1 = mul_g1(g1, k_x)`; `T_C1 = mul_g1(g1, k_ρ)`; `T_C2 = msm_g1` over `(C1_o, neg_scalar(k_x))`, `(F, k_s)`, `(pk1_b, k_ρ)`; `T_pk2 = mul_g2(g2, k_y)`; `T_D1 = mul_g2(g2, k_σ)`; `T_D2 = msm_g2` over `(D1_o, neg_scalar(k_y))`, `(g2, k_s)`, `(pk2_b, k_σ)`; the transcript step over the transfer transcript with first messages `T_pk1`, `T_C1`, `T_C2`, `T_pk2`, `T_D1`, `T_D2`; the challenge step yielding `c`; the response step for `(k_x, x_s)`, `(k_y, y_s)`, `(k_s, s)`, `(k_ρ, ρ)`, `(k_σ, σ)`, `x_s` and `y_s` from `seller_secrets`, `s` from `offset`, and `ρ`, `σ` from `coins` | `Ok(ProveTransferSuccessReturn { proof })` in the adapter's form as `prove_mint` states, the carried second-group first messages `TransferSecondGroupFirstMessages` holding `T_pk2`, `T_D1`, `T_D2` |

## `verify(params: VerifyParams, payload: VerifyPayload<'_, F, P::G1, P::G2, SchnorrFsProof<P>>) -> VerifyReturn`

A proof variant whose relation differs from its algebraic statement, or whose verifier form differs from the adapter's, is rejected with `Ok(VerifySuccessReturn { is_valid: false })`; response and first-message counts are fixed by the proof type. A well-formed proof with invalid algebra also returns `is_valid: false`. With `n = neg_scalar(c)`:

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| both-groups, mint | statement is `AlgebraicStatement::Mint` and form is `BothGroups` | none | `T_hpub = msm_g2` over `(g2, z_α)`, `(hpub, n)`; `T_C1 = msm_g1` over `(g1, z_ρ)`, `(C1, n)`; `T_C2 = msm_g1` over `(g1, z_α)`, `(F, z_r)`, `(pk1, z_ρ)`, `(C2, n)`; `T_D1 = msm_g2` over `(g2, z_σ)`, `(D1, n)`; `T_D2 = msm_g2` over `(g2, z_r)`, `(pk2, z_σ)`, `(D2, n)`; the transcript and challenge steps yielding `c'` | `Ok(VerifySuccessReturn { is_valid })`, `is_valid` the scalar-equality step over `c'` and `c` |
| both-groups, transfer | statement is `AlgebraicStatement::Transfer` and form is `BothGroups` | none | `ΔC2 = add_g1(C2_n, neg_g1(C2_o))`; `ΔD2 = add_g2(D2_n, neg_g2(D2_o))`; `T_pk1 = msm_g1` over `(g1, z_x)`, `(pk1_s, n)`; `T_C1 = msm_g1` over `(g1, z_ρ)`, `(C1_n, n)`; `T_C2 = msm_g1` over `(C1_o, neg_scalar(z_x))`, `(F, z_s)`, `(pk1_b, z_ρ)`, `(ΔC2, n)`; `T_pk2 = msm_g2` over `(g2, z_y)`, `(pk2_s, n)`; `T_D1 = msm_g2` over `(g2, z_σ)`, `(D1_n, n)`; `T_D2 = msm_g2` over `(D1_o, neg_scalar(z_y))`, `(g2, z_s)`, `(pk2_b, z_σ)`, `(ΔD2, n)`; sixteen scalar multiplications in six multi-scalar multiplications; the transcript and challenge steps | as for the mint |
| first-group-only, challenge refused | statement and proof match the relation, form is `FirstGroupOnly`, and the scalar-equality step over `c'` and `c` is false | none | the first-group recomputations the both-groups form states — `T_C1` and `T_C2` for a mint, `T_pk1`, `T_C1`, `T_C2` for a transfer — with the second-group first messages taken from the proof in their order; the transcript and challenge steps yielding `c'` | `Ok(VerifySuccessReturn { is_valid: false })` |
| first-group-only, weights refused | `c'` equals `c` and a weight hash refuses | none | for the positions `0x01` and `0x02`, `self.hash_to_scalar.hash_to_scalar(HashToScalarParams { tag: &self.weight_tag }, HashToScalarPayload { message })` with `message` the exposed `encode_scalar` bytes of `c` and of every response in order followed by the position byte, yielding `w2` and `w3` | `Err(VerifyErrorReturn::SchnorrFs(SchnorrFsVerifyErrorReturn::Weight(error)))`, the refusal unchanged; the keccak256 concrete refuses no tag this adapter holds, so no input takes this branch and it has no unit test; the first equation's weight is one and is applied by leaving its terms unscaled |
| first-group-only, mint product | `c'` equals `c`, weights derived, statement is mint | none | `pairing_product_is_one` once over the terms, each first-group element `e·g1` formed by `mul_g1(g1, e)` with `e` formed through `add_scalar`, `mul_scalar`, `neg_scalar`: `(neg_g1(g1), T_hpub)`, `(−w2·g1, T_D1)`, `(−w3·g1, T_D2)`, `((z_α + w2·z_σ + w3·z_r)·g1, g2)`, `(n·g1, hpub)`, `((w2·n)·g1, D1)`, `((w3·z_σ)·g1, pk2)`, `((w3·n)·g1, D2)` — eight pairs | `Ok(VerifySuccessReturn { is_valid })`, `is_valid` the product's `is_one` |
| first-group-only, transfer product | `c'` equals `c`, weights derived, statement is transfer | none | the terms `(neg_g1(g1), T_pk2)`, `(−w2·g1, T_D1)`, `(−w3·g1, T_D2)`, `((z_y + w2·z_σ + w3·z_s)·g1, g2)`, `(n·g1, pk2_s)`, `((w2·n)·g1, D1_n)`, `((−w3·z_y)·g1, D1_o)`, `((w3·z_σ)·g1, pk2_b)`, `((w3·n)·g1, D2_n)`, `((w3·c)·g1, D2_o)` — ten pairs | as for the mint |

## `proof_components(params: ProofComponentsParams, payload: ProofComponentsPayload<'_, SchnorrFsProof<P>>) -> ProofComponentsReturn<P::Scalar, P::G2>`

Clones the challenge and the relation-specific named response and first-message sets into the matching `DeliveryProofComponents` variant; the return is `Infallible`.

## `proof_from_components(params: ProofFromComponentsParams, payload: ProofFromComponentsPayload<P::Scalar, P::G2>) -> ProofFromComponentsReturn<SchnorrFsProof<P>>`

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| form refused | the typed component variant's verifier form differs from `self.verifier_form` | none | none | `Err(ProofFromComponentsErrorReturn::SchnorrFs(SchnorrFsProofFromComponentsErrorReturn::VerifierFormMismatch))` |
| admitted | the forms match | none | none | `Ok(ProofFromComponentsSuccessReturn { proof })`, the component's fixed fields moved into the matching `SchnorrFsProof` variant |

Untrusted serialized components first pass through the fallible `TryFrom<DeliveryProofWireComponents>` constructor, which validates relation and form, refuses a wrong `responses` count with `ResponseCount { expected, actual }` and a wrong `second_group_first_messages` count with `SecondGroupFirstMessageCount { expected, actual }` — four or five responses and zero or three second-group first messages — checking counts before building the fixed named sets.

## Ordering and invariants

- A prover checks the purpose relation, draws its nonces in witness order, forms the first messages, builds the transcript, computes the challenge, then the responses; the verifier checks the shape, recomputes, checks the challenge, and only in the first-group-only form then derives the weights and checks the product.
- Every point and scalar a pairing payload consumes is a clone of a held value; the statement and proof are unchanged; `params` carries no control and is not read in any method.
- Every proof a prover returns for a statement its witnesses satisfy verifies under the same adapter.
- The challenge is the one `schnorr_fs/challenge` returns for the transcript the statement and first messages define, so the contract recomputes it from the same fields.
- In the first-group-only form, the second-group first messages carried are exactly those the relation defines for the responses and challenge.

