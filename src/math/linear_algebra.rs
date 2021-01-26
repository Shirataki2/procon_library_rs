use crate::math::algebra::num_trait;

pub mod linalg {
    use std::ops::*;
    use super::num_trait::*;

    pub trait Array
    where
        Self: Index<usize, Output=<Self as Array>::Elem>,
        Self: IndexMut<usize, Output=<Self as Array>::Elem>
    {
        type Elem: Copy;

        fn len() -> usize;

        fn dim(&self) -> usize;

        #[inline]
        fn as_ptr(&self) -> *const Self::Elem { &self[0] }

        #[inline]
        fn as_mut_ptr(&mut self) -> *mut Self::Elem { &mut self[0] }

        #[inline]
        fn swap(&mut self, i: usize, j: usize) {
            unsafe { std::ptr::swap(&mut self[i], &mut self[j]) }
        }

        fn sum(&self) -> Self::Elem where Self::Elem: Magma;
    }

    pub trait VectorSpace: Copy + Clone
    where
        Self: Zero + Add<Self, Output=Self> + Sub<Self, Output=Self>,
        Self: Mul<<Self as VectorSpace>::Scalar, Output=Self>,
        Self: Div<<Self as VectorSpace>::Scalar, Output=Self>,
    {
        type Scalar: Signed;

        #[inline]
        fn lerp(self, other: Self, t: Self::Scalar) -> Self {
            self + (other - self) * t
        }
    }

    pub trait MetricSpace: Sized {
        type Metric;
        fn distance2(self, other: Self) -> Self::Metric;

        /// The distance between two values.
        fn distance(self, other: Self) -> Self::Metric
        where
            Self::Metric: BaseFloating,
        {
            BaseFloating::sqrt(Self::distance2(self, other))
        }
    }

    pub trait InnerSpace: VectorSpace
    where
        Self: MetricSpace<Metric=<Self as VectorSpace>::Scalar>
    {
        fn dot(self, other: Self) -> Self::Scalar;

        #[inline]
        fn magnitude2(self) -> Self::Scalar {
            self.dot(self)
        }

        #[inline]
        fn magnitude(self) -> Self::Scalar
        where
            Self::Scalar: BaseFloating
        {
            BaseFloating::sqrt(self.magnitude2())
        }

        #[inline]
        fn normalized(self) -> Self
        where
            Self::Scalar: BaseFloating + Field
        {
            self * (Self::Scalar::one() / self.magnitude())
        }
    }

    pub trait EuclideanSpace: Copy + Clone
    where
        Self: Array<Elem=<Self as EuclideanSpace>::Scalar>,
        Self: Add<<Self as EuclideanSpace>::Diff, Output=Self>,
        Self: Sub<<Self as EuclideanSpace>::Diff, Output=Self>,
        Self: Sub<Self, Output=<Self as EuclideanSpace>::Diff>,
        Self: Mul<<Self as EuclideanSpace>::Scalar, Output=Self>,
        Self: Div<<Self as EuclideanSpace>::Scalar, Output=Self>,
    {
        type Scalar: BaseNumber;
        type Diff: VectorSpace<Scalar=Self::Scalar>;

        fn origin(self) -> Self;
        fn from_vector(v: Self::Diff) -> Self;
        fn to_vector(self) -> Self::Diff;
    }

    pub trait Matrix: VectorSpace
    where
        Self::Scalar: Field,
        Self: Index<usize, Output=<Self as Matrix>::Column>,
        Self: IndexMut<usize, Output=<Self as Matrix>::Column>,
    {
        type Row: VectorSpace<Scalar=Self::Scalar> + Array<Elem=Self::Scalar>;
        type Column: VectorSpace<Scalar=Self::Scalar> + Array<Elem=Self::Scalar>;
        type Transpose: Matrix<Scalar=Self::Scalar, Row=Self::Column, Column=Self::Row>;

        #[inline]
        fn as_ptr(&self) -> *const Self::Scalar {
            &self[0][0]
        }

        #[inline]
        fn as_mut_ptr(&mut self) -> *mut Self::Scalar {
            &mut self[0][0]
        }

        fn len_row() -> usize { Self::Row::len() }

        fn len_col() -> usize { Self::Column::len() }

        fn get_row(self, r: usize) -> Self::Row;

        fn transpose(self) -> Self::Transpose;
    }

