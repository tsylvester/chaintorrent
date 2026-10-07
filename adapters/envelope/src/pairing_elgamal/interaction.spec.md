# `pairing_elgamal` — interaction spec

Branch contract for the `pairing_elgamal` module of the `envelope` crate: the key-agreement family's first concrete, implementing `IKeyAgreementAdapter` over a borrowed pairing concrete, hash-to-scalar adapter, encoder, and randomness source. Each branch states condition, decision, dependency call, and the exact return outcome. Writing `S`, `G1`, and `G2` for `P::Scalar`, `P::G1`, and `P::G2`, and `g1`, `g2` for `g1_generator(G1GeneratorParams, G1GeneratorPayload).point` and the same in the second group — every generator, arithmetic, and identity call returns `Result<_, Infallible>` and is unpacked irrefutably.

## Shared steps

**Sampling** — named by every sampling branch below: `P::Scalar::sample_from_uniform_bytes(SampleUniformScalarParams, SampleUniformScalarPayload { uniform })` with the uniform bytes moved in; its `Err(error)` returns in the calling method's own variant for that draw, and its `Ok` yields a `Secret<S>` whose exposed value is cloned into each pairing payload that needs it.

**Draw** — named by the nonces and coins: `self.random.fill_bytes(FillBytesParams, FillBytesPayload { length: <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH })`; its `Err(error)` returns in the calling method's draw variant unchanged, its `Ok` bytes then sampled. The drawn length is the one the sampling bound takes, so the sampling refuses no drawn bytes; neither the draw's nor the sampling's refusal has a unit test, the operating system's generator being the outer edge.

**Identity keys** — named by key generation and both key constructors: `is_identity_g1(pk1)`, `true` returned in the method's `IdentityKeyG1` variant; then `is_identity_g2(pk2)`, `true` returned in its `IdentityKeyG2` variant.

**Shared secret** — named by key generation and both key constructors: `neg_g1(g1)`, then `pairing_product_is_one` over the terms `(pk1, g2)` and `(-g1, pk2)`, once; `is_one` true returned in the method's `SharedSecret` variant.

**Challenge** — shared by key generation and restored public keys: build a `PossessionG1Statement<P>` from typed `pk` and `R` in G1 or a `PossessionG2Statement<P>` from typed `pk` and `R` in G2; encode with the matching stored description. If `self.encoder.encode` refuses, return the caller method's `Encoding(EncodeErrorReturn)` arm unchanged; otherwise hash the encoded bytes under the matching stored tag and return `HashToScalar(error)` on refusal. No arbitrary byte string becomes a trusted statement.

**Possession** — named by the public keys' constructor over a key `pk`, a commitment `R`, and a response `z` of one group: `c` from the challenge step; for the first group `is_identity_g1(add_g1(msm_g1(terms (g1, z) and (pk, neg_scalar(c))), neg_g1(R)))`, and for the second the same through `msm_g2`, `add_g2`, `neg_g2`, and `is_identity_g2`; `false` means the proof does not satisfy `z·g = R + c·pk`.

## `PairingElGamalKeyAgreement::try_new(params: PairingElGamalKeyAgreementConstructorParams<'a, P, E>) -> PairingElGamalKeyAgreementTryNewReturn<'a, P, E>`

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| tag refused | `DomainTag::try_new(DomainTagConstructorParams { bytes: PAIRING_ELGAMAL_POSSESSION_G1_TAG.to_vec() })`, then the same over `PAIRING_ELGAMAL_POSSESSION_G2_TAG`, returns `Err(error)` | each constructor's result, the first-group tag first | `DomainTag::try_new` at most twice | `Err(PairingElGamalKeyAgreementTryNewErrorReturn::PossessionG1Tag(error))` or `Err(PairingElGamalKeyAgreementTryNewErrorReturn::PossessionG2Tag(error))`, the refusal unchanged |
| admitted | both tags construct | none | both typed description constructors with `pairing: params.pairing`, unpacked irrefutably | `Ok(PairingElGamalKeyAgreement { pairing, hash_to_scalar, encoder, random, g1_statement_description, g2_statement_description, g1_tag, g2_tag })`, holding the params' borrows |

Each tag is 38 visible-ASCII bytes, so no input takes the refused branch and it has no unit test.

## `PairingElGamalKeyAgreement::<'_, P, E>::DECLARATION`

