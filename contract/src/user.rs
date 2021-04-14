use crate::*;

#[derive(BorshDeserialize, BorshSerialize)]
pub struct User {
    pub reputation: u64,

    pub projects_owned: UnorderedSet<String>,

    pub auditor: Option<Auditor>,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct UserView {
    pub reputation: u64,

    pub projects_owned: Vec<String>,

    pub auditor: Option<AuditorView>,
}

impl From<&User> for UserView {
    fn from(u: &User) -> Self {
        Self {
            reputation: u.reputation,
            projects_owned: u.projects_owned.to_vec(),
            auditor: u.auditor.as_ref().map(|a| a.into()),
        }
    }
}

#[near_bindgen]
impl Global {
    pub fn get_user(&self, user_id: ValidAccountId) -> UserView {
        (&self.users.get(user_id.as_ref()).unwrap()).into()
    }

    #[payable]
    pub fn create_user(&mut self, user_id: ValidAccountId) -> UserView {
        let user = self.extract_user_or_create(user_id.as_ref());
        self.save_user_or_panic(user_id.as_ref(), &user);

        (&user).into()
    }
}

impl Global {
    pub(crate) fn extract_user_or_create(&mut self, user_id: &UserId) -> User {
        self.users.remove(&user_id).unwrap_or_else(|| {
            let mut prefix = Vec::with_capacity(33);
            prefix.push(b'u');
            prefix.extend(env::sha256(&user_id.as_bytes()));

            User {
                reputation: 0,
                projects_owned: UnorderedSet::new(prefix),
                auditor: None,
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