    pub trait SqMatrix
    where
        Self::Scalar: Field,
        Self: Matrix<Row=<Self as SqMatrix>::ColumnRow, Column=<Self as SqMatrix>::ColumnRow, Transpose=Self>,
        Self: Mul<Self, Output=Self>
    {
        type ColumnRow: VectorSpace<Scalar=Self::Scalar> + Array<Elem=Self::Scalar>;
        fn det(&self) -> Self::Scalar;
        fn inv(&self) -> Option<Self>;

        fn len() -> usize { Self::len_row() }
        fn eye(self) -> Self {
            let mut res = Self::zero();
            for i in 0..Self::len_row() {
                res[i][i] = Self::Scalar::one();
            }
            res
        }
        fn diag(&self) -> Self::ColumnRow {
            let mut res = Self::ColumnRow::zero();
            for i in 0..Self::len() {
                res[i] = self[i][i];
            }
            res
        }
    }

    impl<T: Field + Signed> Vector3d<T> {
        pub fn cross(self, rhs: Self) -> Self {
            Self::new([
                self[1] * rhs[2] - self[2] * rhs[1],
                self[2] * rhs[0] - self[0] * rhs[2],
                self[0] * rhs[1] - self[1] * rhs[0],
            ])
        }
    }

    impl<T: Field+ Signed> SqMatrix for Matrix2x2<T> {
        type ColumnRow = Vector2d<T>;

        fn det(&self) -> T {
            self[0][0] * self[1][1] - self[0][1] * self[1][0]
        }
        fn inv(&self) -> Option<Self> {
            let det = self.det();
            if T::is_zero(&det) {
                None
            } else {
                Some(Matrix2x2::new([
                    Vector2d::new([ self[1][1], -self[0][1]]),
                    Vector2d::new([-self[1][0],  self[0][0]]),
                ]) / det)
            }
        }
    }

    impl<T: Field+ Signed> SqMatrix for Matrix3x3<T> {
        type ColumnRow = Vector3d<T>;

        fn det(&self) -> T {
            self[0][0] * (self[1][1] * self[2][2] - self[1][2] * self[2][1])
            - self[1][0] * (self[0][1] * self[2][2] - self[2][1] * self[0][2])
            + self[2][0] * (self[0][1] * self[1][2] - self[1][1] * self[0][2])
        }
        fn inv(&self) -> Option<Self> {
            let det = self.det();
            if T::is_zero(&det) {
                None
            } else {
                Some(Matrix3x3::new([
                    self[1].cross(self[2]) / det,
                    self[2].cross(self[0]) / det,
                    self[0].cross(self[1]) / det,
                ]).transpose())
            }
        }
    }

    impl<T: Field + Signed + BaseFloating> SqMatrix for Matrix4x4<T> {
        type ColumnRow = Vector4d<T>;

        fn det(&self) -> T {
            let tmp = unsafe { det_sub_proc_unsafe(self, 1, 2, 3) };
            tmp.dot(Vector4d::new([self[0][0], self[1][0], self[2][0], self[3][0]]))
        }

        fn inv(&self) -> Option<Self> {
            let tmp = unsafe { det_sub_proc_unsafe(self, 1, 2, 3) };
            let det = tmp.dot(Vector4d::new([self[0][0], self[1][0], self[2][0], self[3][0]]));
            if T::is_zero(&det) {
                None
            } else {
                let inv_det = T::one() / det;
                let tmp0 = tmp * inv_det;
                let tmp1 = unsafe { det_sub_proc_unsafe(self, 0, 3, 2) * inv_det };
                let tmp2 = unsafe { det_sub_proc_unsafe(self, 0, 1, 3) * inv_det };
                let tmp3 = unsafe { det_sub_proc_unsafe(self, 0, 2, 1) * inv_det };
                Some(Self::new([tmp0, tmp1, tmp2, tmp3]))
            }
        }
    }

