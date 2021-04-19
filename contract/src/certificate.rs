use crate::*;

#[derive(BorshDeserialize, BorshSerialize)]
pub struct Certificate {
    pub project_name: String,
    pub version: Version,

    pub author: UserId,
    pub report_url: Url,
    pub summary: String,
    pub standards_confirmed: UnorderedSet<Standard>,

    pub approved: bool,
    pub score: Option<u64>,
}

impl Certificate {
    pub(crate) fn id(&self) -> CertificateId {
        env::sha256(self.report_url.as_bytes()).try_into().unwrap()
    }
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct CertificateView {
    pub project_name: String,
    pub version: String,

    pub author: UserId,
    pub report_url: Url,
    pub summary: String,
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
            summary: c.summary.clone(),
            standards_confirmed: c.standards_confirmed.to_vec(),
            approved: c.approved,
            score: c.score,
        }
    }
}

#[near_bindgen]
impl Global {
    pub fn get_certificate(&self, certificate_id: CertificateId) -> Option<CertificateView> {
        self.certificate_id_to_certificate(&certificate_id)
            .map(|c| (&c).into())
    }

    pub fn get_all_certificates(&self, from: u64, to: u64) -> Vec<CertificateView> {
        let from = min(from, self.certificate_id_to_contract_hash.len());
        let to = min(to, self.certificate_id_to_contract_hash.len());
        // keys_as_vector() should work for O(1)
        (from..to)
            .map(|i| {
                (&self
                    .certificate_id_to_certificate(
                        &self
                            .certificate_id_to_contract_hash
                            .keys_as_vector()
                            .get(i)
                            .unwrap(),
                    )
                    .unwrap())
                    .into()
            })
            .collect()
    }
}

impl Global {
    // TODO maybe add a trait CertificateId -> Certificate
    pub(crate) fn certificate_id_to_certificate(
        &self,
        certificate_id: &CertificateId,
    ) -> Option<Certificate> {
        match self.certificate_id_to_contract_hash.get(&certificate_id) {
            None => None,
            Some(contract_hash) => self
                .contract_hash_to_contract(&contract_hash)
                .unwrap()
                .certificates
                .get(&certificate_id),
        }
    }
}
