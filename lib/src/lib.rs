pub trait Transaction<T>
where
    T: Clone,
{
    /**
     * Returns the current
     * version of the contained item
     */
    fn get(&self) -> &T;

    /**
     * Returns a mutable version of the
     * current item
     * Obtaining a mutable reference
     * assumes the state will change
     * This effectively begins a transaction
     * should call begin
     */
    fn get_mut(&mut self) -> &mut T;

    /**
     * Begins a new transaction.
     * Saves the current state to history.
     */
    fn begin(&mut self);

    /**
     * Commites the changes made
     * and removes the last item from history.
     * A commit should commit in FIFO order
     */
    fn commit(&mut self) -> Result<(), TransactionError>;

    /**
     * Reverts the changes made and
     * removes the last item fro mhistory
     * A revert should  revert in LIFO order
     */
    fn revert(&mut self) -> Result<(), TransactionError>;

    /**
     * Accepts all pending changes
     */
    fn commit_all(&mut self) -> Result<(), TransactionError>;

    /**
     * Reverts all pending changes
     */
    fn revert_all(&mut self) -> Result<(), TransactionError>;

    /**
     * Clears histroy
     */
    fn clear(&mut self);

    /**
     * Returns true if a change occured
     */
    fn changed(&self) -> bool;

    /**
     * Returns length of history
     */
    fn len(&self) -> usize;
}

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Revertable<T>
where
    T: Clone,
{
    current: T,

    #[cfg_attr(feature = "serde_skip_history", serde(skip_serializing))]
    history: Vec<T>,
}

impl<T> Revertable<T>
where
    T: Clone,
{
    pub fn new(current: T) -> Self {
        Self {
            current,
            history: vec![],
        }
    }
}

impl<T> Transaction<T> for Revertable<T>
where
    T: Clone,
{
    fn get(&self) -> &T {
        &self.current
    }

    fn get_mut(&mut self) -> &mut T {
        self.begin();
        &mut self.current
    }

    fn begin(&mut self) {
        self.history.push(self.current.clone());
    }

    fn commit(&mut self) -> Result<(), TransactionError> {
        if self.len() > 0 {
            self.history.remove(0);
            Ok(())
        } else {
            Err(TransactionError::TransactionNotStarted)
        }
    }

    fn revert(&mut self) -> Result<(), TransactionError> {
        match self.history.pop() {
            Some(d) => {
                self.current = d;
                Ok(())
            }
            _ => Err(TransactionError::TransactionNotStarted),
        }
    }

    fn commit_all(&mut self) -> Result<(), TransactionError> {
        if self.len() > 0 {
            self.clear();
            Ok(())
        } else {
            Err(TransactionError::TransactionNotStarted)
        }
    }

    fn revert_all(&mut self) -> Result<(), TransactionError> {
        if self.len() > 0 {
            self.current = self.history[0].clone();
            self.clear();
            Ok(())
        } else {
            Err(TransactionError::TransactionNotStarted)
        }
    }

    fn changed(&self) -> bool {
        self.len() > 0
    }

    fn clear(&mut self) {
        self.history.clear();
    }

    fn len(&self) -> usize {
        self.history.len()
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum TransactionError {
    TransactionNotStarted,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone, PartialEq, Eq, Debug)]
    struct Test {
        pub x: i32,
        pub y: i32,
    }

    impl Test {
        pub fn new(x: i32, y: i32) -> Self {
            Self { x, y }
        }
    }

    #[test]
    fn it_should_begin_transactions() {
        let mut t = Revertable::new(Test::new(100, 100));

        let m = t.get_mut();
        m.x = 200;
        m.y = 300;
        assert_eq!(m, &Test::new(200, 300));
        assert_eq!(t.len(), 1);
    }

    #[test]
    fn it_should_allow_many_transactions() {
        let mut t = Revertable::new(Test::new(100, 100));

        let m1 = t.get_mut();
        m1.x = 200;

        let m2 = t.get_mut();
        m2.y = 300;

        assert_eq!(m2, &Test::new(200, 300));
        assert_eq!(t.len(), 2);
    }

    #[test]
    fn it_should_revert_individual_transactions() {
        let mut t = Revertable::new(Test::new(100, 100));

        let m1 = t.get_mut();
        m1.x = 200;

        let m2 = t.get_mut();
        m2.y = 300;

        assert_eq!(Ok(()), t.revert());

        assert_eq!(t.get(), &Test::new(200, 100));
        assert_eq!(t.len(), 1);
    }

    #[test]
    fn it_should_revert_all_transactions() {
        let mut t = Revertable::new(Test::new(100, 100));

        let m1 = t.get_mut();
        m1.x = 200;

        let m2 = t.get_mut();
        m2.y = 300;

        assert_eq!(Ok(()), t.revert_all());

        assert_eq!(t.get(), &Test::new(100, 100));
        assert_eq!(t.len(), 0);
    }

    #[test]
    fn it_should_allow_commit_individual_transactions() {
        let mut t = Revertable::new(Test::new(100, 100));

        let m1 = t.get_mut();
        m1.x = 200;

        let m2 = t.get_mut();
        m2.y = 300;

        assert_eq!(t.commit(), Ok(()));

        assert_eq!(t.get(), &Test::new(200, 300));
        assert_eq!(t.len(), 1);
    }

    #[test]
    fn it_should_allow_commit_all_transactions() {
        let mut t = Revertable::new(Test::new(100, 100));

        let m1 = t.get_mut();
        m1.x = 200;

        let m2 = t.get_mut();
        m2.y = 300;

        assert_eq!(t.commit_all(), Ok(()));

        assert_eq!(t.get(), &Test::new(200, 300));
        assert_eq!(t.len(), 0);
    }

    #[test]
    fn it_should_revert_and_commit_in_order() {
        let mut t = Revertable::new(Test::new(100, 100));

        let m1 = t.get_mut();
        m1.x = 200;

        let m2 = t.get_mut();
        m2.y = 300;

        let m3 = t.get_mut();
        m3.x = 50;

        assert_eq!(t.get(), &Test::new(50, 300));
        assert_eq!(t.len(), 3);

        // m3 is reverted
        assert_eq!(t.revert(), Ok(()));
        assert_eq!(t.get(), &Test::new(200, 300));
        assert_eq!(t.len(), 2);

        // m1 is commited
        assert_eq!(t.commit(), Ok(()));
        assert_eq!(t.get(), &Test::new(200, 300));
        assert_eq!(t.len(), 1);

        // m2 is reverted
        assert_eq!(t.revert(), Ok(()));
        assert_eq!(t.get(), &Test::new(200, 100));
        assert_eq!(t.len(), 0);
    }

    #[test]
    fn commits_should_fail_if_there_are_not_transactions() {
        let mut t = Revertable::new(Test::new(100, 100));

        assert_eq!(t.commit(), Err(TransactionError::TransactionNotStarted));
        assert_eq!(t.commit_all(), Err(TransactionError::TransactionNotStarted));
        assert_eq!(t.len(), 0);
    }

    #[test]
    fn reverts_should_fail_if_there_are_not_transactions() {
        let mut t = Revertable::new(Test::new(100, 100));

        assert_eq!(t.revert(), Err(TransactionError::TransactionNotStarted));
        assert_eq!(t.revert_all(), Err(TransactionError::TransactionNotStarted));
        assert_eq!(t.len(), 0);
    }
}
