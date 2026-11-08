#[cfg(test)]
mod integration_test;
mod interface;
#[cfg(any(test, feature = "mocks"))]
mod mock;
pub mod provides;
#[cfg(test)]
mod test_basic;
#[cfg(test)]
mod test_error;

use chain::IChainForms;
use encoding::IEncoderAdapter;
use envelope::{
    CreateKeyAgreementDeps, CreateKeyAgreementParams, CreateKeyAgreementPayload,
    create_key_agreement,
};
use kem::{
    ConsumeKemParams, ConsumeKemPayload, ICredentialKemAdapter, IKemConsumer, KemIdentity,
    SetupScope,
};
use pairing::{IPairingArithmetic, IPairingReference};

use crate::config::provides::{AdmittedIdentities, AdmittedIdentity};
use crate::delivery::provides::AdmittedKemIdentity;
use crate::run::provides::{KeyAgreementConsumer, KeyAgreementConsumerConstructorParams};
use interface::{
    KemConsumer, KemConsumerConstructorParams, KemConsumerErrorReturn, KemConsumerReturn,
    KemConsumerTryNewReturn,
};

pub(super) fn pair_identities(
    identities: &AdmittedIdentities,
) -> (AdmittedKemIdentity<'_>, Vec<AdmittedKemIdentity<'_>>) {
    fn pair(identity: &AdmittedIdentity) -> AdmittedKemIdentity<'_> {
        match identity {
            AdmittedIdentity::Entitlement { canonical } => AdmittedKemIdentity {
                setup_scope: SetupScope::Entitlement,
                identity: KemIdentity::Entitlement {
                    canonical: canonical.as_slice(),
                },
            },
            AdmittedIdentity::Asset { root } => AdmittedKemIdentity {
                setup_scope: SetupScope::Asset { root },
                identity: KemIdentity::Asset { root },
            },
        }
    }
    (
        pair(&identities.proof),
        identities.mapping.iter().map(pair).collect(),
    )
}

impl<'a, P, E, F> KemConsumer<'a, P, E, F>
where
    P: IPairingArithmetic + IPairingReference,
    E: IEncoderAdapter,
    F: IChainForms,
{
    pub fn try_new(
        params: KemConsumerConstructorParams<'a, P, E, F>,
    ) -> KemConsumerTryNewReturn<'a, P, E, F> {
        Ok(KemConsumer {
            pairing: params.pairing,
            hash_to_scalar: params.hash_to_scalar,
            encoder: params.encoder,
            random: params.random,
            mint: params.mint,
            transfer: params.transfer,
            identities: params.identities,
            key_agreement: params.key_agreement,
            delivery_proof: params.delivery_proof,
            generate: params.generate,
            kinds: params.kinds,
            verifier: params.verifier,
            statement_version: params.statement_version,
            iterations: params.iterations,
        })
    }
}

impl<'a, P, E, F> IKemConsumer<P> for KemConsumer<'a, P, E, F>
where
    P: IPairingArithmetic + IPairingReference,
    E: IEncoderAdapter,
    F: IChainForms,
{
    type Output = KemConsumerReturn;

    fn consume_kem<K: ICredentialKemAdapter<Pairing = P>>(
        &self,
        _params: ConsumeKemParams,
        payload: ConsumeKemPayload<K>,
    ) -> Self::Output {
        let (proof_identity, identities) = pair_identities(self.identities);
        let Ok(consumer) = KeyAgreementConsumer::try_new(KeyAgreementConsumerConstructorParams {
            pairing: self.pairing,
            hash_to_scalar: self.hash_to_scalar,
            encoder: self.encoder,
            random: self.random,
            kem: &payload.adapter,
            mint: self.mint.clone(),
            transfer: self.transfer.clone(),
            identities,
            proof_identity,
            delivery_proof: self.delivery_proof,
            generate: self.generate.clone(),
            kinds: self.kinds.clone(),
            verifier: self.verifier.clone(),
            statement_version: self.statement_version,
            iterations: self.iterations,
        });
        let output = match create_key_agreement(
            &CreateKeyAgreementDeps {
                pairing: self.pairing,
                hash_to_scalar: self.hash_to_scalar,
                encoder: self.encoder,
                random: self.random,
                consumer,
            },
            CreateKeyAgreementParams {
                concrete: self.key_agreement.concrete,
                identifier: self.key_agreement.identifier,
                algebra: self.key_agreement.algebra,
            },
            CreateKeyAgreementPayload,
        ) {
            Err(error) => return Err(KemConsumerErrorReturn::CreateKeyAgreement(error)),
            Ok(success) => success.output,
        };
        match output {
            Err(error) => Err(KemConsumerErrorReturn::KeyAgreement(error)),
            Ok(success) => Ok(success),
        }
    }
}
