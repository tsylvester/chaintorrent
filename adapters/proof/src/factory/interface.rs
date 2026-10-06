use chain::IChainForms;
use core::convert::Infallible;
use domain::{AssetIdentityHash, ParameterSetIdentifier, Secret, SuiteIdentifier};
use encoding::IEncoderAdapter;
use envelope::{
    EnvelopeAlgebra, EnvelopeCoins, EnvelopeComponents, KeyPairComponents, PublicKeysComponents,
};
use hash_to_scalar::IHashToScalarAdapter;
use kem::MasterScalarComponents;
use pairing::{IPairingAdapter, IPairingArithmetic, VerifierGroupArithmetic};
use random::IRandomSourceAdapter;
use zeroize::Zeroize;

use crate::mint_statement::provides::{DeliveryRelation, MintPurpose, TransferPurpose};
use crate::schnorr_fs::provides::{
    SchnorrFsDeliveryProofTryNewErrorReturn, SchnorrFsProofFromComponentsErrorReturn,
    SchnorrFsProveMintErrorReturn, SchnorrFsProveTransferErrorReturn, SchnorrFsVerifyErrorReturn,
};

pub const DELIVERY_PROOF_INTERFACE_VERSION: u32 = 1;

pub struct DeliveryProofDeclaration {
    pub algebras: &'static [EnvelopeAlgebra],
    pub verifier_forms: &'static [VerifierGroupArithmetic],
    pub statement_versions: &'static [u16],
    pub challenge_tag: &'static [u8],
    pub weight_tag: &'static [u8],
    pub adapter_version: u32,
    pub interface_version: u32,
}

pub struct DeliveryContext<F: IChainForms, Purpose> {
    pub suite_identifier: SuiteIdentifier,
    pub chain: F::ChainIdentifier,
    pub entitlement_contract: F::Identity,
    pub asset_identity_hash: AssetIdentityHash,
    pub parameter_set_identifier: ParameterSetIdentifier,
    pub source_entitlement: F::Entitlement,
    pub target_entitlement: F::Entitlement,
    pub old_interval: F::Interval,
    pub new_interval: F::Interval,
    pub purpose: Purpose,
    pub seller: F::Identity,
    pub buyer: F::Identity,
    pub expiry: u64,
}

pub struct AlgebraicMintStatement<F: IChainForms, G1, G2> {
    pub context: DeliveryContext<F, MintPurpose>,
    pub hpub: G2,
    pub identity_element: G1,
    pub buyer_keys: PublicKeysComponents<G1, G2>,
    pub envelope: EnvelopeComponents<G1, G2>,
}

pub struct AlgebraicTransferStatement<F: IChainForms, G1, G2> {
    pub context: DeliveryContext<F, TransferPurpose>,
    pub identity_element: G1,
    pub seller_keys: PublicKeysComponents<G1, G2>,
    pub buyer_keys: PublicKeysComponents<G1, G2>,
    pub old_envelope: EnvelopeComponents<G1, G2>,
    pub new_envelope: EnvelopeComponents<G1, G2>,
}

