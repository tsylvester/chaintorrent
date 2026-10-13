#![allow(clippy::expect_used)]

use super::interface::{
    Bb1DepthOneCredential, Bb1DepthOneIdentityElement, Bb1DepthOneKem,
    Bb1DepthOneKemConstructorParams, Bb1DepthOneMasterScalar, Bb1DepthOneParameterSet,
    Bb1DepthOneParameterSetScope,
};
use domain::{Secret, SecretConstructorParams, build_secret};
use hash_to_scalar::IHashToScalarAdapter;
use pairing::{
    AddG1Params, AddG1Payload, AddG2Params, AddG2Payload, EncodeG1Params, EncodeG1Payload,
    EncodeG2Params, EncodeG2Payload, EncodeScalarParams, EncodeScalarPayload, G1GeneratorParams,
    G1GeneratorPayload, G2GeneratorParams, G2GeneratorPayload, IPairingAdapter, IPairingArithmetic,
    ISampleUniformScalar, MulG1Params, MulG1Payload, MulG2Params, MulG2Payload,
    SampleUniformScalarParams, SampleUniformScalarPayload,
};
use serde_json::{Map, Value};

fn uniform_bytes<S: ISampleUniformScalar>(value: u8) -> Secret<Vec<u8>> {
    let mut uniform = vec![0u8; S::UNIFORM_BYTES_LENGTH];
    uniform[S::UNIFORM_BYTES_LENGTH - 1] = value;
    build_secret(uniform, Default::default())
}

fn default_scalar_secret<S: ISampleUniformScalar>(value: u8) -> Secret<S> {
    S::sample_from_uniform_bytes(
        SampleUniformScalarParams,
        SampleUniformScalarPayload {
            uniform: uniform_bytes::<S>(value),
        },
    )
    .expect("sampling a default scalar always succeeds")
    .scalar
}

fn default_scalar<P: IPairingAdapter>(value: u8) -> P::Scalar {
    default_scalar_secret::<P::Scalar>(value).expose().clone()
}

fn default_alpha<P: IPairingAdapter>() -> Secret<P::Scalar> {
    let Ok(value) = Secret::try_new(SecretConstructorParams {
        value: default_scalar::<P>(2),
    });
    value
}

fn generator_g1<P: IPairingAdapter>(pairing: &P) -> P::G1 {
    let Ok(success) = pairing.g1_generator(G1GeneratorParams, G1GeneratorPayload);
    success.point
}

fn generator_g2<P: IPairingAdapter>(pairing: &P) -> P::G2 {
    let Ok(success) = pairing.g2_generator(G2GeneratorParams, G2GeneratorPayload);
    success.point
}

fn sum_g1<P: IPairingAdapter>(pairing: &P, left: P::G1, right: P::G1) -> P::G1 {
    let Ok(success) = pairing.add_g1(AddG1Params, AddG1Payload { left, right });
    success.sum
}

fn sum_g2<P: IPairingAdapter>(pairing: &P, left: P::G2, right: P::G2) -> P::G2 {
    let Ok(success) = pairing.add_g2(AddG2Params, AddG2Payload { left, right });
    success.sum
}

fn product_g1<P: IPairingAdapter>(pairing: &P, point: P::G1, scalar: P::Scalar) -> P::G1 {
    let Ok(success) = pairing.mul_g1(MulG1Params, MulG1Payload { point, scalar });
    success.product
}

fn product_g2<P: IPairingAdapter>(pairing: &P, point: P::G2, scalar: P::Scalar) -> P::G2 {
    let Ok(success) = pairing.mul_g2(MulG2Params, MulG2Payload { point, scalar });
    success.product
}

fn default_u0<P: IPairingAdapter>(pairing: &P) -> P::G1 {
    let g1 = generator_g1(pairing);
    sum_g1(pairing, g1.clone(), g1)
}

fn default_u1<P: IPairingAdapter>(pairing: &P) -> P::G1 {
    sum_g1(pairing, default_u0(pairing), generator_g1(pairing))
}

fn default_identity_element<P: IPairingAdapter>(pairing: &P) -> P::G1 {
    let i = default_scalar::<P>(1);
    sum_g1(
        pairing,
        default_u0(pairing),
        product_g1(pairing, default_u1(pairing), i),
    )
}

fn byte_array(bytes: &[u8]) -> Value {
    Value::Array(bytes.iter().map(|byte| Value::from(*byte)).collect())
}

fn encoded_g1_value<P: IPairingAdapter>(pairing: &P, point: &P::G1) -> Value {
    let Ok(encoded) = pairing.encode_g1(
        EncodeG1Params,
        EncodeG1Payload {
            point: point.clone(),
        },
    );
    byte_array(encoded.bytes.as_ref())
}

fn encoded_g2_value<P: IPairingAdapter>(pairing: &P, point: &P::G2) -> Value {
    let Ok(encoded) = pairing.encode_g2(
        EncodeG2Params,
        EncodeG2Payload {
            point: point.clone(),
        },
    );
    byte_array(encoded.bytes.as_ref())
}

