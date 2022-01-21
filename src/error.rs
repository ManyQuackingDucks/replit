use thiserror::Error;

#[derive(Error, Debug)]
pub enum DBErrors {
    #[error("The connection to the db could not be establisheed")]
    Conn,
    #[error("One of the keys in the db is not valid utf-8 data")]
    NotText,
    #[error("The env key with the db url does not exist.")]
    NoUrl,
    #[error("The request did not return with success.")]
    NotSucc,
}
