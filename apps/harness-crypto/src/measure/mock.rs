#![allow(dead_code, clippy::too_many_arguments)]

use chain::{
    ConsumeChainFormsParams, ConsumeChainFormsPayload, CreateChainFormsParams, IChainForms,
    IChainFormsConsumer, build_create_chain_forms_deps, build_create_chain_forms_params,
    build_create_chain_forms_payload, create_chain_forms,
};
use core::num::NonZeroU32;
use core::time::Duration;
use encoding::{
    ConsumeEncodingParams, ConsumeEncodingPayload, CreateEncodingParams, IDecoderAdapter,
    IEncoderAdapter, IEncodingConsumer, build_create_encoding_deps, build_create_encoding_params,
    build_create_encoding_payload, create_encoding,
};
use envelope::{
    ConsumeKeyAgreementParams, ConsumeKeyAgreementPayload, CreateKeyAgreementParams,
    IKeyAgreementAdapter, IKeyAgreementConsumer, build_create_key_agreement_deps,
    build_create_key_agreement_params, build_create_key_agreement_payload, create_key_agreement,
};
use hash_to_scalar::{
    CreateHashToScalarParams, IHashToScalarAdapter, build_create_hash_to_scalar_deps,
    build_create_hash_to_scalar_params, build_create_hash_to_scalar_payload, create_hash_to_scalar,
};
use hashing::Root;
use kem::{
    ConsumeKemParams, ConsumeKemPayload, CreateKemParams, ICredentialKemAdapter, IKemConsumer,
    IdentityScope, KemIdentity, SetupScope, build_create_kem_deps, build_create_kem_params,
    build_create_kem_payload, create_kem,
};
use pairing::{
    ConsumePairingParams, ConsumePairingPayload, CreatePairingParams, IPairingAdapter,
    IPairingArithmetic, IPairingConsumer, IPairingReference, build_create_pairing_deps,
    build_create_pairing_params, build_create_pairing_payload, create_pairing,
};
use proof::{
    ConsumeDeliveryProofParams, ConsumeDeliveryProofPayload, CreateDeliveryProofParams,
    DeliveryContext, IDeliveryProofAdapter, IDeliveryProofConsumer, MintPurpose, TransferPurpose,
    build_create_delivery_proof_deps, build_create_delivery_proof_params,
    build_create_delivery_proof_payload, create_delivery_proof,
};
use random::{
    CreateRandomSourceParams, IRandomSourceAdapter, build_create_random_source_deps,
    build_create_random_source_params, build_create_random_source_payload, create_random_source,
};
use serde_json::{Map, Value};

use super::interface::{
    ConsumeMeasureCaseParams, ConsumeMeasureCasePayload, DeliveryCosts, DeliveryMeasurement,
    DeliveryOperationTimings, DeliverySizes, IMeasureCaseConsumer, MeasureCaseDeps,
    MeasureCaseErrorReturn, MeasureCaseParams, MeasureCasePayload, MeasureCaseReturn,
    MeasureCaseSuccessReturn, MeasureDeps, MeasureFn, MeasureParams, MeasurePayload, MeasureReturn,
    MeasureSuccessReturn, RunMeasureCaseFn,
};
use crate::delivery::provides::{AdmittedKemIdentity, build_admitted_kem_identity};
use crate::verifier::provides::{
    ConsumeVerifierParams, ConsumeVerifierPayload, CreateVerifierParams, IVerifierAdapter,
    IVerifierConsumer, VerifyCost, build_create_verifier_deps, build_create_verifier_params,
    build_create_verifier_payload, build_verify_cost, create_verifier,
};

#[derive(Default)]
pub struct DeliveryOperationTimingsOverrides {
    pub prove_mint: Option<Duration>,
    pub prove_transfer: Option<Duration>,
    pub verify_mint: Option<Duration>,
    pub verify_transfer: Option<Duration>,
    pub decapsulate: Option<Duration>,
}

pub fn build_delivery_operation_timings(
    overrides: DeliveryOperationTimingsOverrides,
) -> DeliveryOperationTimings {
    DeliveryOperationTimings {
        prove_mint: overrides.prove_mint.unwrap_or(Duration::ZERO),
        prove_transfer: overrides.prove_transfer.unwrap_or(Duration::ZERO),
        verify_mint: overrides.verify_mint.unwrap_or(Duration::ZERO),
        verify_transfer: overrides.verify_transfer.unwrap_or(Duration::ZERO),
        decapsulate: overrides.decapsulate.unwrap_or(Duration::ZERO),
    }
}

