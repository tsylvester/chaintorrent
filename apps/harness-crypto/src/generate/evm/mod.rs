mod challenge_vectors;
mod constants;
mod group_vectors;
#[cfg(test)]
mod integration_test;
mod interface;
mod mapping_vectors;
#[cfg(any(test, feature = "mocks"))]
mod mock;
mod possession_vectors;
mod proof_vectors;
pub mod provides;
mod rejection_vectors;
mod render;
#[cfg(test)]
mod test;

use domain::{ASSET_IDENTITY_HASH_LENGTH, AssetIdentityHash, AssetIdentityHashConstructorParams};
use encoding::{EncodeParams, IEncoderAdapter, IEncodingContract};
use envelope::IKeyAgreementAdapter;
use kem::{ICredentialKemAdapter, IdentityScope};
use pairing::{IPairingArithmetic, IPairingReference, PairingCurve, VerifierGroupArithmetic};
use proof::IDeliveryProofAdapter;
use std::fs;
use std::path::PathBuf;

use self::challenge_vectors::provides::{
    ChallengeVectorDescription, ChallengeVectorDescriptionConstructorParams, ChallengeVectorsDeps,
    ChallengeVectorsParams, ChallengeVectorsPayload, challenge_vectors,
};
use self::constants::provides::{ConstantsDeps, ConstantsParams, ConstantsPayload, constants};
use self::group_vectors::provides::{
    FirstGroupVectorCounts, GroupOperationVectorDescription,
    GroupOperationVectorDescriptionConstructorParams, GroupVectorsDeps, GroupVectorsErrorReturn,
    GroupVectorsParams, GroupVectorsPayload, MsmVectorSettings, PairingCheckVectorDescription,
    PairingCheckVectorDescriptionConstructorParams, SecondGroupVectorCounts, VectorCount,
    group_vectors,
};
use self::mapping_vectors::provides::{
    HashToScalarVectorDescription, HashToScalarVectorDescriptionConstructorParams,
    IdentityMappingVectorDescription, IdentityMappingVectorDescriptionConstructorParams,
    MappingVectorsDeps, MappingVectorsParams, MappingVectorsPayload, mapping_vectors,
};
use self::possession_vectors::provides::{
    PossessionVectorDescription, PossessionVectorDescriptionConstructorParams,
    PossessionVectorsDeps, PossessionVectorsParams, PossessionVectorsPayload, possession_vectors,
};
use self::proof_vectors::provides::{
    MintProofVectorDescription, MintProofVectorDescriptionConstructorParams, ProofVectorsDeps,
    ProofVectorsParams, ProofVectorsPayload, TransferProofVectorDescription,
    TransferProofVectorDescriptionConstructorParams, proof_vectors,
};
use self::rejection_vectors::provides::{
    RejectionVectorDescription, RejectionVectorDescriptionConstructorParams, RejectionVectorsDeps,
    RejectionVectorsParams, RejectionVectorsPayload, rejection_vectors,
};
use self::render::provides::{
    RenderDeps, RenderParams, RenderPayload, SolidityLibraryBody, SolidityLibraryEntries,
    SolidityLibraryEntriesConstructorParams, SolidityRecordDeclaration, SolidityRecordDeclarations,
    SolidityRecordDeclarationsConstructorParams, SolidityStringLiteral,
    SolidityStringLiteralConstructorParams, render,
};
use super::factory::provides::{
    EmitErrorReturn, EmitParams, EmitPayload, EmitReturn, EmitSuccessReturn, EmittedFile,
    GenerateDeclaration, IGenerateAdapter, VectorKind, VectorKindSize,
};
use interface::{
    EVM_GENERATE_DECLARATION, EvmGenerate, EvmGenerateConstructorParams, EvmGenerateErrorReturn,
    EvmGenerateTryNewReturn, EvmVectorKindNames, GeneratedDirectory,
    GeneratedDirectoryConstructorParams, GeneratedDirectoryTryNewErrorReturn,
    GeneratedDirectoryTryNewReturn, GeneratedFileName, GeneratedFileNameConstructorParams,
    GeneratedFileNameTryNewErrorReturn, GeneratedFileNameTryNewReturn,
};

