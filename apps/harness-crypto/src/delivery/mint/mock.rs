#![allow(dead_code, clippy::expect_used, clippy::too_many_arguments)]

use chain::{
    ConsumeChainFormsParams, ConsumeChainFormsPayload, CreateChainFormsParams, IChainForms,
    IChainFormsConsumer, build_create_chain_forms_deps, build_create_chain_forms_params,
    build_create_chain_forms_payload, create_chain_forms,
};
use core::marker::PhantomData;
use domain::{Secret, SecretConstructorParams};
use encoding::{
    ConsumeEncodingParams, ConsumeEncodingPayload, CreateEncodingParams, IDecoderAdapter,
    IEncoderAdapter, IEncodingConsumer, build_create_encoding_deps, build_create_encoding_params,
    build_create_encoding_payload, create_encoding,
};
use envelope::{
    ConsumeKeyAgreementParams, ConsumeKeyAgreementPayload, CreateKeyAgreementParams, EnvelopeCoins,
    EnvelopeComponentsParams, EnvelopeComponentsPayload, GenerateKeysParams, GenerateKeysPayload,
    IKeyAgreementAdapter, IKeyAgreementConsumer, KeyPairPublicKeysParams, KeyPairPublicKeysPayload,
    PublicKeysFromComponentsParams, PublicKeysFromComponentsPayload, WrapToParams, WrapToPayload,
    build_create_key_agreement_deps, build_create_key_agreement_params,
    build_create_key_agreement_payload, create_key_agreement,
};
use hash_to_scalar::{
    CreateHashToScalarParams, IHashToScalarAdapter, build_create_hash_to_scalar_deps,
    build_create_hash_to_scalar_params, build_create_hash_to_scalar_payload, create_hash_to_scalar,
};
use hashing::Root;
use kem::{
    ConsumeKemParams, ConsumeKemPayload, CreateKemParams, CredentialComponentsParams,
    CredentialComponentsPayload, DeriveIdentityParams, DeriveIdentityPayload,
    ICredentialKemAdapter, IKemConsumer, IdentityElementComponentsParams,
    IdentityElementComponentsPayload, IdentityScope, IssueParams, IssuePayload, KemIdentity,
    MasterScalarComponents, MasterScalarComponentsParams, MasterScalarComponentsPayload,
    ParameterSetComponentsParams, ParameterSetComponentsPayload, SetupParams, SetupPayload,
    SetupScope, build_create_kem_deps, build_create_kem_params, build_create_kem_payload,
    create_kem,
};
use pairing::{
    ConsumePairingParams, ConsumePairingPayload, CreatePairingParams, IPairingAdapter,
    IPairingArithmetic, IPairingConsumer, IPairingReference, ISampleUniformScalar,
    SampleUniformScalarParams, SampleUniformScalarPayload, build_create_pairing_deps,
    build_create_pairing_params, build_create_pairing_payload, create_pairing,
};
use proof::{
    AlgebraicMintStatement, ConsumeDeliveryProofParams, ConsumeDeliveryProofPayload,
    CreateDeliveryProofParams, DeliveryContext, IDeliveryProofAdapter, IDeliveryProofConsumer,
    MintPurpose, ProveMintParams, ProveMintPayload, build_create_delivery_proof_deps,
    build_create_delivery_proof_params, build_create_delivery_proof_payload, create_delivery_proof,
};
use random::{
    CreateRandomSourceParams, IRandomSourceAdapter, build_create_random_source_deps,
    build_create_random_source_params, build_create_random_source_payload, create_random_source,
};

use super::interface::{
    AdmittedKemIdentity, DeliveryHolder, IMintCaseConsumer, MintCaseDeps, MintCaseErrorReturn,
    MintCaseParams, MintCasePayload, MintCaseReturn, MintCaseSuccessReturn, MintDelivery, MintDeps,
    MintErrorReturn, MintFn, MintParams, MintPayload, MintReturn, MintSuccessReturn, MintWitnesses,
    RunMintCaseFn,
};

