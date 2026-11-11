#[cfg(test)]
mod integration_test;
mod interface;
#[cfg(any(test, feature = "mocks"))]
mod mock;
pub(crate) mod provides;
#[cfg(test)]
mod test;

use crate::factory::provides::{
    AuthenticationPath, CHAINING_VALUE_LENGTH, COMMITMENT_INTERFACE_VERSION, ChallengeParams,
    ChallengePayload, ChallengeReturn, ChallengeSuccessReturn, Chunk, CommitErrorReturn,
    CommitParams, CommitPayload, CommitReturn, CommitSuccessReturn, CommitmentDeclaration,
    CommitmentKey, CommitmentSchemeIdentifier, ExtractPathErrorReturn, ExtractPathParams,
    ExtractPathPayload, ExtractPathReturn, ExtractPathSuccessReturn, ICommitmentAdapter,
    OUTBOARD_HEADER_LENGTH, OUTBOARD_PARENT_LENGTH, Outboard, RespondErrorReturn, RespondParams,
    RespondPayload, RespondReturn, RespondSuccessReturn, Root, RootConstructorParams,
    VerifyChunkErrorReturn, VerifyChunkParams, VerifyChunkPayload, VerifyChunkReturn,
    VerifyChunkSuccessReturn, VerifyErrorReturn, VerifyParams, VerifyPayload, VerifyReturn,
    VerifySuccessReturn,
};
use bao::encode::outboard;
use blake3::Hasher;
use blake3::hazmat::{
    ChainingValue, HasherExt, Mode, left_subtree_len, merge_subtrees_non_root, merge_subtrees_root,
};
use domain::Secret;
use interface::{
    BLAKE3_BAO_CHUNK_LENGTH, BLAKE3_BAO_KEY_LENGTH, Blake3BaoCommitment,
    Blake3BaoCommitmentConstructorParams, Blake3BaoCommitmentErrorReturn,
    Blake3BaoCommitmentTryNewReturn,
};
use zeroize::Zeroize;

impl Blake3BaoCommitment {
    pub const DECLARATION: CommitmentDeclaration = CommitmentDeclaration {
        identifier: CommitmentSchemeIdentifier::Blake3BaoV1,
        chunk_length: BLAKE3_BAO_CHUNK_LENGTH,
        key_length: BLAKE3_BAO_KEY_LENGTH,
        adapter_version: 1,
        interface_version: COMMITMENT_INTERFACE_VERSION,
    };

    pub fn try_new(
        _params: Blake3BaoCommitmentConstructorParams,
    ) -> Blake3BaoCommitmentTryNewReturn {
        Ok(Blake3BaoCommitment)
    }

    fn content_len_as_u64(length: usize) -> Result<u64, Blake3BaoCommitmentErrorReturn> {
        u64::try_from(length)
            .map_err(|_| Blake3BaoCommitmentErrorReturn::ContentLengthUnrepresentable { length })
    }

    fn offset_as_usize(offset: u64) -> Result<usize, Blake3BaoCommitmentErrorReturn> {
        usize::try_from(offset)
            .map_err(|_| Blake3BaoCommitmentErrorReturn::OffsetUnrepresentable { offset })
    }

    fn chunk_count(content_length: u64) -> u64 {
        if content_length == 0 {
            1
        } else {
            content_length.div_ceil(BLAKE3_BAO_CHUNK_LENGTH)
        }
    }

    fn expected_chunk_length(content_length: u64, chunk_index: u64) -> u64 {
        BLAKE3_BAO_CHUNK_LENGTH.min(content_length - chunk_index * BLAKE3_BAO_CHUNK_LENGTH)
    }

    fn expected_outboard_length(
        content_length: u64,
    ) -> Result<u64, Blake3BaoCommitmentErrorReturn> {
        let header = Self::content_len_as_u64(OUTBOARD_HEADER_LENGTH)?;
        let parent = Self::content_len_as_u64(OUTBOARD_PARENT_LENGTH)?;
        Ok(header + parent * (Self::chunk_count(content_length) - 1))
    }

    fn key_array(secret: &Secret<Vec<u8>>) -> Result<[u8; 32], usize> {
        <[u8; 32]>::try_from(secret.expose().as_slice()).map_err(|_| secret.expose().len())
    }

    fn admit_key(key: &CommitmentKey<'_>) -> Result<Option<[u8; 32]>, usize> {
        match key {
            CommitmentKey::Unkeyed => Ok(None),
            CommitmentKey::Keyed(secret) => Self::key_array(secret).map(Some),
        }
    }

