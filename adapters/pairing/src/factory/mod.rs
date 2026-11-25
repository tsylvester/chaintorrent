#[cfg(test)]
mod integration_test;
mod interface;
#[cfg(any(test, feature = "mocks"))]
mod mock;
pub mod provides;
#[cfg(test)]
mod test;

use crate::admit_pairing_composition::provides::{
    AdmitPairingCompositionParams, AdmitPairingCompositionPayload,
};
use crate::bls12_381_arkworks::provides::{
    Bls12381ArkworksPairing, Bls12381ArkworksPairingConstructorParams,
};
use crate::bls12_381_halo2curves::provides::{
    Bls12381Halo2curvesPairing, Bls12381Halo2curvesPairingConstructorParams,
};
use crate::bn254_arkworks::provides::{
    Bn254ArkworksPairing, Bn254ArkworksPairingConstructorParams,
};
use crate::bn254_halo2curves::provides::{
    Bn254Halo2curvesPairing, Bn254Halo2curvesPairingConstructorParams,
};
use crate::lookup_pairing_declaration::provides::{
    LookupPairingDeclarationDeps, LookupPairingDeclarationParams, LookupPairingDeclarationPayload,
};
use interface::{
    ConsumePairingParams, ConsumePairingPayload, CreatePairingDeps, CreatePairingErrorReturn,
    CreatePairingParams, CreatePairingPayload, CreatePairingReturn, CreatePairingSuccessReturn,
    IPairingConsumer, PairingConcrete,
};
#[cfg(any(test, feature = "mocks"))]
use mock::{MockIPairingAdapter, MockIPairingAdapterConstructorParams};

pub fn create_pairing<C: IPairingConsumer>(
    deps: &CreatePairingDeps<C>,
    params: CreatePairingParams,
    _payload: CreatePairingPayload,
) -> CreatePairingReturn<C::Output> {
    if let Err(error) = (deps.admit_pairing_composition)(
        &deps.admit_pairing_composition_deps,
        AdmitPairingCompositionParams {
            identifier: params.concrete,
            required_curve: params.required_curve,
            required_verifier_group_arithmetic: params.required_verifier_group_arithmetic,
            required_precompile_encodings: params.required_precompile_encodings,
            required_target_group_encoding: params.required_target_group_encoding,
        },
        AdmitPairingCompositionPayload,
    ) {
        return Err(CreatePairingErrorReturn::Composition(error));
    }
    let Ok(looked_up) = (deps.lookup_pairing_declaration)(
        &LookupPairingDeclarationDeps,
        LookupPairingDeclarationParams {
            concrete: params.concrete,
        },
        LookupPairingDeclarationPayload,
    );
    let declaration = looked_up.declaration;
    match params.concrete {
        PairingConcrete::Bn254Arkworks => {
            let Ok(adapter) = Bn254ArkworksPairing::try_new(Bn254ArkworksPairingConstructorParams);
            Ok(CreatePairingSuccessReturn {
                output: deps
                    .consumer
                    .consume_pairing(
                        ConsumePairingParams,
                        ConsumePairingPayload { adapter, declaration },
                    ),
            })
        }
        PairingConcrete::Bn254Halo2curves => {
            let Ok(adapter) =
                Bn254Halo2curvesPairing::try_new(Bn254Halo2curvesPairingConstructorParams);
            Ok(CreatePairingSuccessReturn {
                output: deps
                    .consumer
                    .consume_pairing(
                        ConsumePairingParams,
                        ConsumePairingPayload { adapter, declaration },
                    ),
            })
        }
        PairingConcrete::Bls12381Arkworks => {
            let Ok(adapter) =
                Bls12381ArkworksPairing::try_new(Bls12381ArkworksPairingConstructorParams);
            Ok(CreatePairingSuccessReturn {
                output: deps
                    .consumer
                    .consume_pairing(
                        ConsumePairingParams,
                        ConsumePairingPayload { adapter, declaration },
                    ),
            })
        }
        PairingConcrete::Bls12381Halo2curves => {
            let Ok(adapter) =
                Bls12381Halo2curvesPairing::try_new(Bls12381Halo2curvesPairingConstructorParams);
            Ok(CreatePairingSuccessReturn {
                output: deps
                    .consumer
                    .consume_pairing(
                        ConsumePairingParams,
                        ConsumePairingPayload { adapter, declaration },
                    ),
            })
        }
        #[cfg(any(test, feature = "mocks"))]
        PairingConcrete::Mock(mode) => {
            let Ok(adapter) = MockIPairingAdapter::try_new(MockIPairingAdapterConstructorParams {
                failure_mode: mode,
            });
            Ok(CreatePairingSuccessReturn {
                output: deps
                    .consumer
                    .consume_pairing(
                        ConsumePairingParams,
                        ConsumePairingPayload { adapter, declaration },
                    ),
            })
        }
    }
}
