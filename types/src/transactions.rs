use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PrepopulateTransaction {
    pub signer_id: crate::AccountId,
    pub receiver_id: crate::AccountId,
    pub actions: Vec<crate::actions::Action>,
}