    fn mode<'a>(key: Option<&'a [u8; 32]>) -> Mode<'a> {
        match key {
            Some(key) => Mode::KeyedHash(key),
            None => Mode::Hash,
        }
    }

    fn hasher(key: Option<&[u8; 32]>) -> Hasher {
        match key {
            Some(key) => Hasher::new_keyed(key),
            None => Hasher::new(),
        }
    }

    fn leaf_cv(bytes: &[u8], offset: u64, key: Option<&[u8; 32]>) -> ChainingValue {
        let mut hasher = Self::hasher(key);
        hasher.set_input_offset(offset);
        hasher.update(bytes);
        let cv = hasher.finalize_non_root();
        hasher.zeroize();
        cv
    }

    fn root_leaf(content: &[u8], key: Option<&[u8; 32]>) -> [u8; 32] {
        let mut hasher = Self::hasher(key);
        hasher.update(content);
        let hash = hasher.finalize();
        let root = *hash.as_bytes();
        hasher.zeroize();
        root
    }

    fn write_subtree(
        content: &[u8],
        offset: u64,
        length: u64,
        key: Option<&[u8; 32]>,
        outboard: &mut Vec<u8>,
    ) -> Result<ChainingValue, Blake3BaoCommitmentErrorReturn> {
        if length <= BLAKE3_BAO_CHUNK_LENGTH {
            let start = Self::offset_as_usize(offset)?;
            let end = start + Self::offset_as_usize(length)?;
            return Ok(Self::leaf_cv(&content[start..end], offset, key));
        }
        let position = outboard.len();
        outboard.resize(position + OUTBOARD_PARENT_LENGTH, 0);
        let left_length = left_subtree_len(length);
        let left = Self::write_subtree(content, offset, left_length, key, outboard)?;
        let right = Self::write_subtree(
            content,
            offset + left_length,
            length - left_length,
            key,
            outboard,
        )?;
        outboard[position..position + CHAINING_VALUE_LENGTH].copy_from_slice(&left);
        outboard[position + CHAINING_VALUE_LENGTH..position + OUTBOARD_PARENT_LENGTH]
            .copy_from_slice(&right);
        Ok(merge_subtrees_non_root(&left, &right, Self::mode(key)))
    }

    fn read_cv(outboard: &[u8], cursor: usize) -> [u8; 32] {
        let mut cv = [0u8; CHAINING_VALUE_LENGTH];
        cv.copy_from_slice(&outboard[cursor..cursor + CHAINING_VALUE_LENGTH]);
        cv
    }

    fn declared_length(outboard: &[u8]) -> u64 {
        let mut header = [0u8; OUTBOARD_HEADER_LENGTH];
        header.copy_from_slice(&outboard[..OUTBOARD_HEADER_LENGTH]);
        u64::from_le_bytes(header)
    }

    fn descend(
        outboard: &[u8],
        content_length: u64,
        chunk_index: u64,
    ) -> Result<Vec<[u8; 32]>, Blake3BaoCommitmentErrorReturn> {
        let mut siblings = Vec::new();
        let mut offset = 0u64;
        let mut length = content_length;
        let mut cursor = OUTBOARD_HEADER_LENGTH;
        while length > BLAKE3_BAO_CHUNK_LENGTH {
            let left = Self::read_cv(outboard, cursor);
            let right = Self::read_cv(outboard, cursor + CHAINING_VALUE_LENGTH);
            let left_length = left_subtree_len(length);
            if chunk_index * BLAKE3_BAO_CHUNK_LENGTH < offset + left_length {
                siblings.push(right);
                cursor += OUTBOARD_PARENT_LENGTH;
                length = left_length;
            } else {
                siblings.push(left);
                let left_count = Self::offset_as_usize(Self::chunk_count(left_length))?;
                cursor += OUTBOARD_PARENT_LENGTH * left_count;
                offset += left_length;
                length -= left_length;
            }
        }
        Ok(siblings)
    }

    fn check_subtree(
        offset: u64,
        length: u64,
        recorded: &[u8; 32],
        cursor: usize,
        outboard: &[u8],
        key: Option<&[u8; 32]>,
        leaves: &mut Vec<(u64, u64, [u8; 32])>,
    ) -> Result<(), VerifyErrorReturn> {
        if length <= BLAKE3_BAO_CHUNK_LENGTH {
            leaves.push((offset / BLAKE3_BAO_CHUNK_LENGTH, offset, *recorded));
            return Ok(());
        }
        let left = Self::read_cv(outboard, cursor);
        let right = Self::read_cv(outboard, cursor + CHAINING_VALUE_LENGTH);
        if merge_subtrees_non_root(&left, &right, Self::mode(key)) != *recorded {
            return Err(VerifyErrorReturn::ParentMismatch { offset, length });
        }
        let left_length = left_subtree_len(length);
        let left_count = Self::offset_as_usize(Self::chunk_count(left_length))
            .map_err(VerifyErrorReturn::Blake3Bao)?;
        Self::check_subtree(
            offset,
            left_length,
            &left,
            cursor + OUTBOARD_PARENT_LENGTH,
            outboard,
            key,
            leaves,
        )?;
        Self::check_subtree(
            offset + left_length,
            length - left_length,
            &right,
            cursor + OUTBOARD_PARENT_LENGTH * left_count,
            outboard,
            key,
            leaves,
        )?;
        Ok(())
    }

    fn chunk_route(content_length: u64, chunk_index: u64) -> (u64, Vec<bool>) {
        let mut sides = Vec::new();
        let mut offset = 0u64;
        let mut length = content_length;
        while length > BLAKE3_BAO_CHUNK_LENGTH {
            let left_length = left_subtree_len(length);
            if chunk_index * BLAKE3_BAO_CHUNK_LENGTH < offset + left_length {
                sides.push(true);
                length = left_length;
            } else {
                sides.push(false);
                offset += left_length;
                length -= left_length;
            }
        }
        (offset, sides)
    }

    fn commit_keyed(content: &[u8], key: &[u8; 32]) -> CommitReturn {
        let length = match Self::content_len_as_u64(content.len()) {
            Ok(length) => length,
            Err(error) => return Err(CommitErrorReturn::Blake3Bao(error)),
        };
        let mut bytes = length.to_le_bytes().to_vec();
        let root = if length <= BLAKE3_BAO_CHUNK_LENGTH {
            Self::root_leaf(content, Some(key))
        } else {
            let position = bytes.len();
            bytes.resize(position + OUTBOARD_PARENT_LENGTH, 0);
            let left_length = left_subtree_len(length);
            let left = match Self::write_subtree(content, 0, left_length, Some(key), &mut bytes) {
                Ok(left) => left,
                Err(error) => return Err(CommitErrorReturn::Blake3Bao(error)),
            };
            let right = match Self::write_subtree(
                content,
                left_length,
                length - left_length,
                Some(key),
                &mut bytes,
            ) {
                Ok(right) => right,
                Err(error) => return Err(CommitErrorReturn::Blake3Bao(error)),
            };
            bytes[position..position + CHAINING_VALUE_LENGTH].copy_from_slice(&left);
            bytes[position + CHAINING_VALUE_LENGTH..position + OUTBOARD_PARENT_LENGTH]
                .copy_from_slice(&right);
            *merge_subtrees_root(&left, &right, Mode::KeyedHash(key)).as_bytes()
        };
        match Root::try_new(RootConstructorParams { bytes: root }) {
            Ok(root) => Ok(CommitSuccessReturn {
                root,
                outboard: Outboard { bytes },
            }),
            Err(error) => Err(CommitErrorReturn::Root(error)),
        }
    }

    fn verify_admitted(payload: &VerifyPayload<'_>, key: Option<&[u8; 32]>) -> VerifyReturn {
        let outboard = payload.outboard.bytes.as_slice();
        if outboard.len() < OUTBOARD_HEADER_LENGTH {
            return Err(VerifyErrorReturn::OutboardTruncated {
                length: outboard.len(),
            });
        }
        let declared = Self::declared_length(outboard);
        let actual = Self::content_len_as_u64(payload.content.len())
            .map_err(VerifyErrorReturn::Blake3Bao)?;
        if declared != actual {
            return Err(VerifyErrorReturn::ContentLengthMismatch { declared, actual });
        }
        let expected =
            Self::expected_outboard_length(declared).map_err(VerifyErrorReturn::Blake3Bao)?;
        let actual =
            Self::content_len_as_u64(outboard.len()).map_err(VerifyErrorReturn::Blake3Bao)?;
        if actual != expected {
            return Err(VerifyErrorReturn::OutboardLengthMismatch { expected, actual });
        }
        if Self::chunk_count(declared) == 1 {
            if &Self::root_leaf(payload.content, key) != payload.root.as_bytes() {
                return Err(VerifyErrorReturn::RootMismatch);
            }
            return Ok(VerifySuccessReturn);
        }
        let left = Self::read_cv(outboard, OUTBOARD_HEADER_LENGTH);
        let right = Self::read_cv(outboard, OUTBOARD_HEADER_LENGTH + CHAINING_VALUE_LENGTH);
        if merge_subtrees_root(&left, &right, Self::mode(key)).as_bytes() != payload.root.as_bytes()
        {
            return Err(VerifyErrorReturn::RootMismatch);
        }
        let left_length = left_subtree_len(declared);
        let left_count = Self::offset_as_usize(Self::chunk_count(left_length))
            .map_err(VerifyErrorReturn::Blake3Bao)?;
        let mut leaves = Vec::new();
        Self::check_subtree(
            0,
            left_length,
            &left,
            OUTBOARD_HEADER_LENGTH + OUTBOARD_PARENT_LENGTH,
            outboard,
            key,
            &mut leaves,
        )?;
        Self::check_subtree(
            left_length,
            declared - left_length,
            &right,
            OUTBOARD_HEADER_LENGTH + OUTBOARD_PARENT_LENGTH * left_count,
            outboard,
            key,
            &mut leaves,
        )?;
        for (chunk_index, offset, recorded) in leaves {
            let start = Self::offset_as_usize(offset).map_err(VerifyErrorReturn::Blake3Bao)?;
            let length = Self::offset_as_usize(Self::expected_chunk_length(declared, chunk_index))
                .map_err(VerifyErrorReturn::Blake3Bao)?;
            if Self::leaf_cv(&payload.content[start..start + length], offset, key) != recorded {
                return Err(VerifyErrorReturn::ChunkMismatch { chunk_index });
            }
        }
        Ok(VerifySuccessReturn)
    }

    fn verify_chunk_admitted(
        payload: &VerifyChunkPayload<'_>,
        key: Option<&[u8; 32]>,
    ) -> VerifyChunkReturn {
        let chunk_count = Self::chunk_count(payload.content_length);
        if payload.chunk_index >= chunk_count {
            return Err(VerifyChunkErrorReturn::ChunkIndexOutOfRange {
                chunk_index: payload.chunk_index,
                chunk_count,
            });
        }
        let expected = Self::expected_chunk_length(payload.content_length, payload.chunk_index);
        let actual = Self::content_len_as_u64(payload.chunk.bytes.len())
            .map_err(VerifyChunkErrorReturn::Blake3Bao)?;
        if actual != expected {
            return Err(VerifyChunkErrorReturn::ChunkLengthMismatch { expected, actual });
        }
        let (offset, sides) = Self::chunk_route(payload.content_length, payload.chunk_index);
        if payload.path.siblings.len() != sides.len() {
            return Err(VerifyChunkErrorReturn::PathLengthMismatch {
                expected: sides.len(),
                actual: payload.path.siblings.len(),
            });
        }
        let mut value = if sides.is_empty() {
            Self::root_leaf(&payload.chunk.bytes, key)
        } else {
            Self::leaf_cv(&payload.chunk.bytes, offset, key)
        };
        for depth in (0..sides.len()).rev() {
            let sibling = &payload.path.siblings[depth];
            let (left, right) = if sides[depth] {
                (value, *sibling)
            } else {
                (*sibling, value)
            };
            value = if depth == 0 {
                *merge_subtrees_root(&left, &right, Self::mode(key)).as_bytes()
            } else {
                merge_subtrees_non_root(&left, &right, Self::mode(key))
            };
        }
        if &value != payload.root.as_bytes() {
            return Err(VerifyChunkErrorReturn::RootMismatch);
        }
        Ok(VerifyChunkSuccessReturn)
    }
}

