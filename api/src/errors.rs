pub use executor::errors::*;
pub use near_types::errors::*;

#[derive(thiserror::Error, Debug)]
pub enum SecretBuilderkError<E: std::fmt::Debug> {
    #[error("Public key is not available")]
    PublicKeyIsNotAvailable,
    #[error("Invalid HD path")]
    InvalidHDPath,
    #[error(transparent)]
    SecretError(#[from] SecretError),
    #[error(transparent)]
    IO(#[from] std::io::Error),
    #[error(transparent)]
    CallbackError(E),
}

#[derive(thiserror::Error, Debug)]
pub enum BuilderError {
    #[error("Incorrect arguments: {0}")]
    IncorrectArguments(#[from] serde_json::Error),
}

#[derive(thiserror::Error, Debug)]
pub enum AccountCreationError {
    #[error(transparent)]
    BuilderError(#[from] BuilderError),

    #[error("Top-level account is not allowed")]
    TopLevelAccountIsNotAllowed,

    #[error("Linkdrop is not defined in the network config")]
    LinkdropIsNotDefined,

    #[error("Account should be created as a subaccount of the signer or linkdrop account")]
    AccountShouldBeSubaccountOfSignerOrLinkdrop,
}

#[derive(thiserror::Error, Debug)]
pub enum FTValidatorError {
    #[error("Metadata is not provided")]
    NoMetadata,
    #[error("Decimals mismatch: expected {expected}, got {got}")]
    DecimalsMismatch { expected: u8, got: u8 },
    #[error("Storage deposit is needed")]
    StorageDepositNeeded,
}

#[derive(thiserror::Error, Debug)]
pub enum FastNearError {
    #[error("FastNear URL is not defined in the network config")]
    FastNearUrlIsNotDefined,
    #[error("Failed to send request: {0}")]
    SendError(#[from] reqwest::Error),
    #[error("Url parsing error: {0}")]
    UrlParseError(#[from] url::ParseError),
}

#[derive(thiserror::Error, Debug)]
pub enum MultiTransactionError {
    #[error(transparent)]
    SignerError(#[from] SignerError),
    #[error("Duplicate signer")]
    DuplicateSigner,

    #[error(transparent)]
    SignedTransactionError(#[from] ExecuteTransactionError),

    #[error("Failed to send meta-transaction: {0}")]
    MetaTransactionError(#[from] ExecuteMetaTransactionsError),
}