#[derive(Default)]
pub struct DeliverySizesOverrides {
    pub capsule: Option<usize>,
    pub envelope: Option<usize>,
    pub mint_proof: Option<usize>,
    pub transfer_proof: Option<usize>,
}

pub fn build_delivery_sizes(overrides: DeliverySizesOverrides) -> DeliverySizes {
    DeliverySizes {
        capsule: overrides.capsule.unwrap_or(0),
        envelope: overrides.envelope.unwrap_or(0),
        mint_proof: overrides.mint_proof.unwrap_or(0),
        transfer_proof: overrides.transfer_proof.unwrap_or(0),
    }
}

#[derive(Default)]
pub struct DeliveryCostsOverrides {
    pub mint: Option<VerifyCost>,
    pub transfer: Option<VerifyCost>,
}

pub fn build_delivery_costs(overrides: DeliveryCostsOverrides) -> DeliveryCosts {
    DeliveryCosts {
        mint: overrides
            .mint
            .unwrap_or_else(|| build_verify_cost(Default::default())),
        transfer: overrides
            .transfer
            .unwrap_or_else(|| build_verify_cost(Default::default())),
    }
}

#[derive(Default)]
pub struct DeliveryMeasurementOverrides {
    pub iterations: Option<NonZeroU32>,
    pub timings: Option<DeliveryOperationTimings>,
    pub sizes: Option<DeliverySizes>,
    pub costs: Option<DeliveryCosts>,
}

pub fn build_delivery_measurement(overrides: DeliveryMeasurementOverrides) -> DeliveryMeasurement {
    DeliveryMeasurement {
        iterations: overrides.iterations.unwrap_or(NonZeroU32::MIN),
        timings: overrides
            .timings
            .unwrap_or_else(|| build_delivery_operation_timings(Default::default())),
        sizes: overrides
            .sizes
            .unwrap_or_else(|| build_delivery_sizes(Default::default())),
        costs: overrides
            .costs
            .unwrap_or_else(|| build_delivery_costs(Default::default())),
    }
}

#[derive(Default)]
pub struct MeasureParamsOverrides {
    pub iterations: Option<NonZeroU32>,
}

pub fn build_measure_params(overrides: MeasureParamsOverrides) -> MeasureParams {
    MeasureParams {
        iterations: overrides.iterations.unwrap_or(NonZeroU32::MIN),
    }
}

#[derive(Default)]
pub struct MeasureParamsCorruptions {
    pub iterations: Option<Value>,
}

pub fn invalidate_measure_params(corruptions: MeasureParamsCorruptions) -> Value {
    let params = build_measure_params(Default::default());
    let mut fields = Map::new();
    fields.insert(
        "iterations".to_string(),
        Value::from(params.iterations.get()),
    );
    if let Some(iterations) = corruptions.iterations {
        fields.insert("iterations".to_string(), iterations);
    }
    Value::Object(fields)
}

#[derive(Default)]
pub struct MeasurePayloadOverrides<'a> {
    pub identity: Option<AdmittedKemIdentity<'a>>,
}

pub fn build_measure_payload<'a, F: IChainForms>(
    mint: &'a DeliveryContext<F, MintPurpose>,
    transfer: &'a DeliveryContext<F, TransferPurpose>,
    overrides: MeasurePayloadOverrides<'a>,
) -> MeasurePayload<'a, F> {
    MeasurePayload {
        mint,
        transfer,
        identity: overrides
            .identity
            .unwrap_or_else(|| build_admitted_kem_identity(Default::default())),
    }
}

#[derive(Default)]
pub struct MeasureDepsOverrides;

pub fn build_measure_deps<
    'a,
    P: IPairingAdapter,
    K: ICredentialKemAdapter<Pairing = P>,
    A: IKeyAgreementAdapter<Pairing = P>,
    D: IDeliveryProofAdapter<Pairing = P>,
    V: IVerifierAdapter<DeliveryProof = D>,
