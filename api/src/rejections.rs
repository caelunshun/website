use warp::{
    reject::{custom, Reject},
    Rejection,
};

#[derive(Debug)]
pub struct Unauthorized;

impl Reject for Unauthorized {}

pub fn unauthorized() -> Rejection {
    custom(Unauthorized)
}

#[derive(Debug)]
pub struct Database;

impl Reject for Database {}

pub fn database() -> Rejection {
    custom(Database)
}
