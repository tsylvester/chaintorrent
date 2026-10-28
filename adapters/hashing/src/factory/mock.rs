#![allow(dead_code, clippy::expect_used, clippy::too_many_arguments)]

use super::interface::{
    AuthenticationPath, CHAINING_VALUE_LENGTH, COMMITMENT_INTERFACE_VERSION, ChallengeParams,
    ChallengePayload, ChallengeReturn, ChallengeSuccessReturn, Chunk, CommitErrorReturn,
    CommitParams, CommitPayload, CommitReturn, CommitSuccessReturn, CommitmentConcrete,
    CommitmentDeclaration, CommitmentKey, CommitmentSchemeIdentifier, CreateCommitmentDeps,
    CreateCommitmentParams, CreateCommitmentPayload, CreateCommitmentReturn,
    CreateCommitmentSuccessReturn, ExtractPathErrorReturn, ExtractPathParams, ExtractPathPayload,
    ExtractPathReturn, ExtractPathSuccessReturn, ICommitmentAdapter,
    MockICommitmentAdapterFailureMode, OUTBOARD_HEADER_LENGTH, OUTBOARD_PARENT_LENGTH, Outboard,
    RespondErrorReturn, RespondParams, RespondPayload, RespondReturn, RespondSuccessReturn, Root,
    RootConstructorParams, VerifyChunkErrorReturn, VerifyChunkParams, VerifyChunkPayload,
    VerifyChunkReturn, VerifyChunkSuccessReturn, VerifyErrorReturn, VerifyParams, VerifyPayload,
    VerifyReturn, VerifySuccessReturn,
};
use core::convert::Infallible;
use domain::Secret;
use serde_json::{Map, Value};
use std::collections::hash_map::DefaultHasher;
use std::hash::Hasher;

pub fn reference_content(length: usize) -> Vec<u8> {
    let mut content = Vec::with_capacity(length + 4);
    let mut counter = 1u32;
    while content.len() < length {
        content.extend_from_slice(&counter.to_le_bytes());
        counter += 1;
    }
    content.truncate(length);
    content
}

pub const REFERENCE_CONTENT_LENGTH: usize = 3073;

pub fn reference_key_bytes() -> Vec<u8> {
    (1u8..=32).collect()
}

#[derive(Default)]
pub struct CommitmentDeclarationOverrides {
    pub identifier: Option<CommitmentSchemeIdentifier>,
    pub chunk_length: Option<u64>,
    pub key_length: Option<usize>,
    pub adapter_version: Option<u32>,
    pub interface_version: Option<u32>,
}

pub fn build_commitment_declaration(
    overrides: CommitmentDeclarationOverrides,
) -> CommitmentDeclaration {
    CommitmentDeclaration {
        identifier: overrides
            .identifier
            .unwrap_or(MockICommitmentAdapter::DECLARATION.identifier),
        chunk_length: overrides
            .chunk_length
            .unwrap_or(MockICommitmentAdapter::DECLARATION.chunk_length),
        key_length: overrides
            .key_length
            .unwrap_or(MockICommitmentAdapter::DECLARATION.key_length),
        adapter_version: overrides
            .adapter_version
            .unwrap_or(MockICommitmentAdapter::DECLARATION.adapter_version),
        interface_version: overrides
            .interface_version
            .unwrap_or(MockICommitmentAdapter::DECLARATION.interface_version),
    }
}

#[derive(Default)]
pub struct CommitmentDeclarationCorruptions {
    pub identifier: Option<Value>,
    pub chunk_length: Option<Value>,
    pub key_length: Option<Value>,
    pub adapter_version: Option<Value>,
    pub interface_version: Option<Value>,
}

pub fn invalidate_commitment_declaration(corruptions: CommitmentDeclarationCorruptions) -> Value {
    let declaration = build_commitment_declaration(Default::default());
    let mut fields = Map::new();
    fields.insert(
        "identifier".to_string(),
        Value::from(match declaration.identifier {
            CommitmentSchemeIdentifier::Blake3BaoV1 => "Blake3BaoV1",
            CommitmentSchemeIdentifier::MockV1 => "MockV1",
        }),
    );
    fields.insert(
        "chunk_length".to_string(),
        Value::from(declaration.chunk_length),
    );
    fields.insert(
        "key_length".to_string(),
        Value::from(declaration.key_length),
    );
    fields.insert(
        "adapter_version".to_string(),
        Value::from(declaration.adapter_version),
    );
    fields.insert(
        "interface_version".to_string(),
        Value::from(declaration.interface_version),
    );
    if let Some(identifier) = corruptions.identifier {
        fields.insert("identifier".to_string(), identifier);
    }
    if let Some(chunk_length) = corruptions.chunk_length {
        fields.insert("chunk_length".to_string(), chunk_length);
    }
    if let Some(key_length) = corruptions.key_length {
        fields.insert("key_length".to_string(), key_length);
    }
    if let Some(adapter_version) = corruptions.adapter_version {
        fields.insert("adapter_version".to_string(), adapter_version);
    }
    if let Some(interface_version) = corruptions.interface_version {
        fields.insert("interface_version".to_string(), interface_version);
    }
    Value::Object(fields)
}

#[derive(Default)]
pub struct RootConstructorParamsOverrides {
    pub bytes: Option<[u8; 32]>,
}