impl ICommitmentAdapter for Blake3BaoCommitment {
    fn declaration(&self) -> CommitmentDeclaration {
        Self::DECLARATION
    }

    fn commit(&self, _params: CommitParams, payload: CommitPayload<'_>) -> CommitReturn {
        let mut key = match Self::admit_key(&payload.key) {
            Ok(key) => key,
            Err(actual) => {
                return Err(CommitErrorReturn::KeyLengthMismatch {
                    expected: BLAKE3_BAO_KEY_LENGTH,
                    actual,
                });
            }
        };
        let result = match &key {
            None => {
                let (bytes, hash) = outboard(payload.content);
                match Root::try_new(RootConstructorParams {
                    bytes: *hash.as_bytes(),
                }) {
                    Ok(root) => Ok(CommitSuccessReturn {
                        root,
                        outboard: Outboard { bytes },
                    }),
                    Err(error) => Err(CommitErrorReturn::Root(error)),
                }
            }
            Some(key) => Self::commit_keyed(payload.content, key),
        };
        key.zeroize();
        result
    }

    fn verify(&self, _params: VerifyParams, payload: VerifyPayload<'_>) -> VerifyReturn {
        let mut key = match Self::admit_key(&payload.key) {
            Ok(key) => key,
            Err(actual) => {
                return Err(VerifyErrorReturn::KeyLengthMismatch {
                    expected: BLAKE3_BAO_KEY_LENGTH,
                    actual,
                });
            }
        };
        let result = Self::verify_admitted(&payload, key.as_ref());
        key.zeroize();
        result
    }

