use ic_cdk::storage;
mod types;
use std::rc::*;
use std::collections::HashMap::*;
use ic_cdk::export::{
    candid::{CandidType, Deserialize},
    serde::Serialize,
    Principal,
};

use types::VideoStreamingCompany;

thread_local!{
    static VIDEO: RefCell<VideoStreamingCompany> = RefCell::default();
    static USERS: RefCell<i128> = RefCell::new(i128);
}

fn caller() -> Principal{
    ic_cdk::caller
}

impl Default for VideoStreamingCompany {
    fn default() -> Self {
        VideoStreamingCompany {
            user_store: HashMap::default(),
            user_count: 0,
        }
    }
}

impl VideoStreamingCompany {
    fn new(user: User) -> VideoStreamingCompany{
        VideoStreamingCompany {
            user_store: HashMap::new(caller(), User),
            user_count: 0u128,
        }
    }

    fn get_user_count() -> u128 {

    }
}



#[cfg(test)]
mod tests {
    #[test]
    fn test_one() {
        
    }
}
