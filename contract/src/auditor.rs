use crate::*;

#[derive(BorshDeserialize, BorshSerialize)]
pub struct Auditor {
    pub description: String,

    pub certificates_issued: UnorderedSet<CertificateId>,

    pub audits_marked_helpful: u64,

    // TODO
    // pub issues_created: UnorderedSet<IssueId>,
    // pub issues_missed: UnorderedSet<IssueId>,
    pub issues_created: u64,
    pub issues_missed: u64,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct AuditorView {
    pub description: String,

    pub certificates_issued: u64,

    pub audits_marked_helpful: u64,

    pub issues_created: u64,
    pub issues_missed: u64,
}

impl From<&Auditor> for AuditorView {
    fn from(a: &Auditor) -> Self {
        Self {
            description: a.description.clone(),
            certificates_issued: a.certificates_issued.len(),
            audits_marked_helpful: a.audits_marked_helpful,
            issues_created: a.issues_created,
            issues_missed: a.issues_missed,
        }
    }
}

#[near_bindgen]
impl Global {
    pub fn get_auditor_certificates(&self, user_id: ValidAccountId) -> Vec<CertificateView> {
        self.users
            .get(user_id.as_ref())
            .unwrap()
            .auditor
            .unwrap()
            .certificates_issued
            .iter()
            .map(|c| (&self.certificate_id_to_certificate(&c).unwrap()).into())
            .collect()
    }

    #[payable]
    pub fn register_auditor(&mut self, user_id: ValidAccountId, description: String) -> bool {
        assert!(env::attached_deposit() >= 1, "{}", ERR_DEPOSIT_NOT_ENOUGH);
        assert_eq!(
            env::predecessor_account_id(),
            self.owner_id,
            "{}",
            ERR_NOT_AN_OWNER
        );
        assert!(
            self.auditors.insert(user_id.as_ref()),
            "{}",
            ERR_ALREADY_EXISTS
        );
        let mut prefix = Vec::with_capacity(33);
        prefix.push(b'g');
        prefix.extend(env::sha256(user_id.as_ref().as_bytes()));
        let auditor = Auditor {
            description,
            certificates_issued: UnorderedSet::new(prefix),
            audits_marked_helpful: 0,
            issues_created: 0,
            issues_missed: 0,
        };

        let mut user = self.extract_user_or_panic(user_id.as_ref());
        user.auditor = Some(auditor);
        self.save_user_or_panic(user_id.as_ref(), &user);

        true
    }

    #[payable]
    pub fn sign_audit(
        &mut self,
        project_name: String,
        version: String,
        report_url: Url,
        standards_confirmed: Vec<Standard>,
        approved: bool,
        score: Option<u64>,
    ) -> bool {
        assert!(
            env::attached_deposit() >= SIGN_AUDIT_DEPOSIT,
            "{}",
            ERR_DEPOSIT_NOT_ENOUGH
        );
        assert!(
            self.auditors.contains(&env::predecessor_account_id()),
            "{}",
            ERR_NOT_AN_AUDITOR
        );
        let version: Version = (&version).into();
        let certificate_id = env::sha256(report_url.as_bytes());
        let mut project = self.extract_project_by_name_or_panic(&project_name);
        let mut contract = project.extract_contract_or_panic(&version);

        let mut prefix = Vec::with_capacity(33);
        prefix.push(b's');
        prefix.extend(certificate_id);
        let mut standards_confirmed_set = UnorderedSet::new(prefix);
        standards_confirmed_set.extend(standards_confirmed.into_iter());

        let certificate = Certificate {
            project_name: project_name.clone(),
            version: version.clone(),
            author: env::predecessor_account_id(),
            report_url,
            standards_confirmed: standards_confirmed_set,
            approved,
            score,
        };

        let mut user = self.extract_user_or_panic(&env::predecessor_account_id());
        user.auditor
            .as_mut()
            .unwrap()
            .certificates_issued
            .insert(&certificate.id());
        self.save_user_or_panic(&env::predecessor_account_id(), &user);

        assert!(self
            .certificate_id_to_contract_hash
            .insert(&certificate.id(), &contract.hash)
            .is_none());
        assert!(contract
            .auditors
            .insert(&env::predecessor_account_id(), &certificate.id())
            .is_none());
        assert!(contract
            .certificates
            .insert(&certificate.id(), &certificate)
            .is_none());
        project.save_contract_or_panic(&version, &contract);
        self.save_project_by_name_or_panic(&project_name, &project);

        true
    }
}