#[derive(Default)]
pub struct AdmittedKemIdentityOverrides<'a> {
    pub setup_scope: Option<SetupScope<'a>>,
    pub identity: Option<KemIdentity<'a>>,
}

pub fn build_admitted_kem_identity<'a>(
    overrides: AdmittedKemIdentityOverrides<'a>,
) -> AdmittedKemIdentity<'a> {
    AdmittedKemIdentity {
        setup_scope: overrides.setup_scope.unwrap_or(SetupScope::Entitlement),
        identity: overrides.identity.unwrap_or(KemIdentity::Entitlement {
            canonical: b"pkg@1.0.0#1",
        }),
    }
}

#[derive(Default)]
pub struct MintParamsOverrides;

pub fn build_mint_params(_overrides: MintParamsOverrides) -> MintParams {
    MintParams
}

#[derive(Default)]
pub struct MintPayloadOverrides<'a> {
    pub identity: Option<AdmittedKemIdentity<'a>>,
}

pub fn build_mint_payload<'a, F: IChainForms>(
    context: &'a DeliveryContext<F, MintPurpose>,
    overrides: MintPayloadOverrides<'a>,
) -> MintPayload<'a, F> {
    MintPayload {
        context,
        identity: overrides
            .identity
            .unwrap_or_else(|| build_admitted_kem_identity(Default::default())),
    }
}

#[derive(Default)]
pub struct MintDepsOverrides;

pub fn build_mint_deps<
    'a,
    K: ICredentialKemAdapter,
    A: IKeyAgreementAdapter<Pairing = K::Pairing>,
    D: IDeliveryProofAdapter<Pairing = K::Pairing>,
>(
    kem: &'a K,
    key_agreement: &'a A,
    delivery_proof: &'a D,
    random: &'a dyn IRandomSourceAdapter,
    _overrides: MintDepsOverrides,
) -> MintDeps<'a, K, A, D> {
    MintDeps {
        kem,
        key_agreement,
        delivery_proof,
        random,
    }
}

#[derive(Default)]
pub struct DeliveryHolderOverrides;

pub fn build_delivery_holder<A: IKeyAgreementAdapter>(
    key_pair: A::KeyPair,
    public_keys: A::PublicKeys,
    envelope: A::Envelope,
    _overrides: DeliveryHolderOverrides,
) -> DeliveryHolder<A> {
    DeliveryHolder {
        key_pair,
        public_keys,
        envelope,
    }
}

pub struct MintWitnessesOverrides<P: IPairingAdapter> {
    pub master_scalar: Option<MasterScalarComponents<P::Scalar>>,
    pub credential_randomness: Option<Secret<P::Scalar>>,
    pub coins: Option<EnvelopeCoins<P::Scalar>>,
}

impl<P: IPairingAdapter> Default for MintWitnessesOverrides<P> {
    fn default() -> Self {
        Self {
            master_scalar: None,
            credential_randomness: None,
            coins: None,
        }
    }
}

fn sampled_scalar<P: IPairingAdapter>(byte: u8) -> Secret<P::Scalar> {
    let Ok(uniform) = Secret::try_new(SecretConstructorParams {
        value: vec![byte; <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH],
    });
    P::Scalar::sample_from_uniform_bytes(
        SampleUniformScalarParams,
        SampleUniformScalarPayload { uniform },
    )
    .expect("constant bytes of the declared length are a valid scalar")
    .scalar
}

pub fn build_mint_witnesses<P: IPairingAdapter>(
    overrides: MintWitnessesOverrides<P>,
) -> MintWitnesses<P> {
    MintWitnesses {
        master_scalar: overrides
            .master_scalar
            .unwrap_or_else(|| MasterScalarComponents {
                value: sampled_scalar::<P>(0x01),
            }),
        credential_randomness: overrides
            .credential_randomness
            .unwrap_or_else(|| sampled_scalar::<P>(0x02)),
        coins: overrides.coins.unwrap_or_else(|| EnvelopeCoins {
            rho: sampled_scalar::<P>(0x03),
            sigma: sampled_scalar::<P>(0x04),
        }),
    }
}

