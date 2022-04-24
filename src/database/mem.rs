use super::*;
use async_trait::async_trait;
use std::sync::Arc;
use tokio::sync::Mutex;
use ulid::Ulid;

// TODO: トランザクションの観点から色々やばいのでどうにかしたい

#[derive(Debug)]
struct MemUserRepositoryInner {
    users: Vec<User>,
}

#[derive(Debug)]
pub struct MemUserRepository {
    inner: Arc<Mutex<MemUserRepositoryInner>>,
}

impl MemUserRepository {
    pub fn new() -> Self {
        let inner = MemUserRepositoryInner { users: Vec::new() };
        let inner = Arc::new(Mutex::new(inner));

        Self { inner }
    }
}

#[async_trait]
impl UserRepository for MemUserRepository {
    async fn save(&mut self, mut user: User) -> Result<User> {
        let users = &mut self.inner.lock().await.users;

        if user.id.is_none() {
            let ulid = Ulid::new().to_string();
            user.id = Some(ID(ulid));
            users.push(user.clone());

            return Ok(user);
        }

        if let Some(old_user) = users
            .iter_mut()
            .find(|u| u.id.as_ref().unwrap() == user.id.as_ref().unwrap())
        {
            let _ = std::mem::replace(old_user, user.clone());
            Ok(user)
        } else {
            Err(DataBaseError::NotFound)
        }
    }

    async fn find_by_discord_id(&self, discord_id: u64) -> Result<Option<User>> {
        let users = &self.inner.lock().await.users;

        let user = users.iter().find(|u| u.discord_id == discord_id).cloned();

        Ok(user)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn test_create() -> Result<()> {
        let mut user_repository = MemUserRepository::new();

        let user = User {
            id: None,
            discord_id: 12345,
            pgp_pub_key: GPGKey {
                fingerprint: "12345".to_string(),
                key: Vec::new(),
            },
        };

        let new_user = user_repository.save(user).await?;

        assert!(new_user.id.is_some());
        assert_eq!(new_user.discord_id, 12345);
        assert_eq!(new_user.pgp_pub_key.fingerprint, "12345");

        Ok(())
    }
}
