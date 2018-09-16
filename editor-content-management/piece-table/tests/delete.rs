extern crate piece_table;

use piece_table::ContentManager;
use std::str::FromStr;

#[test]
fn delete_at_beginning_small_file() {
    let mut buffer = piece_table::PieceTable::from_str(small_file_contents()).unwrap();

    buffer.delete(0,0, 0,1);

    assert_eq!("php
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

#[test]
fn delete_in_the_middle_small_file() {
    let mut buffer = piece_table::PieceTable::from_str(small_file_contents()).unwrap();

    buffer.delete(6,11, 6,15);

    assert_eq!("<?php
namespace Hello\\World;

/**
 * Hello world class.
 */
class Hello
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

#[test]
fn delete_at_end_small_file() {
    let mut buffer = piece_table::PieceTable::from_str(small_file_contents()).unwrap();

    buffer.delete(17,1, 18,0);

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
}", format!("{}", buffer));
}

#[test]
fn delete_with_gap_resize() {
    let mut buffer = piece_table::PieceTable::from_str(small_file_contents()).unwrap();

    buffer.delete(2,0, 18,0);

    assert_eq!("<?php
namespace Hello\\World;
", format!("{}", buffer));
}

fn small_file_contents<'a>() -> &'a str {
    return "<?php
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
";
}

