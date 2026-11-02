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

use super::solidity::provides::{SolidityGenerate, SolidityGenerateConstructorParams};
use interface::{
    ConsumeGenerateParams, ConsumeGeneratePayload, CreateGenerateDeps, CreateGenerateErrorReturn,
    CreateGenerateParams, CreateGeneratePayload, CreateGenerateReturn, CreateGenerateSuccessReturn,
    GenerateConcrete, IGenerateAdapter, IGenerateConsumer, VectorKindQuantity, VectorKindSettings,
    VectorKindSize, VectorKindSizing,
};
#[cfg(any(test, feature = "mocks"))]
use mock::{MockIGenerateAdapter, MockIGenerateAdapterConstructorParams};

pub fn create_generate<
    'a,
    P: IPairingArithmetic + IPairingReference,
    E: IEncoderAdapter,
    K: ICredentialKemAdapter<Pairing = P>,
    A: IKeyAgreementAdapter<Pairing = P>,
    D: IDeliveryProofAdapter<Pairing = P>,
    C: IGenerateConsumer<D::Forms>,
>(
    deps: &CreateGenerateDeps<'a, P, E, K, A, D, C>,
    params: CreateGenerateParams,
    _payload: CreateGeneratePayload,
) -> CreateGenerateReturn<C::Output> {
    let declaration = match &params.concrete {
        GenerateConcrete::Solidity { .. } => {
            <SolidityGenerate<'a, P, E, K, A, D> as IGenerateAdapter>::DECLARATION
        }
        #[cfg(any(test, feature = "mocks"))]
        GenerateConcrete::Mock(..) => {
            <MockIGenerateAdapter<D::Forms> as IGenerateAdapter>::DECLARATION
        }
    };
    if E::DECLARATION.identifier != declaration.encoding_identifier {
        return Err(CreateGenerateErrorReturn::UnsupportedEncodingIdentifier);
    }
    let mut sizes = Vec::with_capacity(declaration.kinds.len());
    for (index, entry) in params.kinds.iter().enumerate() {
        let Some(declared) = declaration
            .kinds
            .iter()
            .find(|declared| declared.kind == entry.kind)
        else {
            return Err(CreateGenerateErrorReturn::UndeclaredKind { kind: entry.kind });
        };
        if params.kinds[..index]
            .iter()
            .any(|earlier| earlier.kind == entry.kind)
        {
            return Err(CreateGenerateErrorReturn::DuplicateKind { kind: entry.kind });
        }
        let size = match declared.sizing {
            VectorKindSizing::Drawn => {
                let Some(count) = entry.quantities.count else {
                    return Err(CreateGenerateErrorReturn::MissingQuantity {
                        kind: entry.kind,
                        quantity: VectorKindQuantity::Count,
                    });
                };
                if entry.quantities.terms.is_some() {
                    return Err(CreateGenerateErrorReturn::UnexpectedQuantity {
                        kind: entry.kind,
                        quantity: VectorKindQuantity::Terms,
                    });
                }
                if entry.quantities.message_length.is_some() {
                    return Err(CreateGenerateErrorReturn::UnexpectedQuantity {
                        kind: entry.kind,
                        quantity: VectorKindQuantity::MessageLength,
                    });
                }
                VectorKindSize::Drawn { count }
            }
            VectorKindSizing::DrawnTerms => {
                let Some(count) = entry.quantities.count else {
                    return Err(CreateGenerateErrorReturn::MissingQuantity {
                        kind: entry.kind,
                        quantity: VectorKindQuantity::Count,
                    });
                };
                let Some(terms) = entry.quantities.terms else {
                    return Err(CreateGenerateErrorReturn::MissingQuantity {
                        kind: entry.kind,
                        quantity: VectorKindQuantity::Terms,
                    });
                };
                if entry.quantities.message_length.is_some() {
                    return Err(CreateGenerateErrorReturn::UnexpectedQuantity {
                        kind: entry.kind,
                        quantity: VectorKindQuantity::MessageLength,
                    });
                }
                VectorKindSize::DrawnTerms { count, terms }
            }
            VectorKindSizing::DrawnMessages => {
                let Some(count) = entry.quantities.count else {
                    return Err(CreateGenerateErrorReturn::MissingQuantity {
                        kind: entry.kind,
                        quantity: VectorKindQuantity::Count,
                    });
                };
                if entry.quantities.terms.is_some() {
                    return Err(CreateGenerateErrorReturn::UnexpectedQuantity {
                        kind: entry.kind,
                        quantity: VectorKindQuantity::Terms,
                    });
                }
                let Some(message_length) = entry.quantities.message_length else {
                    return Err(CreateGenerateErrorReturn::MissingQuantity {
                        kind: entry.kind,
                        quantity: VectorKindQuantity::MessageLength,
                    });
                };
                VectorKindSize::DrawnMessages {
                    count,
                    message_length,
                }
            }
            VectorKindSizing::Enumerated | VectorKindSizing::OverConfiguredInputs => {
                if entry.quantities.count.is_some() {
                    return Err(CreateGenerateErrorReturn::UnexpectedQuantity {
                        kind: entry.kind,
                        quantity: VectorKindQuantity::Count,
                    });
                }
                if entry.quantities.terms.is_some() {
                    return Err(CreateGenerateErrorReturn::UnexpectedQuantity {
                        kind: entry.kind,
                        quantity: VectorKindQuantity::Terms,
                    });
                }
                if entry.quantities.message_length.is_some() {
                    return Err(CreateGenerateErrorReturn::UnexpectedQuantity {
                        kind: entry.kind,
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
        sizes.push(VectorKindSettings {
            kind: entry.kind,
            size,
        });
    }
    for declared in declaration.kinds {
        if !params.kinds.iter().any(|entry| entry.kind == declared.kind) {
            return Err(CreateGenerateErrorReturn::MissingKind {
                kind: declared.kind,
            });
        }
    }
    let output = match params.concrete {
        GenerateConcrete::Solidity { settings } => {
            let Ok(adapter) = SolidityGenerate::try_new(SolidityGenerateConstructorParams {
                pairing: deps.pairing,
                hash_to_scalar: deps.hash_to_scalar,
                encoder: deps.encoder,
                random: deps.random,
                kem: deps.kem,
                key_agreement: deps.key_agreement,
                delivery_proof: deps.delivery_proof,
                statement_version: params.statement_version,
                settings: *settings,
                sizes,
            });
            deps.consumer
                .consume_generate(ConsumeGenerateParams, ConsumeGeneratePayload { adapter })
        }
        #[cfg(any(test, feature = "mocks"))]
        GenerateConcrete::Mock(failure_mode) => {
            let Ok(adapter) =
                MockIGenerateAdapter::<D::Forms>::try_new(MockIGenerateAdapterConstructorParams {
                    failure_mode,
                });
            deps.consumer
                .consume_generate(ConsumeGenerateParams, ConsumeGeneratePayload { adapter })
        }
    };
    Ok(CreateGenerateSuccessReturn { output })
}
