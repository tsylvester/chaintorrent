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

use core::str::FromStr;

use pairing::{PAIRING_CURVES, PairingCurve};

use crate::generate::provides::{
    GENERATE_CONCRETES, GenerateConcrete, GenerateConcreteKind, GeneratedDirectory,
    GeneratedDirectoryConstructorParams, GeneratedFileName, GeneratedFileNameConstructorParams,
    MsmTermCount, MsmTermCountConstructorParams, SOLIDITY_GENERATE_DECLARATION,
    SolidityCompilerVersion, SolidityConstantName, SolidityConstantNameConstructorParams,
    SolidityCurveDirectories, SolidityCurveLibraries, SolidityCurveLibraryNames,
    SolidityGenerateSettings, SolidityLibraryName, SolidityLibraryNameConstructorParams,
    SolidityLibraryNames, SolidityMemberName, SolidityMemberNameConstructorParams,
    SolidityTypeName, SolidityTypeNameConstructorParams, SolidityVectorKindNames,
    SolidityVectorKindSettings, SpdxLicenseIdentifier, SpdxLicenseIdentifierConstructorParams,
    VectorCount, VectorCountConstructorParams, VectorKindConfiguration, VectorKindQuantities,
};
use interface::{
    AdmitGenerateDeps, AdmitGenerateErrorReturn, AdmitGenerateFn, AdmitGenerateParams,
    AdmitGeneratePayload, AdmitGenerateReturn, AdmitGenerateSuccessReturn, AdmittedGenerate,
    CompilerVersionErrorReturn,
};

