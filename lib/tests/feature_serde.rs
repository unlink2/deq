#[cfg(feature = "serde")]
mod tests {
    extern crate deq;
    extern crate serde_json;
    use self::deq::{Revertable, Transaction};

    #[test]
    fn it_should_use_serde() {
        let mut test = Revertable::new(100);

        *test.get_mut() = 300;

        let serialized = serde_json::to_string(&test).unwrap();

        #[cfg(not(feature = "serde_skip_history"))]
        assert_eq!(serialized, "{\"current\":300,\"history\":[100]}");

        #[cfg(feature = "serde_skip_history")]
        assert_eq!(serialized, "{\"current\":300}");
    }
}