>(
    pairing: &'a P,
    kem: &'a K,
    key_agreement: &'a A,
    delivery_proof: &'a D,
    verifier: &'a V,
    random: &'a dyn IRandomSourceAdapter,
    _overrides: MeasureDepsOverrides,
) -> MeasureDeps<'a, P, K, A, D, V> {
    MeasureDeps {
        pairing,
        kem,
        key_agreement,
        delivery_proof,
        verifier,
        random,
    }
}

#[derive(Default)]
pub struct MeasureSuccessReturnOverrides {
    pub measurement: Option<DeliveryMeasurement>,
}

pub fn build_measure_success_return(
    overrides: MeasureSuccessReturnOverrides,
) -> MeasureSuccessReturn {
    MeasureSuccessReturn {
        measurement: overrides
            .measurement
            .unwrap_or_else(|| build_delivery_measurement(Default::default())),
    }
}

pub fn mock_measure<
    'a,
    P: IPairingAdapter,
    K: ICredentialKemAdapter<Pairing = P>,
    A: IKeyAgreementAdapter<Pairing = P>,
    D: IDeliveryProofAdapter<Pairing = P>,
    V: IVerifierAdapter<DeliveryProof = D>,
>(
    _deps: &MeasureDeps<'a, P, K, A, D, V>,
    _params: MeasureParams,
    _payload: MeasurePayload<'_, D::Forms>,
) -> MeasureReturn {
    let _: MeasureFn<'a, P, K, A, D, V> = mock_measure;
    Ok(build_measure_success_return(Default::default()))
}

#[derive(Default)]
pub struct MeasureCaseParamsOverrides {
    pub pairing: Option<CreatePairingParams>,
    pub hash_to_scalar: Option<CreateHashToScalarParams>,
    pub encoding: Option<CreateEncodingParams>,
    pub chain_forms: Option<CreateChainFormsParams>,
    pub kem: Option<CreateKemParams>,
    pub key_agreement: Option<CreateKeyAgreementParams>,
    pub delivery_proof: Option<CreateDeliveryProofParams>,
    pub verifier: Option<CreateVerifierParams>,
    pub random_source: Option<CreateRandomSourceParams>,
}

pub fn build_measure_case_params(overrides: MeasureCaseParamsOverrides) -> MeasureCaseParams {
    MeasureCaseParams {
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
        kem: overrides
            .kem
            .unwrap_or_else(|| build_create_kem_params(Default::default())),
        key_agreement: overrides
            .key_agreement
            .unwrap_or_else(|| build_create_key_agreement_params(Default::default())),
        delivery_proof: overrides
            .delivery_proof
            .unwrap_or_else(|| build_create_delivery_proof_params(Default::default())),
        verifier: overrides
            .verifier
            .unwrap_or_else(|| build_create_verifier_params(Default::default())),
        random_source: overrides
            .random_source
            .unwrap_or_else(|| build_create_random_source_params(Default::default())),
    }
}

#[derive(Default)]
pub struct MeasureCaseDepsOverrides;

pub fn build_measure_case_deps<C: IMeasureCaseConsumer>(
    consumer: C,
    _overrides: MeasureCaseDepsOverrides,
) -> MeasureCaseDeps<C> {
    MeasureCaseDeps { consumer }
}

#[derive(Default)]
pub struct MeasureCasePayloadOverrides;

pub fn build_measure_case_payload<'a>(
    canonical: &'a [u8],
    root: &'a Root,
    _overrides: MeasureCasePayloadOverrides,
) -> MeasureCasePayload<'a> {
    MeasureCasePayload { canonical, root }
}

#[derive(Default)]
pub struct MeasureCaseSuccessReturnOverrides;

pub fn build_measure_case_success_return<O>(
    output: O,
    _overrides: MeasureCaseSuccessReturnOverrides,
) -> MeasureCaseSuccessReturn<O> {
    MeasureCaseSuccessReturn { output }
}

#[derive(Default)]
pub struct ConsumeMeasureCaseParamsOverrides;

pub fn build_consume_measure_case_params(
    _overrides: ConsumeMeasureCaseParamsOverrides,
) -> ConsumeMeasureCaseParams {
    ConsumeMeasureCaseParams
}

#[derive(Default)]
pub struct ConsumeMeasureCasePayloadOverrides;

