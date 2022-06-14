use crate::model::transaction::transaction_server::Transaction;

pub struct TransactionService;

impl TransactionService {
    pub fn new() -> Self {
        Self {}
    }
}

impl Transaction for TransactionService {

}