pub mod cat;
pub mod user;


use crate::utils::models::ModelExt;
use crate::errors::Error;

pub async fn sync_indexes() -> Result<(), Error> {
    user::User::sync_indexes().await?;
    cat::Cat::sync_indexes().await?;

    Ok(())
}