    fn extract_path(
        &self,
        _params: ExtractPathParams,
        payload: ExtractPathPayload<'_>,
    ) -> ExtractPathReturn {
        let outboard = payload.outboard.bytes.as_slice();
        if outboard.len() < OUTBOARD_HEADER_LENGTH {
            return Err(ExtractPathErrorReturn::OutboardTruncated {
                length: outboard.len(),
            });
        }
        let declared = Self::declared_length(outboard);
        let expected =
            Self::expected_outboard_length(declared).map_err(ExtractPathErrorReturn::Blake3Bao)?;
        let actual =
            Self::content_len_as_u64(outboard.len()).map_err(ExtractPathErrorReturn::Blake3Bao)?;
        if actual != expected {
            return Err(ExtractPathErrorReturn::OutboardLengthMismatch { expected, actual });
        }
        let chunk_count = Self::chunk_count(declared);
        if payload.chunk_index >= chunk_count {
            return Err(ExtractPathErrorReturn::ChunkIndexOutOfRange {
                chunk_index: payload.chunk_index,
                chunk_count,
            });
        }
        let siblings = Self::descend(outboard, declared, payload.chunk_index)
            .map_err(ExtractPathErrorReturn::Blake3Bao)?;
        Ok(ExtractPathSuccessReturn {
            path: AuthenticationPath { siblings },
        })
    }

