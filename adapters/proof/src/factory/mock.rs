#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::as_conversions
)]

use chain::IChainForms;
use core::marker::PhantomData;
use domain::{
    AssetIdentityHash, AssetIdentityHashConstructorParamsOverrides, ParameterSetIdentifier,
    ParameterSetIdentifierConstructorParamsOverrides, SecretConstructorParamsOverrides,
    SuiteIdentifier, SuiteIdentifierConstructorParamsOverrides, build_asset_identity_hash,
    build_parameter_set_identifier, build_secret, build_suite_identifier,
};
use encoding::IEncoderAdapter;
use envelope::{EnvelopeAlgebra, build_envelope_components, build_public_keys_components};
use pairing::{
    AddG1Params, AddG1Payload, AddG2Params, AddG2Payload, G1GeneratorParams, G1GeneratorPayload,
    G2GeneratorParams, G2GeneratorPayload, IPairingAdapter, IPairingArithmetic,
    ISampleUniformScalar, SampleUniformScalarParams, SampleUniformScalarPayload,
    VerifierGroupArithmetic,
};

use crate::mint_statement::provides::{
    DELIVERY_PURPOSE_MINT, DELIVERY_PURPOSE_TRANSFER, DELIVERY_STATEMENT_VERSION_ONE, MintPurpose,
    TransferPurpose,
};
use crate::schnorr_fs::provides::{SCHNORR_FS_CHALLENGE_TAG, SCHNORR_FS_WEIGHT_TAG};

use super::interface::{
    AlgebraicMintStatement, AlgebraicTransferStatement, ConsumeDeliveryProofParams,
    ConsumeDeliveryProofPayload, CreateDeliveryProofDeps, CreateDeliveryProofParams,
    CreateDeliveryProofPayload, CreateDeliveryProofReturn, CreateDeliveryProofSuccessReturn,
    DELIVERY_PROOF_INTERFACE_VERSION, DeliveryContext, DeliveryProofComponents,
    DeliveryProofConcrete, DeliveryProofDeclaration, IDeliveryProofAdapter, IDeliveryProofConsumer,
    MintResponses, MintSecondGroupFirstMessages, ProofComponentsParams, ProofComponentsPayload,
    ProofComponentsReturn, ProofComponentsSuccessReturn, ProofFromComponentsParams,
    ProofFromComponentsPayload, ProofFromComponentsReturn, ProofFromComponentsSuccessReturn,
    ProveMintParams, ProveMintPayload, ProveMintReturn, ProveMintSuccessReturn,
    ProveTransferParams, ProveTransferPayload, ProveTransferReturn, ProveTransferSuccessReturn,
    TransferResponses, TransferSecondGroupFirstMessages, VerifyParams, VerifyPayload, VerifyReturn,
    VerifySuccessReturn,
};

impl Default for MintPurpose {
    fn default() -> Self {
        DELIVERY_PURPOSE_MINT
    }
}

impl Default for TransferPurpose {
    fn default() -> Self {
        DELIVERY_PURPOSE_TRANSFER
    }
}

#[derive(Default)]
pub struct DeliveryProofDeclarationOverrides {
    pub algebras: Option<&'static [EnvelopeAlgebra]>,
    pub verifier_forms: Option<&'static [VerifierGroupArithmetic]>,
    pub statement_versions: Option<&'static [u16]>,
    pub challenge_tag: Option<&'static [u8]>,
    pub weight_tag: Option<&'static [u8]>,
    pub adapter_version: Option<u32>,
    pub interface_version: Option<u32>,
}

