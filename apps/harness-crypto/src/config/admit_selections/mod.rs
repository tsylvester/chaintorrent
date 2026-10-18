mod interface;
#[cfg(any(test, feature = "mocks"))]
mod mock;
pub mod provides;
#[cfg(test)]
mod test_mapping;
#[cfg(test)]
mod test_order_families;
#[cfg(test)]
mod test_order_lists;
#[cfg(test)]
mod test_refusal_families;
#[cfg(test)]
mod test_refusal_lists;

use chain::{ChainFormsConcrete, ChainFormsIdentifier};
use encoding::{EncodingConcrete, EncodingIdentifier};
use envelope::{EnvelopeAlgebra, KeyAgreementConcrete, KeyAgreementIdentifier};
use hash_to_scalar::{HashToScalarConcrete, HashToScalarIdentifier};
use kem::{IdentityScope, KemConcrete, KemIdentifier};
use pairing::{PairingConcrete, PrecompileEncoding, TargetGroupEncodingIdentifier};
use proof::DeliveryProofConcrete;
use random::RandomSourceKind;

use interface::{
    AdmitSelectionsDeps, AdmitSelectionsErrorReturn, AdmitSelectionsParams, AdmitSelectionsPayload,
    AdmitSelectionsReturn, AdmitSelectionsSuccessReturn, ChainFormsSelection, CurveSelection,
    DeliveryProofSelection, EncodingSelection, HarnessSelections, HashToScalarSelection,
    KemSelection, KeyAgreementSelection, SelectionField,
};

