use std::collections::HashMap;
use std::cell::RefCell;
use std::rc::Rc;
use ic_cdk::export::{candid, Principal};

type Token = i64;

pub struct User {
    pub name: String,
    pub wallet_id: i64,
    pub token_number: f64,
}

pub struct VideoStreamingCompany {
    pub user_registry: Option<Rc<RefCell<HashMap<Principal,  User>>>>,
    pub user_count: u128,
}
