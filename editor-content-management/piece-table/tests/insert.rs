extern crate piece_table;

use std::str::FromStr;
use piece_table::ContentManager;

#[test]
fn insert_at_beginning_small_file() {
    let mut buffer = piece_table::PieceTable::from_str(small_file_contents()).unwrap();

    buffer.insert(0,0, &mut "Hello".to_string());

    assert_eq!("Hello<?php
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

// #[test]
// fn insert_at_end_small_file() {
//     let mut buffer = gap::GapBuffer::from_str(small_file_contents()).unwrap();

//     buffer.insert(18,0, &mut "Hello".to_string());

//     assert_eq!("<?php
// namespace Hello\\World;

// /**
//  * Hello world class.
//  */
// class HelloWorld
// {
//     /**
//      * Function to say hi.
//      *
//      * @return string
//      */
//     public function hi()
//     {
//         return \"hi\";
//     }
// }
// Hello", format!("{}", buffer));
// }

// #[test]
// fn insert_in_the_middle_small_file() {
//     let mut buffer = gap::GapBuffer::from_str(small_file_contents()).unwrap();

//     buffer.insert(15,18, &mut ", world".to_string());

//     assert_eq!("<?php
// namespace Hello\\World;

// /**
//  * Hello world class.
//  */
// class HelloWorld
// {
//     /**
//      * Function to say hi.
//      *
//      * @return string
//      */
//     public function hi()
//     {
//         return \"hi, world\";
//     }
// }
// ", format!("{}", buffer));
// }

// #[test]
// fn insert_with_gap_resize_small_file() {
//     let mut buffer = gap::GapBuffer::from_str(small_file_contents()).unwrap();

//     buffer.insert(2,0, &mut "123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890".to_string());

//     assert_eq!("<?php
// namespace Hello\\World;
// 123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890
// /**
//  * Hello world class.
//  */
// class HelloWorld
// {
//     /**
//      * Function to say hi.
//      *
//      * @return string
//      */
//     public function hi()
//     {
//         return \"hi\";
//     }
// }
// ", format!("{}", buffer));
// }

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