#[derive(Default)]
pub struct MintDeliveryOverrides {
    pub draws: Option<Vec<u8>>,
}

pub fn build_mint_delivery<
    P: IPairingAdapter,
    K: ICredentialKemAdapter<Pairing = P>,
    A: IKeyAgreementAdapter<Pairing = P>,
    D: IDeliveryProofAdapter<Pairing = P>,
>(
    statement: AlgebraicMintStatement<D::Forms, P::G1, P::G2>,
    proof: D::Proof,
    parameter_set: K::ParameterSet,
    identity_element: K::IdentityElement,
    credential: K::Credential,
    witnesses: MintWitnesses<P>,
    recipient: DeliveryHolder<A>,
    overrides: MintDeliveryOverrides,
) -> MintDelivery<P, K, A, D> {
    MintDelivery {
        statement,
        proof,
        parameter_set,
        identity_element,
        credential,
        witnesses,
        recipient,
        draws: overrides.draws.unwrap_or_else(|| {
            (0x01u8..=0x06u8)
                .flat_map(|byte| {
                    vec![byte; <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH]
                })
                .collect()
        }),
    }
}

#[derive(Default)]
pub struct MintSuccessReturnOverrides;

pub fn build_mint_success_return<
    P: IPairingAdapter,
    K: ICredentialKemAdapter<Pairing = P>,
    A: IKeyAgreementAdapter<Pairing = P>,
    D: IDeliveryProofAdapter<Pairing = P>,
>(
    delivery: MintDelivery<P, K, A, D>,
    _overrides: MintSuccessReturnOverrides,
) -> MintSuccessReturn<P, K, A, D> {
    MintSuccessReturn { delivery }
}

fn uniform<S: ISampleUniformScalar>(byte: u8) -> Secret<Vec<u8>> {
    let Ok(uniform) = Secret::try_new(SecretConstructorParams {
        value: vec![byte; S::UNIFORM_BYTES_LENGTH],
    });
    uniform
}

fn copy_secret<P: IPairingAdapter>(secret: &Secret<P::Scalar>) -> Secret<P::Scalar> {
    let Ok(copy) = Secret::try_new(SecretConstructorParams {
        value: secret.expose().clone(),
    });
    copy
}

pub fn mock_mint<
    'a,
    P: IPairingAdapter,
    K: ICredentialKemAdapter<Pairing = P>,
    A: IKeyAgreementAdapter<Pairing = P>,
    D: IDeliveryProofAdapter<Pairing = P>,