The inherent constant `KeyAgreementDeclaration { identifier: KeyAgreementIdentifier::PairingElGamalV1, algebra: EnvelopeAlgebra::PairingElGamal, possession_g1_tag: PAIRING_ELGAMAL_POSSESSION_G1_TAG, possession_g2_tag: PAIRING_ELGAMAL_POSSESSION_G2_TAG, adapter_version: 1, interface_version: KEY_AGREEMENT_INTERFACE_VERSION }`, its tags the constants `try_new` constructs the adapter's two `DomainTag`s from.

## `generate_keys(&self, params: GenerateKeysParams, payload: GenerateKeysPayload) -> GenerateKeysReturn<KeyPair, Possession>`

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| sampling refused | the `x` or `y` uniform bytes are refused, sampled in that order | the sampling step over `payload.x_uniform`, then `payload.y_uniform` | `P::Scalar::sample_from_uniform_bytes` per the sampling step | `Err(GenerateKeysErrorReturn::PairingElGamal(…))` holding `XSampling(error)` or `YSampling(error)` with the refusal unchanged, for the first refused |
| identity key or shared secret | both samplings succeed; with `pk1 = mul_g1(g1, x)` and `pk2 = mul_g2(g2, y)`, the identity-key step or then the shared-secret step refuses | identity keys, then shared secret | `mul_g1(g1, x)`, `mul_g2(g2, y)`, then the steps' calls | `Err(GenerateKeysErrorReturn::PairingElGamal(…))` holding `IdentityKeyG1`, `IdentityKeyG2`, or `SharedSecret` |
| nonce refused | the keys pass and the draw step for `k1`, then for `k2`, refuses | the draw step, `k1` first | `self.random.fill_bytes`, then the sampling step | `Err(GenerateKeysErrorReturn::PairingElGamal(…))` holding `NonceDraw(error)` or `NonceSampling(error)` unchanged |
| generated | both nonces are drawn | none further | `mul_g1(g1, k1)` for `R1` and `mul_g2(g2, k2)` for `R2`; the challenge step for `(pk1, R1)` and for `(pk2, R2)`, yielding `c1` and `c2`; `add_scalar(k1, mul_scalar(c1, x))` for `z1` and `add_scalar(k2, mul_scalar(c2, y))` for `z2` | `Ok(GenerateKeysSuccessReturn { key_pair: PairingElGamalKeyPair { x, y, pk1, pk2 }, possession: PairingElGamalPossession { r1, z1, r2, z2 } })`, `x` and `y` the sampled `Secret`s moved without copy; the nonces drop and zeroize |

## `key_pair_from_components(&self, params: KeyPairFromComponentsParams, payload: KeyPairFromComponentsPayload<S>) -> KeyPairFromComponentsReturn<KeyPair>`

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| refused | the components' `x` and `y` give `pk1 = mul_g1(g1, x)` and `pk2 = mul_g2(g2, y)` and the identity-key step or then the shared-secret step refuses | identity keys, then shared secret | `mul_g1(g1, x)`, `mul_g2(g2, y)`, then the steps' calls | `Err(KeyPairFromComponentsErrorReturn::PairingElGamal(…))` holding `IdentityKeyG1`, `IdentityKeyG2`, or `SharedSecret` |
| rebuilt | every check passes | none | as above | `Ok(KeyPairFromComponentsSuccessReturn { key_pair: PairingElGamalKeyPair { x, y, pk1, pk2 } })`, the `Secret`s moved |

## `public_keys_from_components(&self, params: PublicKeysFromComponentsParams, payload: PublicKeysFromComponentsPayload<'_, G1, G2, Possession>) -> PublicKeysFromComponentsReturn<PublicKeys>`

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| refused | the identity-key step and the shared-secret step over the components' `pk1` and `pk2`, then the possession step for `(pk1, R1, z1)` and for `(pk2, R2, z2)` read from `payload.possession` | identity keys, then shared secret, then first-group proof, then second-group proof | the steps' calls | `Err(PublicKeysFromComponentsErrorReturn::PairingElGamal(…))` holding `IdentityKeyG1`, `IdentityKeyG2`, `SharedSecret`, `PossessionG1`, `PossessionG2`, or `HashToScalar(error)` — and the challenge step's `Encoding(error)` on an encode refusal |
| admitted | every check passes | none | as above | `Ok(PublicKeysFromComponentsSuccessReturn { public_keys: PairingElGamalPublicKeys { pk1, pk2 } })`, the components moved |

