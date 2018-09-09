extern crate piece_table;

use std::str::FromStr;

#[test]
fn to_string() {
    let buffer = piece_table::PieceTable::from_str("<?php
namespace Hello\\World;

/**
 * Hello world class.
 */
class HelloWorld
{
    /**
     * Function to say hi.
     *
     * @return string
     */
    public function hi()
    {
        return \"hi\";
    }
}
").unwrap();

    assert_eq!("<?php
namespace Hello\\World;

/**
 * Hello world class.
 */
class HelloWorld
{
    /**
     * Function to say hi.
     *
     * @return string
     */
    public function hi()
    {
        return \"hi\";
    }
}
", format!("{}", buffer));
}
