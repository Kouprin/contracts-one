use crate::*;

pub const ERR_REGISTER_PROJECT_DEPOSIT_NOT_ENOUGH: &str =
    "Attached deposit must be no less than REGISTER_PROJECT_DEPOSIT";
pub const ERR_PROJECT_NAME_INVALID: &str = "Project name is invalid";
pub const ERR_NOT_AN_AUDITOR: &str = "Audits can be submitted by auditors only";
pub const ERR_INVALID_REPORT_URL: &str = "Report cannot be used more than once";
pub const ERR_NOT_AN_OWNER: &str = "Only contracts.one owner can do this operation";
pub const ERR_ALREADY_EXISTS: &str = "Already exists";
pub const ERR_ACCESS_DENIED: &str = "Caller is not allowed to do this operation";
pub const ERR_PROJECT_CREATOR_IS_NOT_OWNER: &str =
    "Project creator is not in list of project owners";

pub const SAFETY_LEVEL_LOW: &str = "Low";
pub const SAFETY_LEVEL_LOW_EXPLANATION: &str = "The contract hasn't been audited or audits don't approve safety of the contract. Use it on you own risk.";
pub const SAFETY_LEVEL_MODERATE: &str = "Moderate";
pub const SAFETY_LEVEL_MODERATE_EXPLANATION: &str = "The contract has been audited but some auditors don't approve safety of the contract. Use it on you own risk.";
pub const SAFETY_LEVEL_HIGH: &str = "High";
pub const SAFETY_LEVEL_HIGH_EXPLANATION: &str =
    "The contract has been audited and auditors approved safety of the contract.";

#[derive(Deserialize, Serialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct SafetyReport {
    pub safety_level: String,
    pub safety_explanation: String,
}

impl SafetyReport {
    pub fn low() -> Self {
        Self {
            safety_level: SAFETY_LEVEL_LOW.to_string(),
            safety_explanation: SAFETY_LEVEL_LOW_EXPLANATION.to_string(),
        }
    }
    pub fn moderate() -> Self {
        Self {
            safety_level: SAFETY_LEVEL_MODERATE.to_string(),
            safety_explanation: SAFETY_LEVEL_MODERATE_EXPLANATION.to_string(),
        }
    }
    pub fn high() -> Self {
        Self {
            safety_level: SAFETY_LEVEL_HIGH.to_string(),
            safety_explanation: SAFETY_LEVEL_HIGH_EXPLANATION.to_string(),
        }
    }
}

pub const REGISTER_PROJECT_DEPOSIT: Balance = 100;
pub const REGISTER_AUDITOR_DEPOSIT: Balance = 150;
pub const SIGN_AUDIT_DEPOSIT: Balance = 200;
pub const CREATE_USER_DEPOSIT: Balance = 250;

pub type CertificateId = CryptoHash;
// pub type ContractId = (ProjectId, Version);
pub type ContractHash = CryptoHash;
pub type ProjectId = CryptoHash;
pub type Standard = String;
pub type Url = String;
pub type UserId = AccountId;
