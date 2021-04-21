use crate::*;

const STATE_KEY: &[u8] = b"STATE";

pub const ERR_DEPOSIT_NOT_ENOUGH: &str = "Attached deposit is not enough";
pub const ERR_PROJECT_NAME_INVALID: &str = "Project name is invalid";
pub const ERR_NOT_AN_AUDITOR: &str = "Audits can be submitted by auditors only";
pub const ERR_INVALID_REPORT_URL: &str = "Report cannot be used more than once";
pub const ERR_NOT_AN_OWNER: &str = "Only contracts.one owner can do this operation";
pub const ERR_ALREADY_EXISTS: &str = "Already exists";
pub const ERR_ACCESS_DENIED: &str = "Caller is not allowed to do this operation";
pub const ERR_PROJECT_CREATOR_IS_NOT_OWNER: &str =
    "Project creator is not in list of project owners";
pub const ERR_TEXT_TOO_LONG: &str = "Text field is limited for MAX_TEXT_LENGTH symbols";
pub const ERR_INVALID_SCORE: &str = "The score is invalid";

pub const SAFETY_LEVEL_LOW: &str = "Low";
pub const SAFETY_LEVEL_LOW_EXPLANATION: &str = "The contract hasn't been audited or audits don't approve safety of the contract. Use it on you own risk.";
pub const SAFETY_LEVEL_MODERATE: &str = "Moderate";
pub const SAFETY_LEVEL_MODERATE_EXPLANATION: &str = "The contract has been audited but some auditors don't approve safety of the contract. Use it on you own risk.";
pub const SAFETY_LEVEL_HIGH: &str = "High";
pub const SAFETY_LEVEL_HIGH_EXPLANATION: &str =
    "The contract has been audited and auditors approved safety of the contract.";

pub const ISSUE_LEVEL_CRITICAL: &str = "Critical";
pub const ISSUE_LEVEL_MAJOR: &str = "Major";
pub const ISSUE_LEVEL_MEDIUM: &str = "Medium";
pub const ISSUE_LEVEL_MINOR: &str = "Minor";

pub const REGISTER_PROJECT_DEPOSIT: Balance = 1;
pub const REGISTER_AUDITOR_DEPOSIT: Balance = 1;
pub const SIGN_AUDIT_DEPOSIT: Balance = 1;
pub const CREATE_USER_DEPOSIT: Balance = 1;

pub const MAX_TEXT_LENGTH: usize = 1023;

pub type CertificateId = CryptoHash;
// pub type ContractId = (ProjectId, Version); - unused
pub type ContractHash = CryptoHash;
pub type IssueId = CryptoHash;
pub type ProjectId = CryptoHash;
pub type Standard = String;
pub type Url = String;
pub type UserId = AccountId;

pub trait ReadFromState<T> {
    fn read_from_state(&self) -> T;
}

impl ReadFromState<Project> for ProjectId {
    fn read_from_state(&self) -> Project {
        // TODO how heavy is it?
        Global::try_from_slice(&env::storage_read(STATE_KEY).unwrap())
            .unwrap()
            .projects
            .get(self)
            .unwrap()
    }
}

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

#[derive(BorshDeserialize, BorshSerialize)]
pub struct AuditRequest {
    pub price: Balance,
    pub time: Timestamp,
}

#[derive(BorshDeserialize, BorshSerialize)]
pub enum AuditStatus {
    Unknown,
    Unaudited,
    Audited,
    Challenged,
    OnChallenge(Issue),
    Requested(AuditRequest),
    Responded(AuditRequest, Vec<(UserId, Timestamp)>),
    InProgress(AuditRequest, UserId, Timestamp),
}

impl From<&AuditStatus> for String {
    fn from(a: &AuditStatus) -> Self {
        match a {
            AuditStatus::Unknown => format!("Unknown"),
            AuditStatus::Unaudited => format!("Unaudited"),
            AuditStatus::Audited => format!("Audited"),
            AuditStatus::Challenged => format!("Challenged"),
            AuditStatus::OnChallenge(_) => format!("On challenge"),
            AuditStatus::Requested(_) => format!("Audit requested"),
            AuditStatus::Responded(_, v) => format!("{:?} audit responses", v.len()),
            AuditStatus::InProgress(_, _, _) => format!("Audit in progress"),
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
