pub mod mem;

use async_trait::async_trait;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, DataBaseError>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ID(String);

#[derive(Debug, Clone)]
pub struct User {
    pub id: Option<ID>,
    pub pgp_pub_key: GPGKey,
}

#[derive(Debug, Clone)]
pub struct GPGKey {
    pub fingerprint: String,
    pub key: Vec<u8>,
}

#[async_trait]
pub trait UserRepository {
    async fn save(&mut self, user: User) -> Result<User>;
    async fn find_by_id(&self, id: ID) -> Result<Option<User>>;
}

#[derive(Debug, Error)]
pub enum DataBaseError {
    #[error("IO error")]
    IO(#[from] std::io::Error),
    #[error("Entity not found")]
    NotFound,
}
