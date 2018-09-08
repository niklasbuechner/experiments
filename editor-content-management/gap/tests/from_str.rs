extern crate gap;

use std::str::FromStr;

#[test]
fn before_gap() {
    let buffer = gap::GapBuffer::from_str("Before gap  After gap").unwrap();
    let expected_result = "Before gap ääääääääääääääääääääääääääääääääääääääääääääääääääääääääääääääääääääääääääääääääääääääääääääääääääää After gap --DONE--";

    assert_eq!(expected_result, buffer.visualize_gap());
}
