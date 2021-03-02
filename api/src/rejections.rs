use warp::{
    reject::{custom, Reject},
    Rejection,
};

pub trait IntoRejection<E>: Reject {
    fn reject(error: E) -> Rejection;
}

impl<E, T> IntoRejection<E> for T
where
    T: From<E> + Reject
{
    fn reject(error: E) -> Rejection {
        custom(Self::from(error))
    }
}

#[derive(Debug)]
pub struct Unauthorized;

impl Reject for Unauthorized {}

pub fn unauthorized() -> Rejection {
    custom(Unauthorized)
}

#[derive(Debug)]
pub struct Database(sqlx::Error);

impl Reject for Database {}

impl From<sqlx::Error> for Database {
    fn from(error: sqlx::Error) -> Self {
        Database(error)
    }
}