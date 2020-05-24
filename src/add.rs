use super::*;

impl<T: Integer, U: Integer> Add<B0<U>> for B0<T>
where
    T: Add<U>,
{
    type Result = B0<<T as Add<U>>::Result>;
}

impl<T: Integer, U: Integer> Add<B0<U>> for B1<T>
where
    T: Add<U>,
{
    type Result = B1<<T as Add<U>>::Result>;
}

impl<T: Integer, U: Integer> Add<B1<U>> for B0<T>
where
    T: Add<U>,
{
    type Result = B1<<T as Add<U>>::Result>;
}

impl<T: Integer, U: Integer> Add<B1<U>> for B1<T>
where
    T: Add<U::Succ>,
{
    type Result = B0<<T as Add<U::Succ>>::Result>;
}

impl<T: Integer> Add<Zero> for B0<T> {
    type Result = B0<T>;
}

impl<T: Integer> Add<Zero> for B1<T> {
    type Result = B1<T>;
}

impl<T: Integer> Add<B0<T>> for Zero {
    type Result = B0<T>;
}

impl<T: Integer> Add<B1<T>> for Zero {
    type Result = B1<T>;
}

impl Add<Zero> for Zero {
    type Result = Zero;
}
