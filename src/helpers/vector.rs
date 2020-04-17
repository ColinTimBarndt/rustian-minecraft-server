use std::fmt::Display;
use core::ops::*;

/// The `Vec3d` structure represents a mathematical vector
/// made of three components of any equal type
#[derive(Clone, Debug)]
pub struct Vec3d<T>(T, T, T);

impl <T> Display for Vec3d<T> where T: Display {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "Vec3d({}, {}, {})", self.0, self.1, self.2)
    }
}

impl <T> Vec3d<T> {
    /// Creates a new three-dimensional mathematical vector
    pub fn new(x: T, y: T, z: T) -> Self {
        Vec3d::<T>(x, y, z)
    }
    /// Returns the first component (`x`) as a reference
    pub fn get_x_as_ref(&self) -> &T {&(*self).0}
    /// Returns the second component (`y`) as a reference
    pub fn get_y_as_ref(&self) -> &T {&(*self).1}
    /// Returns the third component (`z`) as a reference
    pub fn get_z_as_ref(&self) -> &T {&(*self).2}
    /// Sets the value of the first component (`x`)
    pub fn set_x(&mut self, x: T) {(*self).0 = x}
    /// Sets the value of the second component (`y`)
    pub fn set_y(&mut self, y: T) {(*self).1 = y}
    /// Sets the value of the third component (`z`)
    pub fn set_z(&mut self, z: T) {(*self).2 = z}
}

impl <T> Vec3d<T> where T: Copy {
    /// Returns a copy of the first component (`x`)
    pub fn get_x(&self) -> T {(*self).0}
    /// Returns a copy of the second component (`y`)
    pub fn get_y(&self) -> T {(*self).1}
    /// Returns a copy of the third component (`z`)
    pub fn get_z(&self) -> T {(*self).2}
}

/*impl<F, T> std::convert::TryFrom<Vec3d<F>> for Vec3d<T>
where T: std::convert::TryFrom<F> {
    fn try_from(from: Vec3d<F>) -> Result<Vec3d<T>, <T as std::convert::TryFrom<F>>::Error> {
        use std::convert::TryFrom;
        Vec3d(
            TryFrom::<F>::try_from(from.0)?,
            TryFrom::<F>::try_from(from.1)?,
            TryFrom::<F>::try_from(from.2)?
        )
    }
}*/

impl<T> std::convert::From<Vec3d<T>> for (T, T, T) {
    fn from(from: Vec3d<T>) -> (T, T, T) {
        (from.0, from.1, from.2)
    }
}

