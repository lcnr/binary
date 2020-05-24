//! Binary numbers at compile time.

use std::marker::PhantomData;

mod add;
mod eq;
mod normalize;
mod sub;

/// Type level `true`.
pub struct True;
/// Type level `false`.
pub struct False;
/// A type level boolean, either `true` or `false`.
pub trait Boolean {}

impl Boolean for True {}
impl Boolean for False {}

/// Branching, implemented for `(T, U)`,
/// meaning `(then, else)`.
pub trait If<T: Boolean> {
    type Result;
}

impl<T, U> If<True> for (T, U) {
    type Result = T;
}

impl<T, U> If<False> for (T, U) {
    type Result = U;
}

/// The terminator for a binary number.
pub struct Z;
/// Binary 0.
pub struct B0<T: Integer>(PhantomData<T>);
/// Binary 1.
pub struct B1<T: Integer = Z>(PhantomData<T>);

#[macro_export]
macro_rules! int {
    (0 $($t:tt)*) => {
        int!(@inner B0, $($t)*)
    };
    (1 $($t:tt)*) => {
        int!(@inner B1, $($t)*)
    };
    (@inner $ty:ty,) => { $ty };
    (@inner $ty:ty, 0 $($t:tt)*) => {
        int!(@inner B0<$ty>, $($t)*)
    };
    (@inner $ty:ty, 1 $($t:tt)*) => {
        int!(@inner B1<$ty>, $($t)*)
    };
}

#[macro_export]
macro_rules! succ {
    ($ty:ty) => {
        <$ty as Integer>::Succ
    };
}

#[macro_export]
macro_rules! pred {
    ($ty:ty) => {
        <$ty as Integer>::Pred
    };
}

#[macro_export]
macro_rules! add {
    ($a:ty, $b:ty $(,)?) => {
        <$a as Add<$b>>::Result
    };
}

#[macro_export]
macro_rules! sub {
    ($a:ty, $b:ty $(,)?) => {
        <$a as Sub<$b>>::Result
    };
}

/// A type representing an integer.
pub trait Integer {
    const VALUE: u128;

    /// The successor of this number.
    ///
    /// ```rust
    /// use binary::*;
    ///
    /// assert_eq!(<<B0<B0<B1>> as Integer>::Succ>::VALUE, 5);
    /// ```
    type Succ: Integer;

    /// The predecessor of this number, saturating at zero.
    /// ```rust
    /// use binary::*;
    ///
    /// assert_eq!(<<B1<B0<B0<B1>>> as Integer>::Pred>::VALUE, 8);
    /// ```
    type Pred: Integer;
}

impl Integer for Z {
    const VALUE: u128 = 0;

    type Succ = B1;

    type Pred = Z;
}

impl<T: Integer> Integer for B0<T> {
    const VALUE: u128 = T::VALUE << 1;

    type Succ = B1<T>;

    type Pred = B1<T::Pred>;
}

impl<T: Integer> Integer for B1<T> {
    const VALUE: u128 = (T::VALUE << 1) + 1;

    type Succ = B0<T::Succ>;

    type Pred = B0<T>;
}

/// Adding two integers.
/// ```rust
/// use binary::*;
///
/// assert_eq!(<<B1<B0<B0<B1>>> as Add<B0<B1<B0<B1>>>>>::Result>::VALUE, 19);
/// ```
pub trait Add<T> {
    type Result: Integer;
}

/// Saturating substraction.
/// ```rust
/// use binary::*;
///
/// assert_eq!(<<B1<B0<B1<B1>>> as Sub<B0<B1<B0<B1>>>>>::Result>::VALUE, 3);
/// ```
pub trait Sub<T> {
    type Result: Integer;
}

/// Overflowing substraction.
/// ```rust
/// use binary::*;
///
/// assert_eq!(<<(B1, Z) as If<<int!(1 1) as OverflowingSub<int!(1 0 1)>>::Overflow>>::Result>::VALUE, 1);
/// assert_eq!(<<(B1, Z) as If<<int!(1 1 1) as OverflowingSub<int!(1 0 1)>>::Overflow>>::Result>::VALUE, 0);
/// ```
pub trait OverflowingSub<T> {
    type Result: Integer;

    type Overflow: Boolean;
}

/// Any `B0` which do not contain a single `B1` are useless and can be discarded.
pub trait Normalize: Integer {
    type Normalized: Integer;
}

pub trait Eq<T> {
    type Output: Boolean;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nums() {
        assert_eq!(<B1<B0<B1>>>::VALUE, 5);
        assert_eq!(<B0<B1<B0<B0<B1>>>>>::VALUE, 18);
        assert_eq!(<B0<B1>>::VALUE, 2);
    }

    #[test]
    fn int() {
        assert_eq!(<int!(1 0 1)>::VALUE, 5);
        assert_eq!(<int!(1 0 0 1 0 1)>::VALUE, 37);
        assert_eq!(<int!(1)>::VALUE, 1);
    }

    #[test]
    fn succ() {
        assert_eq!(<<B1<B0<B1>> as Integer>::Succ>::VALUE, 6);
        assert_eq!(<<B0<B1<B0<B0<B1>>>> as Integer>::Succ>::VALUE, 19);
        assert_eq!(<<B0<B1> as Integer>::Succ>::VALUE, 3);
        assert_eq!(<<Z as Integer>::Succ>::VALUE, 1);
    }

    #[test]
    fn pred() {
        assert_eq!(<<B1<B0<B1>> as Integer>::Pred>::VALUE, 4);
        assert_eq!(<pred!(int!(1 0 0 1 0))>::VALUE, 17);
        assert_eq!(<<B0<B1> as Integer>::Pred>::VALUE, 1);
        assert_eq!(<<Z as Integer>::Pred>::VALUE, 0);
    }

    #[test]
    fn add() {
        assert_eq!(<<B1<B0<B1<B1>>> as Add<B0<B1<B0<B1>>>>>::Result>::VALUE, 23);
        assert_eq!(<<B1<B0<B1>> as Add<B0<B1<B0<B1>>>>>::Result>::VALUE, 15);
        assert_eq!(<<succ!(int!(1 0 0 1 0)) as Add<B1>>::Result>::VALUE, 20);
        assert_eq!(<add!(int!(1 0 0 0 0), int!(1 1 1 1))>::VALUE, 31);
    }

    #[test]
    fn sub() {
        assert_eq!(<<int!(1 1 0 1) as Sub<int!(1 0 1 1)>>::Result>::VALUE, 2);
        assert_eq!(<<int!(1 0 1) as Sub<int!(1 0 1 0)>>::Result>::VALUE, 0);
        assert_eq!(<<int!(1 1) as Sub<Z>>::Result>::VALUE, 3);
    }
}
