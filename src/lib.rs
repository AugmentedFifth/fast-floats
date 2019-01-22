//! Experimental (unstable) “fast-math” wrappers for f32, f64
//!
//! These wrappers enable the [“fast-math”][1] flags for the operations
//! where there are intrinsics for this (add, sub, mul, div, rem).
//! The wrappers exist so that we have a quick & easy way **to experiment**
//! with fast math flags and further that feature in Rust.
//!
//! Note that as of this writing, the Rust instrinsics use the “fast” flag
//! documented in the langref; this enables all the float flags.
//!
//! [1]: http://llvm.org/docs/LangRef.html#fast-math-flags
//!
//! # Rust Version
//!
//! This crate is nightly only and experimental. Breaking changes can occur at
//! any time, if changes in Rust require it.
#![no_std]
#![feature(core_intrinsics)]

#[cfg(feature = "num-traits")]
extern crate num_traits;

#[cfg(feature = "num-traits")]
use num_traits::Zero;

extern crate core as std;

use std::intrinsics::{self, fadd_fast, fsub_fast, fmul_fast, fdiv_fast, frem_fast};
use std::ops::{
    Add,
    Sub,
    Mul,
    Div,
    Rem,
    AddAssign,
    SubAssign,
    MulAssign,
    DivAssign,
    RemAssign,
    Neg,
};

/// “fast-math” wrapper for f32 and f64.
///
/// The `Fast` type enforces no invariant and can hold any f32, f64 values.
/// See crate docs for more details.
#[derive(Copy, Clone, PartialEq, PartialOrd, Default)]
pub struct Fast<F>(pub F);

impl<F> Fast<F> {
    /// Get the inner value
    #[inline(always)]
    pub fn get(self) -> F { self.0 }
}

impl<F> From<F> for Fast<F> {
    #[inline(always)]
    fn from(x: F) -> Self { Fast(x) }
}

// for demonstration purposes
#[cfg(test)]
pub fn fast_sum(xs: &[f64]) -> f64 {
    xs.iter().map(|&x| Fast(x)).fold(Fast(0.), |acc, x| acc + x).get()
}

// for demonstration purposes
#[cfg(test)]
pub fn fast_dot(xs: &[f64], ys: &[f64]) -> f64 {
    xs.iter().zip(ys).fold(Fast(0.), |acc, (&x, &y)| acc + Fast(x) * Fast(y)).get()
}

#[cfg(test)]
pub fn regular_sum(xs: &[f64]) -> f64 {
    xs.iter().map(|&x| x).fold(0., |acc, x| acc + x)
}

macro_rules! impl_op {
    ($($name:ident, $method:ident, $intrins:ident;)*) => {
        $(
        // Fast<F> + F
        impl $name<f64> for Fast<f64> {
            type Output = Self;
            #[inline(always)]
            fn $method(self, rhs: f64) -> Self::Output {
                unsafe {
                    Fast($intrins(self.0, rhs))
                }
            }
        }

        impl $name<f32> for Fast<f32> {
            type Output = Self;
            #[inline(always)]
            fn $method(self, rhs: f32) -> Self::Output {
                unsafe {
                    Fast($intrins(self.0, rhs))
                }
            }
        }

        // F + Fast<F>
        impl $name<Fast<f64>> for f64 {
            type Output = Fast<f64>;
            #[inline(always)]
            fn $method(self, rhs: Fast<f64>) -> Self::Output {
                Fast(self).$method(rhs.0)
            }
        }

        impl $name<Fast<f32>> for f32 {
            type Output = Fast<f32>;
            #[inline(always)]
            fn $method(self, rhs: Fast<f32>) -> Self::Output {
                Fast(self).$method(rhs.0)
            }
        }

        // Fast<F> + Fast<F>
        impl $name for Fast<f64> {
            type Output = Self;
            #[inline(always)]
            fn $method(self, rhs: Self) -> Self::Output {
                self.$method(rhs.0)
            }
        }

        impl $name for Fast<f32> {
            type Output = Self;
            #[inline(always)]
            fn $method(self, rhs: Self) -> Self::Output {
                self.$method(rhs.0)
            }
        }
        )*

    }
}

macro_rules! impl_assignop {
    ($($name:ident, $method:ident, $intrins:ident;)*) => {
        $(
        impl<F, Rhs> $name<Rhs> for Fast<F>
            where Self: Add<Rhs, Output=Self> + Copy,
        {
            #[inline(always)]
            fn $method(&mut self, rhs: Rhs) {
                *self = *self + rhs
            }
        }
        )*

    }
}

impl_op! {
    Add, add, fadd_fast;
    Sub, sub, fsub_fast;
    Mul, mul, fmul_fast;
    Div, div, fdiv_fast;
    Rem, rem, frem_fast;
}