pub fn build_root_constructor_params(
    overrides: RootConstructorParamsOverrides,
) -> RootConstructorParams {
    RootConstructorParams {
        bytes: overrides.bytes.unwrap_or([1u8; 32]),
    }
}

#[derive(Default)]
pub struct RootConstructorParamsCorruptions {
    pub bytes: Option<Value>,
}

pub fn invalidate_root_constructor_params(corruptions: RootConstructorParamsCorruptions) -> Value {
    let params = build_root_constructor_params(Default::default());
    let mut fields = Map::new();
    fields.insert(
        "bytes".to_string(),
        Value::Array(params.bytes.iter().map(|byte| Value::from(*byte)).collect()),
    );
    if let Some(bytes) = corruptions.bytes {
        fields.insert("bytes".to_string(), bytes);
    }
    Value::Object(fields)
}

pub fn build_root(overrides: RootConstructorParamsOverrides) -> Root {
    Root::try_new(build_root_constructor_params(overrides))
        .expect("the built bytes are not all zero")
}

#[derive(Default)]
pub struct OutboardOverrides {
    pub bytes: Option<Vec<u8>>,
}

pub fn build_outboard(overrides: OutboardOverrides) -> Outboard {
    Outboard {
        bytes: overrides
            .bytes
            .unwrap_or_else(|| vec![0; OUTBOARD_HEADER_LENGTH]),
    }
}

#[derive(Default)]
pub struct OutboardCorruptions {
    pub bytes: Option<Value>,
}

pub fn invalidate_outboard(corruptions: OutboardCorruptions) -> Value {
    let outboard = build_outboard(Default::default());
    let mut fields = Map::new();
    fields.insert(
        "bytes".to_string(),
        Value::Array(
            outboard
                .bytes
                .iter()
                .map(|byte| Value::from(*byte))
                .collect(),
        ),
    );
    if let Some(bytes) = corruptions.bytes {
        fields.insert("bytes".to_string(), bytes);
    }
    Value::Object(fields)
}

#[derive(Default)]
pub struct AuthenticationPathOverrides {
    pub siblings: Option<Vec<[u8; 32]>>,
}

pub fn build_authentication_path(overrides: AuthenticationPathOverrides) -> AuthenticationPath {
    AuthenticationPath {
        siblings: overrides.siblings.unwrap_or_default(),
    }
}

#[derive(Default)]
pub struct AuthenticationPathCorruptions {
    pub siblings: Option<Value>,
}

pub fn invalidate_authentication_path(corruptions: AuthenticationPathCorruptions) -> Value {
    let path = build_authentication_path(Default::default());
    let mut fields = Map::new();
    fields.insert(
        "siblings".to_string(),
        Value::Array(
            path.siblings
                .iter()
                .map(|sibling| {
                    Value::Array(sibling.iter().map(|byte| Value::from(*byte)).collect())
                })
                .collect(),
        ),
    );
    if let Some(siblings) = corruptions.siblings {
        fields.insert("siblings".to_string(), siblings);
    }
    Value::Object(fields)
}

#[derive(Default)]
pub struct ChunkOverrides {
    pub bytes: Option<Vec<u8>>,
}

pub fn build_chunk(overrides: ChunkOverrides) -> Chunk {
    Chunk {
        bytes: overrides.bytes.unwrap_or_default(),
    }
}

#[derive(Default)]
pub struct ChunkCorruptions {
    pub bytes: Option<Value>,
}

pub fn invalidate_chunk(corruptions: ChunkCorruptions) -> Value {
    let chunk = build_chunk(Default::default());
    let mut fields = Map::new();
    fields.insert(
        "bytes".to_string(),
        Value::Array(chunk.bytes.iter().map(|byte| Value::from(*byte)).collect()),
    );
    if let Some(bytes) = corruptions.bytes {
        fields.insert("bytes".to_string(), bytes);
    }
    Value::Object(fields)
}

#[derive(Default)]
pub struct CommitParamsOverrides;

pub fn build_commit_params(_overrides: CommitParamsOverrides) -> CommitParams {
    CommitParams
}

#[derive(Default)]
pub struct VerifyParamsOverrides;

pub fn build_verify_params(_overrides: VerifyParamsOverrides) -> VerifyParams {
    VerifyParams
}

#[derive(Default)]
pub struct ExtractPathParamsOverrides;

pub fn build_extract_path_params(_overrides: ExtractPathParamsOverrides) -> ExtractPathParams {
    ExtractPathParams
}

#[derive(Default)]
pub struct VerifyChunkParamsOverrides;

pub fn build_verify_chunk_params(_overrides: VerifyChunkParamsOverrides) -> VerifyChunkParams {
    VerifyChunkParams
}

#[derive(Default)]
pub struct ChallengeParamsOverrides;

pub fn build_challenge_params(_overrides: ChallengeParamsOverrides) -> ChallengeParams {
    ChallengeParams
}

#[derive(Default)]
pub struct RespondParamsOverrides;

pub fn build_respond_params(_overrides: RespondParamsOverrides) -> RespondParams {
    RespondParams
}

#[derive(Default)]
pub struct CommitPayloadOverrides;

pub fn build_commit_payload<'a>(
    content: &'a [u8],
    key: CommitmentKey<'a>,
    _overrides: CommitPayloadOverrides,
) -> CommitPayload<'a> {
    CommitPayload { content, key }
}

