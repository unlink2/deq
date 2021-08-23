extern crate deq_core;
extern crate deq_macros;

pub use deq_core::*;
pub use deq_macros::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[transaction_fields]
    #[derive(Clone, Transaction, PartialEq, Eq, PartialOrd, Ord)]
    struct Test {
        x: i64,
        y: i64,
        z: i64,
    }

    #[test]
    fn it_should_start_a_transaction() {
        let mut t = Test {
            x: 100,
            y: 200,
            z: 300,
            transaction_data: TransactionData::new(),
        };
        t.begin();
        t.x += 20;
        let _ = t.commit().unwrap();

        assert_eq!(t.x, 120);
        assert_eq!(t.transaction_data.t.len(), 0);
    }

    #[test]
    fn it_should_revert_a_transaction() {
        let mut t = Test {
            x: 100,
            y: 200,
            z: 300,
            transaction_data: TransactionData::new(),
        };
        t.begin();
        t.x += 20;
        let _ = t.revert().unwrap();

        assert_eq!(t.x, 100);
        assert_eq!(t.transaction_data.t.len(), 0);
    }

    #[test]
    fn it_should_allow_many_transactions() {
        let mut t = Test {
            x: 100,
            y: 200,
            z: 300,
            transaction_data: TransactionData::new(),
        };
        t.begin();
        t.x += 20;
        t.begin();
        t.x += 30;
        t.begin();
        t.x += 5;
        let _ = t.revert().unwrap();
        let _ = t.commit().unwrap();
        let _ = t.commit().unwrap();

        assert_eq!(t.x, 150);
        assert_eq!(t.len(), 0);
    }

    #[test]
    #[should_panic]
    fn it_should_not_revert_if_transaction_was_not_started() {
        let mut t = Test {
            x: 100,
            y: 200,
            z: 300,
            transaction_data: TransactionData::new(),
        };
        t.x += 20;
        let _ = t.revert().unwrap();

        assert_eq!(t.x, 100);
        assert_eq!(t.len(), 0);
    }

    #[test]
    #[should_panic]
    fn it_should_not_commit_if_transaction_was_not_started() {
        let mut t = Test {
            x: 100,
            y: 200,
            z: 300,
            transaction_data: TransactionData::new(),
        };
        t.x += 20;
        let _ = t.commit().unwrap();

        assert_eq!(t.x, 100);
        assert_eq!(t.len(), 0);
    }
}
