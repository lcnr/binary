use super::*;

impl Eq<True> for True {
    type Output = True;
}

impl Eq<False> for True {
    type Output = False;
}

impl Eq<True> for False {
    type Output = False;
}

impl Eq<False> for False {
    type Output = True;
}

impl Eq<Z> for Z {
    type Output = True;
}

impl<T: Integer> Eq<B1<T>> for Z {
    type Output = False;
}

impl<T: Integer> Eq<Z> for B1<T> {
    type Output = False;
}

impl<T: Integer + Eq<Z>> Eq<B0<T>> for Z {
    type Output = <T as Eq<Z>>::Output;
}

impl<T: Integer + Eq<Z>> Eq<Z> for B0<T> {
    type Output = <T as Eq<Z>>::Output;
}

impl<T: Integer + Eq<U>, U: Integer> Eq<B0<U>> for B0<T> {
    type Output = <T as Eq<U>>::Output;
}

impl<T: Integer, U: Integer> Eq<B1<U>> for B0<T> {
    type Output = False;
}

impl<T: Integer, U: Integer> Eq<B0<U>> for B1<T> {
    type Output = False;
}

impl<T: Integer + Eq<U>, U: Integer> Eq<B1<U>> for B1<T> {
    type Output = <T as Eq<U>>::Output;
}
