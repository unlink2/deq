extern crate deq;

use deq::*;
use std::fmt;

#[derive(Clone, PartialEq, Eq, Debug)]
struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

fn main() {
    // Sample transaction
    let mut container = Revertable::new(Point::new(50, 60));
    println!("{}", container.get());
    println!("History: {}", container.len());

    // begin a transaction
    container.get_mut().x = 70;
    println!("{}", container.get());
    println!("History: {}", container.len());

    // revert a transaction. This fails with an error
    // if there is no history
    container.revert().unwrap();
    println!("{}", container.get());
    println!("History: {}", container.len());

    // being transaction
    container.get_mut().y = 40;
    println!("{}", container.get());
    println!("History: {}", container.len());

    // commit a transaction. This fails with an error
    // if there is no history
    container.commit().unwrap();
    println!("{}", container.get());
    println!("History: {}", container.len());
}
