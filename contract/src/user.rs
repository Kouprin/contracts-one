use crate::*;

#[derive(BorshDeserialize, BorshSerialize)]
pub struct User {
    pub projects_owned: UnorderedSet<ProjectId>,

    pub public_key: Option<PublicKey>,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct UserView {
    pub projects_owned: Vec<ProjectViewLimited>,

    pub public_key: Option<Base58PublicKey>,
}

impl From<(&User, &Main)> for UserView {
    fn from(u: (&User, &Main)) -> Self {
        Self {
            projects_owned: u.0.projects_owned
            .iter()
            .map(|id| (&u.1.projects.get(&id).unwrap()).into())
            .collect(),
            public_key: u.0.public_key.clone().map(|k| k.try_into().unwrap()),
        }
    }
}

#[near_bindgen]
impl Main {
    pub fn get_user(&self, user_id: ValidAccountId) -> Option<UserView> {
        Self::users().get(user_id.as_ref()).map(|u| (&u, self).into())
    }

    #[payable]
    pub fn create_user(&mut self, user_id: ValidAccountId) -> bool {
        assert!(
            Self::users().get(user_id.as_ref()).is_none(),
            "{}",
            ERR_ALREADY_EXISTS
        );
        let user = self.extract_user_or_create(user_id.as_ref());
        self.save_user_or_panic(user_id.as_ref(), &user);

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
                public_key: None,
            }
        })
    }

    pub(crate) fn extract_user_or_panic(&mut self, user_id: &UserId) -> User {
        Self::users().remove(&user_id).unwrap()
    }

    pub(crate) fn save_user_or_panic(&mut self, user_id: &UserId, user: &User) {
        assert!(Self::users().insert(user_id, user).is_none());
    }
}
