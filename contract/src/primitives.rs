use crate::*;

pub const ERR_DEPOSIT_NOT_ENOUGH: &str = "Attached deposit is not enough";
pub const ERR_PROJECT_NAME_INVALID: &str = "Project name is invalid";
pub const ERR_INVALID_REPORT_URL: &str = "Report cannot be used more than once";
pub const ERR_NOT_AN_OWNER: &str = "Only owner can do this operation";
pub const ERR_ALREADY_EXISTS: &str = "Already exists";
pub const ERR_ACCESS_DENIED: &str = "Caller is not allowed to do this operation";
pub const ERR_EMPTY_CERTIFICATE: &str = "Nothing to certify";
pub const ERR_PROJECT_CREATOR_IS_NOT_OWNER: &str =
    "Project creator is not in list of project owners";
pub const ERR_TEXT_TOO_LONG: &str = "Text field is limited for MAX_TEXT_LENGTH symbols";
pub const ERR_INVALID_SCORE: &str = "The score is invalid";

pub(crate) fn assert_one_yocto() {
    assert_eq!(
        env::attached_deposit(),
        1,
        "Requires attached deposit of exactly 1 yoctoNEAR",
    )
}

pub const SAFETY_LEVEL_LOW: &str = "Low";
pub const SAFETY_LEVEL_LOW_EXPLANATION: &str =
    "The contract hasn't been audited or some issues have been found. Use it on you own risk.";
pub const SAFETY_LEVEL_MODERATE: &str = "Moderate";
pub const SAFETY_LEVEL_MODERATE_EXPLANATION: &str = "The contract has been audited and no issues have been found. However, there were no approval from NEAR experts. Use it on you own risk.";
pub const SAFETY_LEVEL_HIGH: &str = "High";
pub const SAFETY_LEVEL_HIGH_EXPLANATION: &str = "NEAR experts approved the contract is safe.";

pub const ISSUE_LEVEL_CRITICAL: &str = "Critical";
pub const ISSUE_LEVEL_MAJOR: &str = "Major";
pub const ISSUE_LEVEL_MEDIUM: &str = "Medium";
pub const ISSUE_LEVEL_MINOR: &str = "Minor";

pub const REGISTER_PROJECT_DEPOSIT: Balance = 1;
pub const SIGN_AUDIT_DEPOSIT: Balance = 1;
pub const CREATE_USER_DEPOSIT: Balance = 1;

pub const MAX_TEXT_LENGTH: usize = 1023;

pub type AuditId = CryptoHash;
// pub type ContractId = (ProjectId, Version); - unused
pub type ContractHash = CryptoHash;
pub type IssueId = CryptoHash;
pub type ProjectId = CryptoHash;
pub type Standard = String;
pub type Url = String;
pub type UserId = AccountId;

#[derive(BorshDeserialize, BorshSerialize)]
pub enum IssueLevel {
    Critical,
    Major,
    Medium,
    Minor,
}

impl From<&IssueLevel> for String {
    fn from(i: &IssueLevel) -> Self {
        match i {
            IssueLevel::Critical => ISSUE_LEVEL_CRITICAL.to_string(),
            IssueLevel::Major => ISSUE_LEVEL_MAJOR.to_string(),
            IssueLevel::Medium => ISSUE_LEVEL_MAJOR.to_string(),
            IssueLevel::Minor => ISSUE_LEVEL_MINOR.to_string(),
        }
    }
}

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