pub fn build_consume_measure_case_payload<'a, P, H, E, F, K, A, D, V>(
    pairing: &'a P,
    hash_to_scalar: &'a H,
    encoder: &'a E,
    forms: &'a F,
    kem: &'a K,
    key_agreement: &'a A,
    delivery_proof: &'a D,
    verifier: &'a V,
    random: &'a dyn IRandomSourceAdapter,
    independent: &'a dyn IRandomSourceAdapter,
    identity: AdmittedKemIdentity<'a>,
    _overrides: ConsumeMeasureCasePayloadOverrides,
) -> ConsumeMeasureCasePayload<'a, P, H, E, F, K, A, D, V>
where
    P: IPairingArithmetic + IPairingReference,
    H: ?Sized + IHashToScalarAdapter<P::Scalar>,
    E: IEncoderAdapter + IDecoderAdapter,
    F: IChainForms,
    K: ICredentialKemAdapter<Pairing = P>,
    A: IKeyAgreementAdapter<Pairing = P>,
    D: IDeliveryProofAdapter<Pairing = P, Forms = F>,
    V: IVerifierAdapter<DeliveryProof = D>,
{
    ConsumeMeasureCasePayload {
        pairing,
        hash_to_scalar,
        encoder,
        forms,
        kem,
        key_agreement,
        delivery_proof,
        verifier,
        random,
        independent,
        identity,
    }
}

struct CaseAtVerifier<
    'a,
    P: IPairingArithmetic,
    E,
    F: IChainForms,
    K,
    A,
    D,
    C: IMeasureCaseConsumer,
> {
    pairing: &'a P,
    hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
    encoder: &'a E,
    forms: &'a F,
    kem: &'a K,
    key_agreement: &'a A,
    delivery_proof: &'a D,
    random: &'a dyn IRandomSourceAdapter,
    independent: &'a dyn IRandomSourceAdapter,
    identity: AdmittedKemIdentity<'a>,
    consumer: &'a C,
}

impl<'a, P, E, F, K, A, D, C> IVerifierConsumer<D> for CaseAtVerifier<'a, P, E, F, K, A, D, C>
where
    P: IPairingArithmetic + IPairingReference,
    E: IEncoderAdapter + IDecoderAdapter,
    F: IChainForms,
    K: ICredentialKemAdapter<Pairing = P>,
    A: IKeyAgreementAdapter<Pairing = P>,
    D: IDeliveryProofAdapter<Pairing = P, Forms = F>,
    C: IMeasureCaseConsumer,
{
    type Output = MeasureCaseReturn<C::Output>;

    fn consume_verifier<V: IVerifierAdapter<DeliveryProof = D>>(
        &self,
        _params: ConsumeVerifierParams,
        payload: ConsumeVerifierPayload<V>,
    ) -> Self::Output {
        Ok(MeasureCaseSuccessReturn {
            output: self.consumer.consume_measure_case(
                build_consume_measure_case_params(Default::default()),
                build_consume_measure_case_payload(
                    self.pairing,
                    self.hash_to_scalar,
                    self.encoder,
                    self.forms,
                    self.kem,
                    self.key_agreement,
                    self.delivery_proof,
                    &payload.adapter,
                    self.random,
                    self.independent,
                    self.identity,
                    Default::default(),
                ),
            ),
        })
    }
}

struct CaseAtDeliveryProof<
    'a,
    P: IPairingArithmetic,
    E,
    F: IChainForms,
    K,
    A,
    C: IMeasureCaseConsumer,
> {
    pairing: &'a P,
    hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
    encoder: &'a E,
    forms: &'a F,
    kem: &'a K,
    key_agreement: &'a A,
    random: &'a dyn IRandomSourceAdapter,
    independent: &'a dyn IRandomSourceAdapter,
    verifier: &'a CreateVerifierParams,
    identity: AdmittedKemIdentity<'a>,
    consumer: &'a C,
}

impl<'a, P, E, F, K, A, C> IDeliveryProofConsumer<P, F>
    for CaseAtDeliveryProof<'a, P, E, F, K, A, C>
