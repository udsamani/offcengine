use serde::{Deserialize, Serialize};

/// Enum Represneting different transaction types in the system.
pub enum TransactionType {
    /// A type of transaction indicating that funds are deposited
    /// into the client's account.
    Deposit,
    /// A type of transaction indicating that funds are withdrawed
    /// from the client's account
    Withdrawal,
    /// A type of transaction indicating that a previous transaction
    /// is disputed. This type of transaciton has no amount to it.
    /// Rather it has a transaction number which is being disputed.
    /// For example, let say we have a following transaction
    ///
    /// {type: deposit, client_id: 1, tx_id: 10, amount: 440.66}
    ///
    /// Let's say later the client disputes the above transaction. Then
    /// the entry for disputed transaction would look something like this
    ///
    /// {type: dispute, client_id: 1, tx_id: 10, amount: None}
    ///
    /// Also we should note that the implication of this type of transaction
    /// is that available funds decrease by the amount mentioned in tx_id and
    /// held funds increase by the same amount.
    Dispute,
    /// A type of transaction indicating that a disputed transaction
    /// is now resolved. The implication of this type of transaction would
    /// be that available funds would increase by the amount in the transaction
    /// which was being disputed and the held funds would decrease by the same
    /// amount.
    Resolve,
    /// A type of transaction indicating that a chargeback has occured for a
    /// disputed transaction. The implication of this type of transaction would
    /// be that held funds decrease and total funds decrease. In addition,
    /// the client's account goes to locked state.
    Chargeback,
}

/// A struct representing a Transaction that has occurred in the system.
#[derive(Deserialize, Serialize)]
pub struct Transaction {
    /// Type of the transaction.
    t_type: TransactionType,
    /// Id of the client who performed this transaction.
    client: u16,
    /// A unique identifier for transaction.
    tx: u32,
    /// Amount deposited or withdrawed in the transaction. Can be
    /// None for dispute, resolve, and chargeback type of transactions.
    amount: Option<f64>,
}
