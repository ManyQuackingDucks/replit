use thiserror::Error;

#[derive(Error, Debug)]
pub enum DBErrors {
    #[error("The connection to the db could not be established")]
    Conn{
        #[from]
        source: hyper::Error,
    },
    #[error("One of the keys in the db is not valid utf-8 data")]
    NotText{
        #[from]
        source: std::str::Utf8Error,
    },
    #[error("Key {0} was not found.")]
    NotFound(String),
    #[error("The request did not return with success.")]
    NotSucc,
}
