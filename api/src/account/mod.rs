use std::convert::Infallible;

use crate::secret::SecretBuilder;
use crate::transactions::ConstructTransaction;
use executor::query::{
    AccessKeyHandler, AccessKeyListHandler, AccountViewHandler, QueryBuilder, QueryRequest,
    RpcBuilder, SimpleQuery,
};
use near_types::{
    actions::{Action, AddKeyAction, DeleteAccountAction, DeleteKeyAction},
    reference::Reference,
    views::{AccessKey, AccessKeyPermission},
    AccountId, PublicKey,
};

use self::create::CreateAccountBuilder;

mod create;

#[derive(Clone, Debug)]
pub struct Account(pub AccountId);

impl Account {
    pub fn view(&self) -> QueryBuilder<AccountViewHandler> {
        let request = QueryRequest::ViewAccount {
            account_id: self.0.clone(),
        };
        QueryBuilder::new(
            SimpleQuery { request },
            Reference::Optimistic,
            Default::default(),
        )
    }

    pub fn access_key(&self, signer_public_key: PublicKey) -> QueryBuilder<AccessKeyHandler> {
        let request = QueryRequest::ViewAccessKey {
            account_id: self.0.clone(),
            public_key: signer_public_key,
        };
        RpcBuilder::new(
            SimpleQuery { request },
            Reference::Optimistic,
            Default::default(),
        )
    }

    pub fn list_keys(&self) -> QueryBuilder<AccessKeyListHandler> {
        let request = QueryRequest::ViewAccessKeyList {
            account_id: self.0.clone(),
        };
        RpcBuilder::new(
            SimpleQuery { request },
            Reference::Optimistic,
            Default::default(),
        )
    }

    pub fn add_key(
        &self,
        permission: AccessKeyPermission,
    ) -> SecretBuilder<ConstructTransaction, Infallible> {
        let account_id = self.0.clone();
        SecretBuilder::new(move |public_key| {
            Ok(
                ConstructTransaction::new(account_id.clone(), account_id.clone()).add_action(
                    Action::AddKey(Box::new(AddKeyAction {
                        access_key: AccessKey {
                            nonce: 0,
                            permission: permission.into(),
                        },
                        public_key,
                    })),
                ),
            )
        })
    }

    pub fn delete_key(&self, public_key: PublicKey) -> ConstructTransaction {
        ConstructTransaction::new(self.0.clone(), self.0.clone())
            .add_action(Action::DeleteKey(Box::new(DeleteKeyAction { public_key })))
    }

    pub fn delete_keys(&self, public_keys: Vec<PublicKey>) -> ConstructTransaction {
        let actions = public_keys
            .into_iter()
            .map(|public_key| Action::DeleteKey(Box::new(DeleteKeyAction { public_key })))
            .collect();

        ConstructTransaction::new(self.0.clone(), self.0.clone()).add_actions(actions)
    }

    pub fn delete_account_with_beneficiary(
        &self,
        beneficiary_id: AccountId,
    ) -> ConstructTransaction {
        ConstructTransaction::new(self.0.clone(), self.0.clone()).add_action(Action::DeleteAccount(
            DeleteAccountAction { beneficiary_id },
        ))
    }

    pub const fn create_account() -> CreateAccountBuilder {
        CreateAccountBuilder
    }
}
