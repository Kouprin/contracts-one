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

    pub standards_declared: UnorderedSet<Standard>,

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

    pub standards_declared: Vec<Standard>,

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
            standards_declared: c.standards_declared.to_vec(),
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

    pub fn get_contract_safety_level(&self, contract_hash: Base58CryptoHash) -> SafetyReport {
        let (project_id, version) = self
            .contract_hash_to_contract_id
            .get(&contract_hash.into())
            .unwrap();
        self.calculate_safety_level(
            &self
                .projects
                .get(&project_id)
                .unwrap()
                .contracts
                .get(&version)
                .unwrap(),
        )
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
        standards_declared: Vec<Standard>,
    ) -> bool {
        let version: Version = (&version).into();
        let mut project = self.extract_project_by_name_or_panic(&project_name);
        assert!(project.owners.contains(&env::predecessor_account_id()));

        let mut prefix = Vec::with_capacity(33);
        prefix.push(b'u');
        prefix.extend(&ContractHash::from(contract_hash));
        let mut standards_declared_set = UnorderedSet::new(prefix);
        standards_declared_set.extend(standards_declared.into_iter());

        let mut prefix2 = Vec::with_capacity(33);
        prefix2.push(b'v');
        prefix2.extend(&ContractHash::from(contract_hash));

        let mut prefix3 = Vec::with_capacity(33);
        prefix3.push(b'w');
        prefix3.extend(&ContractHash::from(contract_hash));

        let contract = Contract {
            hash: contract_hash.into(),
            project_name: project_name.clone(),
            contract_name,
            version: version.clone(),
            published_time: env::block_timestamp(),
            source_code_archived,
            publisher: env::predecessor_account_id(),
            standards_declared: standards_declared_set,
            auditors: UnorderedMap::new(prefix2),
            certificates: UnorderedMap::new(prefix3),
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

impl Global {
    pub(crate) fn calculate_safety_level(&self, contract: &Contract) -> SafetyReport {
        if contract.certificates.len() == 0 {
            return SafetyReport::low();
        }
        let mut yes = 0;
        let mut no = 0;
        for (_, certificate) in contract.certificates.iter() {
            if certificate.approved {
                yes += 1;
            } else {
                no += 1;
            }
        }
        if yes == 0 {
            return SafetyReport::low();
        }
        if no == 0 {
            return SafetyReport::high();
        }
        SafetyReport::moderate()
    }
}
