//! Binary numbers at compile time.

use std::marker::PhantomData;

mod add;
mod eq;
mod mul;
mod normalize;
mod shift;
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

pub trait Eq<T> {
    type Output: Boolean;
}

#[macro_export]
macro_rules! iff {
    ($cond:ty, $tru:ty, $fal:ty $(,)?) => {
        <($tru, $fal) as If<$cond>>::Result
    };
}

#[macro_export]
macro_rules! eq {
    ($a:ty, $b:ty $(,)?) => {
        <$a as Eq<$b>>::Output
    };
}

/// The terminator for a binary number.
pub struct Zero;
/// Binary 0.
pub struct B0<T: Integer>(PhantomData<T>);
/// Binary 1.
pub struct B1<T: Integer = Zero>(PhantomData<T>);

#[macro_export]
macro_rules! int {
    () => { Zero };
    (0 $($t:tt)*) => {
        int!($($t)*)
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

#[macro_export]
macro_rules! shl {
    ($a:ty, $b:ty $(,)?) => {
        <$a as Shl<<$b as Normalize>::Normalized>>::Result
    };
}

#[macro_export]
macro_rules! shr {
    ($a:ty, $b:ty $(,)?) => {
        <$a as Shr<<$b as Normalize>::Normalized>>::Result
    };
}

#[macro_export]
macro_rules! mul {
    ($a:ty, $b:ty $(,)?) => {
        <$a as Mul<$b>>::Result
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

impl Integer for Zero {
    const VALUE: u128 = 0;

    type Succ = B1;

    type Pred = Zero;
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

/// Shift left.
pub trait Shl<T> {
    type Result: Integer;
}

/// Shift right.
///
/// Note that `T` must be normalized, as this may otherwise return incorrect results,
/// consider using the `shr` macro, which normalizes automatically.
pub trait Shr<T> {
    type Result: Integer;
}

/// Multiplication.
pub trait Mul<T> {
    type Result: Integer;
}

/// Any `B0` which do not contain a single `B1` are useless and can be discarded.
pub trait Normalize: Integer {
    type Normalized: Integer;
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! eq {
        ($ty:ty, $expected:expr $(,)?) => {
            assert_eq!(<$ty>::VALUE, $expected);
        };
    }

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
        assert_eq!(<<Zero as Integer>::Succ>::VALUE, 1);
    }

    #[test]
    fn pred() {
        assert_eq!(<<B1<B0<B1>> as Integer>::Pred>::VALUE, 4);
        assert_eq!(<pred!(int!(1 0 0 1 0))>::VALUE, 17);
        assert_eq!(<<B0<B1> as Integer>::Pred>::VALUE, 1);
        assert_eq!(<<Zero as Integer>::Pred>::VALUE, 0);
        eq!(pred!(int!(1 0)), 1);
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
        assert_eq!(<<int!(1 1) as Sub<Zero>>::Result>::VALUE, 3);
    }

    #[test]
    fn shl() {
        eq!(shl!(int!(1 0 0), int!(0)), 0b100);
        eq!(shl!(int!(1 0 0 1), int!(1 0)), 0b100100);
        eq!(shl!(int!(1 0), int!(1 1)), 0b10000);
    }

    #[test]
    fn shr() {
        eq!(shr!(int!(1 0 0), int!(0)), 0b100);
        eq!(shr!(int!(1 0 0 1), int!(1 0)), 0b10);
        eq!(shr!(int!(1 0), int!(1 1)), 0);
    }

    #[test]
    fn mul() {
        eq!(mul!(int!(0), int!(1 0 1 0 0 1)), 0);
        eq!(mul!(int!(1), int!(1 0 1 0 0 1)), 0b101001);
        eq!(mul!(int!(1 0 1 0), int!(1 0 0 1)), 90);
        eq!(mul!(int!(1 0 0 1 0), int!(0)), 0);
    }
}
