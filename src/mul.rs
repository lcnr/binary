use super::*;

impl<T> Mul<Zero> for T {
    type Result = Zero;
}

impl<T: Integer, U: Integer> Mul<B0<U>> for T
where
    T: Mul<U>,
{
    type Result = B0<<T as Mul<U>>::Result>;
}

impl<T: Integer, U: Integer> Mul<B1<U>> for T
where
    T: Mul<U> + Add<B0<<T as Mul<U>>::Result>>,
{
    type Result = <T as Add<B0<<T as Mul<U>>::Result>>>::Result;
}