    macro_rules! define_vector {
        ($Name: ident, $dim: expr) => {
            #[derive(Copy, Clone, Debug, PartialEq)]
            pub struct $Name<T>([T; $dim]);

            impl<T> Index<usize> for $Name<T> {
                type Output = T;
                fn index(&self, i: usize) -> &T { &self.0[i] }
            }
            impl<T> IndexMut<usize> for $Name<T> {
                fn index_mut(&mut self, i: usize) -> &mut T { &mut self.0[i] }
            }

            impl<T: Monoid> Zero for $Name<T>
            where
                Self: Array<Elem=T>
            {
                fn zero() -> Self {
                    Self([T::zero(); $dim])
                }
                fn is_zero(&self) -> bool {
                    let mut b = true;
                    for i in 0..Self::len() {
                        b &= self[i] == T::zero();
                    }
                    b
                }
            }

            impl<T: Magma> Add<Self> for $Name<T>
            where
                Self: Array<Elem=T>,
            {
                type Output = Self;
                fn add(self, rhs: Self) -> Self::Output {
                    let mut res = self;
                    for i in 0..Self::len() {
                        res[i] = res[i] + rhs[i];
                    }
                    res
                }
            }

            impl<T: Group> Sub<Self> for $Name<T>
            where
                Self: Array<Elem=T>,
            {
                type Output = Self;
                fn sub(self, rhs: Self) -> Self::Output {
                    let mut res = self;
                    for i in 0..Self::len() {
                        res[i] = res[i] - rhs[i];
                    }
                    res
                }
            }

            #[allow(clippy::assign_op_pattern)]
            impl<T: Field + Signed> Mul<<Self as VectorSpace>::Scalar> for $Name<T>
            where
                Self: Array<Elem=T>,
            {
                type Output = Self;
                fn mul(self, rhs: <Self as VectorSpace>::Scalar) -> Self::Output {
                    let mut res = self;
                    for i in 0..Self::len() {
                        res[i] = res[i] * rhs;
                    }
                    res
                }
            }

            #[allow(clippy::assign_op_pattern)]
            impl<T: Field + Signed> Div<<Self as VectorSpace>::Scalar> for $Name<T>
            where
                Self: Array<Elem=T>,
            {
                type Output = Self;
                fn div(self, rhs: <Self as VectorSpace>::Scalar) -> Self::Output {
                    let mut res = self;
                    for i in 0..Self::len() {
                        res[i] = res[i] / rhs;
                    }
                    res
                }
            }

            impl<T> Array for $Name<T>
            where
                T: Monoid
            {
                type Elem = T;
                fn len() -> usize { $dim }
                fn dim(&self) -> usize { $dim }
                fn sum(&self) -> Self::Elem {
                    let mut v = T::zero();
                    for i in 0..Self::len() {
                        v = v + self[i];
                    }
                    v
                }
            }

            impl<T: Field + Signed> VectorSpace for $Name<T> {
                type Scalar = T;
            }

            impl<T: Field + Signed + BaseFloating> MetricSpace for $Name<T>
            where
                Self: Array<Elem=T>
            {
                type Metric = T;
                fn distance2(self, rhs: Self) -> Self::Metric {
                    let mut res = T::zero();
                    for i in 0..Self::len() {
                        let d = self[i] - rhs[i];
                        res += d * d;
                    }
                    res
                }
            }

            impl<T: Field + Signed + BaseFloating> InnerSpace for $Name<T>
            where
                Self: MetricSpace<Metric=T>
            {
                fn dot(self, rhs: Self) -> Self::Scalar {
                    let mut res = T::zero();
                    for i in 0..Self::len() {
                        res += self[i] * rhs[i];
                    }
                    res
                }
            }

            impl<T: Field + Signed + BaseFloating> EuclideanSpace for $Name<T>
            where
                Self: VectorSpace<Scalar=T>
            {
                type Scalar = T;
                type Diff = Self;

                fn origin(self) -> Self {
                    Self::zero()
                }

                fn from_vector(v: Self::Diff) -> Self {
                    Self(v.0)
                }

                fn to_vector(self) -> Self::Diff {
                    Self(self.0)
                }
            }

            impl<T: Field + Signed> $Name<T> {
                pub fn new(v: [T; $dim]) -> Self {
                    Self(v)
                }

                pub fn add_elementwise(self, rhs: Self) -> Self {
                    let mut res = Self::zero();
                    for i in 0..Self::len() {
                        res[i] = self[i] + rhs[i];
                    }
                    res
                }
                pub fn sub_elementwise(self, rhs: Self) -> Self {
                    let mut res = Self::zero();
                    for i in 0..Self::len() {
                        res[i] = self[i] - rhs[i];
                    }
                    res
                }
                pub fn mul_elementwise(self, rhs: Self) -> Self {
                    let mut res = Self::zero();
                    for i in 0..Self::len() {
                        res[i] = self[i] * rhs[i];
                    }
                    res
                }
                pub fn div_elementwise(self, rhs: Self) -> Self {
                    let mut res = Self::zero();
                    for i in 0..Self::len() {
                        res[i] = self[i] / rhs[i];
                    }
                    res
                }
            }
        };
    }

