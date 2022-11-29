use std::{collections::HashMap, str};
use std::cell::RefCell;
use std::rc::Rc;
use ic_cdk::export::{
    candid::{CandidType, Deserialize},
    serde::Serialize,
    Principal,
};

type Token = i64;

#[derive(Clone, CandidType, Serialize, Deserialize, Default)]
pub struct User {
    pub name: String,
    pub wallet_id: i64,
    pub token_number: f64,
}
#[derive(Clone, CandidType, Serialize, Deserialize, Default)]
pub struct Video{
    pub name: String,
    pub id: i128,
}

type UserStore = HashMap<Principal,  User>;

#[derive(Clone, CandidType, Serialize, Deserialize)]
pub struct VideoStreamingCompany {
    pub user_store: UserStore,
    pub user_count: u128,
}