pub fn build_delivery_proof_declaration(
    overrides: DeliveryProofDeclarationOverrides,
) -> DeliveryProofDeclaration {
    DeliveryProofDeclaration {
        algebras: overrides
            .algebras
            .unwrap_or(&[EnvelopeAlgebra::PairingElGamal]),
        verifier_forms: overrides.verifier_forms.unwrap_or(&[
            VerifierGroupArithmetic::BothGroups,
            VerifierGroupArithmetic::FirstGroupOnly,
        ]),
        statement_versions: overrides
            .statement_versions
            .unwrap_or(&[DELIVERY_STATEMENT_VERSION_ONE]),
        challenge_tag: overrides.challenge_tag.unwrap_or(SCHNORR_FS_CHALLENGE_TAG),
        weight_tag: overrides.weight_tag.unwrap_or(SCHNORR_FS_WEIGHT_TAG),
        adapter_version: overrides.adapter_version.unwrap_or(1),
        interface_version: overrides
            .interface_version
            .unwrap_or(DELIVERY_PROOF_INTERFACE_VERSION),
    }
}

pub struct DeliveryContextOverrides<F: IChainForms, Purpose> {
    pub suite_identifier: Option<SuiteIdentifier>,
    pub chain: Option<F::ChainIdentifier>,
    pub entitlement_contract: Option<F::Identity>,
    pub asset_identity_hash: Option<AssetIdentityHash>,
    pub parameter_set_identifier: Option<ParameterSetIdentifier>,
    pub source_entitlement: Option<F::Entitlement>,
    pub target_entitlement: Option<F::Entitlement>,
    pub old_interval: Option<F::Interval>,
    pub new_interval: Option<F::Interval>,
    pub purpose: Option<Purpose>,
    pub seller: Option<F::Identity>,
    pub buyer: Option<F::Identity>,
    pub expiry: Option<u64>,
}

impl<F: IChainForms, Purpose> Default for DeliveryContextOverrides<F, Purpose> {
    fn default() -> Self {
        Self {
            suite_identifier: None,
            chain: None,
            entitlement_contract: None,
            asset_identity_hash: None,
            parameter_set_identifier: None,
            source_entitlement: None,
            target_entitlement: None,
            old_interval: None,
            new_interval: None,
            purpose: None,
            seller: None,
            buyer: None,
            expiry: None,
        }
    }
}

pub fn build_delivery_context<F: IChainForms, Purpose: Default>(
    overrides: DeliveryContextOverrides<F, Purpose>,
) -> DeliveryContext<F, Purpose>
where
    F::ChainIdentifier: Default,
    F::Entitlement: Default,
    F::Identity: Default,
    F::Interval: Default,
{
    DeliveryContext {
        suite_identifier: overrides.suite_identifier.unwrap_or_else(|| {
            build_suite_identifier(SuiteIdentifierConstructorParamsOverrides::default())
        }),
        chain: overrides.chain.unwrap_or_default(),
        entitlement_contract: overrides.entitlement_contract.unwrap_or_default(),
        asset_identity_hash: overrides.asset_identity_hash.unwrap_or_else(|| {
            build_asset_identity_hash(AssetIdentityHashConstructorParamsOverrides::default())
        }),
        parameter_set_identifier: overrides.parameter_set_identifier.unwrap_or_else(|| {
            build_parameter_set_identifier(
                ParameterSetIdentifierConstructorParamsOverrides::default(),
            )
        }),
        source_entitlement: overrides.source_entitlement.unwrap_or_default(),
        target_entitlement: overrides.target_entitlement.unwrap_or_default(),
        old_interval: overrides.old_interval.unwrap_or_default(),
        new_interval: overrides.new_interval.unwrap_or_default(),
        purpose: overrides.purpose.unwrap_or_default(),
        seller: overrides.seller.unwrap_or_default(),
        buyer: overrides.buyer.unwrap_or_default(),
        expiry: overrides.expiry.unwrap_or(1_700_000_000),
    }
}

fn sample_scalar_secret<P: IPairingAdapter>(fill: u8) -> domain::Secret<P::Scalar> {
    P::Scalar::sample_from_uniform_bytes(
        SampleUniformScalarParams,
        SampleUniformScalarPayload {
            uniform: build_secret(SecretConstructorParamsOverrides {
                value: Some(vec![fill; P::Scalar::UNIFORM_BYTES_LENGTH]),
            }),
        },
    )
    .unwrap()
    .scalar
}

