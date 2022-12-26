use std::{collections::HashMap, str};
use std::cell::RefCell;
use std::rc::Rc;
use std::borrow::Cow;
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

#[derive(Clone, CandidType, Serialize, Deserialize, Default)]
pub struct StoredVideo{
    pub store: Box<HashMap<i128, u8>>,
}

type UserStore = HashMap<Principal,  User>;

#[derive(Clone, CandidType, Serialize, Deserialize)]
pub struct VideoStreamingCompany {
    pub user_store: UserStore,
    pub user_count: u128,
}

pub enum Result<E, T> {
    Err(E),
    Ok(T)
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug, Hash)]
pub struct TransferArgs {
    amount: Token,
    to_principal: Principal,
    // to_subaccount: Option<Subaccount>,
}

#[derive(CandidType, Deserialize)]
struct HttpRequest {
    method: String,
    url: String,
    headers: HashMap<String, String>,
    body: Vec<u8>,
}

#[derive(CandidType)]
struct HttpResponse<'a> {
    status_code: u16,
    headers: HashMap<&'a str, Cow<'a, str>>,
    body: Cow<'a, [u8]>,
}