#![allow(
    dead_code,
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::too_many_arguments
)]

use super::interface::{
    ConsumeRandomSourceParams, ConsumeRandomSourcePayload, CreateRandomSourceDeps,
    CreateRandomSourceParams, CreateRandomSourcePayload, CreateRandomSourceReturn,
    CreateRandomSourceSuccessReturn, FillBytesErrorReturn, FillBytesParams, FillBytesPayload,
    FillBytesReturn, FillBytesSuccessReturn, IRandomSourceAdapter, IRandomSourceConsumer,
    MockIRandomSourceAdapterFailureMode, RANDOM_SOURCE_INTERFACE_VERSION, RandomSourceDeclaration,
    RandomSourceKind,
};
use core::cell::Cell;
use core::convert::Infallible;
use domain::{Secret, build_secret};

#[derive(Default)]
pub struct RandomSourceDeclarationOverrides {
    pub source: Option<RandomSourceKind>,
    pub adapter_version: Option<u32>,
    pub interface_version: Option<u32>,
}

pub fn build_random_source_declaration(
    overrides: RandomSourceDeclarationOverrides,
) -> RandomSourceDeclaration {
    RandomSourceDeclaration {
        source: overrides
            .source
            .unwrap_or(RandomSourceKind::OperatingSystem),
        adapter_version: overrides.adapter_version.unwrap_or(1),
        interface_version: overrides
            .interface_version
            .unwrap_or(RANDOM_SOURCE_INTERFACE_VERSION),
    }
}

#[derive(Default)]
pub struct FillBytesPayloadOverrides {
    pub length: Option<usize>,
}

pub fn build_fill_bytes_payload(overrides: FillBytesPayloadOverrides) -> FillBytesPayload {
    FillBytesPayload {
        length: overrides.length.unwrap_or(32),
    }
}

#[derive(Default)]
pub struct FillBytesSuccessReturnOverrides {
    pub bytes: Option<Secret<Vec<u8>>>,
}

pub fn build_fill_bytes_success_return(
    overrides: FillBytesSuccessReturnOverrides,
) -> FillBytesSuccessReturn {
    FillBytesSuccessReturn {
        bytes: overrides
            .bytes
            .unwrap_or_else(|| build_secret(vec![0u8; 32], Default::default())),
    }
}

pub struct MockIRandomSourceAdapterConstructorParams {
    pub failure_mode: MockIRandomSourceAdapterFailureMode,
}

#[derive(Default)]
pub struct MockIRandomSourceAdapterConstructorParamsOverrides {
    pub failure_mode: Option<MockIRandomSourceAdapterFailureMode>,
}

pub fn build_mock_i_random_source_adapter_constructor_params(
    overrides: MockIRandomSourceAdapterConstructorParamsOverrides,
) -> MockIRandomSourceAdapterConstructorParams {
    MockIRandomSourceAdapterConstructorParams {
        failure_mode: overrides
            .failure_mode
            .unwrap_or(MockIRandomSourceAdapterFailureMode::Succeeds),
    }
}

pub(crate) struct MockIRandomSourceAdapter {
    pub(super) failure_mode: MockIRandomSourceAdapterFailureMode,
    pub(super) counter: Cell<u64>,
}

pub(crate) type MockIRandomSourceAdapterTryNewReturn = Result<MockIRandomSourceAdapter, Infallible>;

impl MockIRandomSourceAdapter {
    pub(crate) fn try_new(
        params: MockIRandomSourceAdapterConstructorParams,
    ) -> MockIRandomSourceAdapterTryNewReturn {
        Ok(MockIRandomSourceAdapter {
            failure_mode: params.failure_mode,
            counter: Cell::new(0),
        })
    }
}

impl IRandomSourceAdapter for MockIRandomSourceAdapter {
    fn fill_bytes(&self, _params: FillBytesParams, payload: FillBytesPayload) -> FillBytesReturn {
        if matches!(
            self.failure_mode,
            MockIRandomSourceAdapterFailureMode::FillBytesRefused
        ) {
            return Err(FillBytesErrorReturn::MockIRandomSourceAdapter);
        }
        let mut buffer = vec![0u8; payload.length];
        for byte in &mut buffer {
            let counter = self.counter.get().wrapping_add(1);
            self.counter.set(counter);
            let mut mixed = counter;
            mixed ^= mixed >> 30;
            mixed = mixed.wrapping_mul(0xbf58_476d_1ce4_e5b9);
            mixed ^= mixed >> 27;
            mixed = mixed.wrapping_mul(0x94d0_49bb_1331_11eb);
            mixed ^= mixed >> 31;
            *byte =
                u8::try_from(mixed & 0xff).expect("a value masked to its low eight bits fits a u8");
        }
        Ok(FillBytesSuccessReturn {
            bytes: build_secret(buffer, Default::default()),
        })
    }
}

