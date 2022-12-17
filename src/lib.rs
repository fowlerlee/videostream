mod types;
use ic_cdk::api::caller as caller_api;
use ic_cdk::export::candid::Error;
use ic_cdk::export::candid::types::ic_types::principal;
use ic_cdk::export::{candid::Deserialize, serde::Serialize, Principal};
use ic_cdk_macros::*;
use std::cell::RefCell;
use std::collections::{BTreeMap, HashMap};

use types::{VideoStreamingCompany, User, Result};

thread_local! {
    static VIDEO: RefCell<VideoStreamingCompany> = RefCell::default();
    static USERS_COUNT: RefCell<u128> = RefCell::new(0u128);
    static USER_STORE: RefCell<BTreeMap<Principal, types::User>> = RefCell::new(BTreeMap::new())
}

fn get_caller() -> Principal {
    let caller = caller_api();
    if caller == Principal::anonymous() {
        panic!("Anonymous principal not allowed to make calls.")
    }
    caller
}

impl Default for VideoStreamingCompany {
    fn default() -> Self {
        VideoStreamingCompany {
            user_store: HashMap::default(),
            user_count: 0u128,
            
        }
    }
}

impl VideoStreamingCompany {
    pub fn new() -> Self {
        VideoStreamingCompany {
            user_store: HashMap::default(),
            user_count: 0u128,
        
        }
    }
}

fn is_user_registered(user_principal: Principal) -> bool {
    USER_STORE.with(|s|s.borrow().contains_key(&user_principal))
}

fn delete_user(user_principal: Principal) {
    let caller = get_caller();
    assert_eq!(caller, user_principal);
    assert!(is_user_registered(user_principal));

    if let Some(value) = USER_STORE.with(|users|users.borrow_mut().clone().remove(&user_principal)){
        //handle case user is removed
    } else {
        //handle case user is not present - do nothing
    }
}

fn get_user_count() -> u128 {
    USER_STORE.with(|size| size.borrow().keys().len()) as u128
}

// fn get_user(user: Principal) -> Result<String, Option<types::User>> {
//        if is_user_registered(user) {
//             USER_STORE.with(|s| 
//     //             {s.borrow_mut().get(&user)}
//     //         )
//     //    }else {
//     //        return Err("().to_string")
//     //    }

// }

#[update(name = "add_note")]
fn add_user(user_principal: Principal, user: types::User) {
    USER_STORE.with(|us| {
        if let Some(_item) = us.borrow_mut().get_mut(&user_principal) {
            // the user already exists in the storage
        } else {
            us.borrow_mut().insert(user_principal, user);
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_one() {
        let mut company = VideoStreamingCompany::new();
        company.user_store.insert(
            get_caller(),
            types::User {
                name: ("lee".to_string()),
                wallet_id: (1i64),
                token_number: (10.2),
            },
        );
    }

    #[test]
    fn test_get_user_count(){
        let x = get_user_count();
        let y = USER_STORE.with(|s|s.borrow().keys().len()) as u128;
        assert_eq!(x, y)
    }
    
    #[test]
    fn test_add_user() {
        let x = get_caller();
        let y = types::User {
            name: "lee".to_string(),
            wallet_id: 1i64,
            token_number: 5.0,
        };
        USER_STORE.with(|v| v.borrow_mut().insert(x, y));
        // let mut n = USER_STORE.with(|m|m.borrow_mut().get_key_value(&x));
    }
}