>(
    deps: &MintDeps<'a, K, A, D>,
    _params: MintParams,
    payload: MintPayload<'_, D::Forms>,
) -> MintReturn<P, K, A, D> {
    let _: MintFn<'a, P, K, A, D> = mock_mint;
    let setup = match deps.kem.setup(
        SetupParams {
            scope: payload.identity.setup_scope,
        },
        SetupPayload {
            master_uniform: uniform::<P::Scalar>(0x01),
            u0_uniform: uniform::<P::Scalar>(0x02),
            u1_uniform: uniform::<P::Scalar>(0x03),
        },
    ) {
        Ok(success) => success,
        Err(error) => return Err(MintErrorReturn::Setup(error)),
    };
    let derived = match deps.kem.derive_identity(
        DeriveIdentityParams,
        DeriveIdentityPayload {
            parameter_set: &setup.parameter_set,
            identity: payload.identity.identity,
        },
    ) {
        Ok(success) => success,
        Err(error) => return Err(MintErrorReturn::DeriveIdentity(error)),
    };
    let issued = match deps.kem.issue(
        IssueParams,
        IssuePayload {
            parameter_set: &setup.parameter_set,
            master_scalar: &setup.master_scalar,
            identity_element: &derived.identity_element,
            uniform: uniform::<P::Scalar>(0x04),
        },
    ) {
        Ok(success) => success,
        Err(error) => return Err(MintErrorReturn::Issue(error)),
    };
    let generated = match deps.key_agreement.generate_keys(
        GenerateKeysParams,
        GenerateKeysPayload {
            x_uniform: uniform::<P::Scalar>(0x05),
            y_uniform: uniform::<P::Scalar>(0x06),
        },
    ) {
        Ok(success) => success,
        Err(error) => return Err(MintErrorReturn::GenerateKeys(error)),
    };
    let Ok(key_public_keys) = deps.key_agreement.key_pair_public_keys(
        KeyPairPublicKeysParams,
        KeyPairPublicKeysPayload {
            key_pair: &generated.key_pair,
        },
    );
    let admitted = match deps.key_agreement.public_keys_from_components(
        PublicKeysFromComponentsParams,
        PublicKeysFromComponentsPayload {
            components: key_public_keys.components,
            possession: &generated.possession,
        },
    ) {
        Ok(success) => success,
        Err(error) => return Err(MintErrorReturn::PublicKeysFromComponents(error)),
    };
    let Ok(credential) = deps.kem.credential_components(
        CredentialComponentsParams,
        CredentialComponentsPayload {
            credential: &issued.credential,
        },
    );
    let wrapped = match deps.key_agreement.wrap_to(
        WrapToParams,
        WrapToPayload {
            public_keys: &admitted.public_keys,
            credential: credential.components,
        },
    ) {
        Ok(success) => success,
        Err(error) => return Err(MintErrorReturn::WrapTo(error)),
    };
    let Ok(parameter_set_components) = deps.kem.parameter_set_components(
        ParameterSetComponentsParams,
        ParameterSetComponentsPayload {
            parameter_set: &setup.parameter_set,
        },
    );
    let Ok(identity_element_components) = deps.kem.identity_element_components(
        IdentityElementComponentsParams,
        IdentityElementComponentsPayload {
            identity_element: &derived.identity_element,
        },
    );
    let Ok(buyer_public_keys) = deps.key_agreement.key_pair_public_keys(
        KeyPairPublicKeysParams,
        KeyPairPublicKeysPayload {
            key_pair: &generated.key_pair,
        },
    );
    let Ok(envelope_components) = deps.key_agreement.envelope_components(
        EnvelopeComponentsParams,
        EnvelopeComponentsPayload {
            envelope: &wrapped.envelope,
        },
    );
    let statement = AlgebraicMintStatement {
        context: payload.context.clone(),
        hpub: parameter_set_components.components.hpub,
        identity_element: identity_element_components.components.element,
        buyer_keys: buyer_public_keys.components,
        envelope: envelope_components.components,
    };
    let Ok(prover_master_scalar) = deps.kem.master_scalar_components(
        MasterScalarComponentsParams,
        MasterScalarComponentsPayload {
            master_scalar: &setup.master_scalar,
        },
    );
    let Ok(witness_master_scalar) = deps.kem.master_scalar_components(
        MasterScalarComponentsParams,
        MasterScalarComponentsPayload {
            master_scalar: &setup.master_scalar,
        },
    );
    let credential_randomness = copy_secret::<P>(&issued.randomness);
    let rho = copy_secret::<P>(&wrapped.coins.rho);
    let sigma = copy_secret::<P>(&wrapped.coins.sigma);
    let proved = match deps.delivery_proof.prove_mint(
        ProveMintParams,
        ProveMintPayload {
            master_scalar: prover_master_scalar.components,
            credential_randomness: issued.randomness,
            coins: wrapped.coins,
            statement: &statement,
        },
    ) {
        Ok(success) => success,
        Err(error) => return Err(MintErrorReturn::ProveMint(error)),
    };
    Ok(build_mint_success_return(
        build_mint_delivery(
            statement,
            proved.proof,
            setup.parameter_set,
            derived.identity_element,
            issued.credential,
            build_mint_witnesses::<P>(MintWitnessesOverrides {
                master_scalar: Some(witness_master_scalar.components),
                credential_randomness: Some(credential_randomness),
                coins: Some(EnvelopeCoins { rho, sigma }),
            }),
            build_delivery_holder(
                generated.key_pair,
                admitted.public_keys,
                wrapped.envelope,
                Default::default(),
            ),
            Default::default(),
        ),
        Default::default(),
    ))
}

