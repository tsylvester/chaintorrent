mod interface;
#[cfg(any(test, feature = "mocks"))]
mod mock;
pub mod provides;
#[cfg(test)]
mod test;

use crate::generate::provides::{
    EvmCurveDirectories, EvmGenerateSettings, EvmLibraryNames, EvmVectorKindNames,
    EvmVectorKindSettings, GenerateConcrete, GeneratedDirectory,
    GeneratedDirectoryConstructorParams, GeneratedFileName, GeneratedFileNameConstructorParams,
    MsmTermCount, MsmTermCountConstructorParams, SolidityCompilerVersion, SolidityConstantName,
    SolidityConstantNameConstructorParams, SolidityLibraryName,
    SolidityLibraryNameConstructorParams, SolidityMemberName, SolidityMemberNameConstructorParams,
    SolidityTypeName, SolidityTypeNameConstructorParams, SpdxLicenseIdentifier,
    SpdxLicenseIdentifierConstructorParams, VectorCount, VectorCountConstructorParams, VectorKind,
    VectorKindConfiguration, VectorKindQuantities,
};
use interface::{
    AdmitGenerateDeps, AdmitGenerateErrorReturn, AdmitGenerateParams, AdmitGeneratePayload,
    AdmitGenerateReturn, AdmitGenerateSuccessReturn, GenerateField,
};