where
    P: IPairingArithmetic + IPairingReference,
    E: IEncoderAdapter + IDecoderAdapter,
    F: IChainForms,
    K: ICredentialKemAdapter<Pairing = P>,
    A: IKeyAgreementAdapter<Pairing = P>,
    C: IMeasureCaseConsumer,
{
    type Output = MeasureCaseReturn<C::Output>;

    fn consume_delivery_proof<D: IDeliveryProofAdapter<Pairing = P, Forms = F>>(
        &self,
        _params: ConsumeDeliveryProofParams,
        payload: ConsumeDeliveryProofPayload<D>,
    ) -> Self::Output {
        match create_verifier(
            &build_create_verifier_deps(
                self.pairing,
                &payload.adapter,
                CaseAtVerifier {
                    pairing: self.pairing,
                    hash_to_scalar: self.hash_to_scalar,
                    encoder: self.encoder,
                    forms: self.forms,
                    kem: self.kem,
                    key_agreement: self.key_agreement,
                    delivery_proof: &payload.adapter,
                    random: self.random,
                    independent: self.independent,
                    identity: self.identity,
                    consumer: self.consumer,
                },
                Default::default(),
            ),
            self.verifier.clone(),
            build_create_verifier_payload(Default::default()),
        ) {
            Ok(success) => success.output,
            Err(error) => Err(MeasureCaseErrorReturn::CreateVerifier(error)),
        }
    }
}

struct CaseAtKeyAgreement<'a, P: IPairingArithmetic, E, F: IChainForms, K, C: IMeasureCaseConsumer>
{
    pairing: &'a P,
    hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
    encoder: &'a E,
    forms: &'a F,
    kem: &'a K,
    random: &'a dyn IRandomSourceAdapter,
    independent: &'a dyn IRandomSourceAdapter,
    delivery_proof: &'a CreateDeliveryProofParams,
    verifier: &'a CreateVerifierParams,
    identity: AdmittedKemIdentity<'a>,
    consumer: &'a C,
}

impl<'a, P, E, F, K, C> IKeyAgreementConsumer<P> for CaseAtKeyAgreement<'a, P, E, F, K, C>
where
    P: IPairingArithmetic + IPairingReference,
    E: IEncoderAdapter + IDecoderAdapter,
    F: IChainForms,
    K: ICredentialKemAdapter<Pairing = P>,
    C: IMeasureCaseConsumer,
{
    type Output = MeasureCaseReturn<C::Output>;

    fn consume_key_agreement<A: IKeyAgreementAdapter<Pairing = P>>(
        &self,
        _params: ConsumeKeyAgreementParams,
        payload: ConsumeKeyAgreementPayload<A>,
    ) -> Self::Output {
        match create_delivery_proof(
            &build_create_delivery_proof_deps(
                self.pairing,
                self.hash_to_scalar,
                self.encoder,
                self.random,
                CaseAtDeliveryProof {
                    pairing: self.pairing,
                    hash_to_scalar: self.hash_to_scalar,
                    encoder: self.encoder,
                    forms: self.forms,
                    kem: self.kem,
                    key_agreement: &payload.adapter,
                    random: self.random,
                    independent: self.independent,
                    verifier: self.verifier,
                    identity: self.identity,
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
            Err(error) => Err(MeasureCaseErrorReturn::CreateDeliveryProof(error)),
        }
    }
}

struct CaseAtKem<'a, P: IPairingArithmetic, E, F: IChainForms, C: IMeasureCaseConsumer> {
    pairing: &'a P,
    hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
    encoder: &'a E,
    forms: &'a F,
    random: &'a dyn IRandomSourceAdapter,
    independent: &'a dyn IRandomSourceAdapter,
    key_agreement: &'a CreateKeyAgreementParams,
    delivery_proof: &'a CreateDeliveryProofParams,
    verifier: &'a CreateVerifierParams,
    payload: &'a MeasureCasePayload<'a>,
    consumer: &'a C,
}

impl<'a, P, E, F, C> IKemConsumer<P> for CaseAtKem<'a, P, E, F, C>
where
    P: IPairingArithmetic + IPairingReference,
    E: IEncoderAdapter + IDecoderAdapter,
    F: IChainForms,
    C: IMeasureCaseConsumer,
{
    type Output = MeasureCaseReturn<C::Output>;

    fn consume_kem<K: ICredentialKemAdapter<Pairing = P>>(
        &self,
        _params: ConsumeKemParams,
        payload: ConsumeKemPayload<K>,
    ) -> Self::Output {
        let identity = match payload.scope() {
            IdentityScope::Entitlement => AdmittedKemIdentity {
                setup_scope: SetupScope::Entitlement,
                identity: KemIdentity::Entitlement {
                    canonical: self.payload.canonical,
                },
            },
            IdentityScope::Asset => AdmittedKemIdentity {
                setup_scope: SetupScope::Asset {
                    root: self.payload.root,
                },
                identity: KemIdentity::Asset {
                    root: self.payload.root,
                },
            },
        };
        match create_key_agreement(
            &build_create_key_agreement_deps(
                self.pairing,
                self.hash_to_scalar,
                self.encoder,
                self.random,
                CaseAtKeyAgreement {
                    pairing: self.pairing,
                    hash_to_scalar: self.hash_to_scalar,
                    encoder: self.encoder,
                    forms: self.forms,
                    kem: &payload.adapter,
                    random: self.random,
                    independent: self.independent,
                    delivery_proof: self.delivery_proof,
                    verifier: self.verifier,
                    identity,
                    consumer: self.consumer,
                },
                Default::default(),
            ),
            *self.key_agreement,
            build_create_key_agreement_payload(Default::default()),
        ) {
            Ok(success) => success.output,
            Err(error) => Err(MeasureCaseErrorReturn::CreateKeyAgreement(error)),
        }
    }
}

struct CaseAtChainForms<'a, P: IPairingArithmetic, E, C: IMeasureCaseConsumer> {
    pairing: &'a P,
    hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
    encoder: &'a E,
    random: &'a dyn IRandomSourceAdapter,
    independent: &'a dyn IRandomSourceAdapter,
    kem: &'a CreateKemParams,
    key_agreement: &'a CreateKeyAgreementParams,
    delivery_proof: &'a CreateDeliveryProofParams,
    verifier: &'a CreateVerifierParams,
    payload: &'a MeasureCasePayload<'a>,
    consumer: &'a C,
}