#[derive(Default)]
pub struct MintCaseParamsOverrides {
    pub pairing: Option<CreatePairingParams>,
    pub hash_to_scalar: Option<CreateHashToScalarParams>,
    pub encoding: Option<CreateEncodingParams>,
    pub chain_forms: Option<CreateChainFormsParams>,
    pub kem: Option<CreateKemParams>,
    pub key_agreement: Option<CreateKeyAgreementParams>,
    pub delivery_proof: Option<CreateDeliveryProofParams>,
    pub random_source: Option<CreateRandomSourceParams>,
}

pub fn build_mint_case_params(overrides: MintCaseParamsOverrides) -> MintCaseParams {
    MintCaseParams {
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
        random_source: overrides
            .random_source
            .unwrap_or_else(|| build_create_random_source_params(Default::default())),
    }
}

#[derive(Default)]
pub struct MintCaseDepsOverrides;

pub fn build_mint_case_deps<C: IMintCaseConsumer>(
    consumer: C,
    _overrides: MintCaseDepsOverrides,
) -> MintCaseDeps<C> {
    MintCaseDeps { consumer }
}

#[derive(Default)]
pub struct MintCasePayloadOverrides;

pub fn build_mint_case_payload<'a>(
    canonical: &'a [u8],
    identity_hash: &'a Root,
    _overrides: MintCasePayloadOverrides,
) -> MintCasePayload<'a> {
    MintCasePayload {
        canonical,
        identity_hash,
    }
}

#[derive(Default)]
pub struct MintCaseSuccessReturnOverrides;

pub fn build_mint_case_success_return<O>(
    output: O,
    _overrides: MintCaseSuccessReturnOverrides,
) -> MintCaseSuccessReturn<O> {
    MintCaseSuccessReturn { output }
}

struct CaseAtDeliveryProof<'a, P: IPairingArithmetic, F: IChainForms, K, A, C: IMintCaseConsumer> {
    pairing: &'a P,
    kem: &'a K,
    key_agreement: &'a A,
    source: &'a dyn IRandomSourceAdapter,
    identity: AdmittedKemIdentity<'a>,
    consumer: &'a C,
    marker: PhantomData<F>,
}

impl<'a, P, F, K, A, C> IDeliveryProofConsumer<P, F> for CaseAtDeliveryProof<'a, P, F, K, A, C>
where
    P: IPairingArithmetic + IPairingReference,
    F: IChainForms,
    K: ICredentialKemAdapter<Pairing = P>,
    A: IKeyAgreementAdapter<Pairing = P>,
    C: IMintCaseConsumer,
{
    type Output = MintCaseReturn<C::Output>;

    fn consume_delivery_proof<D: IDeliveryProofAdapter<Pairing = P, Forms = F>>(
        &self,
        _params: ConsumeDeliveryProofParams,
        payload: ConsumeDeliveryProofPayload<D>,
    ) -> Self::Output {
        Ok(MintCaseSuccessReturn {
            output: self.consumer.consume_mint_case(
                self.pairing,
                self.kem,
                self.key_agreement,
                &payload.adapter,
                self.source,
                self.identity,
            ),
        })
    }
}

struct CaseAtKeyAgreement<'a, P: IPairingArithmetic, E, F: IChainForms, K, C: IMintCaseConsumer> {
    pairing: &'a P,
    hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
    encoder: &'a E,
    source: &'a dyn IRandomSourceAdapter,
    kem: &'a K,
    delivery_proof: &'a CreateDeliveryProofParams,
    identity: AdmittedKemIdentity<'a>,
    consumer: &'a C,
    marker: PhantomData<F>,
}