impl GeneratedFileName {
    pub fn try_new(params: GeneratedFileNameConstructorParams) -> GeneratedFileNameTryNewReturn {
        let bytes = params.text.as_bytes();
        if bytes.is_empty() {
            return Err(GeneratedFileNameTryNewErrorReturn::Empty);
        }
        for (index, byte) in bytes.iter().enumerate() {
            if !matches!(byte, b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'.' | b'_' | b'-') {
                return Err(GeneratedFileNameTryNewErrorReturn::InvalidCharacter {
                    index,
                    byte: *byte,
                });
            }
        }
        if params.text == "." || params.text == ".." {
            return Err(GeneratedFileNameTryNewErrorReturn::Reserved);
        }
        Ok(GeneratedFileName { text: params.text })
    }

    pub fn as_str(&self) -> &str {
        &self.text
    }
}

impl GeneratedDirectory {
    pub fn try_new(params: GeneratedDirectoryConstructorParams) -> GeneratedDirectoryTryNewReturn {
        if params.segments.is_empty() {
            return Err(GeneratedDirectoryTryNewErrorReturn::Empty);
        }
        Ok(GeneratedDirectory {
            segments: params.segments,
        })
    }
}

impl<'a, P, E, K, A, D> EvmGenerate<'a, P, E, K, A, D>
where
    P: IPairingArithmetic + IPairingReference,
    E: IEncoderAdapter,
    K: ICredentialKemAdapter<Pairing = P>,
    A: IKeyAgreementAdapter<Pairing = P>,
    D: IDeliveryProofAdapter<Pairing = P>,
{
    pub fn try_new(
        params: EvmGenerateConstructorParams<'a, P, E, K, A, D>,
    ) -> EvmGenerateTryNewReturn<'a, P, E, K, A, D> {
        Ok(EvmGenerate {
            pairing: params.pairing,
            hash_to_scalar: params.hash_to_scalar,
            encoder: params.encoder,
            random: params.random,
            kem: params.kem,
            key_agreement: params.key_agreement,
            delivery_proof: params.delivery_proof,
            scope: params.scope,
            statement_version: params.statement_version,
            settings: params.settings,
            sizes: params.sizes,
        })
    }
}

