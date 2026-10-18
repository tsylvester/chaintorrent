#[cfg(test)]
mod integration_test;
mod interface;
#[cfg(any(test, feature = "mocks"))]
mod mock;
pub mod provides;
#[cfg(test)]
mod test;

use encoding::IEncoderAdapter;
use envelope::IKeyAgreementAdapter;
use kem::ICredentialKemAdapter;
use pairing::{IPairingArithmetic, IPairingReference};
use proof::IDeliveryProofAdapter;

use super::evm::provides::{EvmGenerate, EvmGenerateConstructorParams};

use self::interface::{
    ConsumeGenerateParams, ConsumeGeneratePayload, CreateGenerateDeps, CreateGenerateErrorReturn,
    CreateGenerateParams, CreateGeneratePayload, CreateGenerateReturn, CreateGenerateSuccessReturn,
    GenerateConcrete, IGenerateAdapter, IGenerateConsumer, VectorKindQuantity, VectorKindSettings,
    VectorKindSize, VectorKindSizing,
};
#[cfg(any(test, feature = "mocks"))]
use self::mock::{MockIGenerateAdapter, MockIGenerateAdapterConstructorParams};

pub fn create_generate<'a, P, E, K, A, D, C: IGenerateConsumer>(
    deps: &CreateGenerateDeps<'a, P, E, K, A, D, C>,
    params: CreateGenerateParams,
    _payload: CreateGeneratePayload,
) -> CreateGenerateReturn<C::Output>
where
    P: IPairingArithmetic + IPairingReference,
    E: IEncoderAdapter,
    K: ICredentialKemAdapter<Pairing = P>,
    A: IKeyAgreementAdapter<Pairing = P>,
    D: IDeliveryProofAdapter<Pairing = P>,
{
    let declaration = match &params.concrete {
        GenerateConcrete::Evm { .. } => {
            <EvmGenerate<'a, P, E, K, A, D> as IGenerateAdapter>::DECLARATION
        }
        #[cfg(any(test, feature = "mocks"))]
        GenerateConcrete::Mock(_) => <MockIGenerateAdapter as IGenerateAdapter>::DECLARATION,
    };
    let mut admitted: Vec<VectorKindSettings> = Vec::with_capacity(params.kinds.len());
    for entry in params.kinds.iter() {
        let kind = entry.kind;
        let Some(declared) = declaration
            .kinds
            .iter()
            .find(|declared| declared.kind == kind)
        else {
            return Err(CreateGenerateErrorReturn::UndeclaredKind { kind });
        };
        if admitted.iter().any(|settings| settings.kind == kind) {
            return Err(CreateGenerateErrorReturn::DuplicateKind { kind });
        }
        let quantities = entry.quantities;
        let size = match declared.sizing {
            VectorKindSizing::Drawn => {
                let Some(count) = quantities.count else {
                    return Err(CreateGenerateErrorReturn::MissingQuantity {
                        kind,
                        quantity: VectorKindQuantity::Count,
                    });
                };
                if quantities.terms.is_some() {
                    return Err(CreateGenerateErrorReturn::UnexpectedQuantity {
                        kind,
                        quantity: VectorKindQuantity::Terms,
                    });
                }
                if quantities.message_length.is_some() {
                    return Err(CreateGenerateErrorReturn::UnexpectedQuantity {
                        kind,
                        quantity: VectorKindQuantity::MessageLength,
                    });
                }
                VectorKindSize::Drawn { count }
            }
            VectorKindSizing::DrawnTerms => {
                let Some(count) = quantities.count else {
                    return Err(CreateGenerateErrorReturn::MissingQuantity {
                        kind,
                        quantity: VectorKindQuantity::Count,
                    });
                };
                let Some(terms) = quantities.terms else {
                    return Err(CreateGenerateErrorReturn::MissingQuantity {
                        kind,
                        quantity: VectorKindQuantity::Terms,
                    });
                };
                if quantities.message_length.is_some() {
                    return Err(CreateGenerateErrorReturn::UnexpectedQuantity {
                        kind,
                        quantity: VectorKindQuantity::MessageLength,
                    });
                }
                VectorKindSize::DrawnTerms { count, terms }
            }
            VectorKindSizing::DrawnMessages => {
                let Some(count) = quantities.count else {
                    return Err(CreateGenerateErrorReturn::MissingQuantity {
                        kind,
                        quantity: VectorKindQuantity::Count,
                    });
                };
                if quantities.terms.is_some() {
                    return Err(CreateGenerateErrorReturn::UnexpectedQuantity {
                        kind,
                        quantity: VectorKindQuantity::Terms,
                    });
                }
                let Some(message_length) = quantities.message_length else {
                    return Err(CreateGenerateErrorReturn::MissingQuantity {
                        kind,
                        quantity: VectorKindQuantity::MessageLength,
                    });
                };
                VectorKindSize::DrawnMessages {
                    count,
                    message_length,
                }
            }
            VectorKindSizing::Enumerated | VectorKindSizing::OverConfiguredInputs => {
                if quantities.count.is_some() {
                    return Err(CreateGenerateErrorReturn::UnexpectedQuantity {
                        kind,
                        quantity: VectorKindQuantity::Count,
                    });
                }
                if quantities.terms.is_some() {
                    return Err(CreateGenerateErrorReturn::UnexpectedQuantity {
                        kind,
                        quantity: VectorKindQuantity::Terms,
                    });
                }
                if quantities.message_length.is_some() {
                    return Err(CreateGenerateErrorReturn::UnexpectedQuantity {
                        kind,
                        quantity: VectorKindQuantity::MessageLength,
                    });
                }
                if declared.sizing == VectorKindSizing::Enumerated {
                    VectorKindSize::Enumerated
                } else {
                    VectorKindSize::OverConfiguredInputs
                }
            }
        };
        admitted.push(VectorKindSettings { kind, size });
    }
    let mut sizes: Vec<VectorKindSettings> = Vec::with_capacity(declaration.kinds.len());
    for declared in declaration.kinds.iter() {
        match admitted
            .iter()
            .find(|settings| settings.kind == declared.kind)
        {
            Some(settings) => sizes.push(*settings),
            None => {
                return Err(CreateGenerateErrorReturn::MissingKind {
                    kind: declared.kind,
                });
            }
        }
    }
    match params.concrete {
        GenerateConcrete::Evm { settings } => {
            let Ok(adapter) = EvmGenerate::try_new(EvmGenerateConstructorParams {
                pairing: deps.pairing,
                hash_to_scalar: deps.hash_to_scalar,
                encoder: deps.encoder,
                random: deps.random,
                kem: deps.kem,
                key_agreement: deps.key_agreement,
                delivery_proof: deps.delivery_proof,
                scope: params.scope,
                statement_version: params.statement_version,
                settings: *settings,
                sizes,
            });
            let output = deps
                .consumer
                .consume_generate(ConsumeGenerateParams, ConsumeGeneratePayload { adapter });
            Ok(CreateGenerateSuccessReturn { output })
        }
        #[cfg(any(test, feature = "mocks"))]
        GenerateConcrete::Mock(failure_mode) => {
            let Ok(adapter) =
                MockIGenerateAdapter::try_new(MockIGenerateAdapterConstructorParams {
                    failure_mode,
                });
            let output = deps
                .consumer
                .consume_generate(ConsumeGenerateParams, ConsumeGeneratePayload { adapter });
            Ok(CreateGenerateSuccessReturn { output })
        }
    }
}
