extern crate gap;

use std::str::FromStr;

#[test]
fn before_gap() {
    let buffer = gap::GapBuffer::from_str("Before gap -  After gap").unwrap();

    assert_eq!('B', buffer.get_at(0, 0));
}

#[test]
fn after_gap() {
    let buffer = gap::GapBuffer::from_str("Before gap -  After gap").unwrap();

    assert_eq!('A', buffer.get_at(0, 14));
}
