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

use encoding::{
    CanonicalFieldKind, EncodeParams, IEncoderAdapter, IEncodingContract, TypeNameParams,
};
use envelope::IKeyAgreementAdapter;
use kem::ICredentialKemAdapter;
use pairing::{IPairingArithmetic, IPairingReference, PairingCurve, VerifierGroupArithmetic};
use proof::IDeliveryProofAdapter;
use std::fs;
use std::path::PathBuf;

use super::factory::provides::{
    EmitErrorReturn, EmitParams, EmitPayload, EmitReturn, EmitSuccessReturn, EmittedFile,
    GenerateDeclaration, IGenerateAdapter, VectorKind, VectorKindSize,
};

use self::challenge_vectors::provides::{
    ChallengeVectorDescription, ChallengeVectorDescriptionConstructorParams, ChallengeVectorsDeps,
    ChallengeVectorsParams, ChallengeVectorsPayload, challenge_vectors,
};
use self::constants::provides::{ConstantsDeps, ConstantsParams, ConstantsPayload, constants};
use self::group_vectors::provides::{
    FirstGroupVectorCounts, GroupOperationVectorDescription,
    GroupOperationVectorDescriptionConstructorParams, GroupVectorsDeps, GroupVectorsParams,
    GroupVectorsPayload, MsmVectorSettings, PairingCheckVectorDescription,
    PairingCheckVectorDescriptionConstructorParams, SecondGroupVectorCounts, VectorCount,
    group_vectors,
};
use self::interface::{
    GeneratedDirectory, GeneratedDirectoryConstructorParams, GeneratedDirectoryTryNewErrorReturn,
    GeneratedDirectoryTryNewReturn, GeneratedFileName, GeneratedFileNameConstructorParams,
    GeneratedFileNameTryNewErrorReturn, GeneratedFileNameTryNewReturn,
    SOLIDITY_GENERATE_DECLARATION, SolidityGenerate, SolidityGenerateConstructorParams,
    SolidityGenerateErrorReturn, SolidityGenerateTryNewReturn,
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
    RenderDeps, RenderParams, RenderPayload, SolidityAbiType, SolidityAbiTypeConstructorParams,
    SolidityLibraryBody, SolidityLibraryEntries, SolidityLibraryEntriesConstructorParams,
    SolidityMemberName, SolidityRecordDeclaration, SolidityRecordDeclarations,
    SolidityRecordDeclarationsConstructorParams, SolidityRecordMember, SolidityRecordMembers,
    SolidityRecordMembersConstructorParams, SolidityStringLiteral,
    SolidityStringLiteralConstructorParams, render,
};

