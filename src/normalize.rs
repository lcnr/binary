use super::*;

impl Normalize for Zero {
    type Normalized = Zero;
}

impl<T: Normalize> Normalize for B1<T> {
    type Normalized = B1<T::Normalized>;
}

impl<T: Normalize> Normalize for B0<T>
where
    Zero: Eq<<T as Normalize>::Normalized>,
    (Zero, B0<<T as Normalize>::Normalized>):
        If<<Zero as Eq<<T as Normalize>::Normalized>>::Output>,
    <(Zero, B0<<T as Normalize>::Normalized>) as If<
        <Zero as Eq<<T as Normalize>::Normalized>>::Output,
    >>::Result: Integer,
{
    type Normalized =
        <(Zero, B0<T::Normalized>) as If<<Zero as Eq<T::Normalized>>::Output>>::Result;
}
