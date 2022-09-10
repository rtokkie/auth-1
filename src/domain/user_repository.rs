use crate::domain::user::User;
use async_trait::async_trait;

#[async_trait]
pub trait UserRepository {
    async fn find_one(&self, id: &str) -> anyhow::Result<User>;
    async fn insert(&self, user: User) -> anyhow::Result<()>;
    async fn update(&self, user: User) -> anyhow::Result<()>;
    async fn delete(&self, id: &str) -> anyhow::Result<()>;
}
