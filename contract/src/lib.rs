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
use near_sdk::collections::{LookupMap, TreeMap, UnorderedMap, UnorderedSet};
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
    pub owner_id: AccountId,
    pub dao_id: AccountId,
}

#[near_bindgen]
impl Main {
    #[init]
    pub fn test_new() -> Self {
        assert!(!env::state_exists(), "Already initialized");
        Self {
            projects: UnorderedMap::new(b"a".to_vec()),
            contracts: TreeMap::new(b"b".to_vec()),
            owner_id: env::signer_account_id(),
            dao_id: env::signer_account_id(),
        }
    }

    #[init]
    pub fn new(owner_id: ValidAccountId, dao_id: ValidAccountId) -> Self {
        assert!(!env::state_exists(), "Already initialized");
        Self {
            projects: UnorderedMap::new(b"a".to_vec()),
            contracts: TreeMap::new(b"b".to_vec()),
            owner_id: owner_id.into(),
            dao_id: dao_id.into(),
        }
    }
}

impl Main {
    pub(crate) fn users() -> LookupMap<UserId, User> {
        LookupMap::new(b"c".to_vec())
    }

    pub(crate) fn audits() -> LookupMap<AuditId, Audit> {
        LookupMap::new(b"d".to_vec())
    }
}