fn sample_scalar<P: IPairingAdapter>(fill: u8) -> P::Scalar {
    sample_scalar_secret::<P>(fill).expose().clone()
}

fn g1_element<P: IPairingAdapter>(pairing: &P, additions: usize) -> P::G1 {
    let generator = pairing
        .g1_generator(G1GeneratorParams, G1GeneratorPayload)
        .unwrap()
        .point;
    let mut element = generator.clone();
    for _ in 0..additions {
        element = pairing
            .add_g1(
                AddG1Params,
                AddG1Payload {
                    left: element,
                    right: generator.clone(),
                },
            )
            .unwrap()
            .sum;
    }
    element
}

fn g2_element<P: IPairingAdapter>(pairing: &P, additions: usize) -> P::G2 {
    let generator = pairing
        .g2_generator(G2GeneratorParams, G2GeneratorPayload)
        .unwrap()
        .point;
    let mut element = generator.clone();
    for _ in 0..additions {
        element = pairing
            .add_g2(
                AddG2Params,
                AddG2Payload {
                    left: element,
                    right: generator.clone(),
                },
            )
            .unwrap()
            .sum;
    }
    element
}

pub struct AlgebraicMintStatementOverrides<F: IChainForms, G1, G2> {
    pub context: Option<DeliveryContext<F, MintPurpose>>,
    pub hpub: Option<G2>,
    pub identity_element: Option<G1>,
    pub buyer_keys: Option<envelope::PublicKeysComponents<G1, G2>>,
    pub envelope: Option<envelope::EnvelopeComponents<G1, G2>>,
}

impl<F: IChainForms, G1, G2> Default for AlgebraicMintStatementOverrides<F, G1, G2> {
    fn default() -> Self {
        Self {
            context: None,
            hpub: None,
            identity_element: None,
            buyer_keys: None,
            envelope: None,
        }
    }
}

pub fn build_algebraic_mint_statement<P: IPairingAdapter, F: IChainForms>(
    pairing: &P,
    overrides: AlgebraicMintStatementOverrides<F, P::G1, P::G2>,
) -> AlgebraicMintStatement<F, P::G1, P::G2>
where
    F::ChainIdentifier: Default,
    F::Entitlement: Default,
    F::Identity: Default,
    F::Interval: Default,
{
    AlgebraicMintStatement {
        context: overrides
            .context
            .unwrap_or_else(|| build_delivery_context(Default::default())),
        hpub: overrides.hpub.unwrap_or_else(|| g2_element(pairing, 1)),
        identity_element: overrides
            .identity_element
            .unwrap_or_else(|| g1_element(pairing, 2)),
        buyer_keys: overrides
            .buyer_keys
            .unwrap_or_else(|| build_public_keys_components(pairing, Default::default())),
        envelope: overrides
            .envelope
            .unwrap_or_else(|| build_envelope_components(pairing, Default::default())),
    }
}

pub struct AlgebraicTransferStatementOverrides<F: IChainForms, G1, G2> {
    pub context: Option<DeliveryContext<F, TransferPurpose>>,
    pub identity_element: Option<G1>,
    pub seller_keys: Option<envelope::PublicKeysComponents<G1, G2>>,
    pub buyer_keys: Option<envelope::PublicKeysComponents<G1, G2>>,
    pub old_envelope: Option<envelope::EnvelopeComponents<G1, G2>>,
    pub new_envelope: Option<envelope::EnvelopeComponents<G1, G2>>,
}

impl<F: IChainForms, G1, G2> Default for AlgebraicTransferStatementOverrides<F, G1, G2> {
    fn default() -> Self {
        Self {
            context: None,
            identity_element: None,
            seller_keys: None,
            buyer_keys: None,
            old_envelope: None,
            new_envelope: None,
        }
    }
}

