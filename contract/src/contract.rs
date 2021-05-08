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
    pub commit_hash: String,

    pub publisher: UserId,

    pub standards_declared: UnorderedSet<Standard>,

    pub audits: UnorderedSet<AuditId>,

    pub council_approved: Option<UserId>,
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
    pub commit_hash: String,

    pub publisher: UserId,

    pub standards_declared: Vec<Standard>,

    pub audits: Vec<AuditId>,

    pub council_approved: Option<AccountId>,
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
            commit_hash: c.commit_hash.clone(),
            publisher: c.publisher.clone(),
            standards_declared: c.standards_declared.to_vec(),
            audits: c.audits.to_vec(),
            council_approved: c.council_approved.clone(),
        }
    }
}

#[near_bindgen]
impl Main {
    pub fn get_contract(&self, contract_hash: Base58CryptoHash) -> Option<ContractView> {
        self.contract_hash_to_contract(&contract_hash.into())
            .map(|c| (&c).into())
    }

    pub fn get_contract_safety_report(&self, contract_hash: Base58CryptoHash) -> SafetyReport {
        self.calculate_safety_level(&self.contract_hash_to_contract(&contract_hash.into()))
    }

    pub fn get_contract_source_code(&self, contract_hash: Base58CryptoHash) -> Option<String> {
        self.contract_hash_to_contract(&contract_hash.into())
            .map(|c| c.source_code_archived)
    }

    #[payable]
    pub fn register_contract(
        &mut self,
        project_name: String,
        contract_name: String,
        contract_hash: Base58CryptoHash,
        version: String,
        source_code_archived: String,
        commit_hash: String,
        standards_declared: Vec<Standard>,
    ) -> bool {
        let version: Version = (&version).into();
        let mut project = self.extract_project_by_name_or_panic(&project_name);
        assert!(project.owners.contains(&env::predecessor_account_id()));

        let mut prefix = Vec::with_capacity(33);
        prefix.push(b'k');
        prefix.extend(&ContractHash::from(contract_hash));
        let mut standards_declared_set = UnorderedSet::new(prefix);
        standards_declared_set.extend(standards_declared.into_iter());

        let mut prefix2 = Vec::with_capacity(33);
        prefix2.push(b'l');
        prefix2.extend(&ContractHash::from(contract_hash));

        let contract = Contract {
            hash: contract_hash.into(),
            project_name: project_name.clone(),
            contract_name,
            version: version.clone(),
            published_time: env::block_timestamp(),
            source_code_archived,
            commit_hash,
            publisher: env::predecessor_account_id(),
            standards_declared: standards_declared_set,
            audits: UnorderedSet::new(prefix2),
            council_approved: None,
        };

        assert!(project.contracts.insert(&version, &contract).is_none());
        assert!(Self::contract_hash_to_contract_id()
            .insert(&contract.hash, &(Project::get_id(&project_name), version))
            .is_none());
        self.save_project_by_name_or_panic(&project_name, &project);

        true
    }
}

impl Main {
    pub(crate) fn contract_hash_to_contract(
        &self,
        contract_hash: &ContractHash,
    ) -> Option<Contract> {
        match Self::contract_hash_to_contract_id().get(contract_hash) {
            None => None,
            Some((project_id, version)) => self.projects
                .get(&project_id)
                .unwrap()
                .contracts
                .get(&version),
        }
    }

    pub(crate) fn calculate_safety_level(&self, contract: &Option<Contract>) -> SafetyReport {
        if contract.is_none() {
            return SafetyReport::low();
        }
        let contract = contract.as_ref().unwrap();
        if contract.council_approved.is_some() {
            return SafetyReport::high();
        }
        if contract.audits.len() == 0 {
            return SafetyReport::low();
        }
        SafetyReport::moderate()
    }
}
