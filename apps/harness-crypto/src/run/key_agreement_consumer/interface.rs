use chain::IChainForms;
use core::convert::Infallible;
use core::marker::PhantomData;
use encoding::IEncoderAdapter;
use hash_to_scalar::IHashToScalarAdapter;
use kem::{ICredentialKemAdapter, IdentityScope};
use pairing::{IPairingArithmetic, IPairingReference};
use random::IRandomSourceAdapter;

use crate::config::provides::{DeliveryProofSelection, HarnessStatements};
use crate::generate::provides::{GenerateConcrete, VectorKindConfiguration};

pub struct KeyAgreementConsumer<
    'a,
    P: IPairingArithmetic + IPairingReference,
    E: IEncoderAdapter,
    K: ICredentialKemAdapter<Pairing = P>,
    F: IChainForms,
> {
    pub(super) pairing: &'a P,
    pub(super) hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
    pub(super) encoder: &'a E,
    pub(super) random: &'a dyn IRandomSourceAdapter,
    pub(super) kem: &'a K,
    pub(super) generate: &'a GenerateConcrete,
    pub(super) kinds: &'a [VectorKindConfiguration],
    pub(super) scope: IdentityScope,
    pub(super) delivery_proof: DeliveryProofSelection,
    pub(super) statements: &'a HarnessStatements,
    pub(super) forms: PhantomData<F>,
}

pub struct KeyAgreementConsumerConstructorParams<
    'a,
    P: IPairingArithmetic + IPairingReference,
    E: IEncoderAdapter,
    K: ICredentialKemAdapter<Pairing = P>,
> {
    pub pairing: &'a P,
    pub hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
    pub encoder: &'a E,
    pub random: &'a dyn IRandomSourceAdapter,
    pub kem: &'a K,
    pub generate: &'a GenerateConcrete,
    pub kinds: &'a [VectorKindConfiguration],
    pub scope: IdentityScope,
    pub delivery_proof: DeliveryProofSelection,
    pub statements: &'a HarnessStatements,
}

pub type KeyAgreementConsumerTryNewReturn<'a, P, E, K, F> =
    Result<KeyAgreementConsumer<'a, P, E, K, F>, Infallible>;
