use chain::IChainForms;
use core::convert::Infallible;
use core::marker::PhantomData;
use encoding::IEncoderAdapter;
use hash_to_scalar::IHashToScalarAdapter;
use pairing::{IPairingArithmetic, IPairingReference};
use random::IRandomSourceAdapter;

use crate::config::provides::{DeliveryProofSelection, HarnessStatements, KeyAgreementSelection};
use crate::generate::provides::{GenerateConcrete, VectorKindConfiguration};

pub struct KemConsumer<
    'a,
    P: IPairingArithmetic + IPairingReference,
    E: IEncoderAdapter,
    F: IChainForms,
> {
    pub(super) pairing: &'a P,
    pub(super) hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
    pub(super) encoder: &'a E,
    pub(super) random: &'a dyn IRandomSourceAdapter,
    pub(super) generate: &'a GenerateConcrete,
    pub(super) kinds: &'a [VectorKindConfiguration],
    pub(super) key_agreement: KeyAgreementSelection,
    pub(super) delivery_proof: DeliveryProofSelection,
    pub(super) statements: &'a HarnessStatements,
    pub(super) forms: PhantomData<F>,
}

pub struct KemConsumerConstructorParams<
    'a,
    P: IPairingArithmetic + IPairingReference,
    E: IEncoderAdapter,
> {
    pub pairing: &'a P,
    pub hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
    pub encoder: &'a E,
    pub random: &'a dyn IRandomSourceAdapter,
    pub generate: &'a GenerateConcrete,
    pub kinds: &'a [VectorKindConfiguration],
    pub key_agreement: KeyAgreementSelection,
    pub delivery_proof: DeliveryProofSelection,
    pub statements: &'a HarnessStatements,
}

pub type KemConsumerTryNewReturn<'a, P, E, F> = Result<KemConsumer<'a, P, E, F>, Infallible>;
