pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("email is taken")]
    EmailTaken,
    #[error(transparent)]
    Unexpected(#[from] anyhow::Error),
}