## `wrap_to(&self, params: WrapToParams, payload: WrapToPayload<'_, PublicKeys, G1, G2>) -> WrapToReturn<Envelope, S>`

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| coin refused | the draw step for `ρ`, then for `σ`, refuses | the draw step, `ρ` first | `self.random.fill_bytes`, then the sampling step | `Err(WrapToErrorReturn::PairingElGamal(…))` holding `CoinDraw(error)` or `CoinSampling(error)` unchanged |
| wrapped | both coins are drawn, each from its own draw | none | `mul_g1(g1, ρ)` for `C1`, `add_g1(payload.credential.a, mul_g1(pk1, ρ))` for `C2`, `mul_g2(g2, σ)` for `D1`, and `add_g2(payload.credential.b, mul_g2(pk2, σ))` for `D2`, `pk1` and `pk2` read from `payload.public_keys` | `Ok(WrapToSuccessReturn { envelope: PairingElGamalEnvelope { c1, c2, d1, d2 }, coins: EnvelopeCoins { rho, sigma } })`, `rho` and `sigma` the sampled `Secret`s moved without copy; the credential's components, moved into the pairing payloads, drop and zeroize |

## `unwrap(&self, params: UnwrapParams, payload: UnwrapPayload<'_, KeyPair, Envelope>) -> UnwrapReturn<G1, G2>`

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| unwrapped | any admitted key pair and envelope | none | `add_g1(C2, neg_g1(mul_g1(C1, x)))` for `A` and `add_g2(D2, neg_g2(mul_g2(D1, y)))` for `B`, `x` and `y` read from `payload.key_pair`, the elements from `payload.envelope` | `Ok(UnwrapSuccessReturn { credential: CredentialComponents { a, b } })` |

## Component reads and the uninhabited-error constructors

Each one branch with no dependency call and each outcome `Ok`, unpacked irrefutably:

| Method | Outcome |
|---|---|
| `key_pair_components` | `Ok(KeyPairComponentsSuccessReturn { components: KeyPairComponents { x, y } })`, each a new `Secret` from `Secret::try_new(SecretConstructorParams { value: self.x.expose().clone() })` and the same over `y` |
| `key_pair_public_keys` | `Ok(KeyPairPublicKeysSuccessReturn { components: PublicKeysComponents { pk1, pk2 } })`, clones of the key pair's `pk1` and `pk2` |
| `public_keys_components` | `Ok(PublicKeysComponentsSuccessReturn { components })`, clones of `pk1` and `pk2` |
| `possession_components` | `Ok(PossessionComponentsSuccessReturn { components: PossessionComponents { r1, z1, r2, z2 } })`, clones |
| `possession_from_components` | `Ok(PossessionFromComponentsSuccessReturn { possession: PairingElGamalPossession { r1, z1, r2, z2 } })`, the components moved |
| `envelope_components` | `Ok(EnvelopeComponentsSuccessReturn { components: EnvelopeComponents { c1, c2, d1, d2 } })`, clones |
| `envelope_from_components` | `Ok(EnvelopeFromComponentsSuccessReturn { envelope: PairingElGamalEnvelope { c1, c2, d1, d2 } })`, the components moved |

## Ordering and invariants

- Every sampling precedes the arithmetic it feeds. Key generation samples `x`, then `y`, checks the identity keys, then the shared secret, and only then draws `k1`, then `k2`. The public keys' constructor checks the identity keys, the shared secret, the first-group proof, then the second-group proof. The wrap draws `ρ`, then `σ`, before masking.
- Every point and scalar a pairing payload consumes is a clone of a held value, and the held values are unchanged. `params` carries no control and is not read in any method.
- The same key pair, public keys, and envelope always open to the same components. Every key pair and proof `generate_keys` returns, read through `key_pair_public_keys` and `possession_components`, pass `public_keys_from_components`. Every envelope `wrap_to` returns for a key pair's public keys opens under that key pair to the credential components it wrapped.
- `pk1 = x·g1` and `pk2 = y·g2`; `C1 = ρ·g1`, `C2 = A + ρ·pk1`, `D1 = σ·g2`, and `D2 = B + σ·pk2` for the returned coins. Every value rebuilt from its own components behaves as the value it was read from.