    macro_rules! define_matrix {
        ($Name:ident, $row: expr, $col: expr, $RowVector:ident, $ColVector:ident, $TransMatrix:ident) => {
            #[derive(Copy, Clone, Debug, PartialEq)]
            pub struct $Name<T>([$ColVector<T>; $row]);

            impl<T> Index<usize> for $Name<T> {
                type Output = $ColVector<T>;
                fn index(&self, i: usize) -> &Self::Output { &self.0[i] }
            }
            impl<T> IndexMut<usize> for $Name<T> {
                fn index_mut(&mut self, i: usize) -> &mut Self::Output { &mut self.0[i] }
            }

            impl<T: Monoid> Zero for $Name<T> {
                fn zero() -> $Name<T> {
                    $Name([$ColVector::zero(); $row])
                }
                fn is_zero(&self) -> bool { true }
            }

            impl<T: Magma> Add<Self> for $Name<T>
            {
                type Output = Self;
                fn add(self, rhs: Self) -> Self::Output {
                    let mut res = self;
                    for i in 0..$row {
                        for j in 0..$col {
                            res[i][j] = res[i][j] + rhs[i][j];
                        }
                    }
                    res
                }
            }

            impl<T: Group> Sub<Self> for $Name<T>
            {
                type Output = Self;
                fn sub(self, rhs: Self) -> Self::Output {
                    let mut res = self;
                    for i in 0..$row {
                        res[i] = res[i] - rhs[i];
                    }
                    res
                }
            }

            #[allow(clippy::assign_op_pattern)]
            impl<T: Field + Signed> Mul<<Self as VectorSpace>::Scalar> for $Name<T>
            {
                type Output = Self;
                fn mul(self, rhs: <Self as VectorSpace>::Scalar) -> Self::Output {
                    let mut res = self;
                    for i in 0..$row {
                        res[i] = res[i] * rhs;
                    }
                    res
                }
            }

            #[allow(clippy::assign_op_pattern)]
            impl<T: Field + Signed> Div<<Self as VectorSpace>::Scalar> for $Name<T>
            {
                type Output = Self;
                fn div(self, rhs: <Self as VectorSpace>::Scalar) -> Self::Output {
                    let mut res = self;
                    for i in 0..$row {
                        res[i] = res[i] / rhs;
                    }
                    res
                }
            }

            impl<T: Field + Signed> VectorSpace for $Name<T> {
                type Scalar = T;
            }

            impl<T: Field + Signed> $Name<T> {
                pub fn new(v: [$ColVector<T>; $row]) -> Self {
                    Self(v)
                }
            }

            impl<T: Field + Signed> From<[$ColVector<T>; $row]> for $Name<T> {
                fn from(v: [$ColVector<T>; $row]) -> Self {
                    Self::new(v)
                }
            }

            #[allow(clippy::transmute_ptr_to_ptr)]
            impl<T: Field + Signed> AsRef<[[T; $col]; $row]> for $Name<T> {
                fn as_ref(&self) -> &[[T; $col]; $row] {
                    unsafe { std::mem::transmute(self) }
                }
            }

            #[allow(clippy::transmute_ptr_to_ptr)]
            impl<T: Field + Signed> AsMut<[[T; $col]; $row]> for $Name<T> {
                fn as_mut(&mut self) -> &mut [[T; $col]; $row] {
                    unsafe { std::mem::transmute(self) }
                }
            }

            #[allow(clippy::transmute_ptr_to_ptr)]
            impl<T: Field + Signed> AsRef<[T; ($col * $row)]> for $Name<T> {
                fn as_ref(&self) -> &[T; ($col * $row)] {
                    unsafe { std::mem::transmute(self) }
                }
            }

            #[allow(clippy::transmute_ptr_to_ptr)]
            impl<T: Field + Signed> AsMut<[T; ($col * $row)]> for $Name<T> {
                fn as_mut(&mut self) -> &mut [T; ($col * $row)] {
                    unsafe { std::mem::transmute(self) }
                }
            }
        };
    }

    macro_rules! impl_mat_mat_inner_product {
        ($Left:ident, $Right:ident, $Output: ident) => {
            impl<T: Field + Signed> Mul<$Right<T>> for $Left<T> {
                type Output = $Output<T>;
                fn mul(self, rhs: $Right<T>) -> $Output<T> {
                    let mut res = $Output::<T>::zero();
                    for i in 0..$Left::<T>::len_row() {
                        for j in 0..$Right::<T>::len_col() {
                            let mut s = T::zero();
                            for k in 0..$Right::<T>::len_row() {
                                s += self[i][k] * rhs[k][j];
                            }
                            res[i][j] = s;
                        }
                    }
                    res
                }
            }
        }
    }

