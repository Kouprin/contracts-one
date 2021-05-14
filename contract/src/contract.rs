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
    pub source_code_archived: Option<String>,
    pub commit_hash: String,

    pub publisher: UserId,

    pub standards_declared: UnorderedSet<Standard>,

    pub audits: UnorderedSet<AuditId>,

    pub certificates: UnorderedSet<Certificate>,
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
            source_code_size: c
                .source_code_archived
                .as_ref()
                .map(|s| s.len() as u64)
                .unwrap_or(0),
            commit_hash: c.commit_hash.clone(),
            publisher: c.publisher.clone(),
            standards_declared: c.standards_declared.to_vec(),
            audits: c.audits.to_vec(),
            certificates: c.certificates.iter().map(|c| (&c).into()).collect(),
        }
    }
}

#[near_bindgen]
impl Main {
    pub fn get_contract(&self, contract_hash: Base58CryptoHash) -> Option<ContractView> {
        self.contracts
            .get(&contract_hash.into())
            .map(|c| (&c).into())
    }

    pub fn get_contract_safety_report(&self, contract_hash: Base58CryptoHash) -> SafetyReport {
        self.calculate_safety_level(&self.contracts.get(&contract_hash.into()))
    }

    pub fn get_contract_source_code(&self, contract_hash: Base58CryptoHash) -> Option<String> {
        self.contracts
            .get(&contract_hash.into())
            .map(|c| c.source_code_archived)
            .unwrap_or(None)
    }

    #[payable]
    pub fn register_contract(
        &mut self,
        project_name: String,
        contract_name: String,
        contract_hash: Base58CryptoHash,
        version: String,
        commit_hash: String,
        standards_declared: Vec<Standard>,
    ) -> bool {
        Self::assert_deposit(REGISTER_CONTRACT_DEPOSIT);
        Self::assert_text_len(&project_name);
        Self::assert_text_len(&contract_name);
        Self::assert_text_len(&commit_hash);
        Self::assert_vec_len(&standards_declared);

        let version: Version = (&version).into(); // asserts if version is valid
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

        let mut prefix3 = Vec::with_capacity(33);
        prefix3.push(b'm');
        prefix3.extend(&ContractHash::from(contract_hash));

        let contract = Contract {
            hash: contract_hash.into(),
            project_name: project_name.clone(),
            contract_name,
            version: version.clone(),
            published_time: env::block_timestamp(),
            source_code_archived: None,
            commit_hash,
            publisher: env::predecessor_account_id(),
            standards_declared: standards_declared_set,
            audits: UnorderedSet::new(prefix2),
            certificates: UnorderedSet::new(prefix3),
        };

        assert!(self
            .contracts
            .insert(&contract_hash.into(), &contract)
            .is_none());
        assert!(project
            .contracts
            .insert(&version, &contract_hash.into())
            .is_none());
        self.save_project_by_name_or_panic(&project_name, &project);

        true
    }

    #[payable]
    pub fn upload_source_code(
        &mut self,
        contract_hash: Base58CryptoHash,
        source_code_archived: String,
    ) -> bool {
        let mut contract = self.contracts.get(&contract_hash.into()).unwrap();
        if contract.source_code_archived.is_some() {
            Self::assert_council();
            Self::assert_one_yocto();
        } else {
            self.assert_council_or_project_owner(&contract.project_name);
            Self::assert_deposit(source_code_archived.len() as Balance * PRICE_PER_BYTE);
        }
        contract.source_code_archived = Some(source_code_archived);
        self.contracts.insert(&contract_hash.into(), &contract);

        true
    }
}

impl Main {
    pub(crate) fn calculate_safety_level(&self, _contract: &Option<Contract>) -> SafetyReport {
        SafetyReport::low()
        // TODO
        /*
        if contract.is_none() {

        }
        let contract = contract.as_ref().unwrap();
        if contract.council_approved.is_some() {
            return SafetyReport::high();
        }
        if contract.audits.len() == 0 {
            return SafetyReport::low();
        }
        SafetyReport::moderate()
        */
    }
}
