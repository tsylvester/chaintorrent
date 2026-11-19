#![allow(
    dead_code,
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::too_many_arguments
)]

use alloy::transports::mock::Asserter;
use chain::{
    AdmitChainCompositionFn, ConsumeChainFormsParams, ConsumeChainFormsPayload, ConsumeChainParams,
    ConsumeChainPayload, CreateChainFormsParams, CreateChainParams, IChainAdapter, IChainConsumer,
    IChainForms, IChainFormsConsumer, IEntitlementStateAdapter, admit_chain_composition,
    build_create_chain_deps, build_create_chain_forms_deps, build_create_chain_forms_params,
    build_create_chain_forms_payload, build_create_chain_params, build_create_chain_payload,
    create_chain, create_chain_forms,
};
use encoding::{
    CanonicalFieldValue, ConsumeEncodingParams, ConsumeEncodingPayload, CreateEncodingParams,
    IDecoderAdapter, IEncoderAdapter, IEncodingConsumer, build_create_encoding_deps,
    build_create_encoding_params, build_create_encoding_payload, create_encoding,
};
use hash_to_scalar::{
    CreateHashToScalarParams, IHashToScalarAdapter, build_create_hash_to_scalar_deps,
    build_create_hash_to_scalar_params, build_create_hash_to_scalar_payload, create_hash_to_scalar,
};
use pairing::{
    ConsumePairingParams, ConsumePairingPayload, CreatePairingParams, IPairingAdapter,
    IPairingArithmetic, IPairingConsumer, IPairingReference, build_create_pairing_deps,
    build_create_pairing_params, build_create_pairing_payload, create_pairing,
};
use proof::{
    ConsumeDeliveryProofParams, ConsumeDeliveryProofPayload, CreateDeliveryProofParams,
    IDeliveryProofAdapter, IDeliveryProofConsumer, build_create_delivery_proof_deps,
    build_create_delivery_proof_params, build_create_delivery_proof_payload, create_delivery_proof,
};
use random::{
    CreateRandomSourceDeps, CreateRandomSourceParams, CreateRandomSourcePayload,
    IRandomSourceAdapter, build_create_random_source_params, create_random_source,
};

use super::interface::{
    ConsumeEvmVerifierCaseParams, ConsumeEvmVerifierCasePayload, EvmVerifier, EvmVerifierCaseDeps,
    EvmVerifierCaseErrorReturn, EvmVerifierCaseParams, EvmVerifierCasePayload,
    EvmVerifierCaseReturn, EvmVerifierCaseSuccessReturn, EvmVerifierConstructorParams,
    IEvmVerifierCaseConsumer, RunEvmVerifierCaseFn,
};
use super::settings::provides::{EvmVerifierSettings, build_evm_verifier_settings};

#[derive(Default)]
pub struct EvmVerifierConstructorParamsOverrides {
    pub verifier_address: Option<CanonicalFieldValue>,
    pub settings: Option<EvmVerifierSettings>,
}

pub(crate) fn build_evm_verifier_constructor_params<'a, P, C, D>(
    pairing: &'a P,
    chain: &'a C,
    delivery_proof: &'a D,
    overrides: EvmVerifierConstructorParamsOverrides,
) -> EvmVerifierConstructorParams<'a, P, C, D>
where
    P: IPairingAdapter,
    C: IChainAdapter,
    D: IDeliveryProofAdapter<Pairing = P, Forms = <C as IEntitlementStateAdapter>::Forms>,
{
    EvmVerifierConstructorParams {
        pairing,
        chain,
        delivery_proof,
        verifier_address: overrides
            .verifier_address
            .unwrap_or(CanonicalFieldValue::FixedBytes20([0x11; 20])),
        settings: overrides
            .settings
            .unwrap_or_else(|| build_evm_verifier_settings(Default::default())),
    }
}

pub(crate) fn build_evm_verifier<'a, P, C, D>(
    pairing: &'a P,
    chain: &'a C,
    delivery_proof: &'a D,
    asserter: Asserter,
    overrides: EvmVerifierConstructorParamsOverrides,
) -> EvmVerifier<'a, P, C, D>
where
    P: IPairingAdapter,
    C: IChainAdapter,
    D: IDeliveryProofAdapter<Pairing = P, Forms = <C as IEntitlementStateAdapter>::Forms>,
{
    EvmVerifier::try_new_mocked(
        build_evm_verifier_constructor_params(pairing, chain, delivery_proof, overrides),
        asserter,
    )
    .expect("built params are admitted and the runtime builds")
}