impl<'a, P, E, F, K, C> IKeyAgreementConsumer<P> for CaseAtKeyAgreement<'a, P, E, F, K, C>
where
    P: IPairingArithmetic + IPairingReference,
    E: IEncoderAdapter,
    F: IChainForms,
    K: ICredentialKemAdapter<Pairing = P>,
    C: IMintCaseConsumer,
{
    type Output = MintCaseReturn<C::Output>;

    fn consume_key_agreement<A: IKeyAgreementAdapter<Pairing = P>>(
        &self,
        _params: ConsumeKeyAgreementParams,
        payload: ConsumeKeyAgreementPayload<A>,
    ) -> Self::Output {
        match create_delivery_proof(
            &build_create_delivery_proof_deps(
                self.pairing,
                self.hashing,
                self.encoder,
                self.source,
                CaseAtDeliveryProof {
                    pairing: self.pairing,
                    kem: self.kem,
                    key_agreement: &payload.adapter,
                    source: self.source,
                    identity: self.identity,
                    consumer: self.consumer,
                    marker: PhantomData::<F>,
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
            Err(error) => Err(MintCaseErrorReturn::CreateDeliveryProof(error)),
        }
    }
}

struct CaseAtKem<'a, P: IPairingArithmetic, E, F: IChainForms, C: IMintCaseConsumer> {
    pairing: &'a P,
    hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
    encoder: &'a E,
    source: &'a dyn IRandomSourceAdapter,
    key_agreement: &'a CreateKeyAgreementParams,
    delivery_proof: &'a CreateDeliveryProofParams,
    payload: &'a MintCasePayload<'a>,
    consumer: &'a C,
    marker: PhantomData<F>,
}

impl<'a, P, E, F, C> IKemConsumer<P> for CaseAtKem<'a, P, E, F, C>
where
    P: IPairingArithmetic + IPairingReference,
    E: IEncoderAdapter,
    F: IChainForms,
    C: IMintCaseConsumer,
{
    type Output = MintCaseReturn<C::Output>;

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
                    root: self.payload.identity_hash,
                },
                identity: KemIdentity::Asset {
                    root: self.payload.identity_hash,
                },
            },
        };
        match create_key_agreement(
            &build_create_key_agreement_deps(
                self.pairing,
                self.hashing,
                self.encoder,
                self.source,
                CaseAtKeyAgreement {
                    pairing: self.pairing,
                    hashing: self.hashing,
                    encoder: self.encoder,
                    source: self.source,
                    kem: &payload.adapter,
                    delivery_proof: self.delivery_proof,
                    identity,
                    consumer: self.consumer,
                    marker: PhantomData::<F>,
                },
                Default::default(),
            ),
            *self.key_agreement,
            build_create_key_agreement_payload(Default::default()),
        ) {
            Ok(success) => success.output,
            Err(error) => Err(MintCaseErrorReturn::CreateKeyAgreement(error)),
        }
    }
}

struct CaseAtChainForms<'a, P: IPairingArithmetic, E, C: IMintCaseConsumer> {
    pairing: &'a P,
    hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
    encoder: &'a E,
    source: &'a dyn IRandomSourceAdapter,
    kem: &'a CreateKemParams,
    key_agreement: &'a CreateKeyAgreementParams,
    delivery_proof: &'a CreateDeliveryProofParams,
    payload: &'a MintCasePayload<'a>,
    consumer: &'a C,
}

