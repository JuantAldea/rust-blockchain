pub mod block;
pub mod chain;
pub mod id;
pub mod signedtransaction;
pub mod transaction;
pub mod wallet;
#[cfg(test)]
mod tests;


#[derive(PartialEq, Eq, Debug)]
pub enum BlockChainOperationResult {
    BlockChainOk,
    BlockChainUpdated,
    BlockChainKept,
    HashMismatchError,
    ProofOfWorkError,
    IndexMismatchError,
    DoubleSpendingError,
    TXSumError,
    TxIdNotFound,
    InTxOwnershipError,
    InTxTooSmallForTransaction,
    InTxTooSmallForTransactionSet,
    SignatureError,
    SourceBlockIsNewerError,
}

pub trait Hashable {
    fn hash(&self) -> String;
}
