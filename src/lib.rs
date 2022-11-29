mod types;
use std::collections::{HashMap, BTreeMap};
use ic_cdk::export::{
    candid::{Deserialize},
    serde::Serialize,
    Principal,
};
use std::cell::RefCell;

use types::VideoStreamingCompany;

thread_local!{
    static VIDEO: RefCell<VideoStreamingCompany> = RefCell::default();
    static USERS_COUNT: RefCell<u128> = RefCell::new(0u128);
    static USER_STORE: RefCell<BTreeMap<Principal, types::User>> = RefCell::new(BTreeMap::new())
}

fn get_caller() -> Principal{
    ic_cdk::caller()
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
    fn new() -> VideoStreamingCompany{
        VideoStreamingCompany {
            user_store: HashMap::default(),
            user_count: 0u128,
        }
    }

    fn _get_user_count() -> u128 {
        0u128
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_one() {
        let mut company = VideoStreamingCompany::new();
        company.user_store.insert(get_caller(), types::User { name: ("lee".to_string()), wallet_id: (1i64), token_number: (10.2) });
    }
}