pub fn admit_generate(
    _deps: &AdmitGenerateDeps,
    _params: AdmitGenerateParams,
    payload: AdmitGeneratePayload<'_>,
) -> AdmitGenerateReturn {
    let generate = payload.generate;
    match generate.concrete.as_str() {
        "evm" => {
            let mut settings_kinds = Vec::with_capacity(generate.kinds.len());
            let mut kinds = Vec::with_capacity(generate.kinds.len());
            for (index, entry) in generate.kinds.iter().enumerate() {
                let kind = match entry.kind.as_str() {
                    "g1_addition" => VectorKind::G1Addition,
                    "g1_multiplication" => VectorKind::G1Multiplication,
                    "pairing_check" => VectorKind::PairingCheck,
                    "g2_addition" => VectorKind::G2Addition,
                    "g1_multi_scalar_multiplication" => VectorKind::G1MultiScalarMultiplication,
                    "g2_multi_scalar_multiplication" => VectorKind::G2MultiScalarMultiplication,
                    "decode_rejection" => VectorKind::DecodeRejection,
                    "hash_to_scalar" => VectorKind::HashToScalar,
                    "identity_mapping" => VectorKind::IdentityMapping,
                    "possession" => VectorKind::Possession,
                    "challenge" => VectorKind::Challenge,
                    "mint_proof" => VectorKind::MintProof,
                    "transfer_proof" => VectorKind::TransferProof,
                    _ => {
                        return Err(AdmitGenerateErrorReturn::UnknownValue {
                            field: GenerateField::Kind { index },
                            text: entry.kind.clone(),
                        });
                    }
                };
                let count = match entry.count {
                    Some(count) => Some(
                        VectorCount::try_new(VectorCountConstructorParams { count }).map_err(
                            |error| AdmitGenerateErrorReturn::VectorCount { index, error },
                        )?,
                    ),
                    None => None,
                };
                let terms = match entry.terms {
                    Some(count) => Some(
                        MsmTermCount::try_new(MsmTermCountConstructorParams { count }).map_err(
                            |error| AdmitGenerateErrorReturn::MsmTermCount { index, error },
                        )?,
                    ),
                    None => None,
                };
                let message_length = entry.message_length;
                let file_name = GeneratedFileName::try_new(GeneratedFileNameConstructorParams {
                    text: entry.file_name.clone(),
                })
                .map_err(|error| AdmitGenerateErrorReturn::FileName {
                    field: GenerateField::KindFileName { index },
                    error,
                })?;
                let struct_name = SolidityTypeName::try_new(SolidityTypeNameConstructorParams {
                    text: entry.struct_name.clone(),
                })
                .map_err(|error| AdmitGenerateErrorReturn::StructName { index, error })?;
                let decode_function_name =
                    SolidityMemberName::try_new(SolidityMemberNameConstructorParams {
                        text: entry.decode_function_name.clone(),
                    })
                    .map_err(|error| {
                        AdmitGenerateErrorReturn::DecodeFunctionName { index, error }
                    })?;
                let path_constant_name =
                    SolidityConstantName::try_new(SolidityConstantNameConstructorParams {
                        text: entry.path_constant_name.clone(),
                    })
                    .map_err(|error| AdmitGenerateErrorReturn::PathConstantName { index, error })?;
                settings_kinds.push(EvmVectorKindSettings {
                    kind,
                    names: EvmVectorKindNames {
                        file_name,
                        struct_name,
                        decode_function_name,
                        path_constant_name,
                    },
                });
                kinds.push(VectorKindConfiguration {
                    kind,
                    quantities: VectorKindQuantities {
                        count,
                        terms,
                        message_length,
                    },
                });
            }
            let constants_library = EvmLibraryNames {
                file_name: GeneratedFileName::try_new(GeneratedFileNameConstructorParams {
                    text: generate.constants_library.file_name.clone(),
                })
                .map_err(|error| AdmitGenerateErrorReturn::FileName {
                    field: GenerateField::ConstantsLibraryFileName,
                    error,
                })?,
                library_name: SolidityLibraryName::try_new(SolidityLibraryNameConstructorParams {
                    text: generate.constants_library.library_name.clone(),
                })
                .map_err(|error| AdmitGenerateErrorReturn::LibraryName {
                    field: GenerateField::ConstantsLibraryName,
                    error,
                })?,
            };
            let records_library = EvmLibraryNames {
                file_name: GeneratedFileName::try_new(GeneratedFileNameConstructorParams {
                    text: generate.records_library.file_name.clone(),
                })
                .map_err(|error| AdmitGenerateErrorReturn::FileName {
                    field: GenerateField::RecordsLibraryFileName,
                    error,
                })?,
                library_name: SolidityLibraryName::try_new(SolidityLibraryNameConstructorParams {
                    text: generate.records_library.library_name.clone(),
                })
                .map_err(|error| AdmitGenerateErrorReturn::LibraryName {
                    field: GenerateField::RecordsLibraryName,
                    error,
                })?,
            };
            let foundry_root = generate.foundry_root.clone();
            let mut segments = Vec::with_capacity(generate.generated_directory.len());
            for (index, segment) in generate.generated_directory.iter().enumerate() {
                segments.push(
                    GeneratedFileName::try_new(GeneratedFileNameConstructorParams {
                        text: segment.clone(),
                    })
                    .map_err(|error| AdmitGenerateErrorReturn::FileName {
                        field: GenerateField::GeneratedDirectorySegment { index },
                        error,
                    })?,
                );
            }
            let generated_directory =
                GeneratedDirectory::try_new(GeneratedDirectoryConstructorParams { segments })
                    .map_err(AdmitGenerateErrorReturn::GeneratedDirectory)?;
            let curve_directories = EvmCurveDirectories {
                bn254: GeneratedFileName::try_new(GeneratedFileNameConstructorParams {
                    text: generate.curve_directories.bn254.clone(),
                })
                .map_err(|error| AdmitGenerateErrorReturn::FileName {
                    field: GenerateField::Bn254Directory,
                    error,
                })?,
                bls12_381: GeneratedFileName::try_new(GeneratedFileNameConstructorParams {
                    text: generate.curve_directories.bls12_381.clone(),
                })
                .map_err(|error| AdmitGenerateErrorReturn::FileName {
                    field: GenerateField::Bls12381Directory,
                    error,
                })?,
            };
            let compiler_version = SolidityCompilerVersion {
                major: generate.compiler_version.major,
                minor: generate.compiler_version.minor,
                patch: generate.compiler_version.patch,
            };
            let license = SpdxLicenseIdentifier::try_new(SpdxLicenseIdentifierConstructorParams {
                text: generate.license.clone(),
            })
            .map_err(AdmitGenerateErrorReturn::License)?;
            Ok(AdmitGenerateSuccessReturn {
                concrete: GenerateConcrete::Evm {
                    settings: Box::new(EvmGenerateSettings {
                        kinds: settings_kinds,
                        constants_library,
                        records_library,
                        foundry_root,
                        generated_directory,
                        curve_directories,
                        compiler_version,
                        license,
                    }),
                },
                kinds,
            })
        }
        _ => Err(AdmitGenerateErrorReturn::UnknownValue {
            field: GenerateField::Concrete,
            text: generate.concrete.clone(),
        }),
    }
}