    macro_rules! impl_mat_vec_inner_product {
        ($Left:ident, $Right:ident, $Output: ident) => {
            impl<T: Field + Signed> Mul<$Right<T>> for $Left<T> {
                type Output = $Output<T>;
                fn mul(self, rhs: $Right<T>) -> $Output<T> {
                    let mut res = $Output::<T>::zero();
                    for i in 0..$Left::<T>::len_row() {
                        let mut s = T::zero();
                        for j in 0..$Right::<T>::len() {
                            s += self[i][j] * rhs[j];
                        }
                        res[i] = s;
                    }
                    res
                }
            }
        }
    }

    define_vector!(Vector2d, 2);
    define_vector!(Vector3d, 3);
    define_vector!(Vector4d, 4);

    define_matrix!(Matrix2x2, 2, 2, Vector2d, Vector2d, Matrix2x2);
    define_matrix!(Matrix2x3, 2, 3, Vector2d, Vector3d, Matrix3x2);
    define_matrix!(Matrix2x4, 2, 4, Vector2d, Vector4d, Matrix4x2);

    define_matrix!(Matrix3x2, 3, 2, Vector3d, Vector2d, Matrix2x3);
    define_matrix!(Matrix3x3, 3, 3, Vector3d, Vector3d, Matrix3x3);
    define_matrix!(Matrix3x4, 3, 4, Vector3d, Vector4d, Matrix4x3);

    define_matrix!(Matrix4x2, 4, 2, Vector4d, Vector2d, Matrix2x4);
    define_matrix!(Matrix4x3, 4, 3, Vector4d, Vector3d, Matrix3x4);
    define_matrix!(Matrix4x4, 4, 4, Vector4d, Vector4d, Matrix4x4);

    impl_mat_vec_inner_product!(Matrix2x2, Vector2d, Vector2d);
    impl_mat_vec_inner_product!(Matrix2x3, Vector3d, Vector2d);
    impl_mat_vec_inner_product!(Matrix2x4, Vector4d, Vector2d);

    impl_mat_vec_inner_product!(Matrix3x2, Vector2d, Vector3d);
    impl_mat_vec_inner_product!(Matrix3x3, Vector3d, Vector3d);
    impl_mat_vec_inner_product!(Matrix3x4, Vector4d, Vector3d);

    impl_mat_vec_inner_product!(Matrix4x2, Vector2d, Vector4d);
    impl_mat_vec_inner_product!(Matrix4x3, Vector3d, Vector4d);
    impl_mat_vec_inner_product!(Matrix4x4, Vector4d, Vector4d);

    impl_mat_mat_inner_product!(Matrix2x2, Matrix2x2, Matrix2x2);
    impl_mat_mat_inner_product!(Matrix2x3, Matrix3x2, Matrix2x2);
    impl_mat_mat_inner_product!(Matrix2x4, Matrix4x2, Matrix2x2);

    impl_mat_mat_inner_product!(Matrix2x2, Matrix2x3, Matrix2x3);
    impl_mat_mat_inner_product!(Matrix2x3, Matrix3x3, Matrix2x3);
    impl_mat_mat_inner_product!(Matrix2x4, Matrix4x3, Matrix2x3);

    impl_mat_mat_inner_product!(Matrix2x2, Matrix2x4, Matrix2x4);
    impl_mat_mat_inner_product!(Matrix2x3, Matrix3x4, Matrix2x4);
    impl_mat_mat_inner_product!(Matrix2x4, Matrix4x4, Matrix2x4);

    impl_mat_mat_inner_product!(Matrix3x2, Matrix2x2, Matrix3x2);
    impl_mat_mat_inner_product!(Matrix3x3, Matrix3x2, Matrix3x2);
    impl_mat_mat_inner_product!(Matrix3x4, Matrix4x2, Matrix3x2);

    impl_mat_mat_inner_product!(Matrix3x2, Matrix2x3, Matrix3x3);
    impl_mat_mat_inner_product!(Matrix3x3, Matrix3x3, Matrix3x3);
    impl_mat_mat_inner_product!(Matrix3x4, Matrix4x3, Matrix3x3);

    impl_mat_mat_inner_product!(Matrix3x2, Matrix2x4, Matrix3x4);
    impl_mat_mat_inner_product!(Matrix3x3, Matrix3x4, Matrix3x4);
    impl_mat_mat_inner_product!(Matrix3x4, Matrix4x4, Matrix3x4);

    impl_mat_mat_inner_product!(Matrix4x2, Matrix2x2, Matrix4x2);
    impl_mat_mat_inner_product!(Matrix4x3, Matrix3x2, Matrix4x2);
    impl_mat_mat_inner_product!(Matrix4x4, Matrix4x2, Matrix4x2);

