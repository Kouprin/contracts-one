mod certificate;
mod contract;
mod primitives;
mod project;
mod user;
mod version;

pub use crate::certificate::*;
pub use crate::contract::*;
pub use crate::primitives::*;
pub use crate::project::*;
pub use crate::user::*;
pub use crate::version::*;

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{TreeMap, UnorderedMap, UnorderedSet};
use near_sdk::json_types::{Base58CryptoHash, ValidAccountId, WrappedTimestamp};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{env, near_bindgen, AccountId, Balance, CryptoHash, PanicOnDefault, Timestamp};

near_sdk::setup_alloc!();

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Global {
    pub projects: UnorderedMap<ProjectId, Project>,
    pub contract_hash_to_contract_id: UnorderedMap<ContractHash, (ProjectId, Version)>,

    pub users: UnorderedMap<UserId, User>,
    pub auditors: UnorderedSet<UserId>,
    pub certificate_id_to_contract_hash: UnorderedMap<CertificateId, ContractHash>,

    pub owner_id: AccountId,
    pub dao_id: AccountId,
}

#[near_bindgen]
impl Global {
    #[init]
    pub fn test_new() -> Self {
        assert!(!env::state_exists(), "Already initialized");
        Self {
            projects: UnorderedMap::new(b"a".to_vec()),
            contract_hash_to_contract_id: UnorderedMap::new(b"b".to_vec()),
            users: UnorderedMap::new(b"d".to_vec()),
            auditors: UnorderedSet::new(b"e".to_vec()),
            certificate_id_to_contract_hash: UnorderedMap::new(b"f".to_vec()),
            owner_id: env::signer_account_id(),
            dao_id: env::signer_account_id(),
        }
    }

    #[init]
    pub fn new(owner_id: ValidAccountId, dao_id: ValidAccountId) -> Self {
        assert!(!env::state_exists(), "Already initialized");
        Self {
            projects: UnorderedMap::new(b"a".to_vec()),
            contract_hash_to_contract_id: UnorderedMap::new(b"b".to_vec()),
            users: UnorderedMap::new(b"d".to_vec()),
            auditors: UnorderedSet::new(b"e".to_vec()),
            certificate_id_to_contract_hash: UnorderedMap::new(b"f".to_vec()),
            owner_id: owner_id.into(),
            dao_id: dao_id.into(),
        }
    }
}
