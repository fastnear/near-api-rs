use crate::{views::AccessKey, AccountId, PublicKey};
use borsh::{BorshDeserialize, BorshSerialize};
use macros::Equivalent;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq, Eq)]
pub enum Action {
    /// Create an (sub)account using a transaction `receiver_id` as an ID for
    /// a new account ID must pass validation rules described here
    /// <http://nomicon.io/Primitives/Account.html>.
    CreateAccount(CreateAccountAction),
    /// Sets a Wasm code to a receiver_id
    DeployContract(DeployContractAction),
    FunctionCall(Box<FunctionCallAction>),
    Transfer(TransferAction),
    Stake(Box<StakeAction>),
    AddKey(Box<AddKeyAction>),
    DeleteKey(Box<DeleteKeyAction>),
    DeleteAccount(DeleteAccountAction),
}

impl From<Action> for near_primitives::transaction::Action {
    fn from(value: Action) -> Self {
        match value {
            Action::CreateAccount(action) => {
                near_primitives::transaction::Action::CreateAccount(action.into())
            }
            Action::DeployContract(action) => {
                near_primitives::transaction::Action::DeployContract(action.into())
            }
            Action::FunctionCall(action) => {
                near_primitives::transaction::Action::FunctionCall(Box::new((*action).into()))
            }
            Action::Transfer(action) => {
                near_primitives::transaction::Action::Transfer(action.into())
            }
            Action::Stake(action) => {
                near_primitives::transaction::Action::Stake(Box::new((*action).into()))
            }
            Action::AddKey(action) => {
                near_primitives::transaction::Action::AddKey(Box::new((*action).into()))
            }
            Action::DeleteKey(action) => {
                near_primitives::transaction::Action::DeleteKey(Box::new((*action).into()))
            }
            Action::DeleteAccount(action) => {
                near_primitives::transaction::Action::DeleteAccount(action.into())
            }
        }
    }
}

#[derive(
    Serialize,
    Deserialize,
    Equivalent,
    Debug,
    Clone,
    BorshSerialize,
    BorshDeserialize,
    PartialEq,
    Eq,
)]
#[equivalent(near_primitives::transaction::CreateAccountAction)]
pub struct CreateAccountAction {}

#[derive(
    Serialize,
    Deserialize,
    Equivalent,
    Debug,
    Clone,
    BorshSerialize,
    BorshDeserialize,
    PartialEq,
    Eq,
)]
#[equivalent(near_primitives::transaction::DeployContractAction)]
pub struct DeployContractAction {
    pub code: Vec<u8>,
}

#[derive(
    Serialize,
    Deserialize,
    Equivalent,
    Debug,
    Clone,
    BorshSerialize,
    BorshDeserialize,
    PartialEq,
    Eq,
)]
#[equivalent(near_primitives::transaction::FunctionCallAction)]
pub struct FunctionCallAction {
    pub method_name: String,
    pub args: Vec<u8>,
    pub gas: u64,
    pub deposit: u128,
}

#[derive(
    Serialize,
    Deserialize,
    Equivalent,
    Debug,
    Clone,
    BorshSerialize,
    BorshDeserialize,
    PartialEq,
    Eq,
)]
#[equivalent(near_primitives::transaction::TransferAction)]
pub struct TransferAction {
    pub deposit: u128,
}

#[derive(
    Serialize,
    Deserialize,
    Equivalent,
    Debug,
    Clone,
    BorshSerialize,
    BorshDeserialize,
    PartialEq,
    Eq,
)]
#[equivalent(near_primitives::transaction::StakeAction)]
pub struct StakeAction {
    /// Amount of tokens to stake.
    pub stake: u128,
    /// Validator key which will be used to sign transactions on behalf of signer_id
    pub public_key: PublicKey,
}

#[derive(
    Serialize,
    Deserialize,
    Equivalent,
    Debug,
    Clone,
    BorshSerialize,
    BorshDeserialize,
    PartialEq,
    Eq,
)]
#[equivalent(near_primitives::transaction::DeleteKeyAction)]
pub struct DeleteKeyAction {
    /// A public key associated with the access_key to be deleted.
    pub public_key: PublicKey,
}

#[derive(
    Serialize,
    Deserialize,
    Equivalent,
    Debug,
    Clone,
    BorshSerialize,
    BorshDeserialize,
    PartialEq,
    Eq,
)]
#[equivalent(near_primitives::transaction::DeleteAccountAction)]
pub struct DeleteAccountAction {
    pub beneficiary_id: AccountId,
}

#[derive(
    Serialize,
    Deserialize,
    Equivalent,
    Debug,
    Clone,
    BorshSerialize,
    BorshDeserialize,
    PartialEq,
    Eq,
)]
#[equivalent(near_primitives::transaction::AddKeyAction)]
pub struct AddKeyAction {
    /// A public key which will be associated with an access_key
    pub public_key: PublicKey,
    /// An access key with the permission
    pub access_key: AccessKey,
}
