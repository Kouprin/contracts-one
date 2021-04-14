use crate::*;

#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    pub hash: ContractHash,

    pub project_name: String,
    pub contract_name: String,

    pub version: Version,
    pub published_time: Timestamp,

    // Cargo.toml + Cargo.lock + src folder?
    // Full marketplace contract + tests took 30k, 0.3 NEAR
    pub source_code_archived: String,

    pub publisher: UserId,

    pub auditors: UnorderedMap<UserId, CertificateId>,
    pub certificates: UnorderedMap<CertificateId, Certificate>,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct ContractView {
    pub hash: Base58CryptoHash,

    pub project_name: String,
    pub contract_name: String,

    pub version: String,
    pub published_time: WrappedTimestamp,

    pub source_code_size: u64,

    pub publisher: UserId,

    pub certificates: Vec<CertificateView>,
}

impl From<&Contract> for ContractView {
    fn from(c: &Contract) -> Self {
        Self {
            hash: c.hash.into(),
            project_name: c.project_name.clone(),
            contract_name: c.contract_name.clone(),
            version: (&c.version).into(),
            published_time: c.published_time.into(),
            source_code_size: c.source_code_archived.len() as u64,
            publisher: c.publisher.clone(),
            certificates: c.certificates.iter().map(|(_, v)| (&v).into()).collect(),
        }
    }
}

#[near_bindgen]
impl Global {
    pub fn get_contract(&self, contract_hash: Base58CryptoHash) -> ContractView {
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
            .unwrap())
            .into()
    }

    pub fn get_contract_source_code(&self, contract_hash: Base58CryptoHash) -> String {
        let (project_id, version) = self
            .contract_hash_to_contract_id
            .get(&contract_hash.into())
            .unwrap();
        self.projects
            .get(&project_id)
            .unwrap()
            .contracts
            .get(&version)
            .unwrap()
            .source_code_archived
    }

    #[payable]
    pub fn register_contract(
        &mut self,
        project_name: String,
        contract_name: String,
        contract_hash: Base58CryptoHash,
        version: String,
        source_code_archived: String,
    ) -> bool {
        let version: Version = (&version).into();
        let mut project = self.extract_project_by_name_or_panic(&project_name);
        assert!(project.owners.contains(&env::predecessor_account_id()));

        let mut prefix = Vec::with_capacity(33);
        prefix.push(b'u');
        prefix.extend(&ContractHash::from(contract_hash));

        let mut prefix2 = Vec::with_capacity(33);
        prefix2.push(b'v');
        prefix2.extend(&ContractHash::from(contract_hash));

        let contract = Contract {
            hash: contract_hash.into(),
            project_name: project_name.clone(),
            contract_name,
            version: version.clone(),
            published_time: env::block_timestamp(),
            source_code_archived,
            publisher: env::predecessor_account_id(),
            auditors: UnorderedMap::new(prefix),
            certificates: UnorderedMap::new(prefix2),
        };

        assert!(project.contracts.insert(&version, &contract).is_none());
        assert!(self
            .contract_hash_to_contract_id
            .insert(&contract.hash, &(Project::get_id(&project_name), version))
            .is_none());
        self.save_project_by_name_or_panic(&project_name, &project);

        true
    }
}