macro_rules! implement_part {
    ($A:ty, $Trait:ident, $fun:ident, $operand:tt) => {
        impl $Trait for Vec3d<$A> {
            type Output = Self;
            fn $fun(self, other: Self) -> Self::Output {
                Vec3d::new(
                    self.0 $operand other.0,
                    self.1 $operand other.1,
                    self.2 $operand other.2
                )
            }
        }

        impl $Trait<$A> for Vec3d<$A> {
            type Output = Self;
            fn $fun(self, other: $A) -> Self::Output {
                Vec3d::new(
                    self.0 $operand other,
                    self.1 $operand other,
                    self.2 $operand other
                )
            }
        }

        impl<'a> $Trait<&'a Vec3d<$A>> for Vec3d<$A> {
            type Output = Vec3d<$A>;
            fn $fun(self, other: &Self) -> Self::Output {
                Vec3d::<$A>::new(
                    self.0 $operand other.0,
                    self.1 $operand other.1,
                    self.2 $operand other.2
                )
            }
        }

        impl<'a> $Trait<&'a $A> for Vec3d<$A> {
            type Output = Self;
            fn $fun(self, other: &$A) -> Self::Output {
                Vec3d::new(
                    self.0 $operand other,
                    self.1 $operand other,
                    self.2 $operand other
                )
            }
        }

        impl<'a, 'b> $Trait<&'a Vec3d<$A>> for &'b Vec3d<$A> {
            type Output = Vec3d<$A>;
            fn $fun(self, other: &'a Vec3d<$A>) -> Self::Output {
                Vec3d::<$A>::new(
                    self.0 $operand other.0,
                    self.1 $operand other.1,
                    self.2 $operand other.2
                )
            }
        }

        impl<'a, 'b> $Trait<&'a $A> for &'b Vec3d<$A> {
            type Output = Vec3d<$A>;
            fn $fun(self, other: &'a $A) -> Self::Output {
                Vec3d::<$A>::new(
                    self.0 $operand other,
                    self.1 $operand other,
                    self.2 $operand other
                )
            }
        }
    };
    (assign $A:ty, $Trait:ident, $fun:ident, $operand:tt) => {
        impl $Trait for Vec3d<$A> {
            fn $fun(&mut self, other: Self) {
                self.0 $operand other.0;
                self.1 $operand other.1;
                self.2 $operand other.2;
            }
        }

        impl $Trait<$A> for Vec3d<$A> {
            fn $fun(&mut self, other: $A) {
                self.0 $operand other;
                self.1 $operand other;
                self.2 $operand other;
            }
        }

        impl<'a> $Trait<&'a Vec3d<$A>> for Vec3d<$A> {
            fn $fun(&mut self, other: &Self) {
                self.0 $operand other.0;
                self.1 $operand other.1;
                self.2 $operand other.2;
            }
        }

        impl<'a> $Trait<&'a $A> for Vec3d<$A> {
            fn $fun(&mut self, other: &$A) {
                self.0 $operand other;
                self.1 $operand other;
                self.2 $operand other;
            }
        }
    };
    (rem $A:ty, $Input:ty, $Output:ty) => {
        impl PositiveRem<$Input> for $A {
            type Output = $Output;
            fn positive_rem(self, with: $Input) -> Self::Output {
                let r = self % with;
                if r<0 as $A {
                    (r + with) as $Output
                } else {
                    r as $Output
                }
            }
        }
        impl<'a> PositiveRem<$Input> for &'a $A {
            type Output = $Output;
            fn positive_rem(self, with: $Input) -> Self::Output {
                let r = self % with;
                if r<0 as $A {
                    (r + with) as $Output
                } else {
                    r as $Output
                }
            }
        }
        impl PositiveRem<$Input> for Vec3d<$A> {
            type Output = Vec3d<$Output>;
            fn positive_rem(self, with: $Input) -> Self::Output {
                Vec3d::new(
                    self.0.positive_rem(with),
                    self.1.positive_rem(with),
                    self.2.positive_rem(with)
                )
            }
        }
        impl<'a> PositiveRem<$Input> for &'a Vec3d<$A> {
            type Output = Vec3d<$Output>;
            fn positive_rem(self, with: $Input) -> Self::Output {
                Vec3d::new(
                    self.0.positive_rem(with),
                    self.1.positive_rem(with),
                    self.2.positive_rem(with)
                )
            }
        }
        impl PositiveRem<Vec3d<$Input>> for Vec3d<$A> {
            type Output = Vec3d<$Output>;
            fn positive_rem(self, with: Vec3d<$Input>) -> Self::Output {
                Vec3d::new(
                    self.0.positive_rem(with.0),
                    self.1.positive_rem(with.1),
                    self.2.positive_rem(with.2)
                )
            }
        }
        impl<'a> PositiveRem<Vec3d<$Input>> for &'a Vec3d<$A> {
            type Output = Vec3d<$Output>;
            fn positive_rem(self, with: Vec3d<$Input>) -> Self::Output {
                Vec3d::new(
                    self.0.positive_rem(with.0),
                    self.1.positive_rem(with.1),
                    self.2.positive_rem(with.2)
                )
            }
        }
    };
}