pub fn build_algebraic_transfer_statement<P: IPairingAdapter, F: IChainForms>(
    pairing: &P,
    overrides: AlgebraicTransferStatementOverrides<F, P::G1, P::G2>,
) -> AlgebraicTransferStatement<F, P::G1, P::G2>
where
    F::ChainIdentifier: Default,
    F::Entitlement: Default,
    F::Identity: Default,
    F::Interval: Default,
{
    AlgebraicTransferStatement {
        context: overrides
            .context
            .unwrap_or_else(|| build_delivery_context::<F, TransferPurpose>(Default::default())),
        identity_element: overrides
            .identity_element
            .unwrap_or_else(|| g1_element(pairing, 2)),
        seller_keys: overrides
            .seller_keys
            .unwrap_or_else(|| build_public_keys_components(pairing, Default::default())),
        buyer_keys: overrides
            .buyer_keys
            .unwrap_or_else(|| build_public_keys_components(pairing, Default::default())),
        old_envelope: overrides
            .old_envelope
            .unwrap_or_else(|| build_envelope_components(pairing, Default::default())),
        new_envelope: overrides
            .new_envelope
            .unwrap_or_else(|| build_envelope_components(pairing, Default::default())),
    }
}

pub enum DeliveryProofComponentsVariant {
    MintBothGroups,
    MintFirstGroupOnly,
    TransferBothGroups,
    TransferFirstGroupOnly,
}

pub struct MintResponsesOverrides<S> {
    pub alpha: Option<S>,
    pub r: Option<S>,
    pub rho: Option<S>,
    pub sigma: Option<S>,
}

impl<S> Default for MintResponsesOverrides<S> {
    fn default() -> Self {
        Self {
            alpha: None,
            r: None,
            rho: None,
            sigma: None,
        }
    }
}

pub struct TransferResponsesOverrides<S> {
    pub x: Option<S>,
    pub y: Option<S>,
    pub s: Option<S>,
    pub rho: Option<S>,
    pub sigma: Option<S>,
}

impl<S> Default for TransferResponsesOverrides<S> {
    fn default() -> Self {
        Self {
            x: None,
            y: None,
            s: None,
            rho: None,
            sigma: None,
        }
    }
}

pub struct MintSecondGroupFirstMessagesOverrides<G2> {
    pub hpub: Option<G2>,
    pub d1: Option<G2>,
    pub d2: Option<G2>,
}

impl<G2> Default for MintSecondGroupFirstMessagesOverrides<G2> {
    fn default() -> Self {
        Self {
            hpub: None,
            d1: None,
            d2: None,
        }
    }
}

pub struct TransferSecondGroupFirstMessagesOverrides<G2> {
    pub pk2: Option<G2>,
    pub d1: Option<G2>,
    pub d2: Option<G2>,
}

impl<G2> Default for TransferSecondGroupFirstMessagesOverrides<G2> {
    fn default() -> Self {
        Self {
            pk2: None,
            d1: None,
            d2: None,
        }
    }
}

pub struct DeliveryProofComponentsOverrides<S, G2> {
    pub variant: Option<DeliveryProofComponentsVariant>,
    pub challenge: Option<S>,
    pub mint_responses: Option<MintResponsesOverrides<S>>,
    pub transfer_responses: Option<TransferResponsesOverrides<S>>,
    pub mint_first_messages: Option<MintSecondGroupFirstMessagesOverrides<G2>>,
    pub transfer_first_messages: Option<TransferSecondGroupFirstMessagesOverrides<G2>>,
}

impl<S, G2> Default for DeliveryProofComponentsOverrides<S, G2> {
    fn default() -> Self {
        Self {
            variant: None,
            challenge: None,
            mint_responses: None,
            transfer_responses: None,
            mint_first_messages: None,
            transfer_first_messages: None,
        }
    }
}

fn build_mint_responses<P: IPairingAdapter>(
    overrides: MintResponsesOverrides<P::Scalar>,
) -> MintResponses<P::Scalar> {
    MintResponses {
        alpha: overrides.alpha.unwrap_or_else(|| sample_scalar::<P>(0x61)),
        r: overrides.r.unwrap_or_else(|| sample_scalar::<P>(0x62)),
        rho: overrides.rho.unwrap_or_else(|| sample_scalar::<P>(0x63)),
        sigma: overrides.sigma.unwrap_or_else(|| sample_scalar::<P>(0x64)),
    }
}