impl GeneratedFileName {
    pub fn try_new(params: GeneratedFileNameConstructorParams) -> GeneratedFileNameTryNewReturn {
        let bytes = params.text.as_bytes();
        if bytes.is_empty() {
            return Err(GeneratedFileNameTryNewErrorReturn::Empty);
        }
        for (index, &byte) in bytes.iter().enumerate() {
            if !(byte.is_ascii_alphanumeric() || byte == b'.' || byte == b'_' || byte == b'-') {
                return Err(GeneratedFileNameTryNewErrorReturn::InvalidCharacter { index, byte });
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

impl<'a, P, E, K, A, D> SolidityGenerate<'a, P, E, K, A, D>
where
    P: IPairingArithmetic + IPairingReference,
    E: IEncoderAdapter,
    K: ICredentialKemAdapter<Pairing = P>,
    A: IKeyAgreementAdapter<Pairing = P>,
    D: IDeliveryProofAdapter<Pairing = P>,
{
    pub fn try_new(
        params: SolidityGenerateConstructorParams<'a, P, E, K, A, D>,
    ) -> SolidityGenerateTryNewReturn<'a, P, E, K, A, D> {
        Ok(SolidityGenerate {
            pairing: params.pairing,
            hash_to_scalar: params.hash_to_scalar,
            encoder: params.encoder,
            random: params.random,
            kem: params.kem,
            key_agreement: params.key_agreement,
            delivery_proof: params.delivery_proof,
            statement_version: params.statement_version,
            settings: params.settings,
            sizes: params.sizes,
        })
    }
}

impl<P, E, K, A, D> IGenerateAdapter for SolidityGenerate<'_, P, E, K, A, D>
where
    P: IPairingArithmetic + IPairingReference,
    E: IEncoderAdapter,
    K: ICredentialKemAdapter<Pairing = P>,
    A: IKeyAgreementAdapter<Pairing = P>,
    D: IDeliveryProofAdapter<Pairing = P>,
{
    type Forms = D::Forms;

    const DECLARATION: GenerateDeclaration = SOLIDITY_GENERATE_DECLARATION;

    fn emit(&self, _params: EmitParams, payload: EmitPayload<'_, Self::Forms>) -> EmitReturn {
        let emitted: Vec<VectorKind> = Self::DECLARATION
            .kinds
            .iter()
            .filter(
                |declaration| match P::DECLARATION.verifier_group_arithmetic {
                    VerifierGroupArithmetic::FirstGroupOnly => {
                        !declaration.requires_second_group_arithmetic
                    }
                    VerifierGroupArithmetic::BothGroups => true,
                },
            )
            .map(|declaration| declaration.kind)
            .collect();

        let mut kind_settings = Vec::with_capacity(emitted.len());
        for kind in &emitted {
            let names = self.settings.kinds.iter().find(|entry| entry.kind == *kind);
            let size = self.sizes.iter().find(|entry| entry.kind == *kind);
            let (Some(names), Some(size)) = (names, size) else {
                return Err(EmitErrorReturn::Solidity(
                    SolidityGenerateErrorReturn::MissingKindSettings { kind: *kind },
                ));
            };
            kind_settings.push((*kind, names, size));
        }

        let size_of = |kind: VectorKind| -> Result<&VectorKindSize, EmitErrorReturn> {
            let Some((_, _, size)) = kind_settings.iter().find(|entry| entry.0 == kind) else {
                return Err(EmitErrorReturn::Solidity(
                    SolidityGenerateErrorReturn::MissingKindSettings { kind },
                ));
            };
            Ok(&size.size)
        };
        let count_of = |kind: VectorKind| -> Result<VectorCount, EmitErrorReturn> {
            match size_of(kind)? {
                VectorKindSize::Drawn { count }
                | VectorKindSize::DrawnTerms { count, .. }
                | VectorKindSize::DrawnMessages { count, .. } => Ok(*count),
                VectorKindSize::Enumerated | VectorKindSize::OverConfiguredInputs => {
                    Err(EmitErrorReturn::Solidity(
                        SolidityGenerateErrorReturn::MissingKindSettings { kind },
                    ))
                }
            }
        };
        let mms_settings_of = |kind: VectorKind| -> Result<MsmVectorSettings, EmitErrorReturn> {
            match size_of(kind)? {
                VectorKindSize::DrawnTerms { count, terms } => Ok(MsmVectorSettings {
                    count: *count,
                    terms: *terms,
                }),
                _ => Err(EmitErrorReturn::Solidity(
                    SolidityGenerateErrorReturn::MissingKindSettings { kind },
                )),
            }
        };
        let drawn_messages_of =
            |kind: VectorKind| -> Result<(VectorCount, usize), EmitErrorReturn> {
                match size_of(kind)? {
                    VectorKindSize::DrawnMessages {
                        count,
                        message_length,
                    } => Ok((*count, *message_length)),
                    _ => Err(EmitErrorReturn::Solidity(
                        SolidityGenerateErrorReturn::MissingKindSettings { kind },
                    )),
                }
            };

        let g1_add_count = count_of(VectorKind::G1Addition)?;
        let g1_mul_count = count_of(VectorKind::G1Multiplication)?;
        let pairing_check_count = count_of(VectorKind::PairingCheck)?;
        let (hash_to_scalar_count, message_length) = drawn_messages_of(VectorKind::HashToScalar)?;
        let possession_count = count_of(VectorKind::Possession)?;
        let challenge_count = count_of(VectorKind::Challenge)?;
        let mint_count = count_of(VectorKind::MintProof)?;
        let transfer_count = count_of(VectorKind::TransferProof)?;
        let second_group: Option<SecondGroupVectorCounts> =
            match P::DECLARATION.verifier_group_arithmetic {
                VerifierGroupArithmetic::FirstGroupOnly => None,
                VerifierGroupArithmetic::BothGroups => Some(SecondGroupVectorCounts {
                    g2_add: count_of(VectorKind::G2Addition)?,
                    g1_msm: mms_settings_of(VectorKind::G1MultiScalarMultiplication)?,
                    g2_msm: mms_settings_of(VectorKind::G2MultiScalarMultiplication)?,
                }),
            };

        let constants = match constants::<P, E, D::Forms>(
            &ConstantsDeps {
                pairing: self.pairing,
                encoder: self.encoder,
            },
            ConstantsParams {
                statement_version: self.statement_version,
            },
            ConstantsPayload {
                kem: &K::DECLARATION,
                key_agreement: &A::DECLARATION,
                delivery_proof: &D::DECLARATION,
            },
        ) {
            Ok(constants) => constants,
            Err(error) => {
                return Err(EmitErrorReturn::Solidity(
                    SolidityGenerateErrorReturn::Constants(error),
                ));
            }
        };
        let entries =
            match SolidityLibraryEntries::try_new(SolidityLibraryEntriesConstructorParams {
                entries: constants.entries,
            }) {
                Ok(entries) => entries,
                Err(error) => {
                    return Err(EmitErrorReturn::Solidity(
                        SolidityGenerateErrorReturn::Entries(error),
                    ));
                }
            };

        let groups = match group_vectors(
            &GroupVectorsDeps {
                pairing: self.pairing,
                random: self.random,
            },
            GroupVectorsParams {
                first_group: FirstGroupVectorCounts {
                    g1_add: g1_add_count,
                    g1_mul: g1_mul_count,
                    pairing_check: pairing_check_count,
                },
                second_group,
            },
            GroupVectorsPayload,
        ) {
            Ok(groups) => groups,
            Err(error) => {
                return Err(EmitErrorReturn::Solidity(
                    SolidityGenerateErrorReturn::GroupVectors(error),
                ));
            }
        };
        let rejections = match rejection_vectors(
            &RejectionVectorsDeps {
                pairing: self.pairing,
            },
            RejectionVectorsParams,
            RejectionVectorsPayload,
        ) {
            Ok(rejections) => rejections,
            Err(error) => {
                return Err(EmitErrorReturn::Solidity(
                    SolidityGenerateErrorReturn::RejectionVectors(error),
                ));
            }
        };
        let mappings = match mapping_vectors(
            &MappingVectorsDeps {
                pairing: self.pairing,
                hash_to_scalar: self.hash_to_scalar,
                kem: self.kem,
                random: self.random,
            },
            MappingVectorsParams {
                hash_to_scalar_count,
                message_length,
            },
            MappingVectorsPayload {
                key_agreement: &A::DECLARATION,
                delivery_proof: &D::DECLARATION,
                identities: payload.identities,
            },
        ) {
            Ok(mappings) => mappings,
            Err(error) => {
                return Err(EmitErrorReturn::Solidity(
                    SolidityGenerateErrorReturn::MappingVectors(error),
                ));
            }
        };
        let possessions = match possession_vectors(
            &PossessionVectorsDeps {
                pairing: self.pairing,
                key_agreement: self.key_agreement,
                random: self.random,
            },
            PossessionVectorsParams {
                count: possession_count,
            },
            PossessionVectorsPayload,
        ) {
            Ok(possessions) => possessions,
            Err(error) => {
                return Err(EmitErrorReturn::Solidity(
                    SolidityGenerateErrorReturn::PossessionVectors(error),
                ));
            }
        };
        let challenges = match challenge_vectors(
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
                transfer: payload.transfer,
            },
        ) {
            Ok(challenges) => challenges,
            Err(error) => {
                return Err(EmitErrorReturn::Solidity(
                    SolidityGenerateErrorReturn::ChallengeVectors(error),
                ));
            }
        };
        let proofs = match proof_vectors(
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
            },
            ProofVectorsPayload {
                mint: payload.mint,
                transfer: payload.transfer,
                identity: payload.proof_identity,
            },
        ) {
            Ok(proofs) => proofs,
            Err(error) => {
                return Err(EmitErrorReturn::Solidity(
                    SolidityGenerateErrorReturn::ProofVectors(error),
                ));
            }
        };

        let Ok(group_operation) = GroupOperationVectorDescription::try_new(
            GroupOperationVectorDescriptionConstructorParams,
        );
        let Ok(pairing_check_description) =
            PairingCheckVectorDescription::try_new(PairingCheckVectorDescriptionConstructorParams);
        let Ok(rejection) =
            RejectionVectorDescription::try_new(RejectionVectorDescriptionConstructorParams);
        let Ok(hash_to_scalar_description) =
            HashToScalarVectorDescription::try_new(HashToScalarVectorDescriptionConstructorParams);
        let Ok(identity_mapping) = IdentityMappingVectorDescription::try_new(
            IdentityMappingVectorDescriptionConstructorParams,
        );
        let Ok(possession) =
            PossessionVectorDescription::try_new(PossessionVectorDescriptionConstructorParams);
        let Ok(challenge) =
            ChallengeVectorDescription::try_new(ChallengeVectorDescriptionConstructorParams);
        let Ok(mint_proof) = MintProofVectorDescription::<D::Forms>::try_new(
            MintProofVectorDescriptionConstructorParams,
        );
        let Ok(transfer_proof) = TransferProofVectorDescription::<D::Forms>::try_new(
            TransferProofVectorDescriptionConstructorParams,
        );

        let record_members = |names: Vec<SolidityMemberName>,
                              fields: &[CanonicalFieldKind],
                              kind: VectorKind|
         -> Result<SolidityRecordMembers, EmitErrorReturn> {
            if names.len() != fields.len() {
                return Err(EmitErrorReturn::Solidity(
                    SolidityGenerateErrorReturn::MemberCount {
                        kind,
                        expected: fields.len(),
                        actual: names.len(),
                    },
                ));
            }
            let mut members = Vec::with_capacity(names.len());
            for (name, field_kind) in names.into_iter().zip(fields.iter()) {
                let Ok(named) = self.encoder.type_name(TypeNameParams, *field_kind);
                let abi_type = match SolidityAbiType::try_new(SolidityAbiTypeConstructorParams {
                    name: named.name.as_str().to_owned(),
                }) {
                    Ok(abi_type) => abi_type,
                    Err(error) => {
                        return Err(EmitErrorReturn::Solidity(
                            SolidityGenerateErrorReturn::AbiType(error),
                        ));
                    }
                };
                members.push(SolidityRecordMember {
                    name,
                    kind: *field_kind,
                    abi_type,
                });
            }
            match SolidityRecordMembers::try_new(SolidityRecordMembersConstructorParams { members })
            {
                Ok(members) => Ok(members),
                Err(error) => Err(EmitErrorReturn::Solidity(
                    SolidityGenerateErrorReturn::RecordMembers(error),
                )),
            }
        };
        let hex_line = |bytes: &[u8]| -> String {
            let mut line = String::with_capacity(3 + bytes.len() * 2);
            line.push_str("0x");
            for byte in bytes {
                line.push_str(&format!("{byte:02x}"));
            }
            line.push('\n');
            line
        };

        let relative_directory = self
            .settings
            .generated_directory
            .segments
            .iter()
            .map(|segment| segment.as_str())
            .collect::<Vec<_>>()
            .join("/");
        let curve_directory = match P::DECLARATION.curve {
            PairingCurve::Bn254 => &self.settings.curve_directories.bn254,
            PairingCurve::Bls12381 => &self.settings.curve_directories.bls12_381,
            _ => &self.settings.curve_directories.bn254,
        };
        let mut directory_path = self.settings.foundry_root.clone();
        for segment in &self.settings.generated_directory.segments {
            directory_path.push(segment.as_str());
        }
        directory_path.push(curve_directory.as_str());

        let mut declarations = Vec::with_capacity(emitted.len());
        let mut writes: Vec<(VectorKind, String, PathBuf)> = Vec::with_capacity(emitted.len());
        for (kind, settings, _size) in &kind_settings {
            let (contents, members) = match kind {
                VectorKind::G1Addition => {
                    let member_names = match group_operation.member_names() {
                        Ok(names) => names,
                        Err(error) => {
                            return Err(EmitErrorReturn::Solidity(
                                SolidityGenerateErrorReturn::GroupMembers(error),
                            ));
                        }
                    };
                    let members = record_members(
                        member_names,
                        <GroupOperationVectorDescription as IEncodingContract>::FIELDS,
                        *kind,
                    )?;
                    let mut contents = String::new();
                    for record in &groups.first_group.g1_add {
                        let encoded = match self.encoder.encode(
                            EncodeParams {
                                description: &group_operation,
                            },
                            record,
                        ) {
                            Ok(encoded) => encoded,
                            Err(error) => {
                                return Err(EmitErrorReturn::Solidity(
                                    SolidityGenerateErrorReturn::Encode(error),
                                ));
                            }
                        };
                        contents.push_str(&hex_line(&encoded.bytes));
                    }
                    (contents, members)
                }
                VectorKind::G1Multiplication => {
                    let member_names = match group_operation.member_names() {
                        Ok(names) => names,
                        Err(error) => {
                            return Err(EmitErrorReturn::Solidity(
                                SolidityGenerateErrorReturn::GroupMembers(error),
                            ));
                        }
                    };
                    let members = record_members(
                        member_names,
                        <GroupOperationVectorDescription as IEncodingContract>::FIELDS,
                        *kind,
                    )?;
                    let mut contents = String::new();
                    for record in &groups.first_group.g1_mul {
                        let encoded = match self.encoder.encode(
                            EncodeParams {
                                description: &group_operation,
                            },
                            record,
                        ) {
                            Ok(encoded) => encoded,
                            Err(error) => {
                                return Err(EmitErrorReturn::Solidity(
                                    SolidityGenerateErrorReturn::Encode(error),
                                ));
                            }
                        };
                        contents.push_str(&hex_line(&encoded.bytes));
                    }
                    (contents, members)
                }
                VectorKind::PairingCheck => {
                    let member_names = match pairing_check_description.member_names() {
                        Ok(names) => names,
                        Err(error) => {
                            return Err(EmitErrorReturn::Solidity(
                                SolidityGenerateErrorReturn::GroupMembers(error),
                            ));
                        }
                    };
                    let members = record_members(
                        member_names,
                        <PairingCheckVectorDescription as IEncodingContract>::FIELDS,
                        *kind,
                    )?;
                    let mut contents = String::new();
                    for record in &groups.first_group.pairing_check {
                        let encoded = match self.encoder.encode(
                            EncodeParams {
                                description: &pairing_check_description,
                            },
                            record,
                        ) {
                            Ok(encoded) => encoded,
                            Err(error) => {
                                return Err(EmitErrorReturn::Solidity(
                                    SolidityGenerateErrorReturn::Encode(error),
                                ));
                            }
                        };
                        contents.push_str(&hex_line(&encoded.bytes));
                    }
                    (contents, members)
                }
                VectorKind::G2Addition => {
                    let Some(second) = &groups.second_group else {
                        continue;
                    };
                    let member_names = match group_operation.member_names() {
                        Ok(names) => names,
                        Err(error) => {
                            return Err(EmitErrorReturn::Solidity(
                                SolidityGenerateErrorReturn::GroupMembers(error),
                            ));
                        }
                    };
                    let members = record_members(
                        member_names,
                        <GroupOperationVectorDescription as IEncodingContract>::FIELDS,
                        *kind,
                    )?;
                    let mut contents = String::new();
                    for record in &second.g2_add {
                        let encoded = match self.encoder.encode(
                            EncodeParams {
                                description: &group_operation,
                            },
                            record,
                        ) {
                            Ok(encoded) => encoded,
                            Err(error) => {
                                return Err(EmitErrorReturn::Solidity(
                                    SolidityGenerateErrorReturn::Encode(error),
                                ));
                            }
                        };
                        contents.push_str(&hex_line(&encoded.bytes));
                    }
                    (contents, members)
                }
                VectorKind::G1MultiScalarMultiplication => {
                    let Some(second) = &groups.second_group else {
                        continue;
                    };
                    let member_names = match group_operation.member_names() {
                        Ok(names) => names,
                        Err(error) => {
                            return Err(EmitErrorReturn::Solidity(
                                SolidityGenerateErrorReturn::GroupMembers(error),
                            ));
                        }
                    };
                    let members = record_members(
                        member_names,
                        <GroupOperationVectorDescription as IEncodingContract>::FIELDS,
                        *kind,
                    )?;
                    let mut contents = String::new();
                    for record in &second.g1_msm {
                        let encoded = match self.encoder.encode(
                            EncodeParams {
                                description: &group_operation,
                            },
                            record,
                        ) {
                            Ok(encoded) => encoded,
                            Err(error) => {
                                return Err(EmitErrorReturn::Solidity(
                                    SolidityGenerateErrorReturn::Encode(error),
                                ));
                            }
                        };
                        contents.push_str(&hex_line(&encoded.bytes));
                    }
                    (contents, members)
                }
                VectorKind::G2MultiScalarMultiplication => {
                    let Some(second) = &groups.second_group else {
                        continue;
                    };
                    let member_names = match group_operation.member_names() {
                        Ok(names) => names,
                        Err(error) => {
                            return Err(EmitErrorReturn::Solidity(
                                SolidityGenerateErrorReturn::GroupMembers(error),
                            ));
                        }
                    };
                    let members = record_members(
                        member_names,
                        <GroupOperationVectorDescription as IEncodingContract>::FIELDS,
                        *kind,
                    )?;
                    let mut contents = String::new();
                    for record in &second.g2_msm {
                        let encoded = match self.encoder.encode(
                            EncodeParams {
                                description: &group_operation,
                            },
                            record,
                        ) {
                            Ok(encoded) => encoded,
                            Err(error) => {
                                return Err(EmitErrorReturn::Solidity(
                                    SolidityGenerateErrorReturn::Encode(error),
                                ));
                            }
                        };
                        contents.push_str(&hex_line(&encoded.bytes));
                    }
                    (contents, members)
                }
                VectorKind::DecodeRejection => {
                    let member_names = match rejection.member_names() {
                        Ok(names) => names,
                        Err(error) => {
                            return Err(EmitErrorReturn::Solidity(
                                SolidityGenerateErrorReturn::RejectionMembers(error),
                            ));
                        }
                    };
                    let members = record_members(
                        member_names,
                        <RejectionVectorDescription as IEncodingContract>::FIELDS,
                        *kind,
                    )?;
                    let mut contents = String::new();
                    for record in &rejections.records {
                        let encoded = match self.encoder.encode(
                            EncodeParams {
                                description: &rejection,
                            },
                            record,
                        ) {
                            Ok(encoded) => encoded,
                            Err(error) => {
                                return Err(EmitErrorReturn::Solidity(
                                    SolidityGenerateErrorReturn::Encode(error),
                                ));
                            }
                        };
                        contents.push_str(&hex_line(&encoded.bytes));
                    }
                    (contents, members)
                }
                VectorKind::HashToScalar => {
                    let member_names = match hash_to_scalar_description.member_names() {
                        Ok(names) => names,
                        Err(error) => {
                            return Err(EmitErrorReturn::Solidity(
                                SolidityGenerateErrorReturn::MappingMembers(error),
                            ));
                        }
                    };
                    let members = record_members(
                        member_names,
                        <HashToScalarVectorDescription as IEncodingContract>::FIELDS,
                        *kind,
                    )?;
                    let mut contents = String::new();
                    for record in &mappings.hash_to_scalar {
                        let encoded = match self.encoder.encode(
                            EncodeParams {
                                description: &hash_to_scalar_description,
                            },
                            record,
                        ) {
                            Ok(encoded) => encoded,
                            Err(error) => {
                                return Err(EmitErrorReturn::Solidity(
                                    SolidityGenerateErrorReturn::Encode(error),
                                ));
                            }
                        };
                        contents.push_str(&hex_line(&encoded.bytes));
                    }
                    (contents, members)
                }
                VectorKind::IdentityMapping => {
                    let member_names = match identity_mapping.member_names() {
                        Ok(names) => names,
                        Err(error) => {
                            return Err(EmitErrorReturn::Solidity(
                                SolidityGenerateErrorReturn::MappingMembers(error),
                            ));
                        }
                    };
                    let members = record_members(
                        member_names,
                        <IdentityMappingVectorDescription as IEncodingContract>::FIELDS,
                        *kind,
                    )?;
                    let mut contents = String::new();
                    for record in &mappings.identity_mapping {
                        let encoded = match self.encoder.encode(
                            EncodeParams {
                                description: &identity_mapping,
                            },
                            record,
                        ) {
                            Ok(encoded) => encoded,
                            Err(error) => {
                                return Err(EmitErrorReturn::Solidity(
                                    SolidityGenerateErrorReturn::Encode(error),
                                ));
                            }
                        };
                        contents.push_str(&hex_line(&encoded.bytes));
                    }
                    (contents, members)
                }
                VectorKind::Possession => {
                    let member_names = match possession.member_names() {
                        Ok(names) => names,
                        Err(error) => {
                            return Err(EmitErrorReturn::Solidity(
                                SolidityGenerateErrorReturn::PossessionMembers(error),
                            ));
                        }
                    };
                    let members = record_members(
                        member_names,
                        <PossessionVectorDescription as IEncodingContract>::FIELDS,
                        *kind,
                    )?;
                    let mut contents = String::new();
                    for record in &possessions.records {
                        let encoded = match self.encoder.encode(
                            EncodeParams {
                                description: &possession,
                            },
                            record,
                        ) {
                            Ok(encoded) => encoded,
                            Err(error) => {
                                return Err(EmitErrorReturn::Solidity(
                                    SolidityGenerateErrorReturn::Encode(error),
                                ));
                            }
                        };
                        contents.push_str(&hex_line(&encoded.bytes));
                    }
                    (contents, members)
                }
                VectorKind::Challenge => {
                    let member_names = match challenge.member_names() {
                        Ok(names) => names,
                        Err(error) => {
                            return Err(EmitErrorReturn::Solidity(
                                SolidityGenerateErrorReturn::ChallengeMembers(error),
                            ));
                        }
                    };
                    let members = record_members(
                        member_names,
                        <ChallengeVectorDescription as IEncodingContract>::FIELDS,
                        *kind,
                    )?;
                    let mut contents = String::new();
                    for record in &challenges.records {
                        let encoded = match self.encoder.encode(
                            EncodeParams {
                                description: &challenge,
                            },
                            record,
                        ) {
                            Ok(encoded) => encoded,
                            Err(error) => {
                                return Err(EmitErrorReturn::Solidity(
                                    SolidityGenerateErrorReturn::Encode(error),
                                ));
                            }
                        };
                        contents.push_str(&hex_line(&encoded.bytes));
                    }
                    (contents, members)
                }
                VectorKind::MintProof => {
                    let member_names = match mint_proof.member_names() {
                        Ok(names) => names,
                        Err(error) => {
                            return Err(EmitErrorReturn::Solidity(
                                SolidityGenerateErrorReturn::ProofMembers(error),
                            ));
                        }
                    };
                    let members = record_members(
                        member_names,
                        <MintProofVectorDescription<D::Forms> as IEncodingContract>::FIELDS,
                        *kind,
                    )?;
                    let mut contents = String::new();
                    for record in &proofs.mint {
                        let encoded = match self.encoder.encode(
                            EncodeParams {
                                description: &mint_proof,
                            },
                            record,
                        ) {
                            Ok(encoded) => encoded,
                            Err(error) => {
                                return Err(EmitErrorReturn::Solidity(
                                    SolidityGenerateErrorReturn::Encode(error),
                                ));
                            }
                        };
                        contents.push_str(&hex_line(&encoded.bytes));
                    }
                    (contents, members)
                }
                VectorKind::TransferProof => {
                    let member_names = match transfer_proof.member_names() {
                        Ok(names) => names,
                        Err(error) => {
                            return Err(EmitErrorReturn::Solidity(
                                SolidityGenerateErrorReturn::ProofMembers(error),
                            ));
                        }
                    };
                    let members = record_members(
                        member_names,
                        <TransferProofVectorDescription<D::Forms> as IEncodingContract>::FIELDS,
                        *kind,
                    )?;
                    let mut contents = String::new();
                    for record in &proofs.transfer {
                        let encoded = match self.encoder.encode(
                            EncodeParams {
                                description: &transfer_proof,
                            },
                            record,
                        ) {
                            Ok(encoded) => encoded,
                            Err(error) => {
                                return Err(EmitErrorReturn::Solidity(
                                    SolidityGenerateErrorReturn::Encode(error),
                                ));
                            }
                        };
                        contents.push_str(&hex_line(&encoded.bytes));
                    }
                    (contents, members)
                }
            };
            let relative_path = format!(
                "{}/{}/{}",
                relative_directory,
                curve_directory.as_str(),
                settings.names.file_name.as_str()
            );
            let path =
                match SolidityStringLiteral::try_new(SolidityStringLiteralConstructorParams {
                    text: relative_path,
                }) {
                    Ok(path) => path,
                    Err(error) => {
                        return Err(EmitErrorReturn::Solidity(
                            SolidityGenerateErrorReturn::Path(error),
                        ));
                    }
                };
            declarations.push(SolidityRecordDeclaration {
                struct_name: settings.names.struct_name.clone(),
                members,
                decode_function_name: settings.names.decode_function_name.clone(),
                path_constant_name: settings.names.path_constant_name.clone(),
                path,
            });
            writes.push((
                *kind,
                contents,
                directory_path.join(settings.names.file_name.as_str()),
            ));
        }

        let declarations =
            match SolidityRecordDeclarations::try_new(SolidityRecordDeclarationsConstructorParams {
                declarations,
            }) {
                Ok(declarations) => declarations,
                Err(error) => {
                    return Err(EmitErrorReturn::Solidity(
                        SolidityGenerateErrorReturn::Declarations(error),
                    ));
                }
            };
        let Ok(constants_library) = render(
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
        let Ok(records_library) = render(
            &RenderDeps,
            RenderParams {
                license: self.settings.license.clone(),
                compiler_version: self.settings.compiler_version,
            },
            RenderPayload {
                library_name: self.settings.records_library.library_name.clone(),
                body: SolidityLibraryBody::Records(declarations),
            },
        );

        if let Err(error) = fs::create_dir_all(&directory_path) {
            return Err(EmitErrorReturn::Solidity(
                SolidityGenerateErrorReturn::CreateDirectory {
                    path: directory_path.clone(),
                    error,
                },
            ));
        }
        let mut files = Vec::with_capacity(writes.len() + 2);
        for (kind, contents, file_path) in writes {
            if let Err(error) = fs::write(&file_path, contents) {
                return Err(EmitErrorReturn::Solidity(
                    SolidityGenerateErrorReturn::WriteFile {
                        path: file_path,
                        error,
                    },
                ));
            }
            files.push(EmittedFile {
                kind: Some(kind),
                path: file_path,
            });
        }
        let constants_path =
            directory_path.join(self.settings.constants_library.file_name.as_str());
        if let Err(error) = fs::write(&constants_path, constants_library.source.as_str()) {
            return Err(EmitErrorReturn::Solidity(
                SolidityGenerateErrorReturn::WriteFile {
                    path: constants_path,
                    error,
                },
            ));
        }
        files.push(EmittedFile {
            kind: None,
            path: constants_path,
        });
        let records_path = directory_path.join(self.settings.records_library.file_name.as_str());
        if let Err(error) = fs::write(&records_path, records_library.source.as_str()) {
            return Err(EmitErrorReturn::Solidity(
                SolidityGenerateErrorReturn::WriteFile {
                    path: records_path,
                    error,
                },
            ));
        }
        files.push(EmittedFile {
            kind: None,
            path: records_path,
        });
        Ok(EmitSuccessReturn { files })
    }
}