impl_assignop! {
    AddAssign, add_assign, fadd_fast;
    SubAssign, sub_assign, fsub_fast;
    MulAssign, mul_assign, fmul_fast;
    DivAssign, div_assign, fdiv_fast;
    RemAssign, rem_assign, frem_fast;
}

impl Neg for Fast<f64> {
    type Output = Self;

    #[inline(always)]
    fn neg(self) -> Self {
        Fast(-self.0)
    }
}
impl Neg for Fast<f32> {
    type Output = Self;

    #[inline(always)]
    fn neg(self) -> Self {
        Fast(-self.0)
    }
}

impl Fast<f32> {
    #[inline(always)]
    pub fn floor(self) -> Self {
        Self(unsafe { intrinsics::floorf32(self.0) })
    }

    #[inline(always)]
    pub fn ceil(self) -> Self {
        Self(unsafe { intrinsics::ceilf32(self.0) })
    }

    #[inline(always)]
    pub fn round(self) -> Self {
        Self(unsafe { intrinsics::roundf32(self.0) })
    }

    #[inline(always)]
    pub fn trunc(self) -> Self {
        Self(unsafe { intrinsics::truncf32(self.0) })
    }

    #[inline(always)]
    pub fn fract(self) -> Self {
        self - self.trunc()
    }

    #[inline(always)]
    pub fn abs(self) -> Self {
        Self(unsafe { intrinsics::fabsf32(self.0) })
    }

    #[inline(always)]
    pub fn is_nan(self) -> bool {
        self.0.is_nan()
    }

    #[inline(always)]
    pub fn signum(self) -> Self {
        if self.is_nan() {
            Self(std::f32::NAN)
        } else {
            Self(unsafe { intrinsics::copysignf32(1.0, self.0) })
        }
    }

    #[inline(always)]
    pub fn copysign(self, y: Self) -> Self {
        Self(unsafe { intrinsics::copysignf32(self.0, y.0) })
    }

    #[inline(always)]
    pub fn mul_add(self, a: Self, b: Self) -> Self {
        Self(unsafe { intrinsics::fmaf32(self.0, a.0, b.0) })
    }

    #[inline(always)]
    pub fn powi(self, n: i32) -> Self {
        Self(unsafe { intrinsics::powif32(self.0, n) })
    }

    #[inline(always)]
    pub fn powf(self, n: Self) -> Self {
        Self(unsafe { intrinsics::powf32(self.0, n.0) })
    }

    #[inline(always)]
    pub fn sqrt(self) -> Self {
        Self(unsafe { intrinsics::sqrtf32(self.0) })
    }

    #[inline(always)]
    pub fn exp(self) -> Self {
        Self(unsafe { intrinsics::expf32(self.0) })
    }

    #[inline(always)]
    pub fn exp2(self) -> Self {
        Self(unsafe { intrinsics::exp2f32(self.0) })
    }

    #[inline(always)]
    pub fn ln(self) -> Self {
        Self(unsafe { intrinsics::logf32(self.0) })
    }

    #[inline(always)]
    pub fn log(self, base: Self) -> Self {
        self.ln() / base.ln()
    }

    #[inline(always)]
    pub fn log2(self) -> Self {
        Self(unsafe { intrinsics::log2f32(self.0) })
    }

    #[inline(always)]
    pub fn log10(self) -> Self {
        Self(unsafe { intrinsics::log10f32(self.0) })
    }

    #[inline(always)]
    pub fn sin(self) -> Self {
        Self(unsafe { intrinsics::sinf32(self.0) })
    }

    #[inline(always)]
    pub fn cos(self) -> Self {
        Self(unsafe { intrinsics::cosf32(self.0) })
    }

    #[inline(always)]
    pub fn sin_cos(self) -> (Self, Self) {
        (self.sin(), self.cos())
    }

    #[inline]
    pub fn asinh(self) -> Self {
        if self.0 == std::f32::NEG_INFINITY {
            self
        } else {
            (self + ((self * self) + 1.0).sqrt()).ln()
        }
    }

    #[inline]
    pub fn acosh(self) -> Self {
        match self {
            x if x < 1.0.into() => std::f32::NAN.into(),
            x => (x + ((x * x) - 1.0).sqrt()).ln(),
        }
    }
}
impl Fast<f64> {
    #[inline(always)]
    pub fn floor(self) -> Self {
        Self(unsafe { intrinsics::floorf64(self.0) })
    }

    #[inline(always)]
    pub fn ceil(self) -> Self {
        Self(unsafe { intrinsics::ceilf64(self.0) })
    }

