use near_crypto::{PublicKey, SecretKey};
use near_primitives::{
    hash::CryptoHash,
    transaction::Transaction,
    types::{AccountId, Nonce},
    views::AccessKeyPermissionView,
};

use crate::{
    config::NetworkConfig,
    errors::{KeyStoreError, SignerError},
    types::transactions::PrepopulateTransaction,
};

use super::{AccountKeyPair, SignerTrait};

#[derive(Debug, Clone)]
pub struct KeystoreSigner {
    potential_pubkeys: Vec<PublicKey>,
}

#[async_trait::async_trait]
impl SignerTrait for KeystoreSigner {
    fn tx_and_secret(
        &self,
        tr: PrepopulateTransaction,
        public_key: PublicKey,
        nonce: Nonce,
        block_hash: CryptoHash,
    ) -> Result<(Transaction, SecretKey), SignerError> {
        self.potential_pubkeys
            .iter()
            .find(|key| *key == &public_key)
            .ok_or(SignerError::PublicKeyIsNotAvailable)?;

        // TODO: fix this. Well the search is a bit suboptimal, but it's not a big deal for now
        let secret = Self::get_secret_key(&tr.signer_id, &public_key, "mainnet")
            .or_else(|_| Self::get_secret_key(&tr.signer_id, &public_key, "testnet"))
            .map_err(|_| SignerError::SecretKeyIsNotAvailable)?;

        let mut transaction = Transaction::new_v0(
            tr.signer_id.clone(),
            public_key,
            tr.receiver_id,
            nonce,
            block_hash,
        );
        *transaction.actions_mut() = tr.actions;

        Ok((transaction, secret.private_key))
    }

    fn get_public_key(&self) -> Result<PublicKey, SignerError> {
        self.potential_pubkeys
            .first()
            .cloned()
            .ok_or(SignerError::PublicKeyIsNotAvailable)
    }
}

impl KeystoreSigner {
    pub fn new_with_pubkey(pub_key: PublicKey) -> Self {
        Self {
            potential_pubkeys: vec![pub_key],
        }
    }

    pub async fn search_for_keys(
        account_id: AccountId,
        network: &NetworkConfig,
    ) -> Result<Self, KeyStoreError> {
        let account_keys = crate::account::Account(account_id.clone())
            .list_keys()
            .fetch_from(network)
            .await?;

        let potential_pubkeys = account_keys
            .keys
            .into_iter()
            // TODO: support functional access keys
            .filter(|key| key.access_key.permission == AccessKeyPermissionView::FullAccess)
            .flat_map(|key| {
                Self::get_secret_key(&account_id, &key.public_key, &network.network_name)
                    .map(|keypair| keypair.public_key)
                    .ok()
            })
            .collect();

        Ok(Self { potential_pubkeys })
    }

    fn get_secret_key(
        account_id: &AccountId,
        public_key: &PublicKey,
        network_name: &str,
    ) -> Result<AccountKeyPair, KeyStoreError> {
        let service_name =
            std::borrow::Cow::Owned(format!("near-{}-{}", network_name, account_id.as_str()));

        let password =
            keyring::Entry::new(&service_name, &format!("{}:{}", account_id, public_key))?
                .get_password()?;

        Ok(serde_json::from_str(&password)?)
    }
}
