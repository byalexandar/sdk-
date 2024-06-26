use ic_agent::identity::PemError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ValidatePemFileError {
    #[error(transparent)]
    PemError(#[from] ic_agent::identity::PemError),

    #[error(
        "Ed25519 v1 keys (those generated by OpenSSL) are not supported. Try again with a v2 key"
    )]
    UnsupportedKeyVersion(),

    #[error("Failed to validate PEM content")]
    ValidatePemContentFailed(#[source] Box<PemError>),
}