impl<'a, P, E, K, A, D> IGenerateAdapter for EvmGenerate<'a, P, E, K, A, D>
where
    P: IPairingArithmetic + IPairingReference,
    E: IEncoderAdapter,
    K: ICredentialKemAdapter<Pairing = P>,
    A: IKeyAgreementAdapter<Pairing = P>,
    D: IDeliveryProofAdapter<Pairing = P>,
{
    const DECLARATION: GenerateDeclaration = EVM_GENERATE_DECLARATION;

    fn emit(&self, _params: EmitParams, payload: EmitPayload<'_>) -> EmitReturn {
        fn missing(kind: VectorKind) -> EmitErrorReturn {
            EmitErrorReturn::Evm(EvmGenerateErrorReturn::MissingKindSettings { kind })
        }

        fn size_of(
            resolved: &[(VectorKind, EvmVectorKindNames, VectorKindSize)],
            kind: VectorKind,
        ) -> Option<VectorKindSize> {
            resolved
                .iter()
                .find(|(entry, _, _)| *entry == kind)
                .map(|(_, _, size)| *size)
        }

        fn drawn_count(
            resolved: &[(VectorKind, EvmVectorKindNames, VectorKindSize)],
            kind: VectorKind,
        ) -> Option<VectorCount> {
            match size_of(resolved, kind) {
                Some(VectorKindSize::Drawn { count }) => Some(count),
                _ => None,
            }
        }

        fn drawn_terms(
            resolved: &[(VectorKind, EvmVectorKindNames, VectorKindSize)],
            kind: VectorKind,
        ) -> Option<MsmVectorSettings> {
            match size_of(resolved, kind) {
                Some(VectorKindSize::DrawnTerms { count, terms }) => {
                    Some(MsmVectorSettings { count, terms })
                }
                _ => None,
            }
        }

        fn drawn_messages(
            resolved: &[(VectorKind, EvmVectorKindNames, VectorKindSize)],
            kind: VectorKind,
        ) -> Option<(VectorCount, usize)> {
            match size_of(resolved, kind) {
                Some(VectorKindSize::DrawnMessages {
                    count,
                    message_length,
                }) => Some((count, message_length)),
                _ => None,
            }
        }

        fn encode_lines<E2: IEncoderAdapter, C: IEncodingContract>(
            encoder: &E2,
            description: &C,
            records: &[C::Described],
        ) -> Result<String, EmitErrorReturn> {
            let mut contents = String::new();
            for record in records {
                let encoded = encoder
                    .encode(EncodeParams { description }, record)
                    .map_err(EvmGenerateErrorReturn::Encode)
                    .map_err(EmitErrorReturn::Evm)?;
                contents.push_str("0x");
                for byte in &encoded.bytes {
                    contents.push_str(&format!("{byte:02x}"));
                }
                contents.push('\n');
            }
            Ok(contents)
        }

        match self.scope {
            IdentityScope::Entitlement => {}
            IdentityScope::Asset => {
                for (index, identity) in payload.identities.iter().enumerate() {
                    let actual = identity.len();
                    let Ok(bytes) =
                        <[u8; ASSET_IDENTITY_HASH_LENGTH]>::try_from(identity.as_slice())
                    else {
                        return Err(EmitErrorReturn::AssetIdentityLength { index, actual });
                    };
                    if let Err(error) =
                        AssetIdentityHash::try_new(AssetIdentityHashConstructorParams { bytes })
                    {
                        return Err(EmitErrorReturn::AssetIdentityHash { index, error });
                    }
                }
                let actual = payload.proof_identity.len();
                let Ok(bytes) =
                    <[u8; ASSET_IDENTITY_HASH_LENGTH]>::try_from(payload.proof_identity)
                else {
                    return Err(EmitErrorReturn::AssetProofIdentityLength { actual });
                };
                if let Err(error) =
                    AssetIdentityHash::try_new(AssetIdentityHashConstructorParams { bytes })
                {
                    return Err(EmitErrorReturn::AssetProofIdentityHash(error));
                }
            }
        }

        let emitted: Vec<VectorKind> = match P::DECLARATION.verifier_group_arithmetic {
            VerifierGroupArithmetic::FirstGroupOnly => Self::DECLARATION
                .kinds
                .iter()
                .filter(|declaration| !declaration.requires_second_group_arithmetic)
                .map(|declaration| declaration.kind)
                .collect(),
            VerifierGroupArithmetic::BothGroups => Self::DECLARATION
                .kinds
                .iter()
                .map(|declaration| declaration.kind)
                .collect(),
        };

        let mut resolved: Vec<(VectorKind, EvmVectorKindNames, VectorKindSize)> =
            Vec::with_capacity(emitted.len());
        for kind in &emitted {
            let names = self
                .settings
                .kinds
                .iter()
                .find(|entry| entry.kind == *kind)
                .map(|entry| entry.names.clone());
            let size = self
                .sizes
                .iter()
                .find(|entry| entry.kind == *kind)
                .map(|entry| entry.size);
            let (Some(names), Some(size)) = (names, size) else {
                return Err(missing(*kind));
            };
            resolved.push((*kind, names, size));
        }

        let Some(g1_add) = drawn_count(&resolved, VectorKind::G1Addition) else {
            return Err(missing(VectorKind::G1Addition));
        };
        let Some(g1_mul) = drawn_count(&resolved, VectorKind::G1Multiplication) else {
            return Err(missing(VectorKind::G1Multiplication));
        };
        let Some(pairing_check) = drawn_count(&resolved, VectorKind::PairingCheck) else {
            return Err(missing(VectorKind::PairingCheck));
        };
        let Some((hash_to_scalar_count, message_length)) =
            drawn_messages(&resolved, VectorKind::HashToScalar)
        else {
            return Err(missing(VectorKind::HashToScalar));
        };
        let Some(possession_count) = drawn_count(&resolved, VectorKind::Possession) else {
            return Err(missing(VectorKind::Possession));
        };
        let Some(challenge_count) = drawn_count(&resolved, VectorKind::Challenge) else {
            return Err(missing(VectorKind::Challenge));
        };
        let Some(mint_count) = drawn_count(&resolved, VectorKind::MintProof) else {
            return Err(missing(VectorKind::MintProof));
        };
        let Some(transfer_count) = drawn_count(&resolved, VectorKind::TransferProof) else {
            return Err(missing(VectorKind::TransferProof));
        };
        let second_group = match P::DECLARATION.verifier_group_arithmetic {
            VerifierGroupArithmetic::BothGroups => {
                let Some(g2_add) = drawn_count(&resolved, VectorKind::G2Addition) else {
                    return Err(missing(VectorKind::G2Addition));
                };
                let Some(g1_msm) = drawn_terms(&resolved, VectorKind::G1MultiScalarMultiplication)
                else {
                    return Err(missing(VectorKind::G1MultiScalarMultiplication));
                };
                let Some(g2_msm) = drawn_terms(&resolved, VectorKind::G2MultiScalarMultiplication)
                else {
                    return Err(missing(VectorKind::G2MultiScalarMultiplication));
                };
                Some(SecondGroupVectorCounts {
                    g2_add,
                    g1_msm,
                    g2_msm,
                })
            }
            VerifierGroupArithmetic::FirstGroupOnly => None,
        };

        let constants = constants::<P, D::Forms>(
            &ConstantsDeps {
                pairing: self.pairing,
            },
            ConstantsParams {
                statement_version: self.statement_version,
            },
            ConstantsPayload {
                kem: &K::DECLARATION,
                key_agreement: &A::DECLARATION,
                delivery_proof: &D::DECLARATION,
            },
        )
        .map_err(EvmGenerateErrorReturn::Constants)
        .map_err(EmitErrorReturn::Evm)?;
        let entries = SolidityLibraryEntries::try_new(SolidityLibraryEntriesConstructorParams {
            entries: constants.entries,
        })
        .map_err(EvmGenerateErrorReturn::Entries)
        .map_err(EmitErrorReturn::Evm)?;

        let groups = group_vectors::<P>(
            &GroupVectorsDeps {
                pairing: self.pairing,
                random: self.random,
            },
            GroupVectorsParams {
                first_group: FirstGroupVectorCounts {
                    g1_add,
                    g1_mul,
                    pairing_check,
                },
                second_group,
            },
            GroupVectorsPayload,
        )
        .map_err(EvmGenerateErrorReturn::GroupVectors)
        .map_err(EmitErrorReturn::Evm)?;

        let rejections = rejection_vectors::<P>(
            &RejectionVectorsDeps {
                pairing: self.pairing,
            },
            RejectionVectorsParams,
            RejectionVectorsPayload,
        )
        .map_err(EvmGenerateErrorReturn::RejectionVectors)
        .map_err(EmitErrorReturn::Evm)?;

        let mapping = mapping_vectors::<P, K>(
            &MappingVectorsDeps {
                pairing: self.pairing,
                hash_to_scalar: self.hash_to_scalar,
                kem: self.kem,
                random: self.random,
            },
            MappingVectorsParams {
                hash_to_scalar_count,
                message_length,
                scope: self.scope,
            },
            MappingVectorsPayload {
                key_agreement: &A::DECLARATION,
                delivery_proof: &D::DECLARATION,
                identities: payload.identities,
            },
        )
        .map_err(EvmGenerateErrorReturn::MappingVectors)
        .map_err(EmitErrorReturn::Evm)?;

        let possession = possession_vectors::<P, A>(
            &PossessionVectorsDeps {
                pairing: self.pairing,
                key_agreement: self.key_agreement,
                random: self.random,
            },
            PossessionVectorsParams {
                count: possession_count,
            },
            PossessionVectorsPayload,
        )
        .map_err(EvmGenerateErrorReturn::PossessionVectors)
        .map_err(EmitErrorReturn::Evm)?;

        let challenges = challenge_vectors::<P, E, D::Forms>(
            &ChallengeVectorsDeps {
                pairing: self.pairing,
                encoder: self.encoder,
                hash_to_scalar: self.hash_to_scalar,
                random: self.random,
            },
            ChallengeVectorsParams {
                count: challenge_count,
                statement_version: self.statement_version,
            },
            ChallengeVectorsPayload {
                delivery_proof: &D::DECLARATION,
                mint: payload.mint,
                mint_purpose: payload.mint_purpose,
                transfer: payload.transfer,
                transfer_purpose: payload.transfer_purpose,
            },
        )
        .map_err(EvmGenerateErrorReturn::ChallengeVectors)
        .map_err(EmitErrorReturn::Evm)?;

        let proofs = proof_vectors::<P, K, A, D>(
            &ProofVectorsDeps {
                pairing: self.pairing,
                kem: self.kem,
                key_agreement: self.key_agreement,
                delivery_proof: self.delivery_proof,
                random: self.random,
            },
            ProofVectorsParams {
                mint_count,
                transfer_count,
                scope: self.scope,
            },
            ProofVectorsPayload {
                mint: payload.mint,
                mint_purpose: payload.mint_purpose,
                transfer: payload.transfer,
                transfer_purpose: payload.transfer_purpose,
                identity: payload.proof_identity,
            },
        )
        .map_err(EvmGenerateErrorReturn::ProofVectors)
        .map_err(EmitErrorReturn::Evm)?;

        let curve_directory = match P::DECLARATION.curve {
            PairingCurve::Bn254 => &self.settings.curve_directories.bn254,
            PairingCurve::Bls12381 => &self.settings.curve_directories.bls12_381,
            #[cfg(any(test, feature = "mocks"))]
            PairingCurve::Mock => &self.settings.curve_directories.bn254,
        };
        let mut directory = self.settings.foundry_root.clone();
        for segment in &self.settings.generated_directory.segments {
            directory = directory.join(segment.as_str());
        }
        let directory = directory.join(curve_directory.as_str());

        let mut kind_files: Vec<(VectorKind, PathBuf, String)> = Vec::with_capacity(resolved.len());
        let mut declarations: Vec<SolidityRecordDeclaration> = Vec::with_capacity(resolved.len());
        for (kind, names, _size) in &resolved {
            let mut path_segments: Vec<&str> = self
                .settings
                .generated_directory
                .segments
                .iter()
                .map(GeneratedFileName::as_str)
                .collect();
            path_segments.push(curve_directory.as_str());
            path_segments.push(names.file_name.as_str());
            let path = SolidityStringLiteral::try_new(SolidityStringLiteralConstructorParams {
                text: path_segments.join("/"),
            })
            .map_err(EvmGenerateErrorReturn::Path)
            .map_err(EmitErrorReturn::Evm)?;
            let (members, contents) = match *kind {
                VectorKind::G1Addition => {
                    let Ok(description) = GroupOperationVectorDescription::try_new(
                        GroupOperationVectorDescriptionConstructorParams,
                    );
                    let members = description
                        .members()
                        .map_err(EvmGenerateErrorReturn::GroupMembers)
                        .map_err(EmitErrorReturn::Evm)?;
                    (
                        members,
                        encode_lines(self.encoder, &description, &groups.first_group.g1_add)?,
                    )
                }
                VectorKind::G1Multiplication => {
                    let Ok(description) = GroupOperationVectorDescription::try_new(
                        GroupOperationVectorDescriptionConstructorParams,
                    );
                    let members = description
                        .members()
                        .map_err(EvmGenerateErrorReturn::GroupMembers)
                        .map_err(EmitErrorReturn::Evm)?;
                    (
                        members,
                        encode_lines(self.encoder, &description, &groups.first_group.g1_mul)?,
                    )
                }
                VectorKind::PairingCheck => {
                    let Ok(description) = PairingCheckVectorDescription::try_new(
                        PairingCheckVectorDescriptionConstructorParams,
                    );
                    let members = description
                        .members()
                        .map_err(EvmGenerateErrorReturn::GroupMembers)
                        .map_err(EmitErrorReturn::Evm)?;
                    (
                        members,
                        encode_lines(
                            self.encoder,
                            &description,
                            &groups.first_group.pairing_check,
                        )?,
                    )
                }
                VectorKind::G2Addition => {
                    let Some(second) = groups.second_group.as_ref() else {
                        return Err(EmitErrorReturn::Evm(EvmGenerateErrorReturn::GroupVectors(
                            GroupVectorsErrorReturn::SecondGroupSettingsMissing,
                        )));
                    };
                    let Ok(description) = GroupOperationVectorDescription::try_new(
                        GroupOperationVectorDescriptionConstructorParams,
                    );
                    let members = description
                        .members()
                        .map_err(EvmGenerateErrorReturn::GroupMembers)
                        .map_err(EmitErrorReturn::Evm)?;
                    (
                        members,
                        encode_lines(self.encoder, &description, &second.g2_add)?,
                    )
                }
                VectorKind::G1MultiScalarMultiplication => {
                    let Some(second) = groups.second_group.as_ref() else {
                        return Err(EmitErrorReturn::Evm(EvmGenerateErrorReturn::GroupVectors(
                            GroupVectorsErrorReturn::SecondGroupSettingsMissing,
                        )));
                    };
                    let Ok(description) = GroupOperationVectorDescription::try_new(
                        GroupOperationVectorDescriptionConstructorParams,
                    );
                    let members = description
                        .members()
                        .map_err(EvmGenerateErrorReturn::GroupMembers)
                        .map_err(EmitErrorReturn::Evm)?;
                    (
                        members,
                        encode_lines(self.encoder, &description, &second.g1_msm)?,
                    )
                }
                VectorKind::G2MultiScalarMultiplication => {
                    let Some(second) = groups.second_group.as_ref() else {
                        return Err(EmitErrorReturn::Evm(EvmGenerateErrorReturn::GroupVectors(
                            GroupVectorsErrorReturn::SecondGroupSettingsMissing,
                        )));
                    };
                    let Ok(description) = GroupOperationVectorDescription::try_new(
                        GroupOperationVectorDescriptionConstructorParams,
                    );
                    let members = description
                        .members()
                        .map_err(EvmGenerateErrorReturn::GroupMembers)
                        .map_err(EmitErrorReturn::Evm)?;
                    (
                        members,
                        encode_lines(self.encoder, &description, &second.g2_msm)?,
                    )
                }
                VectorKind::DecodeRejection => {
                    let Ok(description) = RejectionVectorDescription::try_new(
                        RejectionVectorDescriptionConstructorParams,
                    );
                    let members = description
                        .members()
                        .map_err(EvmGenerateErrorReturn::RejectionMembers)
                        .map_err(EmitErrorReturn::Evm)?;
                    (
                        members,
                        encode_lines(self.encoder, &description, &rejections.records)?,
                    )
                }
                VectorKind::HashToScalar => {
                    let Ok(description) = HashToScalarVectorDescription::try_new(
                        HashToScalarVectorDescriptionConstructorParams,
                    );
                    let members = description
                        .members()
                        .map_err(EvmGenerateErrorReturn::MappingMembers)
                        .map_err(EmitErrorReturn::Evm)?;
                    (
                        members,
                        encode_lines(self.encoder, &description, &mapping.hash_to_scalar)?,
                    )
                }
                VectorKind::IdentityMapping => {
                    let Ok(description) = IdentityMappingVectorDescription::try_new(
                        IdentityMappingVectorDescriptionConstructorParams,
                    );
                    let members = description
                        .members()
                        .map_err(EvmGenerateErrorReturn::MappingMembers)
                        .map_err(EmitErrorReturn::Evm)?;
                    (
                        members,
                        encode_lines(self.encoder, &description, &mapping.identity_mapping)?,
                    )
                }
                VectorKind::Possession => {
                    let Ok(description) = PossessionVectorDescription::try_new(
                        PossessionVectorDescriptionConstructorParams,
                    );
                    let members = description
                        .members()
                        .map_err(EvmGenerateErrorReturn::PossessionMembers)
                        .map_err(EmitErrorReturn::Evm)?;
                    (
                        members,
                        encode_lines(self.encoder, &description, &possession.records)?,
                    )
                }
                VectorKind::Challenge => {
                    let Ok(description) = ChallengeVectorDescription::try_new(
                        ChallengeVectorDescriptionConstructorParams,
                    );
                    let members = description
                        .members()
                        .map_err(EvmGenerateErrorReturn::ChallengeMembers)
                        .map_err(EmitErrorReturn::Evm)?;
                    (
                        members,
                        encode_lines(self.encoder, &description, &challenges.records)?,
                    )
                }
                VectorKind::MintProof => {
                    let Ok(description) = MintProofVectorDescription::<D::Forms>::try_new(
                        MintProofVectorDescriptionConstructorParams,
                    );
                    let members = description
                        .members()
                        .map_err(EvmGenerateErrorReturn::ProofMembers)
                        .map_err(EmitErrorReturn::Evm)?;
                    (
                        members,
                        encode_lines(self.encoder, &description, &proofs.mint)?,
                    )
                }
                VectorKind::TransferProof => {
                    let Ok(description) = TransferProofVectorDescription::<D::Forms>::try_new(
                        TransferProofVectorDescriptionConstructorParams,
                    );
                    let members = description
                        .members()
                        .map_err(EvmGenerateErrorReturn::ProofMembers)
                        .map_err(EmitErrorReturn::Evm)?;
                    (
                        members,
                        encode_lines(self.encoder, &description, &proofs.transfer)?,
                    )
                }
            };
            declarations.push(SolidityRecordDeclaration {
                struct_name: names.struct_name.clone(),
                members,
                decode_function_name: names.decode_function_name.clone(),
                path_constant_name: names.path_constant_name.clone(),
                path,
            });
            kind_files.push((*kind, directory.join(names.file_name.as_str()), contents));
        }
        let record_declarations =
            SolidityRecordDeclarations::try_new(SolidityRecordDeclarationsConstructorParams {
                declarations,
            })
            .map_err(EvmGenerateErrorReturn::Declarations)
            .map_err(EmitErrorReturn::Evm)?;

        let Ok(constants_source) = render(
            &RenderDeps,
            RenderParams {
                license: self.settings.license.clone(),
                compiler_version: self.settings.compiler_version,
            },
            RenderPayload {
                library_name: self.settings.constants_library.library_name.clone(),
                body: SolidityLibraryBody::Constants(entries),
            },
        );
        let Ok(records_source) = render(
            &RenderDeps,
            RenderParams {
                license: self.settings.license.clone(),
                compiler_version: self.settings.compiler_version,
            },
            RenderPayload {
                library_name: self.settings.records_library.library_name.clone(),
                body: SolidityLibraryBody::Records(record_declarations),
            },
        );

        fs::create_dir_all(&directory).map_err(|error| {
            EmitErrorReturn::Evm(EvmGenerateErrorReturn::CreateDirectory {
                path: directory.clone(),
                error,
            })
        })?;

        let mut files: Vec<EmittedFile> = Vec::with_capacity(resolved.len() + 2);
        for (kind, path, contents) in kind_files {
            fs::write(&path, &contents).map_err(|error| {
                EmitErrorReturn::Evm(EvmGenerateErrorReturn::WriteFile {
                    path: path.clone(),
                    error,
                })
            })?;
            files.push(EmittedFile {
                kind: Some(kind),
                path,
            });
        }
        let constants_path = directory.join(self.settings.constants_library.file_name.as_str());
        fs::write(&constants_path, constants_source.source.as_str()).map_err(|error| {
            EmitErrorReturn::Evm(EvmGenerateErrorReturn::WriteFile {
                path: constants_path.clone(),
                error,
            })
        })?;
        files.push(EmittedFile {
            kind: None,
            path: constants_path,
        });
        let records_path = directory.join(self.settings.records_library.file_name.as_str());
        fs::write(&records_path, records_source.source.as_str()).map_err(|error| {
            EmitErrorReturn::Evm(EvmGenerateErrorReturn::WriteFile {
                path: records_path.clone(),
                error,
            })
        })?;
        files.push(EmittedFile {
            kind: None,
            path: records_path,
        });

        Ok(EmitSuccessReturn { files })
    }
}
