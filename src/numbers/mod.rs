//! Different kinds of numbers.

use std::ops::{Add, Div, Mul, Neg, Sub};
use std::ops::{AddAssign, DivAssign, MulAssign, SubAssign};
use std::cmp::{PartialEq, Eq, PartialOrd, Ord};

use crate::give_const;

pub mod complex;

/// A trait for types with an additive identity.
/// 
/// All stable floats, signed integers, and unsigned integers implement
/// `AddIdentity` with `0` or `0.0` as the additive identity.
pub trait AddIdentity {
    /// The zero of this type. This should be the additive identity.
    const ZERO: Self;
}

/// A trait for types with an multiplicative identity.
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
pub trait Arithmetic: UArithmetic + Neg<Output = Self> { }

impl Arithmetic for f64 {}
impl Arithmetic for f32 {}

impl Arithmetic for i8 {}
impl Arithmetic for i16 {}
impl Arithmetic for i32 {}
impl Arithmetic for i64 {}
impl Arithmetic for i128 {}
impl Arithmetic for isize {}