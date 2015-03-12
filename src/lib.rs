// Copyright (c) 2015, Sam Payson
// 
// Permission is hereby granted, free of charge, to any person obtaining a copy of this software and
// associated documentation files (the "Software"), to deal in the Software without restriction,
// including without limitation the rights to use, copy, modify, merge, publish, distribute,
// sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
// 
// The above copyright notice and this permission notice shall be included in all copies or
// substantial portions of the Software.
// 
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT
// NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
// NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM,
// DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

// Need feature(core) for tests, but the compiler complains that we're using the feature
// unnecessarily when *not* building tests >_<.
#![allow(unused_features)]
#![feature(core, std_misc)]

#[macro_export]
macro_rules! __impl_float_type {
    ( $name:ident , $t:ty ) => {
        impl ::std::ops::Neg for $name {
            type Output = $name;

            #[inline]
            fn neg(self) -> $name {
                $name(-self.0)
            }
        }

        impl ::std::ops::Add for $name {
            type Output = $name;

            #[inline]
            fn add(self, rhs: $name) -> $name {
                $name(self.0 + rhs.0)
            }
        }

        impl ::std::ops::Sub for $name {
            type Output = $name;

            #[inline]
            fn sub(self, rhs: $name) -> $name {
                $name(self.0 - rhs.0)
            }
        }

        impl ::std::ops::Mul for $name {
            type Output = $name;

            #[inline]
            fn mul(self, rhs: $name) -> $name {
                $name(self.0 * rhs.0)
            }
        }

        impl ::std::ops::Div for $name {
            type Output = $name;

            #[inline]
            fn div(self, rhs: $name) -> $name {
                $name(self.0 / rhs.0)
            }
        }

        impl ::std::ops::Rem for $name {
            type Output = $name;

            #[inline]
            fn rem(self, rhs: $name) -> $name {
                $name(self.0 % rhs.0)
            }
        }

        impl ::std::num::NumCast for $name {
            #[inline]
            fn from<T: ::std::num::ToPrimitive>(n: T) -> Option<Self> {
                ::std::num::NumCast::from(n).map($name)
            }
        }

        impl ::std::num::ToPrimitive for $name {

            #[inline]
            fn to_i64(&self) -> Option<i64> { self.0.to_i64() }

            #[inline]
            fn to_u64(&self) -> Option<u64> { self.0.to_u64() }

            #[inline]
            fn to_isize(&self) -> Option<isize> { self.0.to_isize() }

            #[inline]
            fn to_i8(&self) -> Option<i8> { self.0.to_i8() }

            #[inline]
            fn to_i16(&self) -> Option<i16> { self.0.to_i16() }

            #[inline]
            fn to_i32(&self) -> Option<i32> { self.0.to_i32() }

            #[inline]
            fn to_usize(&self) -> Option<usize> { self.0.to_usize() }

            #[inline]
            fn to_u8(&self) -> Option<u8> { self.0.to_u8() }

            #[inline]
            fn to_u16(&self) -> Option<u16> { self.0.to_u16() }

            #[inline]
            fn to_u32(&self) -> Option<u32> { self.0.to_u32() }

            #[inline]
            fn to_f32(&self) -> Option<f32> { self.0.to_f32() }

            #[inline]
            fn to_f64(&self) -> Option<f64> { self.0.to_f64() }
        }

        impl ::std::num::Float for $name {

            #[inline]
            fn nan() -> Self { $name(::std::num::Float::nan()) }

            #[inline]
            fn infinity() -> Self { $name(::std::num::Float::infinity()) }

            #[inline]
            fn neg_infinity() -> Self { $name(::std::num::Float::neg_infinity()) }

            #[inline]
            fn zero() -> Self { $name(::std::num::Float::zero()) }

            #[inline]
            fn neg_zero() -> Self { $name(::std::num::Float::neg_zero()) }

            #[inline]
            fn one() -> Self { $name(::std::num::Float::one()) }

            #[inline]
            #[allow(deprecated)]
            fn mantissa_digits(_unused_self: Option<Self>) -> usize {
                ::std::num::Float::mantissa_digits(None::<$t>)
            }

            #[inline]
            #[allow(deprecated)]
            fn digits(_unused_self: Option<Self>) -> usize {
                ::std::num::Float::digits(None::<$t>)
            }

            #[inline]
            #[allow(deprecated)]
            fn epsilon() -> Self { $name(::std::num::Float::epsilon()) }

            #[inline]
            #[allow(deprecated)]
            fn min_exp(_unused_self: Option<Self>) -> isize {
                ::std::num::Float::min_exp(None::<$t>)
            }

            #[inline]
            #[allow(deprecated)]
            fn max_exp(_unused_self: Option<Self>) -> isize {
                ::std::num::Float::max_exp(None::<$t>)
            }

            #[inline]
            #[allow(deprecated)]
            fn min_10_exp(_unused_self: Option<Self>) -> isize {
                ::std::num::Float::min_10_exp(None::<$t>)
            }

            #[inline]
            #[allow(deprecated)]
            fn max_10_exp(_unused_self: Option<Self>) -> isize {
                ::std::num::Float::max_10_exp(None::<$t>)
            }

            #[inline]
            fn min_value() -> Self {
                $name(::std::num::Float::min_value())
            }

            #[inline]
            fn min_pos_value(_unused_self: Option<Self>) -> Self {
                $name(::std::num::Float::min_pos_value(None::<$t>))
            }

            #[inline]
            fn max_value() -> Self {
                $name(::std::num::Float::max_value())
            }

            #[inline]
            fn is_nan(self) -> bool { self.0.is_nan() }

            #[inline]
            fn is_infinite(self) -> bool { self.0.is_infinite() }

            #[inline]
            fn is_finite(self) -> bool { self.0.is_finite() }

            #[inline]
            fn is_normal(self) -> bool { self.0.is_normal() }

            #[inline]
            fn classify(self) -> ::std::num::FpCategory { self.0.classify() }

            #[inline]
            fn integer_decode(self) -> (u64, i16, i8) { self.0.integer_decode() }

            #[inline]
            fn floor(self) -> Self { $name(self.0.floor()) }

            #[inline]
            fn ceil(self) -> Self { $name(self.0.ceil()) }

            #[inline]
            fn round(self) -> Self { $name(self.0.round()) }

            #[inline]
            fn trunc(self) -> Self { $name(self.0.trunc()) }

            #[inline]
            fn fract(self) -> Self { $name(self.0.fract()) }

            #[inline]
            fn abs(self) -> Self { $name(self.0.abs()) }

            #[inline]
            fn signum(self) -> Self { $name(self.0.signum()) }

            #[inline]
            fn is_positive(self) -> bool { self.0.is_positive() }

            #[inline]
            fn is_negative(self) -> bool { self.0.is_negative() }

            #[inline]
            fn mul_add(self, a: Self, b: Self) -> Self { $name(self.0.mul_add(a.0, b.0)) }

            #[inline]
            fn recip(self) -> Self { $name(self.0.recip()) }

            #[inline]
            fn powi(self, n: i32) -> Self { $name(self.0.powi(n)) }

            #[inline]
            fn powf(self, n: Self) -> Self { $name(self.0.powf(n.0)) }

            #[inline]
            fn sqrt(self) -> Self { $name(self.0.sqrt()) }

            #[inline]
            fn rsqrt(self) -> Self { $name(self.0.rsqrt()) }

            #[inline]
            fn exp(self) -> Self { $name(self.0.exp()) }

            #[inline]
            fn exp2(self) -> Self { $name(self.0.exp2()) }

            #[inline]
            fn ln(self) -> Self { $name(self.0.ln()) }

            #[inline]
            fn log(self, base: Self) -> Self { $name(self.0.log(base.0)) }

            #[inline]
            fn log2(self) -> Self { $name(self.0.log2()) }

            #[inline]
            fn log10(self) -> Self { $name(self.0.log10()) }

            #[inline]
            fn to_degrees(self) -> Self { $name(self.0.to_degrees()) }

            #[inline]
            fn to_radians(self) -> Self { $name(self.0.to_radians()) }

            #[inline]
            fn ldexp(x: Self, exp: isize) -> Self { $name(::std::num::Float::ldexp(x.0, exp)) }

            #[inline]
            fn frexp(self) -> (Self, isize) {
                let (fr, exp) = self.0.frexp();
                ($name(fr), exp)
            }

            #[inline]
            fn next_after(self, other: Self) -> Self { $name(self.0.next_after(other.0)) }

            #[inline]
            fn max(self, other: Self) -> Self { $name(self.0.max(other.0)) }

            #[inline]
            fn min(self, other: Self) -> Self { $name(self.0.min(other.0)) }

            #[inline]
            fn abs_sub(self, other: Self) -> Self { $name(self.0.abs_sub(other.0)) }

            #[inline]
            fn cbrt(self) -> Self { $name(self.0.cbrt()) }

            #[inline]
            fn hypot(self, other: Self) -> Self { $name(self.0.hypot(other.0)) }

            #[inline]
            fn sin(self) -> Self { $name(self.0.sin()) }

            #[inline]
            fn cos(self) -> Self { $name(self.0.cos()) }

            #[inline]
            fn tan(self) -> Self { $name(self.0.tan()) }

            #[inline]
            fn asin(self) -> Self { $name(self.0.asin()) }

            #[inline]
            fn acos(self) -> Self { $name(self.0.acos()) }

            #[inline]
            fn atan(self) -> Self { $name(self.0.atan()) }

            #[inline]
            fn atan2(self, other: Self) -> Self { $name(self.0.atan2(other.0)) }

            #[inline]
            fn sin_cos(self) -> (Self, Self) {
                let (s, c) = self.0.sin_cos();
                ($name(s), $name(c))
            }

            #[inline]
            fn exp_m1(self) -> Self { $name(self.0.exp_m1()) }

            #[inline]
            fn ln_1p(self) -> Self { $name(self.0.ln_1p()) }

            #[inline]
            fn sinh(self) -> Self { $name(self.0.sinh()) }

            #[inline]
            fn cosh(self) -> Self { $name(self.0.cosh()) }

            #[inline]
            fn tanh(self) -> Self { $name(self.0.tanh()) }

            #[inline]
            fn asinh(self) -> Self { $name(self.0.asinh()) }

            #[inline]
            fn acosh(self) -> Self { $name(self.0.acosh()) }

            #[inline]
            fn atanh(self) -> Self { $name(self.0.atanh()) }
        }
    }
}

