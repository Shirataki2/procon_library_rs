pub mod num_trait {
    use std::ops::*;

    /// Additive identity
    pub trait Zero: Sized {
        fn zero() -> Self;
        fn is_zero(&self) -> bool;
    }

    /// Multiplicative identity
    pub trait One: Sized {
        fn one() -> Self;
        fn is_one(&self) -> bool;
    }

    pub trait Signed: Sized {
        fn abs(&self) -> Self;
        fn is_positive(&self) -> bool;
        fn is_negative(&self) -> bool;
    }

    pub trait Unsigned: Sized {}

    pub trait Bounded: Sized {
        fn min_value() -> Self;
        fn max_value() -> Self;
    }

    pub trait BaseNumber {}
    pub trait BaseInteger: BaseNumber {}
    pub trait BaseFloating: BaseNumber {
        fn sqrt(value: Self) -> Self;
    }

    pub trait Elem: Sized + Copy + Clone + PartialEq {}
    impl<T: Sized + Clone + Copy + PartialEq> Elem for T {}

    pub trait Magma: Elem + Add<Output=Self> {}
    impl<T: Elem + Add<Output=Self>> Magma for T {}

    pub trait Associative: Magma {}

    pub trait SemiGroup: Magma + Associative {}
    impl<T: Magma + Associative> SemiGroup for T {}

    pub trait Monoid: SemiGroup + Zero {}
    impl<T: SemiGroup + Zero> Monoid for T {}

    pub trait ComMonoid: Monoid + AddAssign + PartialOrd {}
    impl<T: Monoid + AddAssign + PartialOrd> ComMonoid for T {}

    pub trait Group: Monoid + Neg<Output=Self> + Sub<Output=Self> {}
    impl<T: Monoid + Neg<Output=Self> + SubAssign + Sub<Output=Self>> Group for T {}

    pub trait AbelGroup: ComMonoid + Group + SubAssign {}
    impl<T: Group + ComMonoid + SubAssign> AbelGroup for T {}

    pub trait SemiRing: ComMonoid + Mul<Output=Self> + One {}
    impl<T: ComMonoid + Mul<Output=Self> + One> SemiRing for T {}

    pub trait Ring: AbelGroup + SemiRing {}
    impl<T: AbelGroup + SemiRing> Ring for T {}

    pub trait ComRing: Ring + MulAssign {}
    impl<T: Ring + MulAssign> ComRing for T {}

    pub trait Field: ComRing + Div<Output=Self> + DivAssign {}
    impl<T: ComRing + Div<Output=Self> + DivAssign> Field for T {}

    macro_rules! integer_primitives {
        ($($name: tt)*) => {$(
            impl Zero for $name {
                fn zero() -> Self { 0 }
                fn is_zero(&self) -> bool { self == &0 }
            }
            impl One for $name {
                fn one() -> Self { 1 }
                fn is_one(&self) -> bool { self == &1 }
            }
            impl Bounded for $name {
                fn min_value() -> Self { std::$name::MIN }
                fn max_value() -> Self { std::$name::MAX }
            }
            impl Associative for $name {}
            impl BaseNumber for $name {}
            impl BaseInteger for $name {}
        )*};
    }
    macro_rules! signed_int_primitives {
        ($($name: tt)*) => {$(
            impl Signed for $name {
                fn abs(&self) -> Self { if self >= &0 { *self } else { -self } }
                fn is_positive(&self) -> bool { self > &0 }
                fn is_negative(&self) -> bool { self < &0 }
            }
        )*};
    }
    macro_rules! unsigned_int_primitives {
        ($($name: tt)*) => {$(
            impl Unsigned for $name {}
        )*};
    }
    macro_rules! floating_primitives {
        ($($name: tt)*) => {$(
            impl Zero for $name {
                fn zero() -> Self { 0.0 }
                fn is_zero(&self) -> bool { -1e-6 < *self && 1e-6 > *self }
            }
            impl One for $name {
                fn one() -> Self { 1.0 }
                fn is_one(&self) -> bool { 1.0 - 1e-6 < *self && 1.0 + 1e-6 > *self }
            }
            impl Signed for $name {
                fn abs(&self) -> Self { if self >= &0.0 { *self } else { -self } }
                fn is_positive(&self) -> bool { self > &0.0 }
                fn is_negative(&self) -> bool { self < &0.0 }
            }
            impl Bounded for $name {
                fn min_value() -> Self { std::$name::MIN }
                fn max_value() -> Self { std::$name::MAX }
            }
            impl Associative for $name {}
            impl BaseNumber for $name {}
            impl BaseFloating for $name {
                fn sqrt(value: Self) -> Self { value.sqrt() }
            }
        )*};
    }

    integer_primitives!(u128 u64 u32 u16 u8 usize i128 i64 i32 i16 i8 isize);
    signed_int_primitives!(i128 i64 i32 i16 i8 isize);
    unsigned_int_primitives!(u128 u64 u32 u16 u8 usize);
    floating_primitives!(f32 f64);
}

#[cfg(test)]
mod tests {
    use super::num_trait::*;

    #[test]
    fn test_traits_unsigned() {
        assert_eq!(u128::zero(), 0u128);
        assert!(0u128.is_zero());
        assert!(!1u128.is_zero());
        assert_eq!(u128::one(), 1u128);
        assert!(1u128.is_one());
        assert!(!0u128.is_one());
        assert_eq!(u128::max_value(), std::u128::MAX);
        assert_eq!(u128::min_value(), std::u128::MIN);
    }

    #[test]
    fn test_traits_signed() {
        assert_eq!(i128::zero(), 0i128);
        assert!(0i128.is_zero());
        assert!(!1i128.is_zero());
        assert_eq!(i128::one(), 1i128);
        assert!(1i128.is_one());
        assert!(!0i128.is_one());
        assert_eq!(i128::max_value(), std::i128::MAX);
        assert_eq!(i128::min_value(), std::i128::MIN);
        assert!((-5i128).is_negative());
        assert!(!(-5i128).is_positive());
        assert!(!5i128.is_negative());
        assert!(5i128.is_positive());
        assert_eq!((-3i128).abs(), 3i128);
        assert_eq!(3i128.abs(), 3i128);
    }

    #[test]
    fn test_traits_floating() {
        assert_eq!(f32::zero(), 0f32);
        assert!(0f32.is_zero());
        assert!(!1f32.is_zero());
        assert_eq!(f32::one(), 1f32);
        assert!(1f32.is_one());
        assert!(!0f32.is_one());
        assert_eq!(f32::max_value(), std::f32::MAX);
        assert_eq!(f32::min_value(), std::f32::MIN);
        assert!((-5f32).is_negative());
        assert!(!(-5f32).is_positive());
        assert!(!5f32.is_negative());
        assert!(5f32.is_positive());
        assert_eq!((-3f32).abs(), 3f32);
        assert_eq!(3f32.abs(), 3f32);
    }
}