#[derive(Default)]
pub struct EvmVerifierCaseParamsOverrides {
    pub pairing: Option<CreatePairingParams>,
    pub hash_to_scalar: Option<CreateHashToScalarParams>,
    pub encoding: Option<CreateEncodingParams>,
    pub chain_forms: Option<CreateChainFormsParams>,
    pub chain: Option<CreateChainParams>,
    pub delivery_proof: Option<CreateDeliveryProofParams>,
    pub random_source: Option<CreateRandomSourceParams>,
}

pub fn build_evm_verifier_case_params(
    overrides: EvmVerifierCaseParamsOverrides,
) -> EvmVerifierCaseParams {
    EvmVerifierCaseParams {
        pairing: overrides
            .pairing
            .unwrap_or_else(|| build_create_pairing_params(Default::default())),
        hash_to_scalar: overrides
            .hash_to_scalar
            .unwrap_or_else(|| build_create_hash_to_scalar_params(Default::default())),
        encoding: overrides
            .encoding
            .unwrap_or_else(|| build_create_encoding_params(Default::default())),
        chain_forms: overrides
            .chain_forms
            .unwrap_or_else(|| build_create_chain_forms_params(Default::default())),
        chain: overrides
            .chain
            .unwrap_or_else(|| build_create_chain_params(Default::default())),
        delivery_proof: overrides
            .delivery_proof
            .unwrap_or_else(|| build_create_delivery_proof_params(Default::default())),
        random_source: overrides
            .random_source
            .unwrap_or_else(|| build_create_random_source_params(Default::default())),
    }
}

#[derive(Default)]
pub struct EvmVerifierCaseDepsOverrides;

pub fn build_evm_verifier_case_deps<C: IEvmVerifierCaseConsumer>(
    consumer: C,
    _overrides: EvmVerifierCaseDepsOverrides,
) -> EvmVerifierCaseDeps<C> {
    EvmVerifierCaseDeps { consumer }
}

#[derive(Default)]
pub struct EvmVerifierCasePayloadOverrides;

pub fn build_evm_verifier_case_payload(
    _overrides: EvmVerifierCasePayloadOverrides,
) -> EvmVerifierCasePayload {
    EvmVerifierCasePayload
}

#[derive(Default)]
pub struct EvmVerifierCaseSuccessReturnOverrides;

pub fn build_evm_verifier_case_success_return<O>(
    output: O,
    _overrides: EvmVerifierCaseSuccessReturnOverrides,
) -> EvmVerifierCaseSuccessReturn<O> {
    EvmVerifierCaseSuccessReturn { output }
}

#[derive(Default)]
pub struct ConsumeEvmVerifierCaseParamsOverrides;

pub fn build_consume_evm_verifier_case_params(
    _overrides: ConsumeEvmVerifierCaseParamsOverrides,
) -> ConsumeEvmVerifierCaseParams {
    ConsumeEvmVerifierCaseParams
}

#[derive(Default)]
pub struct ConsumeEvmVerifierCasePayloadOverrides;

pub fn build_consume_evm_verifier_case_payload<'a, P, C, D>(
    pairing: &'a P,
    chain: &'a C,
    delivery_proof: &'a D,
    _overrides: ConsumeEvmVerifierCasePayloadOverrides,
) -> ConsumeEvmVerifierCasePayload<'a, P, C, D>
where
    P: IPairingArithmetic + IPairingReference,
    C: IChainAdapter,
    D: IDeliveryProofAdapter<Pairing = P, Forms = <C as IEntitlementStateAdapter>::Forms>,
{
    ConsumeEvmVerifierCasePayload {
        pairing,
        chain,
        delivery_proof,
    }
}

struct CaseAtDeliveryProof<'a, P: IPairingArithmetic, K: IChainAdapter, C: IEvmVerifierCaseConsumer>
{
    pairing: &'a P,
    chain: &'a K,
    consumer: &'a C,
}