pub enum AlgebraicStatement<'a, F: IChainForms, G1, G2> {
    Mint(&'a AlgebraicMintStatement<F, G1, G2>),
    Transfer(&'a AlgebraicTransferStatement<F, G1, G2>),
}

pub struct MintResponses<S> {
    pub alpha: S,
    pub r: S,
    pub rho: S,
    pub sigma: S,
}

pub struct TransferResponses<S> {
    pub x: S,
    pub y: S,
    pub s: S,
    pub rho: S,
    pub sigma: S,
}

pub struct MintSecondGroupFirstMessages<G2> {
    pub hpub: G2,
    pub d1: G2,
    pub d2: G2,
}

pub struct TransferSecondGroupFirstMessages<G2> {
    pub pk2: G2,
    pub d1: G2,
    pub d2: G2,
}

pub enum DeliveryProofComponents<S, G2> {
    MintBothGroups {
        challenge: S,
        responses: MintResponses<S>,
    },
    MintFirstGroupOnly {
        challenge: S,
        responses: MintResponses<S>,
        first_messages: MintSecondGroupFirstMessages<G2>,
    },
    TransferBothGroups {
        challenge: S,
        responses: TransferResponses<S>,
    },
    TransferFirstGroupOnly {
        challenge: S,
        responses: TransferResponses<S>,
        first_messages: TransferSecondGroupFirstMessages<G2>,
    },
}

pub struct DeliveryProofWireComponents<S, G2> {
    pub relation: DeliveryRelation,
    pub verifier_form: VerifierGroupArithmetic,
    pub challenge: S,
    pub responses: Vec<S>,
    pub second_group_first_messages: Vec<G2>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum DeliveryProofShapeErrorReturn {
    ResponseCount { expected: usize, actual: usize },
    SecondGroupFirstMessageCount { expected: usize, actual: usize },
}

impl<S, G2> TryFrom<DeliveryProofWireComponents<S, G2>> for DeliveryProofComponents<S, G2> {
    type Error = DeliveryProofShapeErrorReturn;

    fn try_from(wire: DeliveryProofWireComponents<S, G2>) -> Result<Self, Self::Error> {
        let expected_responses = match wire.relation {
            DeliveryRelation::Mint => 4,
            DeliveryRelation::Transfer => 5,
        };
        if wire.responses.len() != expected_responses {
            return Err(DeliveryProofShapeErrorReturn::ResponseCount {
                expected: expected_responses,
                actual: wire.responses.len(),
            });
        }
        let expected_first_messages = match wire.verifier_form {
            VerifierGroupArithmetic::BothGroups => 0,
            VerifierGroupArithmetic::FirstGroupOnly => 3,
        };
        if wire.second_group_first_messages.len() != expected_first_messages {
            return Err(
                DeliveryProofShapeErrorReturn::SecondGroupFirstMessageCount {
                    expected: expected_first_messages,
                    actual: wire.second_group_first_messages.len(),
                },
            );
        }
        match (wire.relation, wire.verifier_form) {
            (DeliveryRelation::Mint, VerifierGroupArithmetic::BothGroups) => {
                let [alpha, r, rho, sigma] = match TryInto::<[S; 4]>::try_into(wire.responses) {
                    Ok(responses) => responses,
                    Err(responses) => {
                        return Err(DeliveryProofShapeErrorReturn::ResponseCount {
                            expected: 4,
                            actual: responses.len(),
                        });
                    }
                };
                Ok(DeliveryProofComponents::MintBothGroups {
                    challenge: wire.challenge,
                    responses: MintResponses {
                        alpha,
                        r,
                        rho,
                        sigma,
                    },
                })
            }
            (DeliveryRelation::Mint, VerifierGroupArithmetic::FirstGroupOnly) => {
                let [alpha, r, rho, sigma] = match TryInto::<[S; 4]>::try_into(wire.responses) {
                    Ok(responses) => responses,
                    Err(responses) => {
                        return Err(DeliveryProofShapeErrorReturn::ResponseCount {
                            expected: 4,
                            actual: responses.len(),
                        });
                    }
                };
                let [hpub, d1, d2] =
                    match TryInto::<[G2; 3]>::try_into(wire.second_group_first_messages) {
                        Ok(first_messages) => first_messages,
                        Err(first_messages) => {
                            return Err(
                                DeliveryProofShapeErrorReturn::SecondGroupFirstMessageCount {
                                    expected: 3,
                                    actual: first_messages.len(),
                                },
                            );
                        }
                    };
                Ok(DeliveryProofComponents::MintFirstGroupOnly {
                    challenge: wire.challenge,
                    responses: MintResponses {
                        alpha,
                        r,
                        rho,
                        sigma,
                    },
                    first_messages: MintSecondGroupFirstMessages { hpub, d1, d2 },
                })
            }
            (DeliveryRelation::Transfer, VerifierGroupArithmetic::BothGroups) => {
                let [x, y, s, rho, sigma] = match TryInto::<[S; 5]>::try_into(wire.responses) {
                    Ok(responses) => responses,
                    Err(responses) => {
                        return Err(DeliveryProofShapeErrorReturn::ResponseCount {
                            expected: 5,
                            actual: responses.len(),
                        });
                    }
                };
                Ok(DeliveryProofComponents::TransferBothGroups {
                    challenge: wire.challenge,
                    responses: TransferResponses {
                        x,
                        y,
                        s,
                        rho,
                        sigma,
                    },
                })
            }
            (DeliveryRelation::Transfer, VerifierGroupArithmetic::FirstGroupOnly) => {
                let [x, y, s, rho, sigma] = match TryInto::<[S; 5]>::try_into(wire.responses) {
                    Ok(responses) => responses,
                    Err(responses) => {
                        return Err(DeliveryProofShapeErrorReturn::ResponseCount {
                            expected: 5,
                            actual: responses.len(),
                        });
                    }
                };
                let [pk2, d1, d2] =
                    match TryInto::<[G2; 3]>::try_into(wire.second_group_first_messages) {
                        Ok(first_messages) => first_messages,
                        Err(first_messages) => {
                            return Err(
                                DeliveryProofShapeErrorReturn::SecondGroupFirstMessageCount {
                                    expected: 3,
                                    actual: first_messages.len(),
                                },
                            );
                        }
                    };
                Ok(DeliveryProofComponents::TransferFirstGroupOnly {
                    challenge: wire.challenge,
                    responses: TransferResponses {
                        x,
                        y,
                        s,
                        rho,
                        sigma,
                    },
                    first_messages: TransferSecondGroupFirstMessages { pk2, d1, d2 },
                })
            }
        }
    }
}

pub struct ProveMintParams;

pub struct ProveMintPayload<'a, F: IChainForms, S: Zeroize, G1, G2> {
    pub master_scalar: MasterScalarComponents<S>,
    pub credential_randomness: Secret<S>,
    pub coins: EnvelopeCoins<S>,
    pub statement: &'a AlgebraicMintStatement<F, G1, G2>,
}

pub struct ProveMintSuccessReturn<PR> {
    pub proof: PR,
}

#[derive(Debug, PartialEq, Eq)]
pub enum ProveMintErrorReturn {
    SchnorrFs(SchnorrFsProveMintErrorReturn),
}

pub type ProveMintReturn<PR> = Result<ProveMintSuccessReturn<PR>, ProveMintErrorReturn>;

pub struct ProveTransferParams;

pub struct ProveTransferPayload<'a, F: IChainForms, S: Zeroize, G1, G2> {
    pub seller_secrets: KeyPairComponents<S>,
    pub offset: Secret<S>,
    pub coins: EnvelopeCoins<S>,
    pub statement: &'a AlgebraicTransferStatement<F, G1, G2>,
}

