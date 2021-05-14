use crate::*;

pub const ERR_DEPOSIT_NOT_ENOUGH: &str = "Attached deposit is not enough";
pub const ERR_COUNCIL: &str = "Only council can do this operation";
pub const ERR_COUNCIL_OR_PROJECT_OWNER: &str =
    "Only project owner or council can do this operation";
pub const ERR_PROJECT_NAME_INVALID: &str = "Project name is invalid";
pub const ERR_INVALID_REPORT_URL: &str = "Report cannot be used more than once";
pub const ERR_ALREADY_EXISTS: &str = "Already exists";
pub const ERR_ACCESS_DENIED: &str = "Caller is not allowed to do this operation";
pub const ERR_EMPTY_CERTIFICATE: &str = "Nothing to certify";
pub const ERR_PROJECT_CREATOR_IS_NOT_OWNER: &str =
    "Project creator is not in list of project owners";
pub const ERR_TEXT_TOO_LONG: &str = "Text is too long";
pub const ERR_VEC_TOO_LONG: &str = "Vector is too long";
pub const ERR_INVALID_SCORE: &str = "The score is invalid";

pub const REGISTER_PROJECT_DEPOSIT: Balance = 1_000_000_000_000_000_000_000_000; // 1 NEAR
pub const REGISTER_CONTRACT_DEPOSIT: Balance = 100_000_000_000_000_000_000_000; // 0.1 NEAR
pub const CREATE_USER_DEPOSIT: Balance = 10_000_000_000_000_000_000_000; // 0.01 NEAR
pub const SUBMIT_AUDIT_DEPOSIT: Balance = 100_000_000_000_000_000_000_000; // 0.1 NEAR
pub const MAX_TEXT_LENGTH: usize = 1000;
pub const MAX_VEC_LENGTH: usize = 16;
pub const PRICE_PER_BYTE: Balance = 10_000_000_000_000_000_000;

pub type AuditId = CryptoHash;
// pub type ContractId = (ProjectId, Version); - unused
pub type ContractHash = CryptoHash;
pub type ProjectId = CryptoHash;
pub type Standard = String;
pub type Url = String;
pub type UserId = AccountId;

#[derive(Deserialize, Serialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct SafetyReport {
    pub safety_level: String,
    pub safety_issues: Vec<String>,
}

pub const SAFETY_LEVEL_CRITICAL: &str = "Dangerous";
pub const SAFETY_LEVEL_LOW: &str = "Low";
pub const SAFETY_LEVEL_MODERATE: &str = "Moderate";
pub const SAFETY_LEVEL_HIGH: &str = "High";

pub const SAFETY_REPORT_NO_SOURCE_CODE: &str = "No source code uploaded. Don't trust.";
pub const SAFETY_REPORT_NO_AUDITS: &str = "The contract hasn't been audited.";
pub const SAFETY_REPORT_NO_CERTIFICATES: &str = "The contract hasn't been certified by NEAR council.";

impl SafetyReport {
    pub fn new(safety_level_num: u64, safety_issues: Vec<&str>) -> Self {
        let safety_level = match safety_level_num {
            1 => SAFETY_LEVEL_LOW,
            2 => SAFETY_LEVEL_MODERATE,
            3 => SAFETY_LEVEL_HIGH,
            _ => SAFETY_LEVEL_CRITICAL,
        }.to_string();
        Self {
            safety_level,
            safety_issues: safety_issues.iter().map(|&s| s.into()).collect(),
        }
    }
}
