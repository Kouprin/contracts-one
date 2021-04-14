use crate::*;

pub const ERR_REGISTER_PROJECT_DEPOSIT_NOT_ENOUGH: &str =
    "Attached deposit must be no less than REGISTER_PROJECT_DEPOSIT";
pub const ERR_PROJECT_NAME_INVALID: &str = "Project name is invalid";
pub const ERR_NOT_AN_AUDITOR: &str = "Audits can be submitted by auditors only";
pub const ERR_INVALID_REPORT_URL: &str = "Report cannot be used more than once";
pub const ERR_NOT_AN_OWNER: &str = "Only contract owner can do this operation";
pub const ERR_ALREADY_EXISTS: &str = "Already exists";
pub const ERR_ACCESS_DENIED: &str = "Caller is not allowed to do this operation";

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