pub struct ProveTransferSuccessReturn<PR> {
    pub proof: PR,
}

#[derive(Debug, PartialEq, Eq)]
pub enum ProveTransferErrorReturn {
    SchnorrFs(SchnorrFsProveTransferErrorReturn),
}

pub type ProveTransferReturn<PR> = Result<ProveTransferSuccessReturn<PR>, ProveTransferErrorReturn>;

pub struct VerifyParams;

pub struct VerifyPayload<'a, F: IChainForms, G1, G2, PR> {
    pub statement: AlgebraicStatement<'a, F, G1, G2>,
    pub proof: &'a PR,
}

pub struct VerifySuccessReturn {
    pub is_valid: bool,
}

#[derive(Debug, PartialEq, Eq)]
pub enum VerifyErrorReturn {
    SchnorrFs(SchnorrFsVerifyErrorReturn),
}

pub type VerifyReturn = Result<VerifySuccessReturn, VerifyErrorReturn>;

pub struct ProofComponentsParams;

pub struct ProofComponentsPayload<'a, PR> {
    pub proof: &'a PR,
}

pub struct ProofComponentsSuccessReturn<S, G2> {
    pub components: DeliveryProofComponents<S, G2>,
}

pub type ProofComponentsReturn<S, G2> = Result<ProofComponentsSuccessReturn<S, G2>, Infallible>;

pub struct ProofFromComponentsParams;

pub struct ProofFromComponentsPayload<S, G2> {
    pub components: DeliveryProofComponents<S, G2>,
}

pub struct ProofFromComponentsSuccessReturn<PR> {
    pub proof: PR,
}

#[derive(Debug, PartialEq, Eq)]
pub enum ProofFromComponentsErrorReturn {
    SchnorrFs(SchnorrFsProofFromComponentsErrorReturn),
}

pub type ProofFromComponentsReturn<PR> =
    Result<ProofFromComponentsSuccessReturn<PR>, ProofFromComponentsErrorReturn>;

pub type ProveMintAdapterPayload<'a, A> = ProveMintPayload<
    'a,
    <A as IDeliveryProofAdapter>::Forms,
    <<A as IDeliveryProofAdapter>::Pairing as IPairingAdapter>::Scalar,
    <<A as IDeliveryProofAdapter>::Pairing as IPairingAdapter>::G1,
    <<A as IDeliveryProofAdapter>::Pairing as IPairingAdapter>::G2,
