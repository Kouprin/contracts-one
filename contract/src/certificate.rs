use crate::*;

#[derive(BorshDeserialize, BorshSerialize)]
pub struct Certificate {
    pub issuer: UserId,
    pub issue_time: Timestamp,

    pub is_hash_valid: Option<bool>,
    pub is_audit_accepted: Option<bool>,
    pub is_code_approved: Option<bool>,
    pub is_standards_confirmed: Option<bool>,

    pub details: String,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct CertificateView {
    pub issuer: AccountId,
    pub issue_time: WrappedTimestamp,

    pub is_hash_valid: Option<bool>,
    pub is_audit_accepted: Option<bool>,
    pub is_code_approved: Option<bool>,
    pub is_standards_confirmed: Option<bool>,

    pub details: String,
}

impl From<&Certificate> for CertificateView {
    fn from(c: &Certificate) -> Self {
        Self {
            issuer: c.issuer.clone(),
            issue_time: c.issue_time.into(),
            is_hash_valid: c.is_hash_valid,
            is_audit_accepted: c.is_audit_accepted,
            is_code_approved: c.is_code_approved,
            is_standards_confirmed: c.is_standards_confirmed,
            details: c.details.clone(),
        }
    }
}

#[near_bindgen]
impl Main {
    #[payable]
    pub fn certify_contract(
        &mut self,
        contract_hash: Base58CryptoHash,
        is_hash_valid: Option<bool>,
        is_audit_accepted: Option<bool>,
        is_code_approved: Option<bool>,
        is_standards_confirmed: Option<bool>,
        details: String,
    ) -> bool {
        assert_one_yocto();
        if is_hash_valid.is_none()
            && is_audit_accepted.is_none()
            && is_code_approved.is_none()
            && is_standards_confirmed.is_none()
        {
            // Empty certificate
            assert!(false, "{}", ERR_EMPTY_CERTIFICATE);
        }
        let user = Self::users().get(&env::predecessor_account_id()).unwrap();
        assert!(user.is_council, "{}", ERR_ACCESS_DENIED);

        let mut contract = self.contracts.get(&contract_hash.into()).unwrap();
        contract.certificates.insert(&Certificate {
            issuer: env::predecessor_account_id(),
            issue_time: env::block_timestamp(),
            is_hash_valid,
            is_audit_accepted,
            is_code_approved,
            is_standards_confirmed,
            details,
        });
        self.contracts.insert(&contract_hash.into(), &contract);

        true
    }
}
