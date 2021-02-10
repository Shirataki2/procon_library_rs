//! WIP
use crate::math::algebra::num_trait;

pub mod angle {
    use super::num_trait::*;
    use std::ops::*;
    use std::fmt;

    pub trait Angle
    where
        Self: Copy + Clone + PartialEq + PartialOrd,
        Self: Zero,
        Self: Neg<Output=Self> + Add<Self, Output=Self>,
        Self: Sub<Self, Output=Self> + Mul<Self, Output=Self>,
        Self: Div<Self, Output=Self> + Rem<Self, Output=Self>,
        Self: Mul<<Self as Angle>::Factor, Output=Self>,
        Self: Div<<Self as Angle>::Factor, Output=Self>,
    {
        type Factor: BaseFloating + One + Magma;

        fn fullturn() -> Self;
        fn halfturn() -> Self {
            let two = Self::Factor::one() + Self::Factor::one();
            Self::fullturn() / two
        }

        #[inline]
        fn normalize(self) -> Self {
            let t = self % Self::fullturn();
            if t < Self::zero() {
                t + Self::fullturn()
            } else {
                t
            }
        }

        #[inline]
        fn opposite(self) -> Self {
            Self::normalize(self + Self::halfturn())
        }

        fn sin(self) -> Self::Factor;
        fn cos(self) -> Self::Factor;
        fn tan(self) -> Self::Factor;
        fn asin(r: Self::Factor) -> Self;
        fn acos(r: Self::Factor) -> Self;
        fn atan(r: Self::Factor) -> Self;
        fn atan2(a: Self::Factor, b: Self::Factor) -> Self;
        fn sin_cos(self) -> (Self::Factor, Self::Factor);

        #[inline]
        fn csc(self) -> Self::Factor {
            Self::sin(self).recip()
        }

        #[inline]
        fn cot(self) -> Self::Factor {
            Self::tan(self).recip()
        }

        #[inline]
        fn sec(self) -> Self::Factor {
            Self::cos(self).recip()
        }
    }

    #[repr(C)]
    #[derive(Copy, Clone, PartialOrd, PartialEq)]
    pub struct Rad<T>(pub T);

    #[repr(C)]
    #[derive(Copy, Clone, PartialOrd, PartialEq)]
    pub struct Deg<T>(pub T);

    impl<T: BaseFloating> From<Deg<T>> for Rad<T> {
        fn from(v: Deg<T>) -> Rad<T> {
            Rad(v.0.to_radians())
        }
    }

    impl<T: BaseFloating> From<Rad<T>> for Deg<T> {
        fn from(v: Rad<T>) -> Deg<T> {
            Deg(v.0.to_degrees())
        }
    }

    macro_rules! impl_angle {
        ($Name: ident, $fmt: expr, $fullturn: expr) => {
            impl<T: BaseFloating> Zero for $Name<T> {
                fn zero() -> $Name<T> {
                    $Name(T::zero())
                }
                fn is_zero(&self) -> bool {
                    T::approx_eq(self.0 ,T::zero())
                }
            }

            impl<T: BaseFloating> AddAssign<$Name<T>> for $Name<T> {
                fn add_assign(&mut self, rhs: $Name<T>) { self.0 += rhs.0; }
            }
            impl<T: BaseFloating> SubAssign<$Name<T>> for $Name<T> {
                fn sub_assign(&mut self, rhs: $Name<T>) { self.0 -= rhs.0; }
            }
            impl<T: BaseFloating> MulAssign<$Name<T>> for $Name<T> {
                fn mul_assign(&mut self, rhs: $Name<T>) { self.0 *= rhs.0; }
            }
            impl<T: BaseFloating> DivAssign<$Name<T>> for $Name<T> {
                fn div_assign(&mut self, rhs: $Name<T>) { self.0 /= rhs.0; }
            }
            impl<T: BaseFloating> RemAssign<$Name<T>> for $Name<T> {
                fn rem_assign(&mut self, rhs: $Name<T>) { self.0 %= rhs.0; }
            }
            impl<T: BaseFloating> Neg for $Name<T> {
                type Output = $Name<T>;
                fn neg(self) -> $Name<T> { Self(-self.0) }
            }
            impl<T: BaseFloating> Add<$Name<T>> for $Name<T> {
                type Output = $Name<T>;
                fn add(self, rhs: $Name<T>) -> $Name<T> { Self(self.0 + rhs.0) }
            }
            impl<T: BaseFloating> Sub<$Name<T>> for $Name<T> {
                type Output = $Name<T>;
                fn sub(self, rhs: $Name<T>) -> $Name<T> { Self(self.0 - rhs.0) }
            }
            impl<T: BaseFloating> Mul<$Name<T>> for $Name<T> {
                type Output = $Name<T>;
                fn mul(self, rhs: $Name<T>) -> $Name<T> { Self(self.0 * rhs.0) }
            }
            impl<T: BaseFloating> Div<$Name<T>> for $Name<T> {
                type Output = $Name<T>;
                fn div(self, rhs: $Name<T>) -> $Name<T> { Self(self.0 / rhs.0) }
            }
            impl<T: BaseFloating + From<f64>> Mul<<Self as Angle>::Factor> for $Name<T> {
                type Output = $Name<T>;
                fn mul(self, rhs: <Self as Angle>::Factor) -> $Name<T> { Self(self.0 * rhs) }
            }
            impl<T: BaseFloating + From<f64>> Div<<Self as Angle>::Factor> for $Name<T> {
                type Output = $Name<T>;
                fn div(self, rhs: <Self as Angle>::Factor) -> $Name<T> { Self(self.0 / rhs) }
            }
            impl<T: BaseFloating> Rem<$Name<T>> for $Name<T> {
                type Output = $Name<T>;
                fn rem(self, rhs: $Name<T>) -> $Name<T> { Self(self.0 % rhs.0) }
            }

            impl<T: BaseFloating + From<f64>> Angle for $Name<T> {
                type Factor = T;
                fn fullturn() -> Self { Self($fullturn.into()) }
                fn sin(self) -> T { Rad::from(self).0.sin() }
                fn cos(self) -> T { Rad::from(self).0.cos() }
                fn tan(self) -> T { Rad::from(self).0.tan() }
                fn sin_cos(self) -> (T, T) { Rad::from(self).0.sin_cos() }
                fn asin(v: T) -> Self { Rad(v.asin()).into() }
                fn acos(v: T) -> Self { Rad(v.acos()).into() }
                fn atan(v: T) -> Self { Rad(v.atan()).into() }
                fn atan2(a: T, b: T) -> Self { Rad(a.atan2(b)).into() }
            }
            impl<T: fmt::Debug> fmt::Debug for $Name<T> {
                fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                    write!(f, $fmt, self.0)
                }
            }
        };
    }

    impl_angle!(Rad, "{:?} rad", T::tau());
    impl_angle!(Deg, "{:?}°", T::tau_deg());
}