>;

pub type ProveTransferAdapterPayload<'a, A> = ProveTransferPayload<
    'a,
    <A as IDeliveryProofAdapter>::Forms,
    <<A as IDeliveryProofAdapter>::Pairing as IPairingAdapter>::Scalar,
    <<A as IDeliveryProofAdapter>::Pairing as IPairingAdapter>::G1,
    <<A as IDeliveryProofAdapter>::Pairing as IPairingAdapter>::G2,
>;

pub type VerifyAdapterPayload<'a, A> = VerifyPayload<
    'a,
    <A as IDeliveryProofAdapter>::Forms,
    <<A as IDeliveryProofAdapter>::Pairing as IPairingAdapter>::G1,
    <<A as IDeliveryProofAdapter>::Pairing as IPairingAdapter>::G2,
    <A as IDeliveryProofAdapter>::Proof,
>;

pub trait IDeliveryProofAdapter {
    const DECLARATION: DeliveryProofDeclaration;

    type Pairing: IPairingAdapter;
    type Forms: IChainForms;
    type Proof;

    fn prove_mint(
        &self,
        params: ProveMintParams,
        payload: ProveMintAdapterPayload<'_, Self>,
    ) -> ProveMintReturn<Self::Proof>;

    fn prove_transfer(
        &self,
        params: ProveTransferParams,
        payload: ProveTransferAdapterPayload<'_, Self>,
    ) -> ProveTransferReturn<Self::Proof>;

    fn verify(&self, params: VerifyParams, payload: VerifyAdapterPayload<'_, Self>)
    -> VerifyReturn;

    fn proof_components(
        &self,
        params: ProofComponentsParams,
        payload: ProofComponentsPayload<'_, Self::Proof>,
    ) -> ProofComponentsReturn<
        <Self::Pairing as IPairingAdapter>::Scalar,
        <Self::Pairing as IPairingAdapter>::G2,
    >;

    fn proof_from_components(
        &self,
        params: ProofFromComponentsParams,
        payload: ProofFromComponentsPayload<
            <Self::Pairing as IPairingAdapter>::Scalar,
            <Self::Pairing as IPairingAdapter>::G2,
        >,
    ) -> ProofFromComponentsReturn<Self::Proof>;
}

pub enum DeliveryProofConcrete {
    SchnorrFs,
}

pub trait IDeliveryProofConsumer<P: IPairingAdapter, F: IChainForms> {
    type Output;

    fn consume_delivery_proof<D: IDeliveryProofAdapter<Pairing = P, Forms = F>>(
        &self,
        params: ConsumeDeliveryProofParams,
        payload: ConsumeDeliveryProofPayload<D>,
    ) -> Self::Output;
}

pub struct ConsumeDeliveryProofParams;

pub struct ConsumeDeliveryProofPayload<D: IDeliveryProofAdapter> {
    pub adapter: D,
}

pub struct CreateDeliveryProofDeps<'a, P: IPairingArithmetic, E: IEncoderAdapter, C> {
    pub pairing: &'a P,
    pub hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
    pub encoder: &'a E,
    pub random: &'a dyn IRandomSourceAdapter,
    pub consumer: C,
}

pub struct CreateDeliveryProofParams {
    pub concrete: DeliveryProofConcrete,
    pub algebra: EnvelopeAlgebra,
    pub verifier_form: VerifierGroupArithmetic,
    pub statement_version: u16,
}

pub struct CreateDeliveryProofPayload;

pub struct CreateDeliveryProofSuccessReturn<O> {
    pub output: O,
}

#[derive(Debug, PartialEq, Eq)]
pub enum CreateDeliveryProofErrorReturn {
    UnsupportedEnvelopeAlgebra,
    UnsupportedVerifierForm,
    UnsupportedStatementVersion,
    SchnorrFs(SchnorrFsDeliveryProofTryNewErrorReturn),
}

pub type CreateDeliveryProofReturn<O> =
    Result<CreateDeliveryProofSuccessReturn<O>, CreateDeliveryProofErrorReturn>;

pub type CreateDeliveryProofFn<'a, P, E, F, C> =
    fn(
        &CreateDeliveryProofDeps<'a, P, E, C>,
        CreateDeliveryProofParams,
        CreateDeliveryProofPayload,
    ) -> CreateDeliveryProofReturn<<C as IDeliveryProofConsumer<P, F>>::Output>;