#[derive(Default)]
pub struct VerifyPayloadOverrides;

pub fn build_verify_payload<'a>(
    content: &'a [u8],
    outboard: &'a Outboard,
    root: &'a Root,
    key: CommitmentKey<'a>,
    _overrides: VerifyPayloadOverrides,
) -> VerifyPayload<'a> {
    VerifyPayload {
        content,
        outboard,
        root,
        key,
    }
}

#[derive(Default)]
pub struct ExtractPathPayloadOverrides {
    pub chunk_index: Option<u64>,
}

pub fn build_extract_path_payload<'a>(
    outboard: &'a Outboard,
    overrides: ExtractPathPayloadOverrides,
) -> ExtractPathPayload<'a> {
    ExtractPathPayload {
        outboard,
        chunk_index: overrides.chunk_index.unwrap_or(0),
    }
}

#[derive(Default)]
pub struct VerifyChunkPayloadOverrides {
    pub content_length: Option<u64>,
    pub chunk_index: Option<u64>,
}

pub fn build_verify_chunk_payload<'a>(
    root: &'a Root,
    chunk: &'a Chunk,
    path: &'a AuthenticationPath,
    key: CommitmentKey<'a>,
    overrides: VerifyChunkPayloadOverrides,
) -> VerifyChunkPayload<'a> {
    VerifyChunkPayload {
        root,
        content_length: overrides.content_length.unwrap_or_else(|| {
            u64::try_from(REFERENCE_CONTENT_LENGTH)
                .expect("the reference content length fits a u64")
        }),
        chunk_index: overrides.chunk_index.unwrap_or(0),
        chunk,
        path,
        key,
    }
}

#[derive(Default)]
pub struct ChallengePayloadOverrides {
    pub uniform: Option<[u8; 8]>,
    pub content_length: Option<u64>,
}

pub fn build_challenge_payload(overrides: ChallengePayloadOverrides) -> ChallengePayload {
    ChallengePayload {
        uniform: overrides.uniform.unwrap_or([0u8; 8]),
        content_length: overrides.content_length.unwrap_or_else(|| {
            u64::try_from(REFERENCE_CONTENT_LENGTH)
                .expect("the reference content length fits a u64")
        }),
    }
}

#[derive(Default)]
pub struct RespondPayloadOverrides {
    pub chunk_index: Option<u64>,
}

pub fn build_respond_payload<'a>(
    content: &'a [u8],
    outboard: &'a Outboard,
    overrides: RespondPayloadOverrides,
) -> RespondPayload<'a> {
    RespondPayload {
        content,
        outboard,
        chunk_index: overrides.chunk_index.unwrap_or(0),
    }
}

#[derive(Default)]
pub struct CommitSuccessReturnOverrides {
    pub root: Option<Root>,
    pub outboard: Option<Outboard>,
}

pub fn build_commit_success_return(overrides: CommitSuccessReturnOverrides) -> CommitSuccessReturn {
    CommitSuccessReturn {
        root: overrides
            .root
            .unwrap_or_else(|| build_root(Default::default())),
        outboard: overrides
            .outboard
            .unwrap_or_else(|| build_outboard(Default::default())),
    }
}

#[derive(Default)]
pub struct VerifySuccessReturnOverrides;

pub fn build_verify_success_return(
    _overrides: VerifySuccessReturnOverrides,
) -> VerifySuccessReturn {
    VerifySuccessReturn
}

#[derive(Default)]
pub struct ExtractPathSuccessReturnOverrides {
    pub path: Option<AuthenticationPath>,
}

pub fn build_extract_path_success_return(
    overrides: ExtractPathSuccessReturnOverrides,
) -> ExtractPathSuccessReturn {
    ExtractPathSuccessReturn {
        path: overrides
            .path
            .unwrap_or_else(|| build_authentication_path(Default::default())),
    }
}

#[derive(Default)]
pub struct VerifyChunkSuccessReturnOverrides;

pub fn build_verify_chunk_success_return(
    _overrides: VerifyChunkSuccessReturnOverrides,
) -> VerifyChunkSuccessReturn {
    VerifyChunkSuccessReturn
}

#[derive(Default)]
pub struct ChallengeSuccessReturnOverrides {
    pub chunk_index: Option<u64>,
}

pub fn build_challenge_success_return(
    overrides: ChallengeSuccessReturnOverrides,
) -> ChallengeSuccessReturn {
    ChallengeSuccessReturn {
        chunk_index: overrides.chunk_index.unwrap_or(0),
    }
}

#[derive(Default)]
pub struct RespondSuccessReturnOverrides {
    pub chunk: Option<Chunk>,
    pub path: Option<AuthenticationPath>,
}

