use crate::*;

#[derive(BorshDeserialize, BorshSerialize)]
pub struct Certificate {
    pub id: CertificateId,
    pub project_name: String,
    pub version: Version,

    pub author: UserId,
    pub report_url: Url,
    pub standards_confirmed: UnorderedSet<Standard>,

    pub approved: bool,
    pub score: Option<u64>,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct CertificateView {
    pub project_name: String,
    pub version: String,

    pub author: UserId,
    pub report_url: Url,
    pub standards_confirmed: Vec<Standard>,

    pub approved: bool,
    pub score: Option<u64>,
}

impl From<&Certificate> for CertificateView {
    fn from(c: &Certificate) -> Self {
        Self {
            project_name: c.project_name.clone(),
            version: (&c.version).into(),
            author: c.author.clone(),
            report_url: c.report_url.clone(),
            standards_confirmed: c.standards_confirmed.to_vec(),
            approved: c.approved,
            score: c.score,
        }
    }
}

#[near_bindgen]
impl Global {
    pub fn get_certificate(&self, certificate_id: CertificateId) -> CertificateView {
        let contract_hash = self
            .certificate_id_to_contract_hash
            .get(&certificate_id)
            .unwrap();
        let (project_id, version) = self
            .contract_hash_to_contract_id
            .get(&contract_hash.into())
            .unwrap();
        (&self
            .projects
            .get(&project_id)
            .unwrap()
            .contracts
            .get(&version)
            .unwrap()
            .certificates
            .get(&certificate_id)
            .unwrap())
            .into()
    }
}