    #[inline(always)]
    pub fn round(self) -> Self {
        Self(unsafe { intrinsics::roundf64(self.0) })
    }

    #[inline(always)]
    pub fn trunc(self) -> Self {
        Self(unsafe { intrinsics::truncf64(self.0) })
    }

    #[inline(always)]
    pub fn fract(self) -> Self {
        self - self.trunc()
    }

    #[inline(always)]
    pub fn abs(self) -> Self {
        Self(unsafe { intrinsics::fabsf64(self.0) })
    }

    #[inline(always)]
    pub fn is_nan(self) -> bool {
        self.0.is_nan()
    }

    #[inline(always)]
    pub fn signum(self) -> Self {
        if self.is_nan() {
            Self(std::f64::NAN)
        } else {
            Self(unsafe { intrinsics::copysignf64(1.0, self.0) })
        }
    }

    #[inline(always)]
    pub fn copysign(self, y: Self) -> Self {
        Self(unsafe { intrinsics::copysignf64(self.0, y.0) })
    }

    #[inline(always)]
    pub fn mul_add(self, a: Self, b: Self) -> Self {
        Self(unsafe { intrinsics::fmaf64(self.0, a.0, b.0) })
    }

    #[inline(always)]
    pub fn powi(self, n: i32) -> Self {
        Self(unsafe { intrinsics::powif64(self.0, n) })
    }

    #[inline(always)]
    pub fn powf(self, n: Self) -> Self {
        Self(unsafe { intrinsics::powf64(self.0, n.0) })
    }

    #[inline(always)]
    pub fn sqrt(self) -> Self {
        Self(unsafe { intrinsics::sqrtf64(self.0) })
    }

    #[inline(always)]
    pub fn exp(self) -> Self {
        Self(unsafe { intrinsics::expf64(self.0) })
    }

    #[inline(always)]
    pub fn exp2(self) -> Self {
        Self(unsafe { intrinsics::exp2f64(self.0) })
    }

    #[inline(always)]
    pub fn ln(self) -> Self {
        Self(unsafe { intrinsics::logf64(self.0) })
    }

    #[inline(always)]
    pub fn log(self, base: Self) -> Self {
        self.ln() / base.ln()
    }

    #[inline(always)]
    pub fn log2(self) -> Self {
        Self(unsafe { intrinsics::log2f64(self.0) })
    }

    #[inline(always)]
    pub fn log10(self) -> Self {
        Self(unsafe { intrinsics::log10f64(self.0) })
    }

    #[inline(always)]
    pub fn sin(self) -> Self {
        Self(unsafe { intrinsics::sinf64(self.0) })
    }

    #[inline(always)]
    pub fn cos(self) -> Self {
        Self(unsafe { intrinsics::cosf64(self.0) })
    }

    #[inline(always)]
    pub fn sin_cos(self) -> (Self, Self) {
        (self.sin(), self.cos())
    }

    #[inline]
    pub fn asinh(self) -> Self {
        if self.0 == std::f64::NEG_INFINITY {
            self
        } else {
            (self + ((self * self) + 1.0).sqrt()).ln()
        }
    }

    #[inline]
    pub fn acosh(self) -> Self {
        match self {
            x if x < 1.0.into() => std::f64::NAN.into(),
            x => (x + ((x * x) - 1.0).sqrt()).ln(),
        }
    }
}

/*
impl<Z> Zero for Fast<Z> where Z: Zero {
    fn zero() -> Self { Fast(Z::zero()) }
    fn is_zero(&self) -> bool {
        self.0.is_zero()
    }
}
*/
#[cfg(feature = "num-traits")]
impl Zero for Fast<f64> {
    #[inline(always)]
    fn zero() -> Self { Fast(<_>::zero()) }

    #[inline(always)]
    fn is_zero(&self) -> bool { self.get().is_zero() }
}
#[cfg(feature = "num-traits")]
impl Zero for Fast<f32> {
    #[inline(always)]
    fn zero() -> Self { Fast(<_>::zero()) }

    #[inline(always)]
    fn is_zero(&self) -> bool { self.get().is_zero() }
}

use std::fmt;
macro_rules! impl_format {
    ($($name:ident)+) => {
        $(
        impl<F: fmt::$name> fmt::$name for Fast<F> {
            #[inline(always)]
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                self.0.fmt(f)
            }
        }
        )+
    }
}

impl_format!(Debug Display LowerExp UpperExp);


#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_op {
        ($($op:tt)+) => {
            $(
                assert_eq!(Fast(2.) $op Fast(1.), Fast(2. $op 1.));
            )+
        }
    }

    #[test]
    fn each_op() {
        test_op!(+ - * / %);
    }
}
