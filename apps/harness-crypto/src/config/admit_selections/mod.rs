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

use chain::DECLARED_CHAIN_FORMS_SELECTIONS;
use encoding::DECLARED_ENCODING_SELECTIONS;
use envelope::DECLARED_KEY_AGREEMENT_SELECTIONS;
use hash_to_scalar::DECLARED_HASH_TO_SCALAR_SELECTIONS;
use kem::DECLARED_KEM_SELECTIONS;
use pairing::{PAIRING_CONCRETES, PRECOMPILE_ENCODINGS, TARGET_GROUP_ENCODING_IDENTIFIERS};
use proof::DELIVERY_PROOF_CONCRETES;
use random::RANDOM_SOURCE_KINDS;

use interface::{
    AdmitSelectionsDeps, AdmitSelectionsErrorReturn, AdmitSelectionsFn, AdmitSelectionsParams,
    AdmitSelectionsPayload, AdmitSelectionsReturn, AdmitSelectionsSuccessReturn,
    AdmittedChainForms, AdmittedCurve, AdmittedDeliveryProof, AdmittedEncoding,
    AdmittedHashToScalar, AdmittedKem, AdmittedKeyAgreement, AdmittedPairing, AdmittedSelections,
    PairingField,
};

use crate::verifier::provides::VERIFIER_CONCRETES;

