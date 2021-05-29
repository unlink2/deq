extern crate deq;

use deq::*;

#[transaction_fields]
#[derive(Clone, Transaction)]
struct Example {
    x: i32,
    y: i32,
    z: i32
}

fn main() {
    // You will have to initialize transaction data for each struct
    // TODO maybe find an easier solution in the future?
    let mut t = Example { x: 0, y: 0, z: 0, transaction_data: TransactionData::new() };

    // begin a transaction
    t.begin();
    t.x += 100;

    // this will return an error if
    // no transaction was started
    let _ = t.commit();

    println!("X is {}", t.x);

    // transactions can be fully reverted
    t.begin();
    t.x += 50;

    // also returns an error if there are no open transactions
    let _ = t.revert();
    println!("X is {}", t.x);

    // there can be more than one running transaction
    t.begin();
    t.x += 10;
    t.begin();
    t.x += 20;
    println!("Open transactions {}", t.len());

    println!("X is {}", t.x);
    // revert  one
    let _ = t.revert();

    // commit the other
    let _ = t.commit();
    println!("X is {}", t.x);
}