    impl_mat_mat_inner_product!(Matrix4x2, Matrix2x3, Matrix4x3);
    impl_mat_mat_inner_product!(Matrix4x3, Matrix3x3, Matrix4x3);
    impl_mat_mat_inner_product!(Matrix4x4, Matrix4x3, Matrix4x3);

    impl_mat_mat_inner_product!(Matrix4x2, Matrix2x4, Matrix4x4);
    impl_mat_mat_inner_product!(Matrix4x3, Matrix3x4, Matrix4x4);
    impl_mat_mat_inner_product!(Matrix4x4, Matrix4x4, Matrix4x4);

    impl<T: Field + Signed> Matrix for Matrix2x2<T> {
        type Row = Vector2d<T>;
        type Column = Vector2d<T>;
        type Transpose = Matrix2x2<T>;

        fn get_row(self, r: usize) -> Self::Row {
            Vector2d::new([self[r][0], self[r][1]])
        }
        fn transpose(self) -> Self::Transpose {
            Matrix2x2::new([
                Vector2d::new([self[0][0], self[1][0]]),
                Vector2d::new([self[0][1], self[1][1]]),
            ])
        }
    }

    impl<T: Field + Signed> Matrix for Matrix2x3<T> {
        type Row = Vector2d<T>;
        type Column = Vector3d<T>;
        type Transpose = Matrix3x2<T>;

        fn get_row(self, r: usize) -> Self::Row {
            Vector2d::new([self[r][0], self[r][1]])
        }
        fn transpose(self) -> Self::Transpose {
            Matrix3x2::new([
                Vector2d::new([self[0][0], self[1][0]]),
                Vector2d::new([self[0][1], self[1][1]]),
                Vector2d::new([self[0][2], self[1][2]]),
            ])
        }
    }

    impl<T: Field + Signed> Matrix for Matrix2x4<T> {
        type Row = Vector2d<T>;
        type Column = Vector4d<T>;
        type Transpose = Matrix4x2<T>;

        fn get_row(self, r: usize) -> Self::Row {
            Vector2d::new([self[r][0], self[r][1]])
        }
        fn transpose(self) -> Self::Transpose {
            Matrix4x2::new([
                Vector2d::new([self[0][0], self[1][0]]),
                Vector2d::new([self[0][1], self[1][1]]),
                Vector2d::new([self[0][2], self[1][2]]),
                Vector2d::new([self[0][3], self[1][3]]),
            ])
        }
    }

    impl<T: Field + Signed> Matrix for Matrix3x2<T> {
        type Row = Vector3d<T>;
        type Column = Vector2d<T>;
        type Transpose = Matrix2x3<T>;

        fn get_row(self, r: usize) -> Self::Row {
            Vector3d::new([self[r][0], self[r][1], self[r][2]])
        }
        fn transpose(self) -> Self::Transpose {
            Matrix2x3::new([
                Vector3d::new([self[0][0], self[1][0], self[2][0]]),
                Vector3d::new([self[0][1], self[1][1], self[2][1]]),
            ])
        }
    }

    impl<T: Field + Signed> Matrix for Matrix3x3<T> {
        type Row = Vector3d<T>;
        type Column = Vector3d<T>;
        type Transpose = Matrix3x3<T>;

        fn get_row(self, r: usize) -> Self::Row {
            Vector3d::new([self[r][0], self[r][1], self[r][2]])
        }
        fn transpose(self) -> Self::Transpose {
            Matrix3x3::new([
                Vector3d::new([self[0][0], self[1][0], self[2][0]]),
                Vector3d::new([self[0][1], self[1][1], self[2][1]]),
                Vector3d::new([self[0][2], self[1][2], self[2][2]]),
            ])
        }
    }

    impl<T: Field + Signed> Matrix for Matrix3x4<T> {
        type Row = Vector3d<T>;
        type Column = Vector4d<T>;
        type Transpose = Matrix4x3<T>;

        fn get_row(self, r: usize) -> Self::Row {
            Vector3d::new([self[r][0], self[r][1], self[r][2]])
        }
        fn transpose(self) -> Self::Transpose {
            Matrix4x3::new([
                Vector3d::new([self[0][0], self[1][0], self[2][0]]),
                Vector3d::new([self[0][1], self[1][1], self[2][1]]),
                Vector3d::new([self[0][2], self[1][2], self[2][2]]),
                Vector3d::new([self[0][3], self[1][3], self[2][3]]),
            ])
        }
    }

    impl<T: Field + Signed> Matrix for Matrix4x2<T> {
        type Row = Vector4d<T>;
        type Column = Vector2d<T>;
        type Transpose = Matrix2x4<T>;