pub fn admit_selections(
    _deps: &AdmitSelectionsDeps,
    _params: AdmitSelectionsParams,
    payload: AdmitSelectionsPayload<'_>,
) -> AdmitSelectionsReturn {
    let refuse = |field: SelectionField, text: &str| -> AdmitSelectionsReturn {
        Err(AdmitSelectionsErrorReturn::UnknownValue {
            field,
            text: text.to_string(),
        })
    };
    let random = match payload.random.source.as_str() {
        "operating_system" => RandomSourceKind::OperatingSystem,
        _ => return refuse(SelectionField::RandomSource, &payload.random.source),
    };
    let encoding = EncodingSelection {
        concrete: match payload.encoding.concrete.as_str() {
            "abi" => EncodingConcrete::Abi,
            _ => return refuse(SelectionField::EncodingConcrete, &payload.encoding.concrete),
        },
        identifier: match payload.encoding.identifier.as_str() {
            "ethereum_abi_v1" => EncodingIdentifier::EthereumAbiV1,
            _ => {
                return refuse(
                    SelectionField::EncodingIdentifier,
                    &payload.encoding.identifier,
                );
            }
        },
    };
    let hash_to_scalar = HashToScalarSelection {
        concrete: match payload.hash_to_scalar.concrete.as_str() {
            "keccak256" => HashToScalarConcrete::Keccak256,
            _ => {
                return refuse(
                    SelectionField::HashToScalarConcrete,
                    &payload.hash_to_scalar.concrete,
                );
            }
        },
        identifier: match payload.hash_to_scalar.identifier.as_str() {
            "keccak256_v1" => HashToScalarIdentifier::Keccak256V1,
            _ => {
                return refuse(
                    SelectionField::HashToScalarIdentifier,
                    &payload.hash_to_scalar.identifier,
                );
            }
        },
    };
    let chain_forms = ChainFormsSelection {
        concrete: match payload.chain_forms.concrete.as_str() {
            "evm" => ChainFormsConcrete::Evm,
            _ => {
                return refuse(
                    SelectionField::ChainFormsConcrete,
                    &payload.chain_forms.concrete,
                );
            }
        },
        identifier: match payload.chain_forms.identifier.as_str() {
            "evm_v1" => ChainFormsIdentifier::EvmV1,
            _ => {
                return refuse(
                    SelectionField::ChainFormsIdentifier,
                    &payload.chain_forms.identifier,
                );
            }
        },
    };
    let kem = KemSelection {
        concrete: match payload.kem.concrete.as_str() {
            "bb1_depth_one" => KemConcrete::Bb1DepthOne,
            _ => return refuse(SelectionField::KemConcrete, &payload.kem.concrete),
        },
        identifier: match payload.kem.identifier.as_str() {
            "bb1_depth_one_v1" => KemIdentifier::Bb1DepthOneV1,
            _ => return refuse(SelectionField::KemIdentifier, &payload.kem.identifier),
        },
        scope: match payload.kem.scope.as_str() {
            "entitlement" => IdentityScope::Entitlement,
            "asset" => IdentityScope::Asset,
            _ => return refuse(SelectionField::KemScope, &payload.kem.scope),
        },
    };
    let key_agreement = KeyAgreementSelection {
        concrete: match payload.key_agreement.concrete.as_str() {
            "pairing_elgamal" => KeyAgreementConcrete::PairingElGamal,
            _ => {
                return refuse(
                    SelectionField::KeyAgreementConcrete,
                    &payload.key_agreement.concrete,
                );
            }
        },
        identifier: match payload.key_agreement.identifier.as_str() {
            "pairing_elgamal_v1" => KeyAgreementIdentifier::PairingElGamalV1,
            _ => {
                return refuse(
                    SelectionField::KeyAgreementIdentifier,
                    &payload.key_agreement.identifier,
                );
            }
        },
        algebra: match payload.key_agreement.algebra.as_str() {
            "pairing_elgamal" => EnvelopeAlgebra::PairingElGamal,
            _ => {
                return refuse(
                    SelectionField::KeyAgreementAlgebra,
                    &payload.key_agreement.algebra,
                );
            }
        },
    };
    let delivery_proof = DeliveryProofSelection {
        concrete: match payload.delivery_proof.concrete.as_str() {
            "schnorr_fs" => DeliveryProofConcrete::SchnorrFs,
            _ => {
                return refuse(
                    SelectionField::DeliveryProofConcrete,
                    &payload.delivery_proof.concrete,
                );
            }
        },
        statement_version: payload.delivery_proof.statement_version,
    };
    if payload.curves.is_empty() {
        return Err(AdmitSelectionsErrorReturn::NoCurves);
    }
    let mut curves = Vec::with_capacity(payload.curves.len());
    for (index, curve) in payload.curves.iter().enumerate() {
        let pairing = match curve.pairing.as_str() {
            "bn254_arkworks" => PairingConcrete::Bn254Arkworks,
            "bn254_halo2curves" => PairingConcrete::Bn254Halo2curves,
            "bls12_381_arkworks" => PairingConcrete::Bls12381Arkworks,
            "bls12_381_halo2curves" => PairingConcrete::Bls12381Halo2curves,
            _ => return refuse(SelectionField::CurvePairing { index }, &curve.pairing),
        };
        let target_group_encoding = match curve.target_group_encoding.as_str() {
            "bn254_v1" => TargetGroupEncodingIdentifier::Bn254V1,
            "bls12_381_v1" => TargetGroupEncodingIdentifier::Bls12381V1,
            _ => {
                return refuse(
                    SelectionField::CurveTargetGroupEncoding { index },
                    &curve.target_group_encoding,
                );
            }
        };
        curves.push(CurveSelection {
            pairing,
            target_group_encoding,
        });
    }
    let mut benchmark_concretes = Vec::with_capacity(payload.benchmark_concretes.len());
    for (index, concrete) in payload.benchmark_concretes.iter().enumerate() {
        let pairing = match concrete.pairing.as_str() {
            "bn254_arkworks" => PairingConcrete::Bn254Arkworks,
            "bn254_halo2curves" => PairingConcrete::Bn254Halo2curves,
            "bls12_381_arkworks" => PairingConcrete::Bls12381Arkworks,
            "bls12_381_halo2curves" => PairingConcrete::Bls12381Halo2curves,
            _ => {
                return refuse(
                    SelectionField::BenchmarkPairing { index },
                    &concrete.pairing,
                );
            }
        };
        let target_group_encoding = match concrete.target_group_encoding.as_str() {
            "bn254_v1" => TargetGroupEncodingIdentifier::Bn254V1,
            "bls12_381_v1" => TargetGroupEncodingIdentifier::Bls12381V1,
            _ => {
                return refuse(
                    SelectionField::BenchmarkTargetGroupEncoding { index },
                    &concrete.target_group_encoding,
                );
            }
        };
        benchmark_concretes.push(CurveSelection {
            pairing,
            target_group_encoding,
        });
    }
    let mut precompile_encodings = Vec::with_capacity(payload.precompile_encodings.len());
    for (index, text) in payload.precompile_encodings.iter().enumerate() {
        precompile_encodings.push(match text.as_str() {
            "eip196_eip197" => PrecompileEncoding::Eip196Eip197,
            "eip2537" => PrecompileEncoding::Eip2537,
            _ => return refuse(SelectionField::PrecompileEncoding { index }, text),
        });
    }
    Ok(AdmitSelectionsSuccessReturn {
        selections: HarnessSelections {
            random,
            encoding,
            hash_to_scalar,
            chain_forms,
            kem,
            key_agreement,
            delivery_proof,
            curves,
            benchmark_concretes,
            precompile_encodings,
        },
    })
}
