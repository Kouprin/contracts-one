use crate::*;

#[derive(BorshDeserialize, BorshSerialize)]
pub struct User {
    pub projects_owned: UnorderedSet<ProjectId>,

    pub is_council: bool,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct UserView {
    pub projects_owned: Vec<Base58CryptoHash>,

    pub is_council: bool,
}

impl From<&User> for UserView {
    fn from(u: &User) -> Self {
        Self {
            projects_owned: u.projects_owned.iter().map(|id| id.into()).collect(),
            is_council: u.is_council,
        }
    }
}

#[near_bindgen]
impl Main {
    pub fn get_user(&self, user_id: ValidAccountId) -> Option<UserView> {
        Self::users().get(user_id.as_ref()).map(|u| (&u).into())
    }

    #[payable]
    pub fn create_user(&mut self, user_id: ValidAccountId) -> bool {
        Self::assert_deposit(CREATE_USER_DEPOSIT);
        assert!(
            Self::users().get(user_id.as_ref()).is_none(),
            "{}",
            ERR_ALREADY_EXISTS
        );
        let user = self.extract_user_or_create(user_id.as_ref());
        self.save_user_or_panic(user_id.as_ref(), &user);

        true
    }

    #[payable]
    pub fn register_council(&mut self, user_id: ValidAccountId) -> bool {
        Self::assert_council();
        Self::assert_one_yocto();
        let mut user = self.extract_user_or_create(user_id.as_ref());
        user.is_council = true;
        self.save_user_or_panic(user_id.as_ref(), &user);
        assert!(Self::council().insert(user_id.as_ref()));

        true
    }
}

impl Main {
    pub(crate) fn extract_user_or_create(&mut self, user_id: &UserId) -> User {
        Self::users().remove(&user_id).unwrap_or_else(|| {
            let mut prefix = Vec::with_capacity(33);
            prefix.push(b'u');
            prefix.extend(env::sha256(&user_id.as_bytes()));

            User {
                projects_owned: UnorderedSet::new(prefix),
                is_council: false,
            }
        })
    }

    pub(crate) fn save_user_or_panic(&mut self, user_id: &UserId, user: &User) {
        assert!(Self::users().insert(user_id, user).is_none());
    }
}
