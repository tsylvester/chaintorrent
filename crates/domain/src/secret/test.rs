#![allow(clippy::panic)]

use std::panic::{AssertUnwindSafe, catch_unwind};
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

use static_assertions::assert_not_impl_any;
use zeroize::Zeroize;

use super::provides::{
    Secret, SecretConstructorParamsOverrides, build_secret, build_secret_constructor_params,
};

// Contract: `Secret<T>` implements none of `core::fmt::Debug`, `core::fmt::Display`,
//   `Clone`, or `Copy`.
// Arrange: the type `Secret<Vec<u8>>`.
// Act:     `assert_not_impl_any!` over that type naming `core::fmt::Debug`,
//   `core::fmt::Display`, `Clone`, and `Copy`.
// Assert:  the module compiles only if none of the four traits is implemented.
assert_not_impl_any!(Secret<Vec<u8>>: core::fmt::Debug, core::fmt::Display, Clone, Copy);

/// Contract: any params → `Ok(Secret)` holding `params.value`.
/// Arrange: `build_secret_constructor_params` with the value override set to a nonempty
///   byte sequence, so the default empty vector and the given value differ.
/// Act:     `Secret::try_new` on the built params.
/// Assert:  the result binds through the irrefutable `let Ok(secret) = …;`; the held
///   `value`, read through the field the child test module reaches, equals the same byte
///   sequence written as a literal in the assertion.
#[test]
fn try_new_holds_the_value_in_params() {
    // Arrange
    let params = build_secret_constructor_params::<Vec<u8>>(SecretConstructorParamsOverrides {
        value: Some(vec![1, 2, 3, 4]),
    });

    // Act
    let Ok(secret) = Secret::try_new(params);

    // Assert
    assert_eq!(secret.value, vec![1, 2, 3, 4]);
}

/// Contract: a living secret → a shared reference to the held value, no copy.
/// Arrange: `build_secret` with the value override set to a nonempty byte sequence, so the
///   default empty vector and the held value differ.
/// Act:     `secret.expose()`.
/// Assert:  the returned reference equals the same byte sequence written as a literal in
///   the assertion; the returned reference is address-identical to the secret's held
///   `value` field under `core::ptr::eq`.
#[test]
fn expose_returns_a_reference_to_the_held_value() {
    // Arrange
    let secret = build_secret::<Vec<u8>>(SecretConstructorParamsOverrides {
        value: Some(vec![5, 6, 7, 8]),
    });

    // Act
    let exposed = secret.expose();

    // Assert
    assert_eq!(exposed, &vec![5, 6, 7, 8]);
    assert!(core::ptr::eq(exposed, &secret.value));
}

/// Contract: the secret's lifetime ends by move into a consumer that drops it →
///   `Zeroize::zeroize` is called on the held value exactly once.
/// Arrange: the shared counter at its initial value; the recorder holding a handle to it;
///   `build_secret` with the value override set to the recorder.
/// Act:     `drop(secret)`.
/// Assert:  the shared counter equals the literal one.
#[test]
fn dropping_a_secret_zeroizes_its_value() {
    struct ZeroizeRecorder {
        count: Arc<AtomicUsize>,
    }
    impl Default for ZeroizeRecorder {
        fn default() -> Self {
            Self {
                count: Arc::new(AtomicUsize::new(0)),
            }
        }
    }
    impl Zeroize for ZeroizeRecorder {
        fn zeroize(&mut self) {
            self.count.fetch_add(1, Ordering::SeqCst);
        }
    }

    // Arrange
    let counter = Arc::new(AtomicUsize::new(0));
    let recorder = ZeroizeRecorder {
        count: Arc::clone(&counter),
    };
    let secret = build_secret::<ZeroizeRecorder>(SecretConstructorParamsOverrides {
        value: Some(recorder),
    });

    // Act
    drop(secret);

    // Assert
    assert_eq!(counter.load(Ordering::SeqCst), 1);
}

/// Contract: a living secret → `expose` returns a shared reference with no side effect on
///   the held value.
/// Arrange: the shared counter at its initial value; the recorder holding a handle to it;
///   `build_secret` with the value override set to the recorder.
/// Act:     `secret.expose()`.
/// Assert:  the shared counter equals the literal zero while the secret is alive.
#[test]
fn exposing_a_secret_does_not_zeroize_it() {
    struct ZeroizeRecorder {
        count: Arc<AtomicUsize>,
    }
    impl Default for ZeroizeRecorder {
        fn default() -> Self {
            Self {
                count: Arc::new(AtomicUsize::new(0)),
            }
        }
    }
    impl Zeroize for ZeroizeRecorder {
        fn zeroize(&mut self) {
            self.count.fetch_add(1, Ordering::SeqCst);
        }
    }

    // Arrange
    let counter = Arc::new(AtomicUsize::new(0));
    let recorder = ZeroizeRecorder {
        count: Arc::clone(&counter),
    };
    let secret = build_secret::<ZeroizeRecorder>(SecretConstructorParamsOverrides {
        value: Some(recorder),
    });

    // Act
    let _ = secret.expose();

    // Assert
    assert_eq!(counter.load(Ordering::SeqCst), 0);
}

/// Contract: any params → `Ok(Secret)` holding `params.value`, moved without copy.
/// Arrange: a nonempty byte vector bound to a local whose heap address is read before the
///   call; `build_secret_constructor_params` with the value override set to that vector.
/// Act:     `Secret::try_new` on the built params.
/// Assert:  the result binds through the irrefutable `let Ok(secret) = …;`; the heap
///   address of the held `value` equals the address read before the call.
#[test]
fn try_new_moves_the_value_without_copying_it() {
    // Arrange
    let value = vec![9, 8, 7, 6];
    let heap_address = value.as_ptr();
    let params = build_secret_constructor_params::<Vec<u8>>(SecretConstructorParamsOverrides {
        value: Some(value),
    });

    // Act
    let Ok(secret) = Secret::try_new(params);

    // Assert
    assert_eq!(secret.value.as_ptr(), heap_address);
}

/// Contract: the secret's lifetime ends by unwinding → `Zeroize::zeroize` is called on the
///   held value exactly once.
/// Arrange: the shared counter at its initial value; the recorder holding a handle to it;
///   `build_secret` with the value override set to the recorder; a closure that takes the
///   secret by move and panics.
/// Act:     `std::panic::catch_unwind` on the closure.
/// Assert:  the outcome is `Err`; the shared counter equals the literal one.
#[test]
fn dropping_a_secret_during_unwinding_zeroizes_its_value() {
    struct ZeroizeRecorder {
        count: Arc<AtomicUsize>,
    }
    impl Default for ZeroizeRecorder {
        fn default() -> Self {
            Self {
                count: Arc::new(AtomicUsize::new(0)),
            }
        }
    }
    impl Zeroize for ZeroizeRecorder {
        fn zeroize(&mut self) {
            self.count.fetch_add(1, Ordering::SeqCst);
        }
    }

    // Arrange
    let counter = Arc::new(AtomicUsize::new(0));
    let recorder = ZeroizeRecorder {
        count: Arc::clone(&counter),
    };
    let secret = build_secret::<ZeroizeRecorder>(SecretConstructorParamsOverrides {
        value: Some(recorder),
    });
    let consume_and_panic = move || {
        let _moved = secret;
        panic!("unwind");
    };

    // Act
    let outcome = catch_unwind(AssertUnwindSafe(consume_and_panic));

    // Assert
    assert!(outcome.is_err());
    assert_eq!(counter.load(Ordering::SeqCst), 1);
}