pub fn admit_selections(
    _deps: &AdmitSelectionsDeps,
    _params: AdmitSelectionsParams,
    payload: AdmitSelectionsPayload,
) -> AdmitSelectionsReturn {
    let _: AdmitSelectionsFn = admit_selections;

    let selections = payload.selections;

    let random_source =
        match member_named(&selections.random_source, RANDOM_SOURCE_KINDS, |member| {
            member
        }) {
            Some(member) => *member,
            None => {
                return Err(AdmitSelectionsErrorReturn::RandomSource {
                    text: selections.random_source.clone(),
                });
            }
        };

    let encoding = match member_named(
        &selections.encoding,
        DECLARED_ENCODING_SELECTIONS,
        |member| &member.concrete,
    ) {
        Some(member) => AdmittedEncoding {
            concrete: member.concrete,
            identifier: member.identifier,
        },
        None => {
            return Err(AdmitSelectionsErrorReturn::Encoding {
                text: selections.encoding.clone(),
            });
        }
    };

    let hash_to_scalar = match member_named(
        &selections.hash_to_scalar,
        DECLARED_HASH_TO_SCALAR_SELECTIONS,
        |member| &member.concrete,
    ) {
        Some(member) => AdmittedHashToScalar {
            concrete: member.concrete,
            identifier: member.identifier,
        },
        None => {
            return Err(AdmitSelectionsErrorReturn::HashToScalar {
                text: selections.hash_to_scalar.clone(),
            });
        }
    };

    let chain_forms = match member_named(
        &selections.chain_forms,
        DECLARED_CHAIN_FORMS_SELECTIONS,
        |member| &member.concrete,
    ) {
        Some(member) => AdmittedChainForms {
            concrete: member.concrete,
            identifier: member.identifier,
        },
        None => {
            return Err(AdmitSelectionsErrorReturn::ChainForms {
                text: selections.chain_forms.clone(),
            });
        }
    };

    let verifier = match member_named(&selections.verifier, VERIFIER_CONCRETES, |member| member) {
        Some(member) => *member,
        None => {
            return Err(AdmitSelectionsErrorReturn::Verifier {
                text: selections.verifier.clone(),
            });
        }
    };

    let scope = match member_named(
        &selections.identity_scope,
        DECLARED_KEM_SELECTIONS,
        |member| &member.scope,
    ) {
        Some(member) => member.scope,
        None => {
            return Err(AdmitSelectionsErrorReturn::IdentityScope {
                text: selections.identity_scope.clone(),
            });
        }
    };

    let algebra = match member_named(
        &selections.envelope_algebra,
        DECLARED_KEY_AGREEMENT_SELECTIONS,
        |member| &member.algebra,
    ) {
        Some(member) => member.algebra,
        None => {
            return Err(AdmitSelectionsErrorReturn::EnvelopeAlgebra {
                text: selections.envelope_algebra.clone(),
            });
        }
    };

    let kem = match DECLARED_KEM_SELECTIONS
        .iter()
        .find(|member| format!("{:?}", member.concrete) == selections.kem && member.scope == scope)
    {
        Some(member) => AdmittedKem {
            concrete: member.concrete,
            identifier: member.identifier,
            scope: member.scope,
        },
        None => {
            return Err(AdmitSelectionsErrorReturn::Kem {
                text: selections.kem.clone(),
            });
        }
    };

    let key_agreement = match DECLARED_KEY_AGREEMENT_SELECTIONS.iter().find(|member| {
        format!("{:?}", member.concrete) == selections.key_agreement && member.algebra == algebra
    }) {
        Some(member) => AdmittedKeyAgreement {
            concrete: member.concrete,
            identifier: member.identifier,
            algebra: member.algebra,
        },
        None => {
            return Err(AdmitSelectionsErrorReturn::KeyAgreement {
                text: selections.key_agreement.clone(),
            });
        }
    };

    let delivery_proof = match member_named(
        &selections.delivery_proof,
        DELIVERY_PROOF_CONCRETES,
        |member| member,
    ) {
        Some(member) => AdmittedDeliveryProof {
            concrete: *member,
            algebra,
            statement_version: selections.delivery_statement_version,
        },
        None => {
            return Err(AdmitSelectionsErrorReturn::DeliveryProof {
                text: selections.delivery_proof.clone(),
            });
        }
    };

    let mut curves = Vec::with_capacity(selections.curves.len());
    for (index, curve) in selections.curves.iter().enumerate() {
        let concrete =
            match member_named(&curve.pairing_concrete, PAIRING_CONCRETES, |member| member) {
                Some(member) => *member,
                None => {
                    return Err(AdmitSelectionsErrorReturn::Curve {
                        index,
                        field: PairingField::PairingConcrete,
                        text: curve.pairing_concrete.clone(),
                    });
                }
            };
        let target_group_encoding = match member_named(
            &curve.target_group_encoding,
            TARGET_GROUP_ENCODING_IDENTIFIERS,
            |member| member,
        ) {
            Some(member) => *member,
            None => {
                return Err(AdmitSelectionsErrorReturn::Curve {
                    index,
                    field: PairingField::TargetGroupEncoding,
                    text: curve.target_group_encoding.clone(),
                });
            }
        };
        curves.push(AdmittedCurve {
            label: curve.label.clone(),
            pairing: AdmittedPairing {
                concrete,
                target_group_encoding,
            },
        });
    }

    let mut benchmark_concretes = Vec::with_capacity(selections.benchmark_concretes.len());
    for (index, benchmark) in selections.benchmark_concretes.iter().enumerate() {
        let concrete =
            match member_named(&benchmark.pairing_concrete, PAIRING_CONCRETES, |member| {
                member
            }) {
                Some(member) => *member,
                None => {
                    return Err(AdmitSelectionsErrorReturn::BenchmarkConcrete {
                        index,
                        field: PairingField::PairingConcrete,
                        text: benchmark.pairing_concrete.clone(),
                    });
                }
            };
        let target_group_encoding = match member_named(
            &benchmark.target_group_encoding,
            TARGET_GROUP_ENCODING_IDENTIFIERS,
            |member| member,
        ) {
            Some(member) => *member,
            None => {
                return Err(AdmitSelectionsErrorReturn::BenchmarkConcrete {
                    index,
                    field: PairingField::TargetGroupEncoding,
                    text: benchmark.target_group_encoding.clone(),
                });
            }
        };
        benchmark_concretes.push(AdmittedPairing {
            concrete,
            target_group_encoding,
        });
    }

    let mut precompile_encodings = Vec::with_capacity(payload.precompile_encodings.len());
    for (index, text) in payload.precompile_encodings.iter().enumerate() {
        precompile_encodings.push(
            match member_named(text, PRECOMPILE_ENCODINGS, |member| member) {
                Some(member) => *member,
                None => {
                    return Err(AdmitSelectionsErrorReturn::PrecompileEncoding {
                        index,
                        text: text.clone(),
                    });
                }
            },
        );
    }

    Ok(AdmitSelectionsSuccessReturn {
        admitted: AdmittedSelections {
            random_source,
            encoding,
            hash_to_scalar,
            chain_forms,
            kem,
            key_agreement,
            delivery_proof,
            verifier,
            curves,
            benchmark_concretes,
            precompile_encodings,
        },
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