pub fn admit_generate(
    _deps: &AdmitGenerateDeps,
    _params: AdmitGenerateParams,
    payload: AdmitGeneratePayload,
) -> AdmitGenerateReturn {
    let _: AdmitGenerateFn = admit_generate;

    let generate = payload.generate;

    let concrete_kind = match member_named(&generate.concrete, GENERATE_CONCRETES, |member| member)
    {
        Some(member) => *member,
        None => {
            return Err(AdmitGenerateErrorReturn::Concrete {
                text: generate.concrete.clone(),
            });
        }
    };

    let mut kinds = Vec::with_capacity(generate.kinds.len());
    let mut kind_settings = Vec::with_capacity(generate.kinds.len());
    for (key, entry) in &generate.kinds {
        let kind = match member_named(key, SOLIDITY_GENERATE_DECLARATION.kinds, |declaration| {
            &declaration.kind
        }) {
            Some(declaration) => declaration.kind,
            None => {
                return Err(AdmitGenerateErrorReturn::Kind { text: key.clone() });
            }
        };
        let count = match entry.count {
            Some(count) => match VectorCount::try_new(VectorCountConstructorParams { count }) {
                Ok(count) => Some(count),
                Err(error) => {
                    return Err(AdmitGenerateErrorReturn::Count {
                        kind: key.clone(),
                        error,
                    });
                }
            },
            None => None,
        };
        let terms = match entry.terms {
            Some(terms) => {
                match MsmTermCount::try_new(MsmTermCountConstructorParams { count: terms }) {
                    Ok(terms) => Some(terms),
                    Err(error) => {
                        return Err(AdmitGenerateErrorReturn::Terms {
                            kind: key.clone(),
                            error,
                        });
                    }
                }
            }
            None => None,
        };
        let file_name = match GeneratedFileName::try_new(GeneratedFileNameConstructorParams {
            text: entry.file.clone(),
        }) {
            Ok(file_name) => file_name,
            Err(error) => {
                return Err(AdmitGenerateErrorReturn::File {
                    kind: key.clone(),
                    error,
                });
            }
        };
        let struct_name = match SolidityTypeName::try_new(SolidityTypeNameConstructorParams {
            text: entry.struct_name.clone(),
        }) {
            Ok(struct_name) => struct_name,
            Err(error) => {
                return Err(AdmitGenerateErrorReturn::StructName {
                    kind: key.clone(),
                    error,
                });
            }
        };
        let decode_function_name =
            match SolidityMemberName::try_new(SolidityMemberNameConstructorParams {
                text: entry.decode_function.clone(),
            }) {
                Ok(decode_function_name) => decode_function_name,
                Err(error) => {
                    return Err(AdmitGenerateErrorReturn::DecodeFunction {
                        kind: key.clone(),
                        error,
                    });
                }
            };
        let path_constant_name =
            match SolidityConstantName::try_new(SolidityConstantNameConstructorParams {
                text: entry.path_constant.clone(),
            }) {
                Ok(path_constant_name) => path_constant_name,
                Err(error) => {
                    return Err(AdmitGenerateErrorReturn::PathConstant {
                        kind: key.clone(),
                        error,
                    });
                }
            };
        kind_settings.push(SolidityVectorKindSettings {
            kind,
            names: SolidityVectorKindNames {
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
                message_length: entry.message_length,
            },
        });
    }

    let mut admitted_libraries = Vec::with_capacity(generate.curve_libraries.len());
    for (key, entry) in &generate.curve_libraries {
        let curve = match member_named(key, PAIRING_CURVES, |member| member) {
            Some(member) => *member,
            None => {
                return Err(AdmitGenerateErrorReturn::CurveLibrariesKey { text: key.clone() });
            }
        };
        let constants_library = SolidityLibraryNames {
            file_name: match GeneratedFileName::try_new(GeneratedFileNameConstructorParams {
                text: entry.constants_library.file.clone(),
            }) {
                Ok(file_name) => file_name,
                Err(error) => {
                    return Err(AdmitGenerateErrorReturn::ConstantsLibraryFile { curve, error });
                }
            },
            library_name: match SolidityLibraryName::try_new(SolidityLibraryNameConstructorParams {
                text: entry.constants_library.library.clone(),
            }) {
                Ok(library_name) => library_name,
                Err(error) => {
                    return Err(AdmitGenerateErrorReturn::ConstantsLibraryName { curve, error });
                }
            },
        };
        let records_library = SolidityLibraryNames {
            file_name: match GeneratedFileName::try_new(GeneratedFileNameConstructorParams {
                text: entry.records_library.file.clone(),
            }) {
                Ok(file_name) => file_name,
                Err(error) => {
                    return Err(AdmitGenerateErrorReturn::RecordsLibraryFile { curve, error });
                }
            },
            library_name: match SolidityLibraryName::try_new(SolidityLibraryNameConstructorParams {
                text: entry.records_library.library.clone(),
            }) {
                Ok(library_name) => library_name,
                Err(error) => {
                    return Err(AdmitGenerateErrorReturn::RecordsLibraryName { curve, error });
                }
            },
        };
        admitted_libraries.push((
            curve,
            SolidityCurveLibraryNames {
                constants_library,
                records_library,
            },
        ));
    }
    for curve in PAIRING_CURVES {
        if !admitted_libraries.iter().any(|(named, _)| named == curve) {
            return Err(AdmitGenerateErrorReturn::MissingCurveLibraries { curve: *curve });
        }
    }
    let bn254 = match admitted_libraries
        .iter()
        .find(|(named, _)| *named == PairingCurve::Bn254)
    {
        Some((_, names)) => names.clone(),
        None => {
            return Err(AdmitGenerateErrorReturn::MissingCurveLibraries {
                curve: PairingCurve::Bn254,
            });
        }
    };
    let bls12_381 = match admitted_libraries
        .iter()
        .find(|(named, _)| *named == PairingCurve::Bls12381)
    {
        Some((_, names)) => names.clone(),
        None => {
            return Err(AdmitGenerateErrorReturn::MissingCurveLibraries {
                curve: PairingCurve::Bls12381,
            });
        }
    };
    let curve_libraries = SolidityCurveLibraries {
        bn254,
        bls12_381,
        #[cfg(any(test, feature = "mocks"))]
        mock: mock_curve_libraries()?,
    };

    let mut segments = Vec::with_capacity(generate.generated_directory.len());
    for (index, segment) in generate.generated_directory.iter().enumerate() {
        match GeneratedFileName::try_new(GeneratedFileNameConstructorParams {
            text: segment.clone(),
        }) {
            Ok(file_name) => segments.push(file_name),
            Err(error) => {
                return Err(AdmitGenerateErrorReturn::GeneratedDirectorySegment { index, error });
            }
        }
    }
    let generated_directory =
        match GeneratedDirectory::try_new(GeneratedDirectoryConstructorParams { segments }) {
            Ok(directory) => directory,
            Err(error) => {
                return Err(AdmitGenerateErrorReturn::GeneratedDirectory { error });
            }
        };

    let mut admitted_directories = Vec::with_capacity(generate.curve_directories.len());
    for (key, directory) in &generate.curve_directories {
        let curve = match member_named(key, PAIRING_CURVES, |member| member) {
            Some(member) => *member,
            None => {
                return Err(AdmitGenerateErrorReturn::CurveKey { text: key.clone() });
            }
        };
        match GeneratedFileName::try_new(GeneratedFileNameConstructorParams {
            text: directory.clone(),
        }) {
            Ok(file_name) => admitted_directories.push((curve, file_name)),
            Err(error) => {
                return Err(AdmitGenerateErrorReturn::CurveDirectory { curve, error });
            }
        }
    }
    for curve in PAIRING_CURVES {
        if !admitted_directories.iter().any(|(named, _)| named == curve) {
            return Err(AdmitGenerateErrorReturn::MissingCurveDirectory { curve: *curve });
        }
    }
    let bn254 = match admitted_directories
        .iter()
        .find(|(named, _)| *named == PairingCurve::Bn254)
    {
        Some((_, file_name)) => file_name.clone(),
        None => {
            return Err(AdmitGenerateErrorReturn::MissingCurveDirectory {
                curve: PairingCurve::Bn254,
            });
        }
    };
    let bls12_381 = match admitted_directories
        .iter()
        .find(|(named, _)| *named == PairingCurve::Bls12381)
    {
        Some((_, file_name)) => file_name.clone(),
        None => {
            return Err(AdmitGenerateErrorReturn::MissingCurveDirectory {
                curve: PairingCurve::Bls12381,
            });
        }
    };
    let curve_directories = SolidityCurveDirectories {
        bn254,
        bls12_381,
        #[cfg(any(test, feature = "mocks"))]
        mock: mock_curve_directory()?,
    };

    let compiler_version = match parse_compiler_version(&generate.compiler_version) {
        Ok(version) => version,
        Err(error) => {
            return Err(AdmitGenerateErrorReturn::CompilerVersion {
                text: generate.compiler_version.clone(),
                error,
            });
        }
    };

    let license = match SpdxLicenseIdentifier::try_new(SpdxLicenseIdentifierConstructorParams {
        text: generate.license.clone(),
    }) {
        Ok(license) => license,
        Err(error) => {
            return Err(AdmitGenerateErrorReturn::License { error });
        }
    };

    let settings = SolidityGenerateSettings {
        kinds: kind_settings,
        curve_libraries,
        foundry_root: generate.foundry_root.clone(),
        generated_directory,
        curve_directories,
        compiler_version,
        license,
    };
    let concrete = match concrete_kind {
        GenerateConcreteKind::Solidity => GenerateConcrete::Solidity {
            settings: Box::new(settings),
        },
    };
    Ok(AdmitGenerateSuccessReturn {
        admitted: AdmittedGenerate { concrete, kinds },
    })
}

