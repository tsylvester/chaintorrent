mod interface;
#[cfg(any(test, feature = "mocks"))]
mod mock;
pub mod provides;
#[cfg(test)]
mod test;

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
use interface::{
    ConsumePairingParams, ConsumePairingPayload, CreatePairingDeps, CreatePairingErrorReturn,
    CreatePairingParams, CreatePairingPayload, CreatePairingReturn, CreatePairingSuccessReturn,
    IPairingConsumer, PairingConcrete,
};

pub fn create_pairing<C: IPairingConsumer>(
    deps: &CreatePairingDeps<C>,
    params: CreatePairingParams,
    _payload: CreatePairingPayload,
) -> CreatePairingReturn<C::Output> {
    match params.concrete {
        PairingConcrete::Bn254Arkworks => {
            if !params
                .supported_encodings
                .contains(&Bn254ArkworksPairing::DECLARATION.precompile_encoding)
            {
                return Err(CreatePairingErrorReturn::UnsupportedPrecompileEncoding);
            }
            if Bn254ArkworksPairing::DECLARATION.target_group_encoding
                != params.target_group_encoding
            {
                return Err(CreatePairingErrorReturn::UnsupportedTargetGroupEncoding);
            }
            let Ok(adapter) = Bn254ArkworksPairing::try_new(Bn254ArkworksPairingConstructorParams);
            Ok(CreatePairingSuccessReturn {
                output: deps
                    .consumer
                    .consume_pairing(ConsumePairingParams, ConsumePairingPayload { adapter }),
            })
        }
        PairingConcrete::Bn254Halo2curves => {
            if !params
                .supported_encodings
                .contains(&Bn254Halo2curvesPairing::DECLARATION.precompile_encoding)
            {
                return Err(CreatePairingErrorReturn::UnsupportedPrecompileEncoding);
            }
            if Bn254Halo2curvesPairing::DECLARATION.target_group_encoding
                != params.target_group_encoding
            {
                return Err(CreatePairingErrorReturn::UnsupportedTargetGroupEncoding);
            }
            let Ok(adapter) =
                Bn254Halo2curvesPairing::try_new(Bn254Halo2curvesPairingConstructorParams);
            Ok(CreatePairingSuccessReturn {
                output: deps
                    .consumer
                    .consume_pairing(ConsumePairingParams, ConsumePairingPayload { adapter }),
            })
        }
        PairingConcrete::Bls12381Arkworks => {
            if !params
                .supported_encodings
                .contains(&Bls12381ArkworksPairing::DECLARATION.precompile_encoding)
            {
                return Err(CreatePairingErrorReturn::UnsupportedPrecompileEncoding);
            }
            if Bls12381ArkworksPairing::DECLARATION.target_group_encoding
                != params.target_group_encoding
            {
                return Err(CreatePairingErrorReturn::UnsupportedTargetGroupEncoding);
            }
            let Ok(adapter) =
                Bls12381ArkworksPairing::try_new(Bls12381ArkworksPairingConstructorParams);
            Ok(CreatePairingSuccessReturn {
                output: deps
                    .consumer
                    .consume_pairing(ConsumePairingParams, ConsumePairingPayload { adapter }),
            })
        }
        PairingConcrete::Bls12381Halo2curves => {
            if !params
                .supported_encodings
                .contains(&Bls12381Halo2curvesPairing::DECLARATION.precompile_encoding)
            {
                return Err(CreatePairingErrorReturn::UnsupportedPrecompileEncoding);
            }
            if Bls12381Halo2curvesPairing::DECLARATION.target_group_encoding
                != params.target_group_encoding
            {
                return Err(CreatePairingErrorReturn::UnsupportedTargetGroupEncoding);
            }
            let Ok(adapter) =
                Bls12381Halo2curvesPairing::try_new(Bls12381Halo2curvesPairingConstructorParams);
            Ok(CreatePairingSuccessReturn {
                output: deps
                    .consumer
                    .consume_pairing(ConsumePairingParams, ConsumePairingPayload { adapter }),
            })
        }
    }
}