macro_rules! implement_traits {
    ($A:ty) => {

        implement_part!($A, Add, add, +);
        implement_part!(assign $A, AddAssign, add_assign, +=);

        implement_part!($A, Sub, sub, -);
        implement_part!(assign $A, SubAssign, sub_assign, -=);

        implement_part!($A, Mul, mul, *);
        implement_part!(assign $A, MulAssign, mul_assign, *=);

        implement_part!($A, Div, div, /);
        implement_part!(assign $A, DivAssign, div_assign, /=);

        implement_part!($A, Rem, rem, %);
        implement_part!(assign $A, RemAssign, rem_assign, %=);

        impl Vec3d<$A> {
            /// Returns the squared length of this `Vec3d`.
            /// This method is more efficient when comparing
            /// lengths
            pub fn len_squared(&self) -> $A {
                self.0*self.0 + self.1*self.1 + self.2*self.2
            }

            /// Returns the squared distance of this `Vec3d`
            /// to another vector of the same type `T`.
            /// This method is more efficient when comparing
            /// distances
            pub fn distance_squared(&self, other: &Self) -> $A {
                (other-self).len_squared()
            }
        }

        impl Length<f32> for Vec3d<$A> {
            fn len(&self) -> f32 {
                (self.len_squared() as f32).sqrt()
            }
        }

        impl Length<f64> for Vec3d<$A> {
            fn len(&self) -> f64 {
                (self.len_squared() as f64).sqrt()
            }
        }

        impl Distance<f32> for Vec3d<$A> {
            fn distance(&self, other: &Self) -> f32 {
                (self.distance_squared(other) as f32).sqrt()
            }
        }

        impl Distance<f64> for Vec3d<$A> {
            fn distance(&self, other: &Self) -> f64 {
                (self.distance_squared(other) as f64).sqrt()
            }
        }

    };
    (shift $A:ty) => {
        implement_part!($A, Shl, shl, <<);
        implement_part!(assign $A, ShlAssign, shl_assign, <<=);

        implement_part!($A, Shr, shr, >>);
        implement_part!(assign $A, ShrAssign, shr_assign, >>=);
    };
    (rem $A:ty) => {
        implement_part!(rem $A, $A, $A);
    }
}

pub trait PositiveRem<Input>: Sized {
    type Output;
    /// Returns the positive remainder of a division
    /// ## Example ##
    /// ```rs
    /// assert_eq!(-17i32 % 16i32, -1i32);
    /// assert_eq!(-17i32.positive_rem(16i32), 15i32);
    /// ```
    fn positive_rem(self, with: Input) -> Self::Output;
}

pub trait Length<Ret>: Sized {
    /// Returns the length of this structure
    fn len(&self) -> Ret;
}

pub trait Distance<Ret>: Sized {
    /// Returns the distance of this structure to another one of the same type
    fn distance(&self, other: &Self) -> Ret;
}

pub trait Normalize: Sized {
    /// Returns a normalized `Vec3d`, so that its length is equal to 1
    fn normalize(self) -> Self;
    /// Normalizes this `Vec3d`, so that its length is equal to 1
    fn normalize_assign(&mut self);
}

macro_rules! implement_normalize {
    ($A:ty) => {
        impl Normalize for Vec3d<$A> {
            fn normalize(self) -> Self {
                let len: $A = self.len();
                self / len
            }
            fn normalize_assign(&mut self) {
                *self /= Length::<$A>::len(self);
            }
        }
    };
}

implement_traits!(u8);
implement_traits!(i8);
implement_traits!(u16);
implement_traits!(i16);
implement_traits!(u32);
implement_traits!(i32);
implement_traits!(u64);
implement_traits!(i64);

implement_traits!(shift u8);
implement_traits!(shift i8);
implement_traits!(shift u16);
implement_traits!(shift i16);
implement_traits!(shift u32);
implement_traits!(shift i32);
implement_traits!(shift u64);
implement_traits!(shift i64);

implement_traits!(rem i8);
implement_traits!(rem i16);
implement_traits!(rem i32);
implement_traits!(rem i64);
implement_traits!(rem f32);
implement_traits!(rem f64);

implement_traits!(f32);
implement_traits!(f64);
implement_normalize!(f32);
implement_normalize!(f64);