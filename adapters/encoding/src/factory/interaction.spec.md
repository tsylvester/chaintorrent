# factory interactions

`create_encoding` is the encoding family's construction point. It selects the
concrete the configuration names by `params.concrete`, admits it against the
encoding identifier `params.identifier` requires before anything is
constructed, and hands the admitted concrete, with its declaration, to a
consumer generic over `IEncoderAdapter` and `IDecoderAdapter`. It does not
read a hash-card or the configuration, and it does not encode, decode, or
describe.

## `create_encoding`

`create_encoding<C: IEncodingConsumer>(deps: &CreateEncodingDeps<C>, params: CreateEncodingParams, payload: CreateEncodingPayload) -> CreateEncodingReturn<C::Output>`

Decision: a `match` on `params.concrete`, one arm per `EncodingConcrete`
variant, exhaustive so a variant with no arm fails to compile.

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| unsupported identifier | the named concrete's `DECLARATION.identifier` is not `params.identifier` | equality, read before any construction | none | `Err(CreateEncodingErrorReturn::UnsupportedEncodingIdentifier)`; nothing is constructed and the consumer is not called. `EncodingIdentifier` has the one variant the ABI concrete declares, so no input takes this branch until a further identifier exists, and it has no unit test |
| admitted | the named concrete's `DECLARATION.identifier` is `params.identifier` | the same comparison passes | the concrete's `try_new` with its fieldless constructor params, exactly once, its success destructured irrefutably because its error arm is uninhabited; then `deps.consumer.consume_encoding(ConsumeEncodingParams, ConsumeEncodingPayload { adapter, declaration })` with the concrete's `DECLARATION`, exactly once | `Ok(CreateEncodingSuccessReturn { output })` holding the consumer's output |

`CreateEncodingErrorReturn::Abi` carries the constructor's uninhabited error
type in the return union, so no branch produces it.

`params.concrete` selects and `params.identifier` admits; `payload` carries
nothing and is not read.
