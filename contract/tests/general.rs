use std::collections::HashMap;
use std::convert::TryInto;

/// Import the generated proxy contract
use contracts_one::GlobalContract as COContract;
use contracts_one::{
    ContractView, ProjectView, CREATE_USER_DEPOSIT, ERR_ACCESS_DENIED, ERR_NOT_AN_AUDITOR,
    ERR_PROJECT_NAME_INVALID, REGISTER_AUDITOR_DEPOSIT, REGISTER_PROJECT_DEPOSIT,
    SIGN_AUDIT_DEPOSIT,
};

use near_sdk::json_types::Base58CryptoHash;
use near_sdk_sim::{call, deploy, init_simulator, to_yocto, view, ContractAccount, UserAccount};

// Load in contract bytes at runtime
near_sdk_sim::lazy_static_include::lazy_static_include_bytes! {
  CONTRACT_WASM_BYTES => "res/contracts_one.wasm",
}

const CONTRACT_ID: &str = "contracts_one";

const ERR_ASSERT: Option<&str> = Some("assertion failed");
const ERR_UNWRAP: Option<&str> = Some("called `Option::unwrap()`");

const DEFAULT_PROJECT_ID: &str = "Test_Project_111";
const DEFAULT_CONTRACT_HASH: &str = "FtPgYqXzhGhcsB4rMt8ji5krAQuoDWamLgtUqYMLKnP3";
const DEFAULT_URL: &str = "near.org";
const DEFAULT_VERSION: &str = "1.2.3";
const DEFAULT_STANDARDS_DECLARED: Vec<String>  = vec![];
const ALICE: &str = "alice";
const BOB: &str = "bob";
#[allow(dead_code)]
const CAROL: &str = "carol";
const DEFAULT_PROJECT_OWNERS: &[&'static str; 2] = &[ALICE, BOB];

struct State {
    pub root: UserAccount,
    pub contract: ContractAccount<COContract>,
    pub accounts: HashMap<String, UserAccount>,
}

impl State {
    pub fn new() -> Self {
        let root = init_simulator(None);

        let deployed_contract = deploy!(
            contract: COContract,
            contract_id: CONTRACT_ID,
            bytes: &CONTRACT_WASM_BYTES,
            signer_account: root,
            deposit: to_yocto("1000000000"),
            init_method: test_new()
        );
        let state = State {
            root,
            contract: deployed_contract,
            accounts: HashMap::default(),
        };
        state.do_create_user(&state.root.account_id(), None);
        state.do_register_auditor(&state.root.account_id(), None);
        state
    }

    pub fn create_alice(&mut self) {
        let alice = self.root.create_user(ALICE.into(), to_yocto("1000000000"));
        self.accounts.insert(ALICE.into(), alice);
    }

    pub fn create_bob(&mut self) {
        let bob = self.root.create_user(BOB.into(), to_yocto("1000000000"));
        self.accounts.insert(BOB.into(), bob);
    }

    pub fn get_all_projects(&self) -> Vec<(String, Option<(String, Base58CryptoHash)>)> {
        let contract = &self.contract;
        let res = view!(contract.get_all_projects(0, 1000)).unwrap_json();
        res
    }

    pub fn get_contract(&self, contract_hash: &str) -> ContractView {
        let contract = &self.contract;
        let res = view!(contract.get_contract(contract_hash.try_into().unwrap())).unwrap_json();
        res
    }

    pub fn get_project(&self, project_name: &str) -> ProjectView {
        let contract = &self.contract;
        let res = view!(contract.get_project(project_name.into())).unwrap_json();
        res
    }

    pub fn get_project_id(&self, project_name: &str) -> Base58CryptoHash {
        let contract = &self.contract;
        let res = view!(contract.get_project_id(project_name.into())).unwrap_json();
        res
    }

    pub fn do_register_project(&self, name: &str, owners: &[&str], err: Option<&str>) {
        let contract = &self.contract;
        let outcome = call!(
            self.root,
            contract.register_project(
                name.to_string(),
                "bla".to_string(),
                DEFAULT_URL.to_string(),
                owners
                    .iter()
                    .map(|o| o.to_string().try_into().unwrap())
                    .collect()
            ),
            deposit = REGISTER_PROJECT_DEPOSIT
        );
        if let Some(msg) = err {
            assert!(
                format!("{:?}", outcome.status()).contains(msg),
                "received {:?}",
                outcome.status()
            );
            assert!(!outcome.is_ok(), "Should panic");
        } else {
            outcome.assert_success();
        }
    }

    pub fn do_register_contract(
        &self,
        account_name: &str,
        project: &str,
        hash: &str,
        version: &str,
        err: Option<&str>,
    ) {
        let contract = &self.contract;
        let account = self.accounts.get(account_name).unwrap();
        let outcome = call!(
            account,
            contract.register_contract(
                project.to_string(),
                "default contract name".to_string(),
                hash.try_into().unwrap(),
                version.to_string(),
                "source code".to_string(),
                DEFAULT_STANDARDS_DECLARED
            ),
            deposit = REGISTER_PROJECT_DEPOSIT
        );
        if let Some(msg) = err {
            assert!(
                format!("{:?}", outcome.status()).contains(msg),
                "received {:?}",
                outcome.status()
            );
            assert!(!outcome.is_ok(), "Should panic");
        } else {
            outcome.assert_success();
        }
    }

    fn do_create_user(&self, account_name: &str, err: Option<&str>) {
        let contract = &self.contract;
        let outcome = call!(
            self.root,
            contract.create_user(account_name.try_into().unwrap()),
            deposit = CREATE_USER_DEPOSIT
        );
        if let Some(msg) = err {
            assert!(
                format!("{:?}", outcome.status()).contains(msg),
                "received {:?}",
                outcome.status()
            );
            assert!(!outcome.is_ok(), "Should panic");
        } else {
            outcome.assert_success();
        }
    }

    fn do_register_auditor(&self, account_name: &str, err: Option<&str>) {
        let contract = &self.contract;
        let outcome = call!(
            self.root,
            contract.register_auditor(account_name.try_into().unwrap()),
            deposit = REGISTER_AUDITOR_DEPOSIT
        );
        if let Some(msg) = err {
            assert!(
                format!("{:?}", outcome.status()).contains(msg),
                "received {:?}",
                outcome.status()
            );
            assert!(!outcome.is_ok(), "Should panic");
        } else {
            outcome.assert_success();
        }
    }

    fn do_sign_audit(
        &self,
        account_name: &str,
        project_id: &str,
        version: &str,
        url: &str,
        err: Option<&str>,
    ) {
        let contract = &self.contract;
        let account = self.accounts.get(account_name).unwrap();
        let outcome = call!(
            account,
            contract.sign_audit(
                project_id.to_string(),
                version.to_string(),
                url.to_string(),
                vec!["boom".to_string()],
                true,
                Some(6)
            ),
            deposit = SIGN_AUDIT_DEPOSIT
        );
        if let Some(msg) = err {
            assert!(
                format!("{:?}", outcome.status()).contains(msg),
                "received {:?}",
                outcome.status()
            );
            assert!(!outcome.is_ok(), "Should panic");
        } else {
            outcome.assert_success();
        }
    }

    pub fn validate(&self) {
        let projects = self.get_all_projects();
        //assert_eq!(project_names.len(), 5);

        //assert!(false);
    }
}

#[test]
fn init_sanity() {
    let mut state = State::new();
    state.create_alice();

    state.validate();
}

#[test]
fn project_names() {
    let state = State::new();

    state.do_register_project("test", DEFAULT_PROJECT_OWNERS, None);
    state.do_register_project("Test_Project.123", DEFAULT_PROJECT_OWNERS, None);
    state.do_register_project(
        "Test_Project.123 ",
        DEFAULT_PROJECT_OWNERS,
        Some(ERR_PROJECT_NAME_INVALID),
    );
    state.do_register_project("", DEFAULT_PROJECT_OWNERS, Some(ERR_PROJECT_NAME_INVALID));
    state.do_register_project("0", DEFAULT_PROJECT_OWNERS, None);
    state.do_register_project("a", DEFAULT_PROJECT_OWNERS, None);
    state.do_register_project("#", DEFAULT_PROJECT_OWNERS, Some(ERR_PROJECT_NAME_INVALID));
    state.do_register_project("_", DEFAULT_PROJECT_OWNERS, Some(ERR_PROJECT_NAME_INVALID));
    state.do_register_project(&"1".repeat(64), DEFAULT_PROJECT_OWNERS, None);
    state.do_register_project(
        &"1".repeat(65),
        DEFAULT_PROJECT_OWNERS,
        Some(ERR_PROJECT_NAME_INVALID),
    );

    state.validate();
}

#[test]
fn register_project_by_not_a_user() {
    let mut state = State::new();
    state.create_alice();

    let contract = &state.contract;
    let alice = state.accounts.get(ALICE).unwrap();
    let outcome = call!(
        alice,
        contract.register_project(
            DEFAULT_PROJECT_ID.to_string(),
            "bla".to_string(),
            DEFAULT_URL.to_string(),
            DEFAULT_PROJECT_OWNERS
                .iter()
                .map(|o| o.to_string().try_into().unwrap())
                .collect()
        ),
        deposit = REGISTER_PROJECT_DEPOSIT
    );
    assert!(
        format!("{:?}", outcome.status()).contains(ERR_ACCESS_DENIED),
        "received {:?}",
        outcome.status()
    );
    assert!(!outcome.is_ok(), "Should panic");

    state.do_create_user(ALICE, None);
    let outcome = call!(
        alice,
        contract.register_project(
            DEFAULT_PROJECT_ID.to_string(),
            "bla".to_string(),
            DEFAULT_URL.to_string(),
            DEFAULT_PROJECT_OWNERS
                .iter()
                .map(|o| o.to_string().try_into().unwrap())
                .collect()
        ),
        deposit = REGISTER_PROJECT_DEPOSIT
    );
    outcome.assert_success();
}

#[test]
fn version_sanity() {
    let mut state = State::new();
    state.create_alice();

    state.do_register_project(DEFAULT_PROJECT_ID, DEFAULT_PROJECT_OWNERS, None);
    state.do_register_contract(
        ALICE,
        DEFAULT_PROJECT_ID,
        DEFAULT_CONTRACT_HASH,
        "0.0.0",
        None,
    );
    state.do_register_contract(ALICE, DEFAULT_PROJECT_ID, &"1".repeat(32), "0.0.1", None);
    // TODO random hashes
}

#[test]
fn reproduce_1() {
    let mut state = State::new();
    state.create_alice();
    state.create_bob();

    state.do_register_project("contract.one_test", DEFAULT_PROJECT_OWNERS, None);
    state.do_register_contract(
        ALICE,
        "contract.one_test",
        "5suuACmAzbTj8oyv4bQUjuJZbRinGMAKMLorDDEFzu4a",
        "1.2.3",
        None,
    );
    let ver = &state.get_project("contract.one_test").contracts[0].version;

    state.do_register_project("contract.two_test", DEFAULT_PROJECT_OWNERS, None);
    state.do_register_project("contract.three_test", DEFAULT_PROJECT_OWNERS, None);
    state.do_register_contract(
        ALICE,
        "contract.three_test",
        "5suuACmAzbTj8oyv4bQUjuJZbRinGMAKMLorDDEFzu4b",
        "0.0.0",
        None,
    );

    assert_eq!(
        ver,
        &state.get_project("contract.one_test").contracts[0].version
    );
}

#[test]
fn sign_audit_collect_all_errors() {
    let mut state = State::new();
    state.create_alice();

    state.do_sign_audit(
        ALICE,
        DEFAULT_PROJECT_ID,
        DEFAULT_VERSION,
        DEFAULT_URL,
        Some(ERR_NOT_AN_AUDITOR),
    );
    state.do_register_auditor(ALICE, ERR_UNWRAP);
    state.do_create_user(ALICE, None);

    state.do_sign_audit(
        ALICE,
        DEFAULT_PROJECT_ID,
        DEFAULT_VERSION,
        DEFAULT_URL,
        Some(ERR_NOT_AN_AUDITOR),
    );
    state.do_register_auditor(ALICE, None);

    state.do_sign_audit(
        ALICE,
        DEFAULT_PROJECT_ID,
        DEFAULT_VERSION,
        DEFAULT_URL,
        ERR_UNWRAP,
    );

    state.do_register_project(DEFAULT_PROJECT_ID, DEFAULT_PROJECT_OWNERS, None);
    state.do_register_contract(
        ALICE,
        DEFAULT_PROJECT_ID,
        DEFAULT_CONTRACT_HASH,
        DEFAULT_VERSION,
        None,
    );

    state.do_sign_audit(
        ALICE,
        DEFAULT_PROJECT_ID,
        DEFAULT_VERSION,
        DEFAULT_URL,
        None,
    );

    state.do_sign_audit(
        ALICE,
        DEFAULT_PROJECT_ID,
        DEFAULT_VERSION,
        DEFAULT_URL,
        ERR_ASSERT,
    );

    state.do_sign_audit(
        ALICE,
        DEFAULT_PROJECT_ID,
        DEFAULT_VERSION,
        "another url for same auditor, project and contract version",
        ERR_ASSERT,
    );

    state.do_sign_audit(
        ALICE,
        DEFAULT_PROJECT_ID,
        "123.456.789",
        "url - no such version",
        ERR_UNWRAP,
    );

    state.validate();
}