fn encoded_scalar_value<P: IPairingAdapter>(pairing: &P, scalar: &P::Scalar) -> Value {
    let Ok(encoded) = pairing.encode_scalar(
        EncodeScalarParams,
        EncodeScalarPayload {
            scalar: scalar.clone(),
        },
    );
    byte_array(encoded.bytes.expose().as_ref())
}

fn scope_value<G1>(scope: &Bb1DepthOneParameterSetScope<G1>) -> Value {
    Value::from(match scope {
        Bb1DepthOneParameterSetScope::Entitlement => "Entitlement",
        Bb1DepthOneParameterSetScope::Asset { .. } => "Asset",
    })
}

#[derive(Default)]
pub struct Bb1DepthOneKemConstructorParamsOverrides;

pub fn build_bb1_depth_one_kem_constructor_params<'a, P: IPairingArithmetic>(
    pairing: &'a P,
    hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
    _overrides: Bb1DepthOneKemConstructorParamsOverrides,
) -> Bb1DepthOneKemConstructorParams<'a, P> {
    Bb1DepthOneKemConstructorParams {
        pairing,
        hash_to_scalar,
    }
}

pub fn build_bb1_depth_one_kem<'a, P: IPairingArithmetic>(
    pairing: &'a P,
    hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
    overrides: Bb1DepthOneKemConstructorParamsOverrides,
) -> Bb1DepthOneKem<'a, P> {
    Bb1DepthOneKem::try_new(build_bb1_depth_one_kem_constructor_params(
        pairing,
        hash_to_scalar,
        overrides,
    ))
    .expect("built params are valid")
}

pub struct Bb1DepthOneParameterSetOverrides<P: IPairingAdapter> {
    pub g1: Option<P::G1>,
    pub u0: Option<P::G1>,
    pub u1: Option<P::G1>,
    pub g2: Option<P::G2>,
    pub hpub: Option<P::G2>,
    pub scope: Option<Bb1DepthOneParameterSetScope<P::G1>>,
}

impl<P: IPairingAdapter> Default for Bb1DepthOneParameterSetOverrides<P> {
    fn default() -> Self {
        Self {
            g1: None,
            u0: None,
            u1: None,
            g2: None,
            hpub: None,
            scope: None,
        }
    }
}

pub fn build_bb1_depth_one_parameter_set<P: IPairingAdapter>(
    pairing: &P,
    overrides: Bb1DepthOneParameterSetOverrides<P>,
) -> Bb1DepthOneParameterSet<P> {
    let g1 = overrides.g1.unwrap_or_else(|| generator_g1(pairing));
    let g2 = overrides.g2.unwrap_or_else(|| generator_g2(pairing));
    let u0 = overrides.u0.unwrap_or_else(|| default_u0(pairing));
    let u1 = overrides.u1.unwrap_or_else(|| default_u1(pairing));
    let hpub = overrides
        .hpub
        .unwrap_or_else(|| sum_g2(pairing, g2.clone(), g2.clone()));
    Bb1DepthOneParameterSet {
        g1,
        u0,
        u1,
        g2,
        hpub,
        scope: overrides
            .scope
            .unwrap_or(Bb1DepthOneParameterSetScope::Entitlement),
    }
}

#[derive(Default)]
pub struct Bb1DepthOneParameterSetCorruptions {
    pub g1: Option<Value>,
    pub u0: Option<Value>,
    pub u1: Option<Value>,
    pub g2: Option<Value>,
    pub hpub: Option<Value>,
    pub scope: Option<Value>,
}

pub fn invalidate_bb1_depth_one_parameter_set<P: IPairingAdapter>(
    pairing: &P,
    corruptions: Bb1DepthOneParameterSetCorruptions,
) -> Value {
    let parameter_set = build_bb1_depth_one_parameter_set(pairing, Default::default());
    let mut fields = Map::new();
    fields.insert(
        "g1".to_string(),
        encoded_g1_value(pairing, &parameter_set.g1),
    );
    fields.insert(
        "u0".to_string(),
        encoded_g1_value(pairing, &parameter_set.u0),
    );
    fields.insert(
        "u1".to_string(),
        encoded_g1_value(pairing, &parameter_set.u1),
    );
    fields.insert(
        "g2".to_string(),
        encoded_g2_value(pairing, &parameter_set.g2),
    );
    fields.insert(
        "hpub".to_string(),
        encoded_g2_value(pairing, &parameter_set.hpub),
    );
    fields.insert("scope".to_string(), scope_value(&parameter_set.scope));
    if let Some(g1) = corruptions.g1 {
        fields.insert("g1".to_string(), g1);
    }
    if let Some(u0) = corruptions.u0 {
        fields.insert("u0".to_string(), u0);
    }
    if let Some(u1) = corruptions.u1 {
        fields.insert("u1".to_string(), u1);
    }
    if let Some(g2) = corruptions.g2 {
        fields.insert("g2".to_string(), g2);
    }
    if let Some(hpub) = corruptions.hpub {
        fields.insert("hpub".to_string(), hpub);
    }
    if let Some(scope) = corruptions.scope {
        fields.insert("scope".to_string(), scope);
    }
    Value::Object(fields)
}

