pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("email is taken")]
    EmailTaken,
    #[error("an unexpected error occurred")]
    Unexpected(#[from] anyhow::Error),
}
