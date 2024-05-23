use crate::domain::error::ApiError;
use async_trait::async_trait;

#[async_trait(?Send)]
pub trait AbstractRequest<T> {
    async fn execute(&self) -> Result<T, ApiError>;
}
