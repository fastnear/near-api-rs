use crate::{create_equivalent_types, AccountId, PublicKey};
use borsh::{BorshDeserialize, BorshSerialize};
use serde::{Deserialize, Serialize};

create_equivalent_types!(
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
    } as near_primitives::transaction::Action;

    #[derive(
        Serialize, Deserialize, Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq, Eq,
    )]
    pub struct CreateAccountAction {} as near_primitives::transaction::CreateAccountAction;

    #[derive(
        Serialize, Deserialize, Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq, Eq,
    )]
    pub struct DeployContractAction {
        pub code: Vec<u8>,
    } as near_primitives::transaction::DeployContractAction;

    #[derive(
        Serialize, Deserialize, Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq, Eq,
    )]
    pub struct FunctionCallAction {
        pub method_name: String,
        pub args: Vec<u8>,
        pub gas: u64,
        pub deposit: u128,
    } as near_primitives::transaction::FunctionCallAction;

    #[derive(
        Serialize, Deserialize, Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq, Eq,
    )]
    pub struct TransferAction {
        pub deposit: u128,
    } as near_primitives::transaction::TransferAction;

    #[derive(
        Serialize, Deserialize, Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq, Eq,
    )]
    pub struct StakeAction {
        /// Amount of tokens to stake.
        pub stake: u128,
        /// Validator key which will be used to sign transactions on behalf of signer_id
        pub public_key: PublicKey,
    } as near_primitives::transaction::StakeAction;

    #[derive(
        Serialize, Deserialize, Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq, Eq,
    )]
    pub struct DeleteKeyAction {
        /// A public key associated with the access_key to be deleted.
        pub public_key: PublicKey,
    } as near_primitives::transaction::DeleteKeyAction;

    #[derive(
        Serialize, Deserialize, Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq, Eq,
    )]
    pub struct DeleteAccountAction {
        pub beneficiary_id: AccountId,
    } as near_primitives::transaction::DeleteAccountAction;
);
