use std::convert::TryInto;

use crate::*;

#[derive(BorshDeserialize, BorshSerialize)]
pub struct User {
    pub reputation: u64,
    pub is_auditor: bool,

    pub projects_owned: UnorderedSet<String>,
    pub certificates_issued: UnorderedSet<CertificateId>,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct UserView {
    pub reputation: u64,
    pub is_auditor: bool,

    pub projects_owned: Vec<String>,
    pub certificates_issued: u64,
}

impl From<&User> for UserView {
    fn from(u: &User) -> Self {
        Self {
            reputation: u.reputation,
            is_auditor: u.is_auditor,
            projects_owned: u.projects_owned.to_vec(),
            certificates_issued: u.certificates_issued.len(),
        }
    }
}

#[near_bindgen]
impl Global {
    pub fn get_user(&self, user_id: ValidAccountId) -> UserView {
        (&self.users.get(user_id.as_ref()).unwrap()).into()
    }

    pub fn get_user_audits(&self, user_id: ValidAccountId) -> Vec<CertificateView> {
        self.users
            .get(user_id.as_ref())
            .unwrap()
            .certificates_issued
            .iter()
            .map(|c| {
                let (project_id, version) = self
                    .contract_hash_to_contract_id
                    .get(&self.certificate_id_to_contract_hash.get(&c).unwrap())
                    .unwrap();
                (&self
                    .projects
                    .get(&project_id)
                    .unwrap()
                    .contracts
                    .get(&version)
                    .unwrap()
                    .certificates
                    .get(&c)
                    .unwrap())
                    .into()
            })
            .collect()
    }

    #[payable]
    pub fn create_user(&mut self, user_id: ValidAccountId) -> UserView {
        let user = self.extract_user_or_create(user_id.as_ref());
        self.save_user_or_panic(user_id.as_ref(), &user);

        (&user).into()
    }

    #[payable]
    pub fn register_auditor(&mut self, user_id: ValidAccountId) -> bool {
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
        let mut user = self.extract_user_or_panic(user_id.as_ref());
        user.is_auditor = true;
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
        prefix.extend(certificate_id.clone());
        let mut standards_confirmed_set = UnorderedSet::new(prefix);
        standards_confirmed_set.extend(standards_confirmed.into_iter());

        let certificate = Certificate {
            id: certificate_id.try_into().unwrap(),
            project_name: project_name.clone(),
            version: version.clone(),
            author: env::predecessor_account_id(),
            report_url,
            standards_confirmed: standards_confirmed_set,
            approved,
            score,
        };

        let mut user = self.extract_user_or_create(&env::predecessor_account_id());
        user.certificates_issued.insert(&certificate.id);
        self.save_user_or_panic(&env::predecessor_account_id(), &user);

        assert!(self
            .certificate_id_to_contract_hash
            .insert(&certificate.id, &contract.hash)
            .is_none());
        assert!(contract
            .auditors
            .insert(&env::predecessor_account_id(), &certificate.id)
            .is_none());
        assert!(contract
            .certificates
            .insert(&certificate.id, &certificate)
            .is_none());
        project.save_contract_or_panic(&version, &contract);
        self.save_project_by_name_or_panic(&project_name, &project);

        true
    }
}

impl Global {
    pub(crate) fn can_user_create_project(&self, account_id: &AccountId) -> bool {
        self.users.get(&account_id).is_some()
    }

    pub(crate) fn extract_user_or_create(&mut self, user_id: &UserId) -> User {
        self.users.remove(&user_id).unwrap_or_else(|| {
            let mut prefix = Vec::with_capacity(33);
            prefix.push(b'u');
            prefix.extend(env::sha256(&user_id.as_bytes()));

            let mut prefix2 = Vec::with_capacity(33);
            prefix2.push(b'g');
            prefix2.extend(env::sha256(&user_id.as_bytes()));

            User {
                reputation: 0,
                is_auditor: false,
                projects_owned: UnorderedSet::new(prefix),
                certificates_issued: UnorderedSet::new(prefix2),
            }
        })
    }

    pub(crate) fn extract_user_or_panic(&mut self, user_id: &UserId) -> User {
        self.users.remove(&user_id).unwrap()
    }

    pub(crate) fn save_user_or_panic(&mut self, user_id: &UserId, user: &User) {
        assert!(self.users.insert(user_id, user).is_none());
    }
}
