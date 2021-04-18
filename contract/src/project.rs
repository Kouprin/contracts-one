use regex::Regex;
use std::cmp::min;
use std::convert::TryInto;

use crate::*;

#[derive(BorshDeserialize, BorshSerialize)]
pub struct Project {
    pub project_name: String,
    pub description: String,
    pub url: String,

    pub owners: UnorderedSet<AccountId>,

    pub contracts: TreeMap<Version, Contract>,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct ProjectView {
    pub project_id: Base58CryptoHash,

    pub project_name: String,
    pub description: String,
    pub url: String,

    pub owners: Vec<AccountId>,

    pub contracts: Vec<ContractView>,

    pub audit_status: String,
}

impl From<&Project> for ProjectView {
    fn from(p: &Project) -> Self {
        Self {
            project_id: Project::get_id(&p.project_name).into(),
            project_name: p.project_name.clone(),
            description: p.description.clone(),
            url: p.url.clone(),
            owners: p.owners.to_vec(),
            contracts: p.contracts.iter().map(|(_, c)| (&c).into()).collect(),
            audit_status: (&p.get_last_version().map_or(AuditStatus::Unknown, |v| {
                p.contracts.get(&v).unwrap().audit_status
            }))
                .into(),
        }
    }
}

impl Project {
    pub(crate) fn get_id(project_name: &String) -> ProjectId {
        env::sha256(project_name.as_bytes())
            .as_slice()
            .try_into()
            .unwrap()
    }

    pub(crate) fn extract_contract_or_panic(&mut self, version: &Version) -> Contract {
        self.contracts.remove(version).unwrap()
    }

    pub(crate) fn save_contract_or_panic(&mut self, version: &Version, contract: &Contract) {
        assert!(self.contracts.insert(version, contract).is_none());
    }

    pub(crate) fn get_last_version(&self) -> Option<Version> {
        self.contracts.iter_rev().next().map(|(v, _)| v)
    }

    pub(crate) fn get_last_version_and_hash(&self) -> Option<(Version, ContractHash)> {
        self.contracts.iter_rev().next().map(|(v, c)| (v, c.hash))
    }
}

#[near_bindgen]
impl Global {
    pub fn get_project(&self, project_name: String) -> Option<ProjectView> {
        self.projects
            .get(&Project::get_id(&project_name))
            .map(|p| (&p).into())
    }

    pub fn get_all_projects(
        &self,
        from: u64,
        to: u64,
    ) -> Vec<(String, Option<(String, Base58CryptoHash)>)> {
        let from = min(from, self.projects.len());
        let to = min(to, self.projects.len());
        let mut res = vec![];
        for i in from..to {
            // values_as_vector() should work for O(1)
            res.push(
                self.projects
                    .values_as_vector()
                    .get(i)
                    .map(|p| {
                        (
                            p.project_name.clone(),
                            p.get_last_version_and_hash()
                                .map(|(v, c)| ((&v).into(), c.into())),
                        )
                    })
                    .unwrap(),
            )
        }
        res
    }

    pub fn get_project_last_version(&self, project_name: String) -> Option<String> {
        self.projects
            .get(&Project::get_id(&project_name))
            .unwrap()
            .get_last_version()
            .map(|v| (&v).into())
    }

    #[payable]
    pub fn register_project(
        &mut self,
        project_name: String,
        description: String,
        url: String,
        owners: Vec<ValidAccountId>,
    ) -> bool {
        assert!(
            project_name.len() > 0 && project_name.len() <= 64,
            "{}",
            ERR_PROJECT_NAME_INVALID
        );
        let re = Regex::new(r"^(([A-Z|a-z|0-9]+[\-_\.])*[A-Z|a-z|0-9]+)$").unwrap();
        assert!(re.is_match(&project_name), "{}", ERR_PROJECT_NAME_INVALID);

        let project_id = Project::get_id(&project_name);
        let mut is_predecessor_found = false;
        for user_id in owners.iter() {
            if user_id.as_ref() == &env::predecessor_account_id() {
                is_predecessor_found = true;
            }
            let mut user = self.extract_user_or_create(user_id.as_ref());
            user.projects_owned.insert(&project_name);
            self.save_user_or_panic(user_id.as_ref(), &user);
        }
        assert!(is_predecessor_found, "{}", ERR_PROJECT_CREATOR_IS_NOT_OWNER);

        let mut prefix = Vec::with_capacity(33);
        prefix.push(b'x');
        prefix.extend(&project_id);
        let mut owners_set = UnorderedSet::new(prefix);
        owners_set.extend(owners.into_iter().map(|o| o.into()));

        let mut prefix2 = Vec::with_capacity(33);
        prefix2.push(b'y');
        prefix2.extend(&project_id);

        assert!(
            self.projects
                .insert(
                    &project_id,
                    &Project {
                        project_name,
                        description,
                        url,
                        owners: owners_set,
                        contracts: TreeMap::new(prefix2),
                    }
                )
                .is_none(),
            "{}",
            ERR_ALREADY_EXISTS
        );

        true
    }
}

impl Global {
    pub(crate) fn extract_project_or_panic(&mut self, project_id: &ProjectId) -> Project {
        self.projects.remove(project_id).unwrap()
    }

    pub(crate) fn extract_project_by_name_or_panic(&mut self, project_name: &String) -> Project {
        self.extract_project_or_panic(&Project::get_id(project_name))
    }

    pub(crate) fn save_project_or_panic(&mut self, project_id: &ProjectId, project: &Project) {
        assert!(self.projects.insert(project_id, project).is_none())
    }

    pub(crate) fn save_project_by_name_or_panic(
        &mut self,
        project_name: &String,
        project: &Project,
    ) {
        self.save_project_or_panic(&Project::get_id(project_name), project)
    }
}
