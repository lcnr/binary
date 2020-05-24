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

impl Eq<Zero> for Zero {
    type Output = True;
}

impl<T: Integer> Eq<B1<T>> for Zero {
    type Output = False;
}

impl<T: Integer> Eq<Zero> for B1<T> {
    type Output = False;
}

impl<T: Integer + Eq<Zero>> Eq<B0<T>> for Zero {
    type Output = eq!(T, Zero);
}

impl<T: Integer + Eq<Zero>> Eq<Zero> for B0<T> {
    type Output = eq!(T, Zero);
}

impl<T: Integer + Eq<U>, U: Integer> Eq<B0<U>> for B0<T> {
    type Output = eq!(T, U);
}

impl<T: Integer, U: Integer> Eq<B1<U>> for B0<T> {
    type Output = False;
}

impl<T: Integer, U: Integer> Eq<B0<U>> for B1<T> {
    type Output = False;
}

impl<T: Integer + Eq<U>, U: Integer> Eq<B1<U>> for B1<T> {
    type Output = eq!(T, U);
}