    fn verify_chunk(
        &self,
        _params: VerifyChunkParams,
        payload: VerifyChunkPayload<'_>,
    ) -> VerifyChunkReturn {
        let mut key = match Self::admit_key(&payload.key) {
            Ok(key) => key,
            Err(actual) => {
                return Err(VerifyChunkErrorReturn::KeyLengthMismatch {
                    expected: BLAKE3_BAO_KEY_LENGTH,
                    actual,
                });
            }
        };
        let result = Self::verify_chunk_admitted(&payload, key.as_ref());
        key.zeroize();
        result
    }

    fn challenge(&self, _params: ChallengeParams, payload: ChallengePayload) -> ChallengeReturn {
        Ok(ChallengeSuccessReturn {
            chunk_index: u64::from_le_bytes(payload.uniform)
                % Self::chunk_count(payload.content_length),
        })
    }

    fn respond(&self, _params: RespondParams, payload: RespondPayload<'_>) -> RespondReturn {
        let outboard = payload.outboard.bytes.as_slice();
        if outboard.len() < OUTBOARD_HEADER_LENGTH {
            return Err(RespondErrorReturn::OutboardTruncated {
                length: outboard.len(),
            });
        }
        let declared = Self::declared_length(outboard);
        let actual = Self::content_len_as_u64(payload.content.len())
            .map_err(RespondErrorReturn::Blake3Bao)?;
        if declared != actual {
            return Err(RespondErrorReturn::ContentLengthMismatch { declared, actual });
        }
        let expected =
            Self::expected_outboard_length(declared).map_err(RespondErrorReturn::Blake3Bao)?;
        let actual =
            Self::content_len_as_u64(outboard.len()).map_err(RespondErrorReturn::Blake3Bao)?;
        if actual != expected {
            return Err(RespondErrorReturn::OutboardLengthMismatch { expected, actual });
        }
        let chunk_count = Self::chunk_count(declared);
        if payload.chunk_index >= chunk_count {
            return Err(RespondErrorReturn::ChunkIndexOutOfRange {
                chunk_index: payload.chunk_index,
                chunk_count,
            });
        }
        let offset = payload.chunk_index * BLAKE3_BAO_CHUNK_LENGTH;
        let length = Self::expected_chunk_length(declared, payload.chunk_index);
        let start = Self::offset_as_usize(offset).map_err(RespondErrorReturn::Blake3Bao)?;
        let end = start + Self::offset_as_usize(length).map_err(RespondErrorReturn::Blake3Bao)?;
        let siblings = Self::descend(outboard, declared, payload.chunk_index)
            .map_err(RespondErrorReturn::Blake3Bao)?;
        Ok(RespondSuccessReturn {
            chunk: Chunk {
                bytes: payload.content[start..end].to_vec(),
            },
            path: AuthenticationPath { siblings },
        })
    }
}
