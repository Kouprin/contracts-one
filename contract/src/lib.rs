use regex::Regex;
use std::cmp::min;
use std::convert::TryInto;

mod audit;
mod certificate;
mod contract;
mod primitives;
mod project;
mod user;
mod version;

pub use crate::audit::*;
pub use crate::certificate::*;
pub use crate::contract::*;
pub use crate::primitives::*;
pub use crate::project::*;
pub use crate::user::*;
pub use crate::version::*;

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupMap, LookupSet, TreeMap, UnorderedMap, UnorderedSet};
use near_sdk::json_types::{Base58CryptoHash, Base58PublicKey, ValidAccountId, WrappedTimestamp};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{
    env, near_bindgen, AccountId, Balance, CryptoHash, PanicOnDefault, PublicKey, Timestamp,
};

near_sdk::setup_alloc!();

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Main {
    pub projects: UnorderedMap<ProjectId, Project>,
    pub contracts: TreeMap<ContractHash, Contract>,
    // users: LookupMap<UserId, User>,
    // audits: LookupMap<AuditId, Audit>,
    // council: LookupSet<UserId>,
}

#[near_bindgen]
impl Main {
    #[init]
    pub fn new(owner_id: ValidAccountId) -> Self {
        assert!(!env::state_exists(), "Already initialized");
        let mut main = Self {
            projects: UnorderedMap::new(b"a".to_vec()),
            contracts: TreeMap::new(b"b".to_vec()),
        };

        let mut user = main.extract_user_or_create(owner_id.as_ref());
        user.is_council = true;
        main.save_user_or_panic(owner_id.as_ref(), &user);

        let mut user = main.extract_user_or_create(&env::signer_account_id());
        user.is_council = true;
        main.save_user_or_panic(&env::signer_account_id(), &user);

        main
    }
}

impl Main {
    pub(crate) fn users() -> LookupMap<UserId, User> {
        LookupMap::new(b"c".to_vec())
    }

    pub(crate) fn audits() -> LookupMap<AuditId, Audit> {
        LookupMap::new(b"d".to_vec())
    }

    pub(crate) fn council() -> LookupSet<UserId> {
        LookupSet::new(b"e".to_vec())
    }

    pub(crate) fn is_council() -> bool {
        Self::council().contains(&env::predecessor_account_id())
    }

    pub(crate) fn assert_deposit(required_deposit: Balance) {
        assert!(
            env::attached_deposit() >= required_deposit,
            "{}: required {}, received {}",
            ERR_DEPOSIT_NOT_ENOUGH,
            required_deposit,
            env::attached_deposit()
        )
    }

    pub(crate) fn assert_one_yocto() {
        assert_eq!(env::attached_deposit(), 1, "Must be 1 yocto")
    }

    pub(crate) fn assert_council() {
        assert!(
            Self::is_council(),
            "{}: account {}",
            ERR_COUNCIL,
            env::predecessor_account_id()
        )
    }

    pub(crate) fn assert_text_len(text: &String) {
        assert!(
            text.len() < MAX_TEXT_LENGTH,
            "{}: length of {} is {}, max allowed length is {}",
            ERR_TEXT_TOO_LONG,
            text,
            text.len(),
            MAX_TEXT_LENGTH,
        )
    }

    pub(crate) fn assert_vec_len<T>(vec: &Vec<T>) {
        assert!(
            vec.len() < MAX_VEC_LENGTH,
            "{}: length of vector is {}, max allowed length is {}",
            ERR_VEC_TOO_LONG,
            vec.len(),
            MAX_VEC_LENGTH,
        )
    }

    pub(crate) fn assert_council_or_project_owner(&self, project_name: &String) {
        assert!(
            Self::is_council()
                || self
                    .projects
                    .get(&Project::get_id(project_name))
                    .unwrap()
                    .owners
                    .contains(&env::predecessor_account_id()),
            "{}: account {}",
            ERR_COUNCIL_OR_PROJECT_OWNER,
            env::predecessor_account_id()
        )
    }
}
