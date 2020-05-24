use super::*;

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

impl<T: Integer> OverflowingSub<Z> for B0<T> {
    type Result = B0<T>;

    type Overflow = False;
}

impl<T: Integer> OverflowingSub<Z> for B1<T> {
    type Result = B1<T>;

    type Overflow = False;
}

impl<T: Integer> OverflowingSub<B0<T>> for Z
where
    Z: OverflowingSub<T>,
{
    type Result = Z;

    type Overflow = <Z as OverflowingSub<T>>::Overflow;
}

impl<T: Integer> OverflowingSub<B1<T>> for Z {
    type Result = Z;

    type Overflow = True;
}

impl OverflowingSub<Z> for Z {
    type Result = Z;

    type Overflow = False;
}

impl<T, U> Sub<U> for T
where
    T: OverflowingSub<U>,
    (Z, <T as OverflowingSub<U>>::Result): If<<T as OverflowingSub<U>>::Overflow>,
    <(Z, <T as OverflowingSub<U>>::Result) as If<<T as OverflowingSub<U>>::Overflow>>::Result:
        Integer,
{
    type Result =
        <(Z, <T as OverflowingSub<U>>::Result) as If<<T as OverflowingSub<U>>::Overflow>>::Result;
}
