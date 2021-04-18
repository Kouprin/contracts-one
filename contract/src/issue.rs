use std::convert::TryInto;

use crate::*;

#[derive(BorshDeserialize, BorshSerialize)]
pub struct Issue {
    pub contract_hash: ContractHash,

    pub description: String,
    pub url: String,
    pub level: IssueLevel,

    pub opened_by: UserId,
}

impl Issue {
    pub(crate) fn id(&self) -> IssueId {
        env::sha256(self.url.as_bytes()).try_into().unwrap()
    }
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct IssueView {
    pub issue_id: Base58CryptoHash,

    pub contract_hash: Base58CryptoHash,

    pub description: String,
    pub url: String,
    pub level: String,

    pub opened_by: UserId,
}

impl From<&Issue> for IssueView {
    fn from(i: &Issue) -> Self {
        Self {
            issue_id: i.id().into(),
            contract_hash: i.contract_hash.into(),
            description: i.description.clone(),
            url: i.url.clone(),
            level: (&i.level).into(),

            opened_by: i.opened_by.clone(),
        }
    }
}
