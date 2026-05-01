//! Different kinds of numbers.

use std::cmp::{Eq, PartialEq};
use std::ops::{Add, Div, Mul, Neg, Sub};
use std::ops::{AddAssign, DivAssign, MulAssign, SubAssign};

use crate::give_const;

pub mod complex;

/// A trait for types with one additive identity.
///
/// All stable floats, signed integers, and unsigned integers implement
/// `AddIdentity` with `0` or `0.0` as the additive identity.
pub trait AddIdentity {
    /// The zero of this type. This should be the additive identity.
    const ZERO: Self;
}

/// A trait for types with one multiplicative identity.
///
/// All stable floats, signed integers, and unsigned integers implement
/// `MulIdentity` with `1` or `1.0` as the multiplicative identity.
pub trait MulIdentity {
    /// The one of this type. This should be the multiplicative identity.
    const ONE: Self;
}

give_const!(f32, AddIdentity, ZERO, 0.0);
give_const!(f64, AddIdentity, ZERO, 0.0);

give_const!(i8, AddIdentity, ZERO, 0);
give_const!(i16, AddIdentity, ZERO, 0);
give_const!(i32, AddIdentity, ZERO, 0);
give_const!(i64, AddIdentity, ZERO, 0);
give_const!(i128, AddIdentity, ZERO, 0);
give_const!(isize, AddIdentity, ZERO, 0);

give_const!(u8, AddIdentity, ZERO, 0);
give_const!(u16, AddIdentity, ZERO, 0);
give_const!(u32, AddIdentity, ZERO, 0);
give_const!(u64, AddIdentity, ZERO, 0);
give_const!(u128, AddIdentity, ZERO, 0);
give_const!(usize, AddIdentity, ZERO, 0);

give_const!(f32, MulIdentity, ONE, 1.0);
give_const!(f64, MulIdentity, ONE, 1.0);

give_const!(i8, MulIdentity, ONE, 1);
give_const!(i16, MulIdentity, ONE, 1);
give_const!(i32, MulIdentity, ONE, 1);
give_const!(i64, MulIdentity, ONE, 1);
give_const!(i128, MulIdentity, ONE, 1);
give_const!(isize, MulIdentity, ONE, 1);

give_const!(u8, MulIdentity, ONE, 1);
give_const!(u16, MulIdentity, ONE, 1);
give_const!(u32, MulIdentity, ONE, 1);
give_const!(u64, MulIdentity, ONE, 1);
give_const!(u128, MulIdentity, ONE, 1);
give_const!(usize, MulIdentity, ONE, 1);

/// A trait for doing basic arithmetic on unsigned numbers.
///
/// All stable floats, signed integers, and unsigned integers implement
/// `UArithmetic`.
pub trait UArithmetic:
    Sized
    + Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
{
}

impl UArithmetic for f64 {}
impl UArithmetic for f32 {}

impl UArithmetic for i8 {}
impl UArithmetic for i16 {}
impl UArithmetic for i32 {}
impl UArithmetic for i64 {}
impl UArithmetic for i128 {}
impl UArithmetic for isize {}

impl UArithmetic for u8 {}
impl UArithmetic for u16 {}
impl UArithmetic for u32 {}
impl UArithmetic for u64 {}
impl UArithmetic for u128 {}
impl UArithmetic for usize {}

/// A trait for doing basic arithmetic on signed numbers.
///
/// Note that this is the same as `UArithmetic` but with the added
/// constraint that the type must be able to be negated.
///
/// All stable floats and signed integers implement `Arithmetic`.
pub trait Arithmetic: UArithmetic + Neg<Output = Self> {}

impl Arithmetic for f64 {}
impl Arithmetic for f32 {}

impl Arithmetic for i8 {}
impl Arithmetic for i16 {}
impl Arithmetic for i32 {}
impl Arithmetic for i64 {}
impl Arithmetic for i128 {}
impl Arithmetic for isize {}

