use super::*;

pub trait OverflowingSub<T> {
    type Result: Integer;

    type Overflow: Boolean;
}

impl<T: Integer, U: Integer> OverflowingSub<B0<U>> for B0<T>
where
    T: OverflowingSub<U>,
{
    type Result = B0<<T as OverflowingSub<U>>::Result>;

    type Overflow = <T as OverflowingSub<U>>::Overflow;
}

impl<T: Integer, U: Integer> OverflowingSub<B0<U>> for B1<T>
where
    T: OverflowingSub<U>,
{
    type Result = B1<<T as OverflowingSub<U>>::Result>;

    type Overflow = <T as OverflowingSub<U>>::Overflow;
}

impl<T: Integer, U: Integer> OverflowingSub<B1<U>> for B0<T>
where
    T::Pred: OverflowingSub<U>,
{
    type Result = B1<<T::Pred as OverflowingSub<U>>::Result>;

    type Overflow = <T::Pred as OverflowingSub<U>>::Overflow;
}

impl<T: Integer, U: Integer> OverflowingSub<B1<U>> for B1<T>
where
    T: OverflowingSub<U>,
{
    type Result = B0<<T as OverflowingSub<U>>::Result>;

    type Overflow = <T as OverflowingSub<U>>::Overflow;
}

impl<T: Integer> OverflowingSub<Zero> for B0<T> {
    type Result = B0<T>;

    type Overflow = False;
}

impl<T: Integer> OverflowingSub<Zero> for B1<T> {
    type Result = B1<T>;

    type Overflow = False;
}

impl<T: Integer> OverflowingSub<B0<T>> for Zero
where
    Zero: OverflowingSub<T>,
{
    type Result = Zero;

    type Overflow = <Zero as OverflowingSub<T>>::Overflow;
}

impl<T: Integer> OverflowingSub<B1<T>> for Zero {
    type Result = Zero;

    type Overflow = True;
}

impl OverflowingSub<Zero> for Zero {
    type Result = Zero;

    type Overflow = False;
}

impl<T, U> Sub<U> for T
where
    T: OverflowingSub<U>,
    (Zero, <T as OverflowingSub<U>>::Result): If<<T as OverflowingSub<U>>::Overflow>,
    <(Zero, <T as OverflowingSub<U>>::Result) as If<<T as OverflowingSub<U>>::Overflow>>::Result:
        Normalize,
{
    type Result = <iff!(
        <T as OverflowingSub<U>>::Overflow,
        Zero,
        <T as OverflowingSub<U>>::Result
    ) as Normalize>::Normalized;
}