fn build_transfer_responses<P: IPairingAdapter>(
    overrides: TransferResponsesOverrides<P::Scalar>,
) -> TransferResponses<P::Scalar> {
    TransferResponses {
        x: overrides.x.unwrap_or_else(|| sample_scalar::<P>(0x65)),
        y: overrides.y.unwrap_or_else(|| sample_scalar::<P>(0x66)),
        s: overrides.s.unwrap_or_else(|| sample_scalar::<P>(0x67)),
        rho: overrides.rho.unwrap_or_else(|| sample_scalar::<P>(0x68)),
        sigma: overrides.sigma.unwrap_or_else(|| sample_scalar::<P>(0x69)),
    }
}

fn build_mint_second_group_first_messages<P: IPairingAdapter>(
    pairing: &P,
    overrides: MintSecondGroupFirstMessagesOverrides<P::G2>,
) -> MintSecondGroupFirstMessages<P::G2> {
    MintSecondGroupFirstMessages {
        hpub: overrides.hpub.unwrap_or_else(|| g2_element(pairing, 1)),
        d1: overrides.d1.unwrap_or_else(|| g2_element(pairing, 2)),
        d2: overrides.d2.unwrap_or_else(|| g2_element(pairing, 3)),
    }
}

fn build_transfer_second_group_first_messages<P: IPairingAdapter>(
    pairing: &P,
    overrides: TransferSecondGroupFirstMessagesOverrides<P::G2>,
) -> TransferSecondGroupFirstMessages<P::G2> {
    TransferSecondGroupFirstMessages {
        pk2: overrides.pk2.unwrap_or_else(|| g2_element(pairing, 1)),
        d1: overrides.d1.unwrap_or_else(|| g2_element(pairing, 2)),
        d2: overrides.d2.unwrap_or_else(|| g2_element(pairing, 3)),
    }
}

pub fn build_delivery_proof_components<P: IPairingAdapter>(
    pairing: &P,
    overrides: DeliveryProofComponentsOverrides<P::Scalar, P::G2>,
) -> DeliveryProofComponents<P::Scalar, P::G2> {
    let challenge = overrides
        .challenge
        .unwrap_or_else(|| sample_scalar::<P>(0x60));
    match overrides
        .variant
        .unwrap_or(DeliveryProofComponentsVariant::MintBothGroups)
    {
        DeliveryProofComponentsVariant::MintBothGroups => DeliveryProofComponents::MintBothGroups {
            challenge,
            responses: build_mint_responses::<P>(overrides.mint_responses.unwrap_or_default()),
        },
        DeliveryProofComponentsVariant::MintFirstGroupOnly => {
            DeliveryProofComponents::MintFirstGroupOnly {
                challenge,
                responses: build_mint_responses::<P>(overrides.mint_responses.unwrap_or_default()),
                first_messages: build_mint_second_group_first_messages::<P>(
                    pairing,
                    overrides.mint_first_messages.unwrap_or_default(),
                ),
            }
        }
        DeliveryProofComponentsVariant::TransferBothGroups => {
            DeliveryProofComponents::TransferBothGroups {
                challenge,
                responses: build_transfer_responses::<P>(
                    overrides.transfer_responses.unwrap_or_default(),
                ),
            }
        }
        DeliveryProofComponentsVariant::TransferFirstGroupOnly => {
            DeliveryProofComponents::TransferFirstGroupOnly {
                challenge,
                responses: build_transfer_responses::<P>(
                    overrides.transfer_responses.unwrap_or_default(),
                ),
                first_messages: build_transfer_second_group_first_messages::<P>(
                    pairing,
                    overrides.transfer_first_messages.unwrap_or_default(),
                ),
            }
        }
    }
}

#[derive(Default)]
pub struct ProveMintSuccessReturnOverrides<PR> {
    pub proof: Option<PR>,
}

