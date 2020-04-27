//! Traits for operating on Groups and Elliptic Curves.

use rand_core::RngCore;
use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Display};
use std::marker::PhantomData;

/// Element represents an element of a group with the additive notation
/// which is also equipped with a multiplication transformation.
/// Two implementations are for Scalar which forms a ring so RHS is the same
/// and Point which can be multiplied by a scalar of its prime field.
pub trait Element: Clone + Display + Debug + Eq + Serialize + for<'a> Deserialize<'a> {
    type RHS;

    /// new MUST return the zero element of the group.
    fn new() -> Self;

    fn one() -> Self;

    fn add(&mut self, s2: &Self);

    fn mul(&mut self, mul: &Self::RHS);

    fn rand<R: RngCore>(rng: &mut R) -> Self;

    fn zero() -> Self {
        Self::new()
    }
}

/// Scalar can be multiplied by only a Scalar, no other elements.
// TODO: is that truly enforced by Rust ?
pub trait Scalar: Element {
    fn set_int(&mut self, i: u64);
    fn inverse(&self) -> Option<Self>;
    fn negate(&mut self);
    fn sub(&mut self, other: &Self);
    // TODO
}

/// Basic point functionality that can be multiplied by a scalar
pub trait Point: Element {
    type Error: Debug;

    fn map(&mut self, data: &[u8]) -> Result<(), <Self as Point>::Error>;
}

/// A group holds functionalities to create scalar and points related; it is
/// similar to the Engine definition, just much more simpler.
pub trait Curve: Clone + Debug {
    type Scalar: Scalar<RHS = Self::Scalar>;
    type Point: Point<RHS = Self::Scalar>;

    /// scalar returns the identity element of the field.
    fn scalar() -> Self::Scalar {
        Self::Scalar::new()
    }

    /// point returns the default additive generator of the group.
    fn point() -> Self::Point {
        Self::Point::one()
    }
}

pub trait PairingCurve: Debug {
    type Scalar: Scalar<RHS = Self::Scalar>;
    type G1: Point<RHS = Self::Scalar>;
    type G2: Point<RHS = Self::Scalar>;
    type GT: Element;

    fn pair(a: &Self::G1, b: &Self::G2) -> Self::GT;
}

#[derive(Debug, Clone, PartialEq)]
pub struct CurveFrom<S: Scalar, P: Point> {
    m: PhantomData<S>,
    mm: PhantomData<P>,
}

impl<S, P> Curve for CurveFrom<S, P>
where
    S: Scalar<RHS = S>,
    P: Point<RHS = S>,
{
    type Scalar = S;
    type Point = P;
}

pub type G1Curve<C> = CurveFrom<<C as PairingCurve>::Scalar, <C as PairingCurve>::G1>;
pub type G2Curve<C> = CurveFrom<<C as PairingCurve>::Scalar, <C as PairingCurve>::G2>;