/// A trait for types that form a ring under `+` and `*`.
///
/// All ring axioms must be upheld:
/// - Addition is associative: `(a + b) + c == a + (b + c)`
/// - Addition is commutative: `a + b == b + a`
/// - There is an additive identity element `N::ZERO`:
///     `n + N::ZERO == N::ZERO + n == n`
/// - All `n` in `N` have an additive inverse `n.neg()`:
///     `n + n.neg() == n.neg() + n == N::ZERO`
/// - Multiplication is associative: `(a * b) * c == a * (b * c)`
/// - There is a multiplicative identity element `N::ONE`:
///     `n * N::ONE == N::ONE * n == n`
/// - Multiplication is left- and right- distributive over addition:
///     `a * (b + c) == a * b + a * c` and `(b + c) * a == b * a + c * a`
///
/// Some notable consequences of being a ring are:
/// - `N::ZERO * n == n * N::ZERO == N::ZERO`
/// - `N::ONE.neg() * n == n * N::ONE.neg() == n.neg()`
/// - `n.neg().neg() == n`
/// - `N::ZERO.neg() == N::ZERO`
pub trait Ring:
    Sized
    + Add<Output = Self>
    + Mul<Output = Self>
    + Neg<Output = Self>
    + AddIdentity
    + MulIdentity
{
}

/// A trait for types that form a field under `+`, `-`, `*`, and `/`.
///
/// All field axioms must be upheld:
/// - Addition and multiplication are associative:
///     `a + (b + c) == (a + b) + c` and `a * (b * c) == (a * b) * c`
/// - Addition and multiplication are commutative:
///     `a + b == b + a` and `a * b == b * a`
/// - There is an additive identity element `N::ZERO`:
///     `n + N::ZERO == N::ZERO + n == n`
/// - There is a multiplicative identity element `N::ONE`:
///     `n * N::ONE == N::ONE * n == n`
/// - The additive identity is not the multiplicative identity:
///     `N::ZERO != N::ONE`
/// - All `n` in `N` have an additive inverse `n.neg()`:
///     `n + n.neg() == n.neg() + n == N::ZERO`
/// - All `n` in `N` except for `N::ZERO` have a multiplicative inverse
///     `n.reciprocal()`: `n * n.reciprocal() == n.reciprocal() * n == N::ONE`
/// - Multiplication is left- and right- distributive over addition:
///     `a * (b + c) == a * b + a * c` and `(b + c) * a == b * a + c * a`
/// 
/// Some notable consequences of being a field are:
/// - `N::ZERO * n == n * N::ZERO == N::ZERO`
/// - `N::ONE.neg() * n == n * N::ONE.neg() == n.neg()`
/// - `n.neg().neg() == n`
/// - `N::ZERO.neg() == N::ZERO`
/// - `N::ONE.reciprocal() == N::ONE`
/// - `n.reciprocal().reciprocal() == n` if `n != N::ZERO`
pub trait Field:
    Ring + Sub<Output = Self> + Div<Output = Self> + PartialEq
{
    /// Returns the reciprocal or the multiplicative identity, or `None`
    /// if `self == Self::ZERO`, as there is no multiplicative inverse.
    /// 
    /// A field may not have a multiplicative inverse for the zero element,
    /// as it would imply the zero and one elements are identical, which breaks
    /// the field axioms:
    /// 
    /// Let `0` be `N::ZERO` and `1` be `N::ONE` \
    /// Let `x` be the multiplicative inverse of `0`, meaning `x * 0 = 1`
    /// ```fix
    ///               x * 0 = 1
    ///      x * 0 + x + -x = 1
    ///  x * 0 + x * 1 + -x = 1
    ///    x * (0 + 1) + -x = 1
    ///          x * 1 + -x = 1
    ///              x + -x = 1
    ///                   0 = 1 // violates axiom stating `N::ZERO != N::ONE`
    /// ```
    fn reciprocal(self) -> Option<Self> {
        if self == Self::ZERO {
            None
        } else {
            Some(Self::ONE / self)
        }
    }
}