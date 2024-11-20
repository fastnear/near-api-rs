use near_primitives::transaction::SignedTransaction;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PrepopulateTransaction {
    pub signer_id: crate::AccountId,
    pub receiver_id: crate::AccountId,
    pub actions: Vec<crate::actions::Action>,
}

impl From<SignedTransaction> for PrepopulateTransaction {
    fn from(tr: SignedTransaction) -> Self {
        Self {
            signer_id: tr.transaction.signer_id().clone(),
            receiver_id: tr.transaction.receiver_id().clone(),
            actions: tr
                .transaction
                .take_actions()
                .into_iter()
                .map(Into::into)
                .collect(),
        }
    }
}
