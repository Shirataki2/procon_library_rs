use crate::math::algebra::num_trait;

macro_rules! bin_op_vec2d {
    ($AssignTrait:ident, $assign_fn:ident, $OpTrait:ident, $op_func:ident) => {
        impl<T: Field> $AssignTrait for Vector2d<T>{
            fn $assign_fn(&mut self, rhs: Self) {
                self.0.$assign_fn(rhs.0);
                self.1.$assign_fn(rhs.1);
            }
        }
        impl<T: Field> $OpTrait for Vector2d<T> {
            type Output = Self;
            fn $op_func(self, rhs: Self) -> Self {
                Self(self.0.$op_func(rhs.0), self.1.$op_func(rhs.1))
            }
        }
    };
}

pub mod linear_algebra {
    use super::num_trait::*;
    use std::ops::*;

    pub trait Norm: Field {
        fn sq_norm(&self) -> f64;

        fn norm(&self) -> f64 { self.sq_norm().sqrt() }
    }

    pub trait Geometry: Field + Into<f64> {
        fn dot(&self, other: &Self) -> f64;
        fn cross(&self, other: &Self) -> Self;
    }

    #[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
    pub struct Vector2d<T: Field>(T, T);

    bin_op_vec2d!(AddAssign, add_assign, Add, add);
    bin_op_vec2d!(SubAssign, sub_assign, Sub, sub);
    bin_op_vec2d!(MulAssign, mul_assign, Mul, mul);
    bin_op_vec2d!(DivAssign, div_assign, Div, div);

    impl<T: Field> Associative for Vector2d<T> {}
    impl<T: Field> Zero for Vector2d<T> {
        fn zero() -> Self { Self(T::zero(), T::zero()) }
        fn is_zero(&self) -> bool { self.0 == T::zero() && self.1 == T::zero() }
    }
    impl<T: Field> One for Vector2d<T> {
        fn one() -> Self { Self(T::one(), T::one()) }
        fn is_one(&self) -> bool { self.0 == T::one() && self.1 == T::one() }
    }
    impl<T: Field> Neg for Vector2d<T> {
        type Output = Self;
        fn neg(self) -> Self { Self(-self.0, -self.1) }
    }
    impl<T: Field + Into<f64>> Norm for Vector2d<T> {
        fn sq_norm(&self) -> f64 { (self.0 * self.0 + self.1 * self.1).into() }
    }

}