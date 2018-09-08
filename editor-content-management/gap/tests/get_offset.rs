extern crate gap;

use std::str::FromStr;

#[test]
fn before_gap() {
    let buffer = gap::GapBuffer::from_str("Before gap  After gap").unwrap();

    assert_eq!(0, buffer.get_offset(0,0));
}

#[test]
fn after_gap() {
    let buffer = gap::GapBuffer::from_str("Before gap  After gap").unwrap();

    assert_eq!(13, buffer.get_offset(0,13));
}

#[test]
fn line_counting() {
    let buffer = gap::GapBuffer::from_str("Before gap\n\n\n\nAfter gap").unwrap();

    assert_eq!(14, buffer.get_offset(4,0));
}