pub struct Bb1DepthOneMasterScalarOverrides<P: IPairingAdapter> {
    pub value: Option<Secret<P::Scalar>>,
}

impl<P: IPairingAdapter> Default for Bb1DepthOneMasterScalarOverrides<P> {
    fn default() -> Self {
        Self { value: None }
    }
}

pub fn build_bb1_depth_one_master_scalar<P: IPairingAdapter>(
    overrides: Bb1DepthOneMasterScalarOverrides<P>,
) -> Bb1DepthOneMasterScalar<P> {
    Bb1DepthOneMasterScalar {
        value: overrides.value.unwrap_or_else(|| default_alpha::<P>()),
    }
}

#[derive(Default)]
pub struct Bb1DepthOneMasterScalarCorruptions {
    pub value: Option<Value>,
}

pub fn invalidate_bb1_depth_one_master_scalar<P: IPairingAdapter>(
    pairing: &P,
    corruptions: Bb1DepthOneMasterScalarCorruptions,
) -> Value {
    let master_scalar = build_bb1_depth_one_master_scalar::<P>(Default::default());
    let mut fields = Map::new();
    fields.insert(
        "value".to_string(),
        encoded_scalar_value(pairing, master_scalar.value.expose()),
    );
    if let Some(value) = corruptions.value {
        fields.insert("value".to_string(), value);
    }
    Value::Object(fields)
}

pub struct Bb1DepthOneIdentityElementOverrides<P: IPairingAdapter> {
    pub scalar: Option<P::Scalar>,
    pub element: Option<P::G1>,
}

impl<P: IPairingAdapter> Default for Bb1DepthOneIdentityElementOverrides<P> {
    fn default() -> Self {
        Self {
            scalar: None,
            element: None,
        }
    }
}

pub fn build_bb1_depth_one_identity_element<P: IPairingAdapter>(
    pairing: &P,
    overrides: Bb1DepthOneIdentityElementOverrides<P>,
) -> Bb1DepthOneIdentityElement<P> {
    Bb1DepthOneIdentityElement {
        scalar: overrides.scalar.unwrap_or_else(|| default_scalar::<P>(1)),
        element: overrides
            .element
            .unwrap_or_else(|| default_identity_element(pairing)),
    }
}

#[derive(Default)]
pub struct Bb1DepthOneIdentityElementCorruptions {
    pub scalar: Option<Value>,
    pub element: Option<Value>,
}

pub fn invalidate_bb1_depth_one_identity_element<P: IPairingAdapter>(
    pairing: &P,
    corruptions: Bb1DepthOneIdentityElementCorruptions,
) -> Value {
    let identity_element = build_bb1_depth_one_identity_element(pairing, Default::default());
    let mut fields = Map::new();
    fields.insert(
        "scalar".to_string(),
        encoded_scalar_value(pairing, &identity_element.scalar),
    );
    fields.insert(
        "element".to_string(),
        encoded_g1_value(pairing, &identity_element.element),
    );
    if let Some(scalar) = corruptions.scalar {
        fields.insert("scalar".to_string(), scalar);
    }
    if let Some(element) = corruptions.element {
        fields.insert("element".to_string(), element);
    }
    Value::Object(fields)
}

pub struct Bb1DepthOneCredentialOverrides<P: IPairingAdapter> {
    pub a: Option<P::G1>,
    pub b: Option<P::G2>,
}

impl<P: IPairingAdapter> Default for Bb1DepthOneCredentialOverrides<P> {
    fn default() -> Self {
        Self { a: None, b: None }
    }
}

pub fn build_bb1_depth_one_credential<P: IPairingAdapter>(
    pairing: &P,
    overrides: Bb1DepthOneCredentialOverrides<P>,
) -> Bb1DepthOneCredential<P> {
    Bb1DepthOneCredential {
        a: overrides.a.unwrap_or_else(|| {
            let alpha = default_scalar::<P>(2);
            let r = default_scalar::<P>(1);
            sum_g1(
                pairing,
                product_g1(pairing, generator_g1(pairing), alpha),
                product_g1(pairing, default_identity_element(pairing), r),
            )
        }),
        b: overrides.b.unwrap_or_else(|| {
            let r = default_scalar::<P>(1);
            product_g2(pairing, generator_g2(pairing), r)
        }),
    }
}

#[derive(Default)]
pub struct Bb1DepthOneCredentialCorruptions {
    pub a: Option<Value>,
    pub b: Option<Value>,
}

pub fn invalidate_bb1_depth_one_credential<P: IPairingAdapter>(
    pairing: &P,
    corruptions: Bb1DepthOneCredentialCorruptions,
) -> Value {
    let credential = build_bb1_depth_one_credential(pairing, Default::default());
    let mut fields = Map::new();
    fields.insert("a".to_string(), encoded_g1_value(pairing, &credential.a));
    fields.insert("b".to_string(), encoded_g2_value(pairing, &credential.b));
    if let Some(a) = corruptions.a {
        fields.insert("a".to_string(), a);
    }
    if let Some(b) = corruptions.b {
        fields.insert("b".to_string(), b);
    }
    Value::Object(fields)
}