impl<'a, P, K, C> IDeliveryProofConsumer<P, K::Forms> for CaseAtDeliveryProof<'a, P, K, C>
where
    P: IPairingArithmetic + IPairingReference,
    K: IChainAdapter,
    C: IEvmVerifierCaseConsumer,
{
    type Output = EvmVerifierCaseReturn<C::Output>;

    fn consume_delivery_proof<D: IDeliveryProofAdapter<Pairing = P, Forms = K::Forms>>(
        &self,
        _params: ConsumeDeliveryProofParams,
        payload: ConsumeDeliveryProofPayload<D>,
    ) -> Self::Output {
        Ok(EvmVerifierCaseSuccessReturn {
            output: self.consumer.consume_evm_verifier_case(
                build_consume_evm_verifier_case_params(Default::default()),
                build_consume_evm_verifier_case_payload(
                    self.pairing,
                    self.chain,
                    &payload.adapter,
                    Default::default(),
                ),
            ),
        })
    }
}

struct CaseAtChain<'a, P: IPairingArithmetic, E, C: IEvmVerifierCaseConsumer> {
    pairing: &'a P,
    hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
    encoder: &'a E,
    source: &'a dyn IRandomSourceAdapter,
    delivery_proof: &'a CreateDeliveryProofParams,
    consumer: &'a C,
}

impl<'a, P, E, C> IChainConsumer for CaseAtChain<'a, P, E, C>
where
    P: IPairingArithmetic + IPairingReference,
    E: IEncoderAdapter,
    C: IEvmVerifierCaseConsumer,
{
    type Output = EvmVerifierCaseReturn<C::Output>;

    fn consume_chain<K: IChainAdapter>(
        &self,
        _params: ConsumeChainParams,
        payload: ConsumeChainPayload<K>,
    ) -> Self::Output {
        match create_delivery_proof(
            &build_create_delivery_proof_deps(
                self.pairing,
                self.hashing,
                self.encoder,
                self.source,
                CaseAtDeliveryProof {
                    pairing: self.pairing,
                    chain: &payload.adapter,
                    consumer: self.consumer,
                },
                Default::default(),
            ),
            CreateDeliveryProofParams {
                concrete: self.delivery_proof.concrete,
                algebra: self.delivery_proof.algebra,
                statement_version: self.delivery_proof.statement_version,
            },
            build_create_delivery_proof_payload(Default::default()),
        ) {
            Ok(success) => success.output,
            Err(error) => Err(EvmVerifierCaseErrorReturn::DeliveryProof(error)),
        }
    }
}

struct CaseAtChainForms<'a, P: IPairingArithmetic, E, C: IEvmVerifierCaseConsumer> {
    pairing: &'a P,
    hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
    encoder: &'a E,
    source: &'a dyn IRandomSourceAdapter,
    chain: &'a CreateChainParams,
    delivery_proof: &'a CreateDeliveryProofParams,
    consumer: &'a C,
}

impl<'a, P, E, C> IChainFormsConsumer for CaseAtChainForms<'a, P, E, C>
where
    P: IPairingArithmetic + IPairingReference,
    E: IEncoderAdapter,
    C: IEvmVerifierCaseConsumer,
{
    type Output = EvmVerifierCaseReturn<C::Output>;

    fn consume_chain_forms<F: IChainForms>(
        &self,
        _params: ConsumeChainFormsParams,
        _payload: ConsumeChainFormsPayload<F>,
    ) -> Self::Output {
        match create_chain::<F, _>(
            &build_create_chain_deps(
                admit_chain_composition as AdmitChainCompositionFn,
                CaseAtChain {
                    pairing: self.pairing,
                    hashing: self.hashing,
                    encoder: self.encoder,
                    source: self.source,
                    delivery_proof: self.delivery_proof,
                    consumer: self.consumer,
                },
                Default::default(),
            ),
            self.chain.clone(),
            build_create_chain_payload(Default::default()),
        ) {
            Ok(success) => success.output,
            Err(error) => Err(EvmVerifierCaseErrorReturn::Chain(error)),
        }
    }
}

struct CaseAtEncoding<'a, P: IPairingArithmetic, C: IEvmVerifierCaseConsumer> {
    pairing: &'a P,
    hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
    source: &'a dyn IRandomSourceAdapter,
    chain_forms: &'a CreateChainFormsParams,
    chain: &'a CreateChainParams,
    delivery_proof: &'a CreateDeliveryProofParams,
    consumer: &'a C,
}