        fn get_row(self, r: usize) -> Self::Row {
            Vector4d::new([self[r][0], self[r][1], self[r][2], self[r][3]])
        }
        fn transpose(self) -> Self::Transpose {
            Matrix2x4::new([
                Vector4d::new([self[0][0], self[1][0], self[2][0], self[3][0]]),
                Vector4d::new([self[0][1], self[1][1], self[2][1], self[3][1]]),
            ])
        }
    }

    impl<T: Field + Signed> Matrix for Matrix4x3<T> {
        type Row = Vector4d<T>;
        type Column = Vector3d<T>;
        type Transpose = Matrix3x4<T>;

        fn get_row(self, r: usize) -> Self::Row {
            Vector4d::new([self[r][0], self[r][1], self[r][2], self[r][3]])
        }
        fn transpose(self) -> Self::Transpose {
            Matrix3x4::new([
                Vector4d::new([self[0][0], self[1][0], self[2][0], self[3][0]]),
                Vector4d::new([self[0][1], self[1][1], self[2][1], self[3][1]]),
                Vector4d::new([self[0][2], self[1][2], self[2][2], self[3][2]]),
            ])
        }
    }

    impl<T: Field + Signed> Matrix for Matrix4x4<T> {
        type Row = Vector4d<T>;
        type Column = Vector4d<T>;
        type Transpose = Matrix4x4<T>;

        fn get_row(self, r: usize) -> Self::Row {
            Vector4d::new([self[r][0], self[r][1], self[r][2], self[r][3]])
        }
        fn transpose(self) -> Self::Transpose {
            Matrix4x4::new([
                Vector4d::new([self[0][0], self[1][0], self[2][0], self[3][0]]),
                Vector4d::new([self[0][1], self[1][1], self[2][1], self[3][1]]),
                Vector4d::new([self[0][2], self[1][2], self[2][2], self[3][2]]),
                Vector4d::new([self[0][3], self[1][3], self[2][3], self[3][3]]),
            ])
        }
    }

    #[inline]
    #[allow(clippy::many_single_char_names)]
    unsafe fn det_sub_proc_unsafe<T: Field + Signed>(
        m: &Matrix4x4<T>,
        x: usize,
        y: usize,
        z: usize,
    ) -> Vector4d<T> {
        let s: &[T; 16] = m.as_ref();
        let a = Vector4d::new([
            *s.get_unchecked(4 + x),
            *s.get_unchecked(12 + x),
            *s.get_unchecked(x),
            *s.get_unchecked(8 + x),
        ]);
        let b = Vector4d::new([
            *s.get_unchecked(8 + y),
            *s.get_unchecked(8 + y),
            *s.get_unchecked(4 + y),
            *s.get_unchecked(4 + y),
        ]);
        let c = Vector4d::new([
            *s.get_unchecked(12 + z),
            *s.get_unchecked(z),
            *s.get_unchecked(12 + z),
            *s.get_unchecked(z),
        ]);

        let d = Vector4d::new([
            *s.get_unchecked(8 + x),
            *s.get_unchecked(8 + x),
            *s.get_unchecked(4 + x),
            *s.get_unchecked(4 + x),
        ]);
        let e = Vector4d::new([
            *s.get_unchecked(12 + y),
            *s.get_unchecked(y),
            *s.get_unchecked(12 + y),
            *s.get_unchecked(y),
        ]);
        let f = Vector4d::new([
            *s.get_unchecked(4 + z),
            *s.get_unchecked(12 + z),
            *s.get_unchecked(z),
            *s.get_unchecked(8 + z),
        ]);

        let g = Vector4d::new([
            *s.get_unchecked(12 + x),
            *s.get_unchecked(x),
            *s.get_unchecked(12 + x),
            *s.get_unchecked(x),
        ]);
        let h = Vector4d::new([
            *s.get_unchecked(4 + y),
            *s.get_unchecked(12 + y),
            *s.get_unchecked(y),
            *s.get_unchecked(8 + y),
        ]);
        let i = Vector4d::new([
            *s.get_unchecked(8 + z),
            *s.get_unchecked(8 + z),
            *s.get_unchecked(4 + z),
            *s.get_unchecked(4 + z),
        ]);
        let mut tmp = a.mul_elementwise(b.mul_elementwise(c));
        tmp = tmp + d.mul_elementwise(e.mul_elementwise(f));
        tmp = tmp + g.mul_elementwise(h.mul_elementwise(i));
        tmp = tmp - a.mul_elementwise(e.mul_elementwise(i));
        tmp = tmp - d.mul_elementwise(h.mul_elementwise(c));
        tmp = tmp - g.mul_elementwise(b.mul_elementwise(f));
        tmp
    }
}