impl<'a, P, E, C> IChainFormsConsumer for CaseAtChainForms<'a, P, E, C>
where
    P: IPairingArithmetic + IPairingReference,
    E: IEncoderAdapter + IDecoderAdapter,
    C: IMeasureCaseConsumer,
{
    type Output = MeasureCaseReturn<C::Output>;

    fn consume_chain_forms<F: IChainForms>(
        &self,
        _params: ConsumeChainFormsParams,
        payload: ConsumeChainFormsPayload<F>,
    ) -> Self::Output {
        match create_kem(
            &build_create_kem_deps(
                self.pairing,
                self.hash_to_scalar,
                CaseAtKem {
                    pairing: self.pairing,
                    hash_to_scalar: self.hash_to_scalar,
                    encoder: self.encoder,
                    forms: &payload.forms,
                    random: self.random,
                    independent: self.independent,
                    key_agreement: self.key_agreement,
                    delivery_proof: self.delivery_proof,
                    verifier: self.verifier,
                    payload: self.payload,
                    consumer: self.consumer,
                },
                Default::default(),
            ),
            *self.kem,
            build_create_kem_payload(Default::default()),
        ) {
            Ok(success) => success.output,
            Err(error) => Err(MeasureCaseErrorReturn::CreateKem(error)),
        }
    }
}

struct CaseAtEncoding<'a, P: IPairingArithmetic, C: IMeasureCaseConsumer> {
    pairing: &'a P,
    hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
    random: &'a dyn IRandomSourceAdapter,
    independent: &'a dyn IRandomSourceAdapter,
    chain_forms: &'a CreateChainFormsParams,
    kem: &'a CreateKemParams,
    key_agreement: &'a CreateKeyAgreementParams,
    delivery_proof: &'a CreateDeliveryProofParams,
    verifier: &'a CreateVerifierParams,
    payload: &'a MeasureCasePayload<'a>,
    consumer: &'a C,
}

