use crate::*;

#[derive(BorshDeserialize, BorshSerialize)]
pub struct Audit {
    pub publisher: UserId,

    pub auditor_url: Url,
    pub report_url: Url,
    pub summary: String,
    pub date: Timestamp,
}

impl Audit {
    pub(crate) fn id(&self) -> AuditId {
        env::sha256(self.report_url.as_bytes()).try_into().unwrap()
    }
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct AuditView {
    pub audit_id: Base58CryptoHash,

    pub publisher: AccountId,

    pub auditor_url: String,
    pub report_url: String,
    pub summary: String,
    pub date: WrappedTimestamp,
}

impl From<&Audit> for AuditView {
    fn from(a: &Audit) -> Self {
        Self {
            audit_id: a.id().into(),
            publisher: a.publisher.clone(),
            auditor_url: a.auditor_url.clone(),
            report_url: a.report_url.clone(),
            summary: a.summary.clone(),
            date: a.date.into(),
        }
    }
}

#[near_bindgen]
impl Main {
    pub fn get_audit(&self, audit_id: AuditId) -> Option<AuditView> {
        Self::audits().get(&audit_id).map(|a| (&a).into())
    }

    #[payable]
    pub fn submit_audit(
        &mut self,
        contract_hash: Base58CryptoHash,
        auditor_url: String,
        report_url: String,
        summary: String,
        date: WrappedTimestamp,
    ) -> bool {
        let mut audits = Self::audits();
        let audit = Audit {
            publisher: env::predecessor_account_id(),
            auditor_url,
            report_url,
            summary,
            date: date.into(),
        };
        assert!(audits.insert(&audit.id(), &audit).is_none());

        let mut contract = self.contracts.get(&contract_hash.into()).unwrap();
        assert!(contract.audits.insert(&audit.id()));

        true
    }
}