#[cfg(test)]
mod tests {
    use super::linalg::*;
    use crate::matrix;

    #[test]
    fn test_add_vec() {
        let v1 = Vector2d::new([3, 5]);
        let v2 = Vector2d::new([2, 7]);
        let v3 = v1 + v2;
        assert_eq!(v3, Vector2d::new([5, 12]));

        let v1 = Vector3d::new([3, 5, 7]);
        let v2 = Vector3d::new([2, 7, 9]);
        let v3 = v1 + v2;
        assert_eq!(v3, Vector3d::new([5, 12, 16]));

        let v1 = Vector4d::new([3, 5, 7, 9]);
        let v2 = Vector4d::new([2, 7, 9, 10]);
        let v3 = v1 + v2;
        assert_eq!(v3, Vector4d::new([5, 12, 16, 19]));
    }

    #[test]
    fn test_sub_vec() {
        let v1 = Vector2d::new([3, 5]);
        let v2 = Vector2d::new([2, 7]);
        let v3 = v1 - v2;
        assert_eq!(v3, Vector2d::new([1, -2]));

        let v1 = Vector3d::new([3, 5, 7]);
        let v2 = Vector3d::new([2, 7, 9]);
        let v3 = v1 - v2;
        assert_eq!(v3, Vector3d::new([1, -2, -2]));

        let v1 = Vector4d::new([3, 5, 7, 9]);
        let v2 = Vector4d::new([2, 7, 9, 10]);
        let v3 = v1 - v2;
        assert_eq!(v3, Vector4d::new([1, -2, -2, -1]));
    }

    #[test]
    fn test_mat_add() {
        let m1: Matrix3x2<i64> = matrix!(1, 2; 3, 4; 5, 6);
        let m2: Matrix3x2<i64> = matrix!(6, 5; 4, 3; 2, 1);
        let m3: Matrix3x2<i64> = matrix!(7, 7; 7, 7; 7, 7);
        let m = m1 + m2;
        assert_eq!(m, m3);
    }

    #[test]
    fn test_mat_inner_product() {
        let m1: Matrix2x2<i64> = matrix!(3, 1; 2, 5);
        let m2: Matrix2x2<i64> = matrix!(4, 6; -3, 2);
        let m3: Matrix2x2<i64> = matrix!(9, 20; -7, 22);
        let m4: Matrix2x2<i64> = matrix!(24, 34; -5, 7);
        assert_eq!(m1 * m2, m3);
        assert_eq!(m2 * m1, m4);

        let m1: Matrix3x2<i64> = matrix!(3, 1; 2, 5; -4, 8);
        let m2: Matrix2x3<i64> = matrix!(4, 6, 7; -3, 2, -5);
        let m3: Matrix3x3<i64> = matrix!(9, 20, 16; -7, 22, -11; -40, -8, -68);
        let m4: Matrix2x2<i64> = matrix!(-4, 90; 15, -33);
        assert_eq!(m1 * m2, m3);
        assert_eq!(m2 * m1, m4);
    }

    #[test]
    fn test_inv_matrix() {
        let m1: Matrix2x2<f64> = matrix!(2.0, -4.0; 2.0, -2.0);
        let m2: Matrix2x2<f64> = matrix!(-0.5, 1.0; -0.5, 0.5);
        assert_eq!(m1.inv(), Some(m2));

        let m1: Matrix3x3<f64> = matrix!(2.0, -4.0, 1.0; 2.0, -2.0, 1.0; 1.0, -1.0, 0.0);
        let m2: Matrix3x3<f64> = matrix!(-1.0, 1.0, 2.0; -1.0, 1.0, 0.0; 0.0, 2.0, -4.0);
        assert_eq!(m1.inv(), Some(m2 / 2.0));

        let m1: Matrix4x4<f64> = matrix!(2.0, -4.0, 1.0, -1.0; 2.0, -2.0, 1.0, -1.0; 1.0, -1.0, 0.0, 2.0; 0.0, 0.0, 0.0, 1.0);
        let m2: Matrix4x4<f64> = matrix!(-1.0, 1.0, 2.0, -4.0; -1.0, 1.0, 0.0, 0.0; 0.0, 2.0, -4.0, 10.0; 0.0, 0.0, 0.0, 2.0);
        assert_eq!(m1.inv(), Some(m2 / 2.0));
    }
}
