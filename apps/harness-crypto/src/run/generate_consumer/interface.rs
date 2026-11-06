use chain::IChainForms;
use core::convert::Infallible;
use proof::{DeliveryContext, MintPurpose, TransferPurpose};

use crate::delivery::provides::AdmittedKemIdentity;
use crate::generate::provides::EmitReturn;

pub struct GenerateConsumer<'a, F: IChainForms> {
    pub(super) mint: DeliveryContext<F, MintPurpose>,
    pub(super) transfer: DeliveryContext<F, TransferPurpose>,
    pub(super) identities: Vec<AdmittedKemIdentity<'a>>,
    pub(super) proof_identity: AdmittedKemIdentity<'a>,
}

pub struct GenerateConsumerConstructorParams<'a, F: IChainForms> {
    pub mint: DeliveryContext<F, MintPurpose>,
    pub transfer: DeliveryContext<F, TransferPurpose>,
    pub identities: Vec<AdmittedKemIdentity<'a>>,
    pub proof_identity: AdmittedKemIdentity<'a>,
}

pub type GenerateConsumerTryNewReturn<'a, F> = Result<GenerateConsumer<'a, F>, Infallible>;

pub type GenerateConsumerReturn = EmitReturn;
