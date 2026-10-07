# `schnorr_fs/challenge` — interaction spec

Branch contract for the `challenge` function of the `schnorr_fs` concrete of the `proof` crate: the Fiat–Shamir challenge of the delivery proof, the injected hash-to-scalar adapter's mapping, under the proof's domain tag `SCHNORR_FS_CHALLENGE_TAG`, of the injected encoder's encoding of a mint or transfer transcript through that transcript's own pairing-borrowing description. Each branch states condition, decision, dependency call, and the exact return outcome.

The trusted form: the transcript arrives as a typed `ChallengeTranscript` from `proof/schnorr_fs`, so no entry guard exists and `payload` is not `unknown`.

## `challenge<S: ISampleUniformScalar + Clone, E: IEncoderAdapter, P: IPairingAdapter<Scalar = S>, F: IChainForms>(deps: &ChallengeDeps<'_, P, E>, params: ChallengeParams, payload: ChallengePayload<'_, P, F>) -> ChallengeReturn<S>`

The challenge tag is constructed once in `SchnorrFsDeliveryProof::try_new` from `SCHNORR_FS_CHALLENGE_TAG` and borrowed in `ChallengeDeps`; each challenge call uses that admitted tag without constructing a new one.

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| encode mint transcript | `payload.transcript` is `ChallengeTranscript::Mint(statement)` | none | `MintStatementDescription::<P, F>::try_new(MintStatementDescriptionConstructorParams { pairing: deps.pairing })`, unpacked irrefutably, then `deps.encoder.encode(EncodeParams { description: &description }, statement)`, once | `Err(ChallengeErrorReturn::Encoding(error))` on refusal, the refusal unchanged; otherwise yields the encoding's `bytes` |
| encode transfer transcript | `payload.transcript` is `ChallengeTranscript::Transfer(statement)` | none | `TransferStatementDescription::<P, F>::try_new(TransferStatementDescriptionConstructorParams { pairing: deps.pairing })`, unpacked irrefutably, then `deps.encoder.encode(EncodeParams { description: &description }, statement)`, once | `Err(ChallengeErrorReturn::Encoding(error))` on refusal, the refusal unchanged; otherwise yields the encoding's `bytes` |
| hash refused | `deps.hash_to_scalar.hash_to_scalar(…)` returns `Err(error)` | none | `deps.hash_to_scalar.hash_to_scalar(HashToScalarParams { tag: deps.tag }, HashToScalarPayload { message: &bytes })`, once, after the encoding | `Err(ChallengeErrorReturn::HashToScalar(error))`, the refusal unchanged; the keccak256 concrete refuses no tag this function holds and no scalar type whose uniform length is at least its digest's, which every current scalar type is, so no input takes this branch and it has no unit test |
| computed | the hash returns `Ok(success)` | none | as above | `Ok(ChallengeSuccessReturn { challenge: success.scalar })` |

## Ordering and invariants

- The already-admitted tag is borrowed, then the matching pairing-borrowing description is constructed, then encoding is attempted, then hashing; an encoding refusal prevents the hash call; the transcript is borrowed and unchanged.
- `params` carries no control and is not read.
- For a given encoder and hash-to-scalar adapter, the challenge is a function of the transcript alone: the mapping under `SCHNORR_FS_CHALLENGE_TAG` of the transcript's encoding through its own description, which the contract recomputes from the same fields.