pub(crate) fn build_mock_i_random_source_adapter(
    overrides: MockIRandomSourceAdapterConstructorParamsOverrides,
) -> MockIRandomSourceAdapter {
    let Ok(adapter) = MockIRandomSourceAdapter::try_new(
        build_mock_i_random_source_adapter_constructor_params(overrides),
    );
    adapter
}

pub(crate) const MOCK_I_RANDOM_SOURCE_ADAPTER_DECLARATION: RandomSourceDeclaration =
    RandomSourceDeclaration {
        source: RandomSourceKind::Mock(MockIRandomSourceAdapterFailureMode::Succeeds),
        adapter_version: 1,
        interface_version: RANDOM_SOURCE_INTERFACE_VERSION,
    };

#[derive(Default)]
pub struct RandomSourceKindOverrides;

pub fn build_random_source_kind(
    kind: RandomSourceKind,
    _overrides: RandomSourceKindOverrides,
) -> RandomSourceKind {
    match kind {
        RandomSourceKind::OperatingSystem => RandomSourceKind::OperatingSystem,
        RandomSourceKind::Mock(_) => {
            RandomSourceKind::Mock(MockIRandomSourceAdapterFailureMode::Succeeds)
        }
    }
}

pub struct MockIRandomSourceConsumer;

impl IRandomSourceConsumer for MockIRandomSourceConsumer {
    type Output = ();

    fn consume_random_source<K: IRandomSourceAdapter>(
        &self,
        _params: ConsumeRandomSourceParams,
        _payload: ConsumeRandomSourcePayload<K>,
    ) -> Self::Output {
    }
}

#[derive(Default)]
pub struct ConsumeRandomSourceParamsOverrides;

pub fn build_consume_random_source_params(
    _overrides: ConsumeRandomSourceParamsOverrides,
) -> ConsumeRandomSourceParams {
    ConsumeRandomSourceParams
}

#[derive(Default)]
pub struct ConsumeRandomSourcePayloadOverrides;

pub fn build_consume_random_source_payload<K: IRandomSourceAdapter>(
    source: K,
    _overrides: ConsumeRandomSourcePayloadOverrides,
) -> ConsumeRandomSourcePayload<K> {
    ConsumeRandomSourcePayload { source }
}

#[derive(Default)]
pub struct CreateRandomSourceDepsOverrides;

pub fn build_create_random_source_deps<C: IRandomSourceConsumer>(
    consumer: C,
    _overrides: CreateRandomSourceDepsOverrides,
) -> CreateRandomSourceDeps<C> {
    CreateRandomSourceDeps { consumer }
}

#[derive(Default)]
pub struct CreateRandomSourceParamsOverrides {
    pub kind: Option<RandomSourceKind>,
}

pub fn build_create_random_source_params(
    overrides: CreateRandomSourceParamsOverrides,
) -> CreateRandomSourceParams {
    CreateRandomSourceParams {
        kind: overrides.kind.unwrap_or(RandomSourceKind::Mock(
            MockIRandomSourceAdapterFailureMode::Succeeds,
        )),
    }
}

#[derive(Default)]
pub struct CreateRandomSourcePayloadOverrides;

pub fn build_create_random_source_payload(
    _overrides: CreateRandomSourcePayloadOverrides,
) -> CreateRandomSourcePayload {
    CreateRandomSourcePayload
}

#[derive(Default)]
pub struct CreateRandomSourceSuccessReturnOverrides;

pub fn build_create_random_source_success_return<O>(
    output: O,
    _overrides: CreateRandomSourceSuccessReturnOverrides,
) -> CreateRandomSourceSuccessReturn<O> {
    CreateRandomSourceSuccessReturn { output }
}

pub fn mock_create_random_source<C: IRandomSourceConsumer>(
    deps: &CreateRandomSourceDeps<C>,
    _params: CreateRandomSourceParams,
    _payload: CreateRandomSourcePayload,
) -> CreateRandomSourceReturn<C::Output> {
    let source = build_mock_i_random_source_adapter(Default::default());
    let output = deps.consumer.consume_random_source(
        ConsumeRandomSourceParams,
        ConsumeRandomSourcePayload { source },
    );
    Ok(build_create_random_source_success_return(
        output,
        Default::default(),
    ))
}
