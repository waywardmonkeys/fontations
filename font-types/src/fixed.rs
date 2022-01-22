//! fixed-point numerical types

use std::ops::{Add, AddAssign, Sub, SubAssign};

// shared between Fixed and F2dot14
macro_rules! fixed_impl {
    ($name:ident, $bits:literal, $fract_bits:literal, $ty:ty) => {
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Default)]
        #[doc = concat!(stringify!($bits), "-bit signed fixed point number with ", stringify!($fract_bits), " bits of fraction." )]
        pub struct $name($ty);
        impl $name {
            /// Minimum value.
            pub const MIN: Self = Self(<$ty>::MIN);

            /// Maximum value.
            pub const MAX: Self = Self(<$ty>::MAX);

            const INT_MASK: $ty = !0 << $fract_bits;
            const ROUND: $ty = 1 << ($fract_bits - 1);
            const ONE: $ty = 1 << $fract_bits;
            const FRACT_BITS: usize = $fract_bits;

            //TODO: is this actually useful?
            /// Returns the nearest integer value.
            pub fn round(self) -> Self {
                Self(self.0.wrapping_add(Self::ROUND) & Self::INT_MASK)
            }

            /// Returns the absolute value of the number.
            pub fn abs(self) -> Self {
                Self(self.0.abs())
            }

            /// Returns the largest integer less than or equal to the number.
            pub fn floor(self) -> Self {
                Self(self.0 & Self::INT_MASK)
            }

            /// Returns the fractional part of the number.
            pub fn fract(self) -> Self {
                Self(self.0 - self.floor().0)
            }

            /// Wrapping addition.
            pub fn wrapping_add(self, other: Self) -> Self {
                Self(self.0.wrapping_add(other.0))
            }

            /// Saturating addition.
            pub fn saturating_add(self, other: Self) -> Self {
                Self(self.0.saturating_add(other.0))
            }

            /// Wrapping substitution.
            pub fn wrapping_sub(self, other: Self) -> Self {
                Self(self.0.wrapping_sub(other.0))
            }

            /// Saturating substitution.
            pub fn saturating_sub(self, other: Self) -> Self {
                Self(self.0.saturating_sub(other.0))
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                let int = (self.0 & Self::INT_MASK) >> $fract_bits;
                write!(f, "{}.", int)?;

                let mut fract = (self.0 & !Self::INT_MASK) as u32;
                while fract > 0 {
                    fract *= 10;
                    let val = fract >> $fract_bits;
                    debug_assert!(val < 10);
                    write!(f, "{}", val)?;
                    fract &= ((1 << $fract_bits) - 1);
                }
                Ok(())
            }
        }

        //FIXME: this should respect padding and other format options
        impl std::fmt::Debug for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "{}", self)
            }
        }

        impl Add for $name {
            type Output = Self;
            #[inline(always)]
            fn add(self, other: Self) -> Self {
                // same overflow semantics as std: panic in debug, wrap in release
                Self(self.0 + other.0)
            }
        }

        impl AddAssign for $name {
            fn add_assign(&mut self, other: Self) {
                *self = *self + other;
            }
        }

        impl Sub for $name {
            type Output = Self;
            #[inline(always)]
            fn sub(self, other: Self) -> Self {
                Self(self.0 - other.0)
            }
        }

        impl SubAssign for $name {
            fn sub_assign(&mut self, other: Self) {
                *self = *self - other;
            }
        }
    };
}

/// impl float conversion methods.
///
/// We convert to different float types in order to ensure we can roundtrip
/// without floating point error.
macro_rules! float_conv {
    ($name:ident, $to:ident, $from:ident, $ty:ty) => {
        impl $name {
            #[doc = concat!("Creates a fixed point value from a", stringify!($ty), ".")]
            ///
            /// This operation is lossy; the float will be rounded to the nearest
            /// representable value.
            pub fn $from(x: $ty) -> Self {
                Self((x * Self::ONE as $ty) as _)
            }

            #[doc = concat!("Returns the value as an ", stringify!($ty), ".")]
            ///
            /// This operation is lossless: all representable values can be
            /// round-tripped.
            pub fn $to(self) -> $ty {
                let int = ((self.0 & Self::INT_MASK) >> Self::FRACT_BITS) as $ty;
                let fract = (self.0 & !Self::INT_MASK) as $ty / Self::ONE as $ty;
                int + fract
            }
        }
    };
}

fixed_impl!(F2dot14, 16, 14, i16);
fixed_impl!(Fixed, 32, 16, i32);
float_conv!(F2dot14, to_f32, from_f32, f32);
float_conv!(Fixed, to_f64, from_f64, f64);

#[cfg(test)]
mod tests {
    #![allow(overflowing_literals)] // we want to specify byte values directly
    use super::*;

    #[test]
    fn f2dot14_floats() {
        // Examples from https://docs.microsoft.com/en-us/typography/opentype/spec/otff#data-types
        assert_eq!(F2dot14(0x7fff), F2dot14::from_f32(1.999939));
        assert_eq!(F2dot14(0x7000), F2dot14::from_f32(1.75));
        assert_eq!(F2dot14(0x0001), F2dot14::from_f32(0.0000610356));
        assert_eq!(F2dot14(0x0000), F2dot14::from_f32(0.0));
        assert_eq!(F2dot14(0xffff), F2dot14::from_f32(-0.000075));
        assert_eq!(F2dot14(0x8000), F2dot14::from_f32(-2.0));
    }

    #[test]
    fn roundtrip_f2dot14() {
        for i in i16::MIN..=i16::MAX {
            let val = F2dot14(i);
            assert_eq!(val, F2dot14::from_f32(val.to_f32()));
        }
    }

    #[test]
    fn round_f2dot14() {
        assert_eq!(F2dot14(0x7000).round(), F2dot14::from_f32(-2.0));
        assert_eq!(F2dot14(0x1F00).round(), F2dot14::from_f32(0.0));
        assert_eq!(F2dot14(0x2000).round(), F2dot14::from_f32(1.0));
    }

    #[test]
    fn round_fixed() {
        //TODO: make good test cases
        assert_eq!(Fixed(0x0001_7FFE).round(), Fixed(0x0001_0000));
        assert_eq!(Fixed(0x0001_7FFF).round(), Fixed(0x0001_0000));
        assert_eq!(Fixed(0x0001_8000).round(), Fixed(0x0002_0000));
    }

    // disabled because it's slow
    //#[test]
    //fn roundtrip_fixed() {
    //for i in i32::MIN..=i32::MAX {
    //let val = Fixed(i);
    //assert_eq!(val, Fixed::from_f64(val.to_f64()));
    //}
    //}

    #[test]
    fn fixed_floats() {
        assert_eq!(Fixed(0x7fff_0000), Fixed::from_f64(32767.));
        assert_eq!(Fixed(0x7000_0001), Fixed::from_f64(28672.00001525879));
        assert_eq!(Fixed(0x0001_0000), Fixed::from_f64(1.0));
        assert_eq!(Fixed(0x0000_0000), Fixed::from_f64(0.0));
        assert_eq!(
            Fixed(i32::from_be_bytes([0xff; 4])),
            Fixed::from_f64(-0.000015259)
        );
        assert_eq!(Fixed(0x7fff_ffff), Fixed::from_f64(32768.0));
    }
}