impl<'a, P, C> IEncodingConsumer for CaseAtEncoding<'a, P, C>
where
    P: IPairingArithmetic + IPairingReference,
    C: IEvmVerifierCaseConsumer,
{
    type Output = EvmVerifierCaseReturn<C::Output>;

    fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
        &self,
        _params: ConsumeEncodingParams,
        payload: ConsumeEncodingPayload<E>,
    ) -> Self::Output {
        match create_chain_forms(
            &build_create_chain_forms_deps(
                CaseAtChainForms {
                    pairing: self.pairing,
                    hashing: self.hashing,
                    encoder: &payload.adapter,
                    source: self.source,
                    chain: self.chain,
                    delivery_proof: self.delivery_proof,
                    consumer: self.consumer,
                },
                &payload.adapter,
                Default::default(),
            ),
            *self.chain_forms,
            build_create_chain_forms_payload(Default::default()),
        ) {
            Ok(success) => success.output,
            Err(error) => Err(EvmVerifierCaseErrorReturn::ChainForms(error)),
        }
    }
}

struct CaseAtPairing<'a, C: IEvmVerifierCaseConsumer> {
    hash_to_scalar: &'a CreateHashToScalarParams,
    random_source: &'a CreateRandomSourceParams,
    encoding: &'a CreateEncodingParams,
    chain_forms: &'a CreateChainFormsParams,
    chain: &'a CreateChainParams,
    delivery_proof: &'a CreateDeliveryProofParams,
    consumer: &'a C,
}

impl<C: IEvmVerifierCaseConsumer> IPairingConsumer for CaseAtPairing<'_, C> {
    type Output = EvmVerifierCaseReturn<C::Output>;

    fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
        &self,
        _params: ConsumePairingParams,
        payload: ConsumePairingPayload<P>,
    ) -> Self::Output {
        let hashing = match create_hash_to_scalar::<P::Scalar>(
            &build_create_hash_to_scalar_deps(Default::default()),
            *self.hash_to_scalar,
            build_create_hash_to_scalar_payload(Default::default()),
        ) {
            Ok(hashing) => hashing,
            Err(error) => {
                return Err(EvmVerifierCaseErrorReturn::HashToScalar(error));
            }
        };
        let source = match create_random_source(
            &CreateRandomSourceDeps,
            CreateRandomSourceParams {
                kind: self.random_source.kind,
            },
            CreateRandomSourcePayload,
        ) {
            Ok(source) => source,
            Err(error) => {
                return Err(EvmVerifierCaseErrorReturn::RandomSource(error));
            }
        };
        match create_encoding(
            &build_create_encoding_deps(CaseAtEncoding {
                pairing: &payload.adapter,
                hashing: &*hashing.adapter,
                source: &*source.adapter,
                chain_forms: self.chain_forms,
                chain: self.chain,
                delivery_proof: self.delivery_proof,
                consumer: self.consumer,
            }),
            CreateEncodingParams {
                concrete: self.encoding.concrete,
                identifier: self.encoding.identifier,
            },
            build_create_encoding_payload(Default::default()),
        ) {
            Ok(success) => success.output,
            Err(error) => Err(EvmVerifierCaseErrorReturn::Encoding(error)),
        }
    }
}

pub fn run_evm_verifier_case<C: IEvmVerifierCaseConsumer>(
    deps: &EvmVerifierCaseDeps<C>,
    params: EvmVerifierCaseParams,
    _payload: EvmVerifierCasePayload,
) -> EvmVerifierCaseReturn<C::Output> {
    let _: RunEvmVerifierCaseFn<C> = run_evm_verifier_case;
    match create_pairing(
        &build_create_pairing_deps(
            CaseAtPairing {
                hash_to_scalar: &params.hash_to_scalar,
                random_source: &params.random_source,
                encoding: &params.encoding,
                chain_forms: &params.chain_forms,
                chain: &params.chain,
                delivery_proof: &params.delivery_proof,
                consumer: &deps.consumer,
            },
            Default::default(),
        ),
        params.pairing,
        build_create_pairing_payload(Default::default()),
    ) {
        Ok(success) => success.output,
        Err(error) => Err(EvmVerifierCaseErrorReturn::Pairing(error)),
    }
}
