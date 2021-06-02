use std::fmt;
use std::cmp::Ordering;

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

impl<T> PartialEq<TransactionData<T>> for TransactionData<T>
where T: PartialEq<T> {
    fn eq(&self, other: &TransactionData<T>) -> bool {
        return self.t == other.t;
    }

    fn ne(&self, other: &TransactionData<T>) -> bool {
        return self.t != other.t;
    }
}

impl<T> Eq for TransactionData<T>
where T: Eq {}

impl<T> PartialOrd for TransactionData<T>
where T: PartialOrd {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.t.partial_cmp(&other.t)
    }
}

impl<T> Ord for TransactionData<T>
where T: Ord {
    fn cmp(&self, other: &Self) -> Ordering {
        self.t.cmp(&other.t)
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