pub fn build_prove_mint_success_return<PR: Default>(
    overrides: ProveMintSuccessReturnOverrides<PR>,
) -> ProveMintSuccessReturn<PR> {
    ProveMintSuccessReturn {
        proof: overrides.proof.unwrap_or_default(),
    }
}

#[derive(Default)]
pub struct ProveTransferSuccessReturnOverrides<PR> {
    pub proof: Option<PR>,
}

pub fn build_prove_transfer_success_return<PR: Default>(
    overrides: ProveTransferSuccessReturnOverrides<PR>,
) -> ProveTransferSuccessReturn<PR> {
    ProveTransferSuccessReturn {
        proof: overrides.proof.unwrap_or_default(),
    }
}

#[derive(Default)]
pub struct VerifySuccessReturnOverrides {
    pub is_valid: Option<bool>,
}

pub fn build_verify_success_return(overrides: VerifySuccessReturnOverrides) -> VerifySuccessReturn {
    VerifySuccessReturn {
        is_valid: overrides.is_valid.unwrap_or(true),
    }
}

pub struct ProofComponentsSuccessReturnOverrides<S, G2> {
    pub components: Option<DeliveryProofComponents<S, G2>>,
}

impl<S, G2> Default for ProofComponentsSuccessReturnOverrides<S, G2> {
    fn default() -> Self {
        Self { components: None }
    }
}

pub fn build_proof_components_success_return<P: IPairingAdapter>(
    pairing: &P,
    overrides: ProofComponentsSuccessReturnOverrides<P::Scalar, P::G2>,
) -> ProofComponentsSuccessReturn<P::Scalar, P::G2> {
    ProofComponentsSuccessReturn {
        components: overrides
            .components
            .unwrap_or_else(|| build_delivery_proof_components(pairing, Default::default())),
    }
}

#[derive(Default)]
pub struct ProofFromComponentsSuccessReturnOverrides<PR> {
    pub proof: Option<PR>,
}

pub fn build_proof_from_components_success_return<PR: Default>(
    overrides: ProofFromComponentsSuccessReturnOverrides<PR>,
) -> ProofFromComponentsSuccessReturn<PR> {
    ProofFromComponentsSuccessReturn {
        proof: overrides.proof.unwrap_or_default(),
    }
}

pub struct MockIDeliveryProofAdapter<'a, P, F, PR> {
    pub pairing: &'a P,
    pub forms: PhantomData<F>,
    pub proof: PhantomData<PR>,
}

impl<'a, P, F, PR> IDeliveryProofAdapter for MockIDeliveryProofAdapter<'a, P, F, PR>
where
    P: IPairingAdapter,
    F: IChainForms,
    PR: Default,
{
    const DECLARATION: DeliveryProofDeclaration = DeliveryProofDeclaration {
        algebras: &[EnvelopeAlgebra::PairingElGamal],
        verifier_forms: &[
            VerifierGroupArithmetic::BothGroups,
            VerifierGroupArithmetic::FirstGroupOnly,
        ],
        statement_versions: &[DELIVERY_STATEMENT_VERSION_ONE],
        challenge_tag: SCHNORR_FS_CHALLENGE_TAG,
        weight_tag: SCHNORR_FS_WEIGHT_TAG,
        adapter_version: 1,
        interface_version: DELIVERY_PROOF_INTERFACE_VERSION,
    };

    type Pairing = P;
    type Forms = F;
    type Proof = PR;

    fn prove_mint(
        &self,
        _params: ProveMintParams,
        _payload: ProveMintPayload<'_, F, P::Scalar, P::G1, P::G2>,
    ) -> ProveMintReturn<Self::Proof> {
        Ok(build_prove_mint_success_return(Default::default()))
    }

    fn prove_transfer(
        &self,
        _params: ProveTransferParams,
        _payload: ProveTransferPayload<'_, F, P::Scalar, P::G1, P::G2>,
    ) -> ProveTransferReturn<Self::Proof> {
        Ok(build_prove_transfer_success_return(Default::default()))
    }

    fn verify(
        &self,
        _params: VerifyParams,
        _payload: VerifyPayload<'_, F, P::G1, P::G2, Self::Proof>,
    ) -> VerifyReturn {
        Ok(build_verify_success_return(Default::default()))
    }

    fn proof_components(
        &self,
        _params: ProofComponentsParams,
        _payload: ProofComponentsPayload<'_, Self::Proof>,
    ) -> ProofComponentsReturn<P::Scalar, P::G2> {
        Ok(build_proof_components_success_return(
            self.pairing,
            Default::default(),
        ))
    }

    fn proof_from_components(
        &self,
        _params: ProofFromComponentsParams,
        _payload: ProofFromComponentsPayload<P::Scalar, P::G2>,
    ) -> ProofFromComponentsReturn<Self::Proof> {
        Ok(build_proof_from_components_success_return(
            Default::default(),
        ))
    }
}