fn member_named<'a, M, D: core::fmt::Debug>(
    text: &str,
    members: &'a [M],
    name_of: impl Fn(&M) -> &D,
) -> Option<&'a M> {
    members
        .iter()
        .find(|member| format!("{:?}", name_of(member)) == text)
}

#[cfg(any(test, feature = "mocks"))]
fn mock_curve_directory() -> Result<GeneratedFileName, AdmitGenerateErrorReturn> {
    GeneratedFileName::try_new(GeneratedFileNameConstructorParams {
        text: "mock".to_string(),
    })
    .map_err(|error| AdmitGenerateErrorReturn::CurveDirectory {
        curve: PairingCurve::Mock,
        error,
    })
}

#[cfg(any(test, feature = "mocks"))]
fn mock_curve_libraries() -> Result<SolidityCurveLibraryNames, AdmitGenerateErrorReturn> {
    let constants_library = SolidityLibraryNames {
        file_name: GeneratedFileName::try_new(GeneratedFileNameConstructorParams {
            text: "Constants.sol".to_string(),
        })
        .map_err(|error| AdmitGenerateErrorReturn::ConstantsLibraryFile {
            curve: PairingCurve::Mock,
            error,
        })?,
        library_name: SolidityLibraryName::try_new(SolidityLibraryNameConstructorParams {
            text: "MockConstants".to_string(),
        })
        .map_err(|error| AdmitGenerateErrorReturn::ConstantsLibraryName {
            curve: PairingCurve::Mock,
            error,
        })?,
    };
    let records_library = SolidityLibraryNames {
        file_name: GeneratedFileName::try_new(GeneratedFileNameConstructorParams {
            text: "VectorRecords.sol".to_string(),
        })
        .map_err(|error| AdmitGenerateErrorReturn::RecordsLibraryFile {
            curve: PairingCurve::Mock,
            error,
        })?,
        library_name: SolidityLibraryName::try_new(SolidityLibraryNameConstructorParams {
            text: "MockVectorRecords".to_string(),
        })
        .map_err(|error| AdmitGenerateErrorReturn::RecordsLibraryName {
            curve: PairingCurve::Mock,
            error,
        })?,
    };
    Ok(SolidityCurveLibraryNames {
        constants_library,
        records_library,
    })
}

fn parse_compiler_version(
    text: &str,
) -> Result<SolidityCompilerVersion, CompilerVersionErrorReturn> {
    let parts: Vec<&str> = text.split('.').collect();
    if parts.len() != 3 {
        return Err(CompilerVersionErrorReturn::Parts { count: parts.len() });
    }
    let mut parsed = [0_u16; 3];
    for (index, part) in parts.iter().enumerate() {
        parsed[index] = match u16::from_str(part) {
            Ok(value) => value,
            Err(_) => {
                return Err(CompilerVersionErrorReturn::Part {
                    index,
                    text: (*part).to_string(),
                });
            }
        };
    }
    Ok(SolidityCompilerVersion {
        major: parsed[0],
        minor: parsed[1],
        patch: parsed[2],
    })
}
