#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::as_conversions
)]

use super::interface::Secret;
use super::mock::{SecretConstructorParamsOverrides, build_secret};
use static_assertions::assert_not_impl_any;
use std::cell::Cell;
use std::rc::Rc;
use zeroize::Zeroize;

// Contract: a secret has no formatting, cloning, or copying implementation.
assert_not_impl_any!(Secret<[u8; 32]>: core::fmt::Debug, core::fmt::Display, Clone, Copy);

/// Contract: a living secret exposes a shared reference to the held value, no
///   copy, no side effect.
/// Arrange: a secret built around `[7u8; 32]`, differing from the builder's
///   all-zero default.
/// Act:     `secret.expose()`.
/// Assert:  the returned reference equals `&[7u8; 32]`.
#[test]
fn expose_returns_the_value_the_secret_was_constructed_with() {
    // Arrange
    let secret = build_secret(SecretConstructorParamsOverrides {
        value: Some([7u8; 32]),
    });

    // Act
    let exposed = secret.expose();

    // Assert
    assert_eq!(exposed, &[7u8; 32]);
}

#[derive(Default)]
struct ZeroizeProbe {
    zeroized: Rc<Cell<bool>>,
}

impl Zeroize for ZeroizeProbe {
    fn zeroize(&mut self) {
        self.zeroized.set(true);
    }
}

/// Contract: when the secret's lifetime ends, `Zeroize::zeroize` runs on the
///   held value exactly once and the held value's memory is zeroized before
///   release.
/// Arrange: a flag `Rc<Cell<bool>>` starting `false`, and a secret built
///   around a `ZeroizeProbe` holding a clone of it.
/// Act:     `drop(secret)`.
/// Assert:  the flag reads `true`.
#[test]
fn dropping_a_secret_zeroizes_its_value() {
    // Arrange
    let flag = Rc::new(Cell::new(false));
    let secret = build_secret(SecretConstructorParamsOverrides {
        value: Some(ZeroizeProbe {
            zeroized: flag.clone(),
        }),
    });

    // Act
    drop(secret);

    // Assert
    assert!(flag.get());
}