#[derive(Default)]
pub struct CreateDeliveryProofParamsOverrides {
    pub concrete: Option<DeliveryProofConcrete>,
    pub algebra: Option<EnvelopeAlgebra>,
    pub verifier_form: Option<VerifierGroupArithmetic>,
    pub statement_version: Option<u16>,
}

pub fn build_create_delivery_proof_params(
    overrides: CreateDeliveryProofParamsOverrides,
) -> CreateDeliveryProofParams {
    CreateDeliveryProofParams {
        concrete: overrides
            .concrete
            .unwrap_or(DeliveryProofConcrete::SchnorrFs),
        algebra: overrides.algebra.unwrap_or(EnvelopeAlgebra::PairingElGamal),
        verifier_form: overrides
            .verifier_form
            .unwrap_or(VerifierGroupArithmetic::BothGroups),
        statement_version: overrides
            .statement_version
            .unwrap_or(DELIVERY_STATEMENT_VERSION_ONE),
    }
}

#[derive(Default)]
pub struct CreateDeliveryProofSuccessReturnOverrides<O> {
    pub output: Option<O>,
}

pub fn build_create_delivery_proof_success_return<O: Default>(
    overrides: CreateDeliveryProofSuccessReturnOverrides<O>,
) -> CreateDeliveryProofSuccessReturn<O> {
    CreateDeliveryProofSuccessReturn {
        output: overrides.output.unwrap_or_default(),
    }
}

#[derive(Default)]
pub struct ConsumeDeliveryProofPayloadOverrides<D: IDeliveryProofAdapter> {
    pub adapter: Option<D>,
}

pub fn build_consume_delivery_proof_payload<D: IDeliveryProofAdapter + Default>(
    overrides: ConsumeDeliveryProofPayloadOverrides<D>,
) -> ConsumeDeliveryProofPayload<D> {
    ConsumeDeliveryProofPayload {
        adapter: overrides.adapter.unwrap_or_default(),
    }
}

pub struct MockIDeliveryProofConsumer;

impl<P: IPairingAdapter, F: IChainForms> IDeliveryProofConsumer<P, F>
    for MockIDeliveryProofConsumer
{
    type Output = ();

    fn consume_delivery_proof<D: IDeliveryProofAdapter<Pairing = P, Forms = F>>(
        &self,
        _params: ConsumeDeliveryProofParams,
        _payload: ConsumeDeliveryProofPayload<D>,
    ) -> Self::Output {
    }
}

pub fn mock_create_delivery_proof<
    'a,
    P: IPairingArithmetic,
    E: IEncoderAdapter,
    F: IChainForms,
    C: IDeliveryProofConsumer<P, F>,
>(
    _deps: &CreateDeliveryProofDeps<'a, P, E, C>,
    _params: CreateDeliveryProofParams,
    _payload: CreateDeliveryProofPayload,
) -> CreateDeliveryProofReturn<C::Output>
where
    C::Output: Default,
{
    Ok(build_create_delivery_proof_success_return(
        Default::default(),
    ))
}