impl<'a, P, C> IEncodingConsumer for CaseAtEncoding<'a, P, C>
where
    P: IPairingArithmetic + IPairingReference,
    C: IMeasureCaseConsumer,
{
    type Output = MeasureCaseReturn<C::Output>;

    fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
        &self,
        _params: ConsumeEncodingParams,
        payload: ConsumeEncodingPayload<E>,
    ) -> Self::Output {
        match create_chain_forms(
            &build_create_chain_forms_deps(
                CaseAtChainForms {
                    pairing: self.pairing,
                    hash_to_scalar: self.hash_to_scalar,
                    encoder: &payload.adapter,
                    random: self.random,
                    independent: self.independent,
                    kem: self.kem,
                    key_agreement: self.key_agreement,
                    delivery_proof: self.delivery_proof,
                    verifier: self.verifier,
                    payload: self.payload,
                    consumer: self.consumer,
                },
                &payload.adapter,
                Default::default(),
            ),
            *self.chain_forms,
            build_create_chain_forms_payload(Default::default()),
        ) {
            Ok(success) => success.output,
            Err(error) => Err(MeasureCaseErrorReturn::CreateChainForms(error)),
        }
    }
}

struct CaseAtPairing<'a, C: IMeasureCaseConsumer> {
    hash_to_scalar: &'a CreateHashToScalarParams,
    random_source: &'a CreateRandomSourceParams,
    encoding: &'a CreateEncodingParams,
    chain_forms: &'a CreateChainFormsParams,
    kem: &'a CreateKemParams,
    key_agreement: &'a CreateKeyAgreementParams,
    delivery_proof: &'a CreateDeliveryProofParams,
    verifier: &'a CreateVerifierParams,
    payload: &'a MeasureCasePayload<'a>,
    consumer: &'a C,
}

impl<C: IMeasureCaseConsumer> IPairingConsumer for CaseAtPairing<'_, C> {
    type Output = MeasureCaseReturn<C::Output>;

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
                return Err(MeasureCaseErrorReturn::CreateHashToScalar(error));
            }
        };
        let random = match create_random_source(
            &build_create_random_source_deps(Default::default()),
            CreateRandomSourceParams {
                kind: self.random_source.kind,
            },
            build_create_random_source_payload(Default::default()),
        ) {
            Ok(source) => source,
            Err(error) => {
                return Err(MeasureCaseErrorReturn::CreateRandomSource(error));
            }
        };
        let independent = match create_random_source(
            &build_create_random_source_deps(Default::default()),
            CreateRandomSourceParams {
                kind: self.random_source.kind,
            },
            build_create_random_source_payload(Default::default()),
        ) {
            Ok(source) => source,
            Err(error) => {
                return Err(MeasureCaseErrorReturn::CreateRandomSource(error));
            }
        };
        match create_encoding(
            &build_create_encoding_deps(CaseAtEncoding {
                pairing: &payload.adapter,
                hash_to_scalar: &*hashing.adapter,
                random: &*random.adapter,
                independent: &*independent.adapter,
                chain_forms: self.chain_forms,
                kem: self.kem,
                key_agreement: self.key_agreement,
                delivery_proof: self.delivery_proof,
                verifier: self.verifier,
                payload: self.payload,
                consumer: self.consumer,
            }),
            CreateEncodingParams {
                concrete: self.encoding.concrete,
                identifier: self.encoding.identifier,
            },
            build_create_encoding_payload(Default::default()),
        ) {
            Ok(success) => success.output,
            Err(error) => Err(MeasureCaseErrorReturn::CreateEncoding(error)),
        }
    }
}

pub fn run_measure_case<C: IMeasureCaseConsumer>(
    deps: &MeasureCaseDeps<C>,
    params: MeasureCaseParams,
    payload: MeasureCasePayload<'_>,
) -> MeasureCaseReturn<C::Output> {
    let _: RunMeasureCaseFn<C> = run_measure_case;
    match create_pairing(
        &build_create_pairing_deps(
            CaseAtPairing {
                hash_to_scalar: &params.hash_to_scalar,
                random_source: &params.random_source,
                encoding: &params.encoding,
                chain_forms: &params.chain_forms,
                kem: &params.kem,
                key_agreement: &params.key_agreement,
                delivery_proof: &params.delivery_proof,
                verifier: &params.verifier,
                payload: &payload,
                consumer: &deps.consumer,
            },
            Default::default(),
        ),
        params.pairing,
        build_create_pairing_payload(Default::default()),
    ) {
        Ok(success) => success.output,
        Err(error) => Err(MeasureCaseErrorReturn::CreatePairing(error)),
    }
}
