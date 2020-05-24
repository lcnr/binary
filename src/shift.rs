use super::*;

impl<T: Integer> Shl<Zero> for T {
    type Result = T;
}

impl<T: Integer, U: Integer> Shl<B0<U>> for T
where
    <U as Integer>::Pred: Normalize,
    T: Shl<<B1<<U as Integer>::Pred> as Normalize>::Normalized>,
{
    type Result = B0<shl!(T, pred!(B0<U>))>;
}

impl<T: Integer, U: Integer> Shl<B1<U>> for T
where
    U: Normalize,
    Zero: Eq<<U as Normalize>::Normalized>,
    (Zero, B0<<U as Normalize>::Normalized>): If<eq!(Zero, <U as Normalize>::Normalized)>,
    T: Shl<
        iff!(
            eq!(Zero, <U as Normalize>::Normalized),
            Zero,
            B0<<U as Normalize>::Normalized>
        ),
    >,
    iff!(
        eq!(Zero, <U as Normalize>::Normalized),
        Zero,
        B0<<U as Normalize>::Normalized>
    ): Integer,
{
    type Result = B0<shl!(T, pred!(B1<U>))>;
}

impl<T: Integer> Shr<Zero> for T {
    type Result = T;
}

impl<T: Integer, U: Integer> Shr<B0<U>> for B0<T>
where
    <U as Integer>::Pred: Normalize,
    T: Shr<<<B0<U> as Integer>::Pred as Normalize>::Normalized>,
{
    type Result = shr!(T, pred!(B0<U>));
}

impl<T: Integer, U: Integer> Shr<B0<U>> for B1<T>
where
    <U as Integer>::Pred: Normalize,
    T: Shr<<<B0<U> as Integer>::Pred as Normalize>::Normalized>,
{
    type Result = shr!(T, pred!(B0<U>));
}

impl<T: Integer, U: Integer> Shr<B1<U>> for B0<T>
where
    U: Normalize,
    Zero: Eq<<U as Normalize>::Normalized>,
    T: Shr<pred!(B1<U>)>,
    T: Shr<
        iff!(
            eq!(Zero, <U as Normalize>::Normalized),
            Zero,
            B0<<U as Normalize>::Normalized>
        ),
    >,
    (Zero, B0<<U as Normalize>::Normalized>):
        If<<Zero as Eq<<U as Normalize>::Normalized>>::Output>,

    iff!(
        eq!(Zero, <U as Normalize>::Normalized),
        Zero,
        B0<<U as Normalize>::Normalized>
    ): Integer,
{
    type Result = shr!(T, pred!(B1<U>));
}

impl<T: Integer, U: Integer> Shr<B1<U>> for B1<T>
where
    T: Shr<<B1<U> as Integer>::Pred>,
{
    type Result = <T as Shr<<B1<U> as Integer>::Pred>>::Result;
}

impl<U: Integer> Shr<B0<U>> for Zero {
    type Result = Zero;
}

impl<U: Integer> Shr<B1<U>> for Zero {
    type Result = Zero;
}
