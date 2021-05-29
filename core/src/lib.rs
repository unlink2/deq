use std::fmt;

/// Transaction trait
pub trait Transaction: Clone {
    /// begin a transaction
    fn begin(&mut self);

    /// commit a transaction
    fn commit(&mut self) -> Result<(), TransactionError>;

    /// revert a transaction
    fn revert(&mut self) -> Result<(), TransactionError>;

    /// amount of open transactions
    fn len(&self) -> usize;
}

#[derive(Clone)]
pub struct TransactionData<T> {
    pub t: Vec<T>
}

impl<T> TransactionData<T> {
    pub fn new() -> Self {
        Self {
            t: vec![]
        }
    }
}

/// Transaction Errors
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum TransactionErrorType {
    TransactionNotStarted
}

/// Transaction Error Type
#[derive(Debug, Copy, Clone)]
pub struct TransactionError {
    pub cause: TransactionErrorType
}

impl TransactionError {
    pub fn new(cause: TransactionErrorType) -> Self {
        Self {
            cause
        }
    }

    pub fn to_string(&self) -> &str {
        match self.cause {
            TransactionErrorType::TransactionNotStarted => "Transaction not started"
        }
    }
}

impl std::error::Error for TransactionError {
    fn description(&self) -> &str {
        return self.to_string();
    }
}

impl fmt::Display for TransactionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}