impl<'a, P, E, C> IChainFormsConsumer for CaseAtChainForms<'a, P, E, C>
where
    P: IPairingArithmetic + IPairingReference,
    E: IEncoderAdapter,
    C: IMintCaseConsumer,
{
    type Output = MintCaseReturn<C::Output>;

    fn consume_chain_forms<F: IChainForms>(
        &self,
        _params: ConsumeChainFormsParams,
        _payload: ConsumeChainFormsPayload<F>,
    ) -> Self::Output {
        match create_kem(
            &build_create_kem_deps(
                self.pairing,
                self.hashing,
                CaseAtKem {
                    pairing: self.pairing,
                    hashing: self.hashing,
                    encoder: self.encoder,
                    source: self.source,
                    key_agreement: self.key_agreement,
                    delivery_proof: self.delivery_proof,
                    payload: self.payload,
                    consumer: self.consumer,
                    marker: PhantomData::<F>,
                },
                Default::default(),
            ),
            *self.kem,
            build_create_kem_payload(Default::default()),
        ) {
            Ok(success) => success.output,
            Err(error) => Err(MintCaseErrorReturn::CreateKem(error)),
        }
    }
}

struct CaseAtEncoding<'a, P: IPairingArithmetic, C: IMintCaseConsumer> {
    pairing: &'a P,
    hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
    source: &'a dyn IRandomSourceAdapter,
    chain_forms: &'a CreateChainFormsParams,
    kem: &'a CreateKemParams,
    key_agreement: &'a CreateKeyAgreementParams,
    delivery_proof: &'a CreateDeliveryProofParams,
    payload: &'a MintCasePayload<'a>,
    consumer: &'a C,
}

impl<'a, P, C> IEncodingConsumer for CaseAtEncoding<'a, P, C>
where
    P: IPairingArithmetic + IPairingReference,
    C: IMintCaseConsumer,
{
    type Output = MintCaseReturn<C::Output>;

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
                    kem: self.kem,
                    key_agreement: self.key_agreement,
                    delivery_proof: self.delivery_proof,
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
            Err(error) => Err(MintCaseErrorReturn::CreateChainForms(error)),
        }
    }
}

struct CaseAtPairing<'a, C: IMintCaseConsumer> {
    hash_to_scalar: &'a CreateHashToScalarParams,
    random_source: &'a CreateRandomSourceParams,
    encoding: &'a CreateEncodingParams,
    chain_forms: &'a CreateChainFormsParams,
    kem: &'a CreateKemParams,
    key_agreement: &'a CreateKeyAgreementParams,
    delivery_proof: &'a CreateDeliveryProofParams,
    payload: &'a MintCasePayload<'a>,
    consumer: &'a C,
}

impl<C: IMintCaseConsumer> IPairingConsumer for CaseAtPairing<'_, C> {
    type Output = MintCaseReturn<C::Output>;

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
                return Err(MintCaseErrorReturn::CreateHashToScalar(error));
            }
        };
        let source = match create_random_source(
            &build_create_random_source_deps(Default::default()),
            CreateRandomSourceParams {
                kind: self.random_source.kind,
            },
            build_create_random_source_payload(Default::default()),
        ) {
            Ok(source) => source,
            Err(error) => {
                return Err(MintCaseErrorReturn::CreateRandomSource(error));
            }
        };
        match create_encoding(
            &build_create_encoding_deps(CaseAtEncoding {
                pairing: &payload.adapter,
                hashing: &*hashing.adapter,
                source: &*source.adapter,
                chain_forms: self.chain_forms,
                kem: self.kem,
                key_agreement: self.key_agreement,
                delivery_proof: self.delivery_proof,
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
            Err(error) => Err(MintCaseErrorReturn::CreateEncoding(error)),
        }
    }
}

pub fn run_mint_case<C: IMintCaseConsumer>(
    deps: &MintCaseDeps<C>,
    params: MintCaseParams,
    payload: MintCasePayload<'_>,
) -> MintCaseReturn<C::Output> {
    let _: RunMintCaseFn<C> = run_mint_case;
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
                payload: &payload,
                consumer: &deps.consumer,
            },
            Default::default(),
        ),
        params.pairing,
        build_create_pairing_payload(Default::default()),
    ) {
        Ok(success) => success.output,
        Err(error) => Err(MintCaseErrorReturn::CreatePairing(error)),
    }
}