/// The `mkprim` macro allows the declaration of newtype wrappers for primitive rust types which
/// retain the mathematical operations of the underlying type. For floating point types this means
/// that the resulting newtype will implement `std::num::Float`, for integral types **(not yet
/// implemented)** this means the newtype will implement `std::num::Int`. 
///
/// ```rust
///    #[macro_use]
///    extern crate mkprim;
///
///    mkprim! {
///        Meters(f32);
///        Feet(f32);
///    }
///
///    impl Feet {
///        fn to_meters(self) -> Meters {
///            // So sayeth the Google
///            Meters(self.0 * 0.3048)
///        }
///    }
///
///    fn add_one<F: std::num::Float>(f: F) -> F {
///        f + std::num::Float::one()
///    }
///
///    fn main() {
///        // This gives us access to the `abs` method
///        use ::std::num::Float;
///
///        let f = Feet(5.0);
///
///        // This example is totally straightforward and useful, right? RIGHT?
///        let abs_diff = (add_one(f).to_meters() - Meters(1.8288)).abs();
///
///        assert!(abs_diff < Meters(1e-10));
///    }
/// ```
#[macro_export]
macro_rules! mkprim {

    { $name:ident ( pub $t:ty ) ; $($rest:tt)* } => {
        #[derive(Copy, Clone, PartialOrd, PartialEq, Debug)]
        struct $name(pub $t);
        __impl_float_type!($name, $t);

        mkprim!{ $($rest)* }
    };

    { $name:ident ( $t:ty ) ; $($rest:tt)* } => {
        #[derive(Copy, Clone, PartialOrd, PartialEq, Debug)]
        struct $name($t);
        __impl_float_type!($name, $t);

        mkprim!{ $($rest)* }
    };

    { pub $name:ident ( pub $t:ty ) ; $($rest:tt)* } => {
        #[derive(Copy, Clone, PartialOrd, PartialEq, Debug)]
        pub struct $name(pub $t);
        __impl_float_type!($name, $t);

        mkprim!{ $($rest)* }
    };

    { pub $name:ident ( $t:ty ) ; $($rest:tt)* } => {
        #[derive(Copy, Clone, PartialOrd, PartialEq, Debug)]
        pub struct $name($t);
        __impl_float_type!($name, $t);

        mkprim!{ $($rest)* }
    };

    { } => { }
}

#[cfg(test)]
mod tests {
    use std::num::Float;

    mkprim! {
        A(f32);
        pub B(f32);
        C(pub f32);
        pub D(pub f32);
    }

    #[test]
    fn operators_work() {
        let x = A(22.0);

        // Not so much concerned about checking the math as about making sure the compiler doesn't
        // complain.
        assert_eq!(x + x, A(44.0));
        assert_eq!(x - x, A(0.0));
        assert_eq!(x * x, A(484.0));
        assert_eq!(x / x, A(1.0));
    }

    fn floaty<F: Float>(f: F) -> F {
        f + f * f
    }

    #[test]
    fn traits_implemented() {
        let x = A(1.1);

        assert_eq!(floaty(x), A(1.1 + 1.1 * 1.1))
    }
}