pub fn build_respond_success_return(
    overrides: RespondSuccessReturnOverrides,
) -> RespondSuccessReturn {
    RespondSuccessReturn {
        chunk: overrides
            .chunk
            .unwrap_or_else(|| build_chunk(Default::default())),
        path: overrides
            .path
            .unwrap_or_else(|| build_authentication_path(Default::default())),
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum MockICommitmentAdapterErrorReturn {
    ContentLengthUnrepresentable { length: usize },
    OffsetUnrepresentable { offset: u64 },
}

pub(crate) struct MockICommitmentAdapterConstructorParams {
    pub failure_mode: MockICommitmentAdapterFailureMode,
}

pub(crate) type MockICommitmentAdapterTryNewReturn = Result<MockICommitmentAdapter, Infallible>;

const MOCK_CHUNK_LENGTH: u64 = 64;
const MOCK_KEY_LENGTH: usize = 32;
const MOCK_NON_ROOT_LEAF: u8 = 0;
const MOCK_NON_ROOT_PARENT: u8 = 1;
const MOCK_ROOT_LEAF: u8 = 2;
const MOCK_ROOT_PARENT: u8 = 3;

fn mock_node(key: Option<&[u8; 32]>, role: u8, offset: u64, input: &[u8]) -> [u8; 32] {
    let mut node = [0u8; CHAINING_VALUE_LENGTH];
    for (index, word) in node.as_chunks_mut::<8>().0.iter_mut().enumerate() {
        let mut hasher = DefaultHasher::new();
        if let Some(key) = key {
            hasher.write(key);
        }
        hasher.write_u8(role);
        if role == MOCK_NON_ROOT_LEAF || role == MOCK_ROOT_LEAF {
            hasher.write(&offset.to_le_bytes());
        }
        hasher.write(input);
        hasher.write_usize(index);
        word.copy_from_slice(&hasher.finish().to_le_bytes());
    }
    node
}

fn mock_parent(key: Option<&[u8; 32]>, role: u8, left: &[u8; 32], right: &[u8; 32]) -> [u8; 32] {
    let mut input = [0u8; OUTBOARD_PARENT_LENGTH];
    input[..CHAINING_VALUE_LENGTH].copy_from_slice(left);
    input[CHAINING_VALUE_LENGTH..].copy_from_slice(right);
    mock_node(key, role, 0, &input)
}

fn mock_chunk_count(content_length: u64) -> u64 {
    if content_length == 0 {
        1
    } else {
        content_length.div_ceil(MOCK_CHUNK_LENGTH)
    }
}

fn mock_left_subtree_length(length: u64) -> u64 {
    let chunks = mock_chunk_count(length);
    let mut left_chunks = 1u64;
    while left_chunks * 2 < chunks {
        left_chunks *= 2;
    }
    left_chunks * MOCK_CHUNK_LENGTH
}

fn mock_expected_chunk_length(content_length: u64, chunk_index: u64) -> u64 {
    MOCK_CHUNK_LENGTH.min(content_length - chunk_index * MOCK_CHUNK_LENGTH)
}

fn mock_expected_outboard_length(content_length: u64) -> u64 {
    u64::try_from(OUTBOARD_HEADER_LENGTH).expect("the header length fits a u64")
        + u64::try_from(OUTBOARD_PARENT_LENGTH).expect("the parent length fits a u64")
            * (mock_chunk_count(content_length) - 1)
}

fn mock_key_bytes(secret: &Secret<Vec<u8>>) -> Result<[u8; 32], usize> {
    <[u8; 32]>::try_from(secret.expose().as_slice()).map_err(|_| secret.expose().len())
}

fn mock_admit_key(key: &CommitmentKey<'_>) -> Result<Option<[u8; 32]>, usize> {
    match key {
        CommitmentKey::Unkeyed => Ok(None),
        CommitmentKey::Keyed(secret) => mock_key_bytes(secret).map(Some),
    }
}

fn mock_declared_length(outboard: &Outboard) -> Result<u64, usize> {
    let bytes = outboard.bytes.as_slice();
    if bytes.len() < OUTBOARD_HEADER_LENGTH {
        return Err(bytes.len());
    }
    let header = <[u8; 8]>::try_from(&bytes[..OUTBOARD_HEADER_LENGTH])
        .expect("the header slice is eight bytes");
    Ok(u64::from_le_bytes(header))
}

fn mock_read_cv(outboard: &[u8], cursor: usize) -> [u8; 32] {
    let mut cv = [0u8; CHAINING_VALUE_LENGTH];
    cv.copy_from_slice(&outboard[cursor..cursor + CHAINING_VALUE_LENGTH]);
    cv
}

fn mock_commit_tree(
    content: &[u8],
    offset: u64,
    length: u64,
    key: Option<&[u8; 32]>,
    is_root: bool,
    outboard: &mut Vec<u8>,
) -> [u8; 32] {
    if length <= MOCK_CHUNK_LENGTH {
        let role = if is_root {
            MOCK_ROOT_LEAF
        } else {
            MOCK_NON_ROOT_LEAF
        };
        let start = usize::try_from(offset).expect("a subtree offset fits a usize");
        let end = start + usize::try_from(length).expect("a leaf length fits a usize");
        mock_node(key, role, offset, &content[start..end])
    } else {
        let role = if is_root {
            MOCK_ROOT_PARENT
        } else {
            MOCK_NON_ROOT_PARENT
        };
        let position = outboard.len();
        outboard.resize(position + OUTBOARD_PARENT_LENGTH, 0);
        let left_length = mock_left_subtree_length(length);
        let left = mock_commit_tree(content, offset, left_length, key, false, outboard);
        let right = mock_commit_tree(
            content,
            offset + left_length,
            length - left_length,
            key,
            false,
            outboard,
        );
        outboard[position..position + CHAINING_VALUE_LENGTH].copy_from_slice(&left);
        outboard[position + CHAINING_VALUE_LENGTH..position + OUTBOARD_PARENT_LENGTH]
            .copy_from_slice(&right);
        mock_parent(key, role, &left, &right)
    }
}

fn mock_extract_path(outboard: &[u8], content_length: u64, chunk_index: u64) -> Vec<[u8; 32]> {
    let mut siblings = Vec::new();
    let mut offset = 0u64;
    let mut length = content_length;
    let mut cursor = OUTBOARD_HEADER_LENGTH;
    while length > MOCK_CHUNK_LENGTH {
        let left_cv = mock_read_cv(outboard, cursor);
        let right_cv = mock_read_cv(outboard, cursor + CHAINING_VALUE_LENGTH);
        let left_length = mock_left_subtree_length(length);
        if chunk_index * MOCK_CHUNK_LENGTH < offset + left_length {
            siblings.push(right_cv);
            cursor += OUTBOARD_PARENT_LENGTH;
            length = left_length;
        } else {
            siblings.push(left_cv);
            let left_count =
                usize::try_from(mock_chunk_count(left_length)).expect("a chunk count fits a usize");
            cursor += OUTBOARD_PARENT_LENGTH * left_count;
            offset += left_length;
            length -= left_length;
        }
    }
    siblings
}

fn mock_chunk_route(content_length: u64, chunk_index: u64) -> (u64, Vec<bool>) {
    let mut sides = Vec::new();
    let mut offset = 0u64;
    let mut length = content_length;
    while length > MOCK_CHUNK_LENGTH {
        let left_length = mock_left_subtree_length(length);
        if chunk_index * MOCK_CHUNK_LENGTH < offset + left_length {
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

fn mock_verify_subtree(
    _content: &[u8],
    offset: u64,
    length: u64,
    recorded: &[u8; 32],
    cursor: usize,
    outboard: &[u8],
    key: Option<&[u8; 32]>,
    leaves: &mut Vec<(u64, u64, [u8; 32])>,
) -> Result<(), VerifyErrorReturn> {
    if length <= MOCK_CHUNK_LENGTH {
        leaves.push((offset / MOCK_CHUNK_LENGTH, offset, *recorded));
        return Ok(());
    }
    let left = mock_read_cv(outboard, cursor);
    let right = mock_read_cv(outboard, cursor + CHAINING_VALUE_LENGTH);
    if mock_parent(key, MOCK_NON_ROOT_PARENT, &left, &right) != *recorded {
        return Err(VerifyErrorReturn::ParentMismatch { offset, length });
    }
    let left_length = mock_left_subtree_length(length);
    let left_count =
        usize::try_from(mock_chunk_count(left_length)).expect("a chunk count fits a usize");
    mock_verify_subtree(
        _content,
        offset,
        left_length,
        &left,
        cursor + OUTBOARD_PARENT_LENGTH,
        outboard,
        key,
        leaves,
    )?;
    mock_verify_subtree(
        _content,
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

pub(crate) struct MockICommitmentAdapter {
    failure_mode: MockICommitmentAdapterFailureMode,
}

impl MockICommitmentAdapter {
    pub const DECLARATION: CommitmentDeclaration = CommitmentDeclaration {
        identifier: CommitmentSchemeIdentifier::MockV1,
        chunk_length: MOCK_CHUNK_LENGTH,
        key_length: MOCK_KEY_LENGTH,
        adapter_version: 1,
        interface_version: COMMITMENT_INTERFACE_VERSION,
    };

    pub fn try_new(
        params: MockICommitmentAdapterConstructorParams,
    ) -> MockICommitmentAdapterTryNewReturn {
        Ok(MockICommitmentAdapter {
            failure_mode: params.failure_mode,
        })
    }
}

impl ICommitmentAdapter for MockICommitmentAdapter {
    fn declaration(&self) -> CommitmentDeclaration {
        Self::DECLARATION
    }

    fn commit(&self, _params: CommitParams, payload: CommitPayload<'_>) -> CommitReturn {
        match self.failure_mode {
            MockICommitmentAdapterFailureMode::NoFailure => {}
            MockICommitmentAdapterFailureMode::ContentLengthUnrepresentable => {
                return Err(CommitErrorReturn::Mock(
                    MockICommitmentAdapterErrorReturn::ContentLengthUnrepresentable {
                        length: payload.content.len(),
                    },
                ));
            }
            MockICommitmentAdapterFailureMode::OffsetUnrepresentable => {
                return Err(CommitErrorReturn::Mock(
                    MockICommitmentAdapterErrorReturn::OffsetUnrepresentable {
                        offset: u64::try_from(payload.content.len())
                            .expect("a usize fits a u64 on every supported target"),
                    },
                ));
            }
        }
        let key = match mock_admit_key(&payload.key) {
            Ok(key) => key,
            Err(actual) => {
                return Err(CommitErrorReturn::KeyLengthMismatch {
                    expected: MOCK_KEY_LENGTH,
                    actual,
                });
            }
        };
        let content_length = u64::try_from(payload.content.len())
            .expect("a usize fits a u64 on every supported target");
        let mut bytes = content_length.to_le_bytes().to_vec();
        let root = mock_commit_tree(
            payload.content,
            0,
            content_length,
            key.as_ref(),
            true,
            &mut bytes,
        );
        let root = match Root::try_new(RootConstructorParams { bytes: root }) {
            Ok(root) => root,
            Err(error) => return Err(CommitErrorReturn::Root(error)),
        };
        Ok(CommitSuccessReturn {
            root,
            outboard: Outboard { bytes },
        })
    }

    fn verify(&self, _params: VerifyParams, payload: VerifyPayload<'_>) -> VerifyReturn {
        match self.failure_mode {
            MockICommitmentAdapterFailureMode::NoFailure => {}
            MockICommitmentAdapterFailureMode::ContentLengthUnrepresentable => {
                return Err(VerifyErrorReturn::Mock(
                    MockICommitmentAdapterErrorReturn::ContentLengthUnrepresentable {
                        length: payload.content.len(),
                    },
                ));
            }
            MockICommitmentAdapterFailureMode::OffsetUnrepresentable => {
                return Err(VerifyErrorReturn::Mock(
                    MockICommitmentAdapterErrorReturn::OffsetUnrepresentable {
                        offset: u64::try_from(payload.content.len())
                            .expect("a usize fits a u64 on every supported target"),
                    },
                ));
            }
        }
        let key = match mock_admit_key(&payload.key) {
            Ok(key) => key,
            Err(actual) => {
                return Err(VerifyErrorReturn::KeyLengthMismatch {
                    expected: MOCK_KEY_LENGTH,
                    actual,
                });
            }
        };
        let declared = match mock_declared_length(payload.outboard) {
            Ok(declared) => declared,
            Err(length) => return Err(VerifyErrorReturn::OutboardTruncated { length }),
        };
        let actual = u64::try_from(payload.content.len())
            .expect("a usize fits a u64 on every supported target");
        if declared != actual {
            return Err(VerifyErrorReturn::ContentLengthMismatch { declared, actual });
        }
        let expected = mock_expected_outboard_length(declared);
        let actual = u64::try_from(payload.outboard.bytes.len())
            .expect("a usize fits a u64 on every supported target");
        if actual != expected {
            return Err(VerifyErrorReturn::OutboardLengthMismatch { expected, actual });
        }
        let outboard = payload.outboard.bytes.as_slice();
        if mock_chunk_count(declared) == 1 {
            if &mock_node(key.as_ref(), MOCK_ROOT_LEAF, 0, payload.content)
                != payload.root.as_bytes()
            {
                return Err(VerifyErrorReturn::RootMismatch);
            }
            return Ok(VerifySuccessReturn);
        }
        let left = mock_read_cv(outboard, OUTBOARD_HEADER_LENGTH);
        let right = mock_read_cv(outboard, OUTBOARD_HEADER_LENGTH + CHAINING_VALUE_LENGTH);
        if &mock_parent(key.as_ref(), MOCK_ROOT_PARENT, &left, &right) != payload.root.as_bytes() {
            return Err(VerifyErrorReturn::RootMismatch);
        }
        let left_length = mock_left_subtree_length(declared);
        let left_count =
            usize::try_from(mock_chunk_count(left_length)).expect("a chunk count fits a usize");
        let mut leaves = Vec::new();
        mock_verify_subtree(
            payload.content,
            0,
            left_length,
            &left,
            OUTBOARD_HEADER_LENGTH + OUTBOARD_PARENT_LENGTH,
            outboard,
            key.as_ref(),
            &mut leaves,
        )?;
        mock_verify_subtree(
            payload.content,
            left_length,
            declared - left_length,
            &right,
            OUTBOARD_HEADER_LENGTH + OUTBOARD_PARENT_LENGTH * left_count,
            outboard,
            key.as_ref(),
            &mut leaves,
        )?;
        for (chunk_index, offset, recorded) in leaves {
            let start = usize::try_from(offset).expect("a leaf offset fits a usize");
            let length = usize::try_from(mock_expected_chunk_length(declared, chunk_index))
                .expect("a leaf length fits a usize");
            if mock_node(
                key.as_ref(),
                MOCK_NON_ROOT_LEAF,
                offset,
                &payload.content[start..start + length],
            ) != recorded
            {
                return Err(VerifyErrorReturn::ChunkMismatch { chunk_index });
            }
        }
        Ok(VerifySuccessReturn)
    }

    fn extract_path(
        &self,
        _params: ExtractPathParams,
        payload: ExtractPathPayload<'_>,
    ) -> ExtractPathReturn {
        match self.failure_mode {
            MockICommitmentAdapterFailureMode::NoFailure => {}
            MockICommitmentAdapterFailureMode::ContentLengthUnrepresentable => {
                return Err(ExtractPathErrorReturn::Mock(
                    MockICommitmentAdapterErrorReturn::ContentLengthUnrepresentable {
                        length: payload.outboard.bytes.len(),
                    },
                ));
            }
            MockICommitmentAdapterFailureMode::OffsetUnrepresentable => {
                return Err(ExtractPathErrorReturn::Mock(
                    MockICommitmentAdapterErrorReturn::OffsetUnrepresentable {
                        offset: payload.chunk_index,
                    },
                ));
            }
        }
        let declared = match mock_declared_length(payload.outboard) {
            Ok(declared) => declared,
            Err(length) => return Err(ExtractPathErrorReturn::OutboardTruncated { length }),
        };
        let expected = mock_expected_outboard_length(declared);
        let actual = u64::try_from(payload.outboard.bytes.len())
            .expect("a usize fits a u64 on every supported target");
        if actual != expected {
            return Err(ExtractPathErrorReturn::OutboardLengthMismatch { expected, actual });
        }
        let chunk_count = mock_chunk_count(declared);
        if payload.chunk_index >= chunk_count {
            return Err(ExtractPathErrorReturn::ChunkIndexOutOfRange {
                chunk_index: payload.chunk_index,
                chunk_count,
            });
        }
        Ok(ExtractPathSuccessReturn {
            path: AuthenticationPath {
                siblings: mock_extract_path(
                    payload.outboard.bytes.as_slice(),
                    declared,
                    payload.chunk_index,
                ),
            },
        })
    }

    fn verify_chunk(
        &self,
        _params: VerifyChunkParams,
        payload: VerifyChunkPayload<'_>,
    ) -> VerifyChunkReturn {
        match self.failure_mode {
            MockICommitmentAdapterFailureMode::NoFailure => {}
            MockICommitmentAdapterFailureMode::ContentLengthUnrepresentable => {
                return Err(VerifyChunkErrorReturn::Mock(
                    MockICommitmentAdapterErrorReturn::ContentLengthUnrepresentable {
                        length: payload.chunk.bytes.len(),
                    },
                ));
            }
            MockICommitmentAdapterFailureMode::OffsetUnrepresentable => {
                return Err(VerifyChunkErrorReturn::Mock(
                    MockICommitmentAdapterErrorReturn::OffsetUnrepresentable {
                        offset: payload.chunk_index,
                    },
                ));
            }
        }
        let key = match mock_admit_key(&payload.key) {
            Ok(key) => key,
            Err(actual) => {
                return Err(VerifyChunkErrorReturn::KeyLengthMismatch {
                    expected: MOCK_KEY_LENGTH,
                    actual,
                });
            }
        };
        let chunk_count = mock_chunk_count(payload.content_length);
        if payload.chunk_index >= chunk_count {
            return Err(VerifyChunkErrorReturn::ChunkIndexOutOfRange {
                chunk_index: payload.chunk_index,
                chunk_count,
            });
        }
        let expected = mock_expected_chunk_length(payload.content_length, payload.chunk_index);
        let actual = u64::try_from(payload.chunk.bytes.len())
            .expect("a usize fits a u64 on every supported target");
        if actual != expected {
            return Err(VerifyChunkErrorReturn::ChunkLengthMismatch { expected, actual });
        }
        let (offset, sides) = mock_chunk_route(payload.content_length, payload.chunk_index);
        if payload.path.siblings.len() != sides.len() {
            return Err(VerifyChunkErrorReturn::PathLengthMismatch {
                expected: sides.len(),
                actual: payload.path.siblings.len(),
            });
        }
        let mut value = if sides.is_empty() {
            mock_node(key.as_ref(), MOCK_ROOT_LEAF, 0, &payload.chunk.bytes)
        } else {
            mock_node(
                key.as_ref(),
                MOCK_NON_ROOT_LEAF,
                offset,
                &payload.chunk.bytes,
            )
        };
        for depth in (0..sides.len()).rev() {
            let sibling = &payload.path.siblings[depth];
            let (left, right) = if sides[depth] {
                (value, *sibling)
            } else {
                (*sibling, value)
            };
            let role = if depth == 0 {
                MOCK_ROOT_PARENT
            } else {
                MOCK_NON_ROOT_PARENT
            };
            value = mock_parent(key.as_ref(), role, &left, &right);
        }
        if &value != payload.root.as_bytes() {
            return Err(VerifyChunkErrorReturn::RootMismatch);
        }
        Ok(VerifyChunkSuccessReturn)
    }

    fn challenge(&self, _params: ChallengeParams, payload: ChallengePayload) -> ChallengeReturn {
        Ok(ChallengeSuccessReturn {
            chunk_index: u64::from_le_bytes(payload.uniform)
                % mock_chunk_count(payload.content_length),
        })
    }

    fn respond(&self, _params: RespondParams, payload: RespondPayload<'_>) -> RespondReturn {
        match self.failure_mode {
            MockICommitmentAdapterFailureMode::NoFailure => {}
            MockICommitmentAdapterFailureMode::ContentLengthUnrepresentable => {
                return Err(RespondErrorReturn::Mock(
                    MockICommitmentAdapterErrorReturn::ContentLengthUnrepresentable {
                        length: payload.content.len(),
                    },
                ));
            }
            MockICommitmentAdapterFailureMode::OffsetUnrepresentable => {
                return Err(RespondErrorReturn::Mock(
                    MockICommitmentAdapterErrorReturn::OffsetUnrepresentable {
                        offset: payload.chunk_index,
                    },
                ));
            }
        }
        let declared = match mock_declared_length(payload.outboard) {
            Ok(declared) => declared,
            Err(length) => return Err(RespondErrorReturn::OutboardTruncated { length }),
        };
        let actual = u64::try_from(payload.content.len())
            .expect("a usize fits a u64 on every supported target");
        if declared != actual {
            return Err(RespondErrorReturn::ContentLengthMismatch { declared, actual });
        }
        let expected = mock_expected_outboard_length(declared);
        let actual = u64::try_from(payload.outboard.bytes.len())
            .expect("a usize fits a u64 on every supported target");
        if actual != expected {
            return Err(RespondErrorReturn::OutboardLengthMismatch { expected, actual });
        }
        let chunk_count = mock_chunk_count(declared);
        if payload.chunk_index >= chunk_count {
            return Err(RespondErrorReturn::ChunkIndexOutOfRange {
                chunk_index: payload.chunk_index,
                chunk_count,
            });
        }
        let offset = payload.chunk_index * MOCK_CHUNK_LENGTH;
        let length = mock_expected_chunk_length(declared, payload.chunk_index);
        let start = usize::try_from(offset).expect("a chunk offset fits a usize");
        let end = start + usize::try_from(length).expect("a chunk length fits a usize");
        Ok(RespondSuccessReturn {
            chunk: Chunk {
                bytes: payload.content[start..end].to_vec(),
            },
            path: AuthenticationPath {
                siblings: mock_extract_path(
                    payload.outboard.bytes.as_slice(),
                    declared,
                    payload.chunk_index,
                ),
            },
        })
    }
}

#[derive(Default)]
pub struct MockICommitmentAdapterConstructorParamsOverrides {
    pub failure_mode: Option<MockICommitmentAdapterFailureMode>,
}

pub(crate) fn build_mock_i_commitment_adapter_constructor_params(
    overrides: MockICommitmentAdapterConstructorParamsOverrides,
) -> MockICommitmentAdapterConstructorParams {
    MockICommitmentAdapterConstructorParams {
        failure_mode: overrides
            .failure_mode
            .unwrap_or(MockICommitmentAdapterFailureMode::NoFailure),
    }
}

pub(crate) fn build_mock_i_commitment_adapter(
    overrides: MockICommitmentAdapterConstructorParamsOverrides,
) -> MockICommitmentAdapter {
    let Ok(adapter) = MockICommitmentAdapter::try_new(
        build_mock_i_commitment_adapter_constructor_params(overrides),
    );
    adapter
}

#[derive(Default)]
pub struct CreateCommitmentDepsOverrides;

pub fn build_create_commitment_deps(
    _overrides: CreateCommitmentDepsOverrides,
) -> CreateCommitmentDeps {
    CreateCommitmentDeps
}

#[derive(Default)]
pub struct CreateCommitmentPayloadOverrides;

pub fn build_create_commitment_payload(
    _overrides: CreateCommitmentPayloadOverrides,
) -> CreateCommitmentPayload {
    CreateCommitmentPayload
}

#[derive(Default)]
pub struct CreateCommitmentParamsOverrides {
    pub concrete: Option<CommitmentConcrete>,
    pub identifier: Option<CommitmentSchemeIdentifier>,
}

pub fn build_create_commitment_params(
    overrides: CreateCommitmentParamsOverrides,
) -> CreateCommitmentParams {
    CreateCommitmentParams {
        concrete: overrides.concrete.unwrap_or(CommitmentConcrete::Mock(
            MockICommitmentAdapterFailureMode::NoFailure,
        )),
        identifier: overrides
            .identifier
            .unwrap_or(CommitmentSchemeIdentifier::MockV1),
    }
}

#[derive(Default)]
pub struct CreateCommitmentParamsCorruptions {
    pub concrete: Option<Value>,
    pub identifier: Option<Value>,
}

pub fn invalidate_create_commitment_params(
    corruptions: CreateCommitmentParamsCorruptions,
) -> Value {
    let params = build_create_commitment_params(Default::default());
    let mut fields = Map::new();
    fields.insert(
        "concrete".to_string(),
        match params.concrete {
            CommitmentConcrete::Blake3Bao => Value::from("Blake3Bao"),
            CommitmentConcrete::Mock(failure_mode) => {
                let mut entry = Map::new();
                entry.insert(
                    "Mock".to_string(),
                    Value::from(match failure_mode {
                        MockICommitmentAdapterFailureMode::NoFailure => "NoFailure",
                        MockICommitmentAdapterFailureMode::ContentLengthUnrepresentable => {
                            "ContentLengthUnrepresentable"
                        }
                        MockICommitmentAdapterFailureMode::OffsetUnrepresentable => {
                            "OffsetUnrepresentable"
                        }
                    }),
                );
                Value::Object(entry)
            }
        },
    );
    fields.insert(
        "identifier".to_string(),
        Value::from(match params.identifier {
            CommitmentSchemeIdentifier::Blake3BaoV1 => "Blake3BaoV1",
            CommitmentSchemeIdentifier::MockV1 => "MockV1",
        }),
    );
    if let Some(concrete) = corruptions.concrete {
        fields.insert("concrete".to_string(), concrete);
    }
    if let Some(identifier) = corruptions.identifier {
        fields.insert("identifier".to_string(), identifier);
    }
    Value::Object(fields)
}

#[derive(Default)]
pub struct CreateCommitmentSuccessReturnOverrides;

pub fn build_create_commitment_success_return(
    adapter: Box<dyn ICommitmentAdapter>,
    _overrides: CreateCommitmentSuccessReturnOverrides,
) -> CreateCommitmentSuccessReturn {
    CreateCommitmentSuccessReturn { adapter }
}

pub fn mock_create_commitment(
    _deps: &CreateCommitmentDeps,
    _params: CreateCommitmentParams,
    _payload: CreateCommitmentPayload,
) -> CreateCommitmentReturn {
    let Ok(adapter) = MockICommitmentAdapter::try_new(MockICommitmentAdapterConstructorParams {
        failure_mode: MockICommitmentAdapterFailureMode::NoFailure,
    });
    Ok(build_create_commitment_success_return(
        Box::new(adapter),
        Default::default(),
    ))
}
