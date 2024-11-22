use std::fmt;

pub mod actions;
pub mod contract;
pub mod errors;
pub mod reference;
pub mod stake;
pub mod storage;
pub mod tokens;
pub mod transactions;
pub mod utils;
pub mod views;

use errors::CryptoHashError;
pub use near_abi::AbiRoot;
pub use near_account_id::AccountId;
pub use near_gas::NearGas;
pub use near_token::NearToken;
// TODO: remove this
pub use near_crypto::{ED25519PublicKey, ED25519SecretKey, PublicKey, SecretKey, Signature};
// TODO: remove this
pub use near_primitives::{
    transaction::Transaction,
    views::{EpochValidatorInfo, FinalExecutionOutcomeView},
};
// TODO: remove this
pub use near_contract_standards::{
    fungible_token::metadata::FungibleTokenMetadata,
    non_fungible_token::metadata::TokenMetadata,
    non_fungible_token::{metadata::NFTContractMetadata, Token},
};

pub type BlockHeight = u64;
pub type Nonce = u64;
pub type ShardId = u64;
pub type StorageUsage = u64;
pub use near_gas::NearGas as Gas;

fn from_base58(s: &str) -> Result<Vec<u8>, bs58::decode::Error> {
    bs58::decode(s).into_vec()
}

// type taken from near_primitives::hash::CryptoHash.
/// CryptoHash is type for storing the hash of a specific block.
#[derive(
    Copy,
    Clone,
    Default,
    Hash,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    serde::Serialize,
    serde::Deserialize,
    borsh::BorshDeserialize,
    borsh::BorshSerialize,
)]
pub struct CryptoHash(pub [u8; 32]);

impl std::str::FromStr for CryptoHash {
    type Err = CryptoHashError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bytes = from_base58(s)?;
        Self::try_from(bytes)
    }
}

impl TryFrom<&[u8]> for CryptoHash {
    type Error = CryptoHashError;

    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        if bytes.len() != 32 {
            return Err(CryptoHashError::IncorrectHashLength(bytes.len()));
        }
        let mut buf = [0; 32];
        buf.copy_from_slice(bytes);
        Ok(Self(buf))
    }
}

impl TryFrom<Vec<u8>> for CryptoHash {
    type Error = CryptoHashError;

    fn try_from(v: Vec<u8>) -> Result<Self, Self::Error> {
        <Self as TryFrom<&[u8]>>::try_from(v.as_ref())
    }
}

impl std::fmt::Debug for CryptoHash {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl std::fmt::Display for CryptoHash {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        std::fmt::Display::fmt(&bs58::encode(self.0).into_string(), f)
    }
}

impl From<near_primitives::hash::CryptoHash> for CryptoHash {
    fn from(hash: near_primitives::hash::CryptoHash) -> Self {
        Self(hash.0)
    }
}

impl From<CryptoHash> for near_primitives::hash::CryptoHash {
    fn from(hash: CryptoHash) -> Self {
        Self(hash.0)
    }
}
