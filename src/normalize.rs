use super::*;

impl Normalize for Z {
    type Normalized = Z;
}

impl<T: Normalize> Normalize for B1<T> {
    type Normalized = B1<T::Normalized>;
}

impl<T: Normalize> Normalize for B0<T>
where
    Z: Eq<<T as Normalize>::Normalized>,
    (Z, B0<<T as Normalize>::Normalized>): If<<Z as Eq<<T as Normalize>::Normalized>>::Output>,
    <(Z, B0<<T as Normalize>::Normalized>) as If<
        <Z as Eq<<T as Normalize>::Normalized>>::Output,
    >>::Result: Integer,
{
    type Normalized = <(Z, B0<T::Normalized>) as If<<Z as Eq<T::Normalized>>::Output>>::Result;
}
