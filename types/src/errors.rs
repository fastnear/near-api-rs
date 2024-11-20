#[derive(Debug, Clone, thiserror::Error, PartialEq, Eq)]
pub enum CryptoHashError {
    #[error(transparent)]
    Base58DecodeError(#[from] bs58::decode::Error),
    #[error("Incorrect hash length (expected 32, but {0} was given)")]
    IncorrectHashLength(usize),
}

#[derive(Debug, Clone, thiserror::Error, PartialEq, Eq)]
pub enum DecimalNumberParsingError {
    #[error("Invalid number: {0}")]
    InvalidNumber(String),
    #[error("Too long whole part: {0}")]
    LongWhole(String),
    #[error("Too long fractional part: {0}")]
    LongFractional(String),
}
