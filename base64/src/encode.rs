use super::BASE64_ALPHABET;
use std::slice::Iter;

struct Base64Encoder<'a, I>
where
    I: Iterator<Item = &'a u8>,
{
    iterator: I,
    output_buffer: Vec<char>,
}

impl<'a, I> Base64Encoder<'a, I>
where
    I: Iterator<Item = &'a u8>,
{
    pub fn new(byte_iteror: I) -> Self {
        Base64Encoder {
            iterator: byte_iteror,
            output_buffer: Vec::with_capacity(4),
        }
    }

    fn fill_output_buffer(&mut self) {
        let first_byte = self.iterator.next();
        let second_byte = self.iterator.next();
        let third_byte = self.iterator.next();

        let mut base64_characters = match (first_byte, second_byte, third_byte) {
            (Some(first), Some(second), Some(third)) => vec![
                BASE64_ALPHABET[extract_first_character_bits(*first)],
                BASE64_ALPHABET[extract_second_character_bits(*first, *second)],
                BASE64_ALPHABET[extract_third_character_bits(*second, *third)],
                BASE64_ALPHABET[extract_fourth_character_bits(*third)],
            ],
            (Some(first), Some(second), None) => vec![
                BASE64_ALPHABET[extract_first_character_bits(*first)],
                BASE64_ALPHABET[extract_second_character_bits(*first, *second)],
                BASE64_ALPHABET[extract_third_character_bits(*second, 0)],
                '=',
            ],
            (Some(first), None, None) => vec![
                BASE64_ALPHABET[extract_first_character_bits(*first)],
                BASE64_ALPHABET[extract_second_character_bits(*first, 0)],
                '=',
                '=',
            ],
            _ => Vec::new(),
        };

        self.output_buffer.append(&mut base64_characters);
    }
}

impl<'a, I> Iterator for Base64Encoder<'a, I>
where
    I: Iterator<Item = &'a u8>,
{
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        if self.output_buffer.len() == 0 {
            self.fill_output_buffer();
        }

        if self.output_buffer.len() == 0 {
            None
        } else {
            Some(self.output_buffer.remove(0))
        }
    }
}

trait Base64Encoding<'a, I>
where
    I: Iterator<Item = &'a u8>,
{
    fn base64_encode(&'a self) -> Base64Encoder<'a, I>;
}

impl<'a> Base64Encoding<'a, Iter<'a, u8>> for str {
    fn base64_encode(&'a self) -> Base64Encoder<'a, Iter<'a, u8>> {
        Base64Encoder::new(self.as_bytes().iter())
    }
}

impl<'a> Base64Encoding<'a, Iter<'a, u8>> for String {
    fn base64_encode(&'a self) -> Base64Encoder<'a, Iter<'a, u8>> {
        Base64Encoder::new(self.as_bytes().iter())
    }
}

pub fn base64_encode(content: &str) -> String {
    content.base64_encode().collect::<String>()
}

fn extract_first_character_bits(first_byte: u8) -> usize {
    ((first_byte & 0b1111100) >> 2) as usize
}

fn extract_second_character_bits(first_byte: u8, second_byte: u8) -> usize {
    ((first_byte & 0b00000011) << 4 | ((second_byte & 0b11110000) >> 4)) as usize
}

fn extract_third_character_bits(second_byte: u8, third_byte: u8) -> usize {
    ((second_byte & 0b00001111) << 2 | ((third_byte & 0b11000000) >> 6)) as usize
}

fn extract_fourth_character_bits(third_byte: u8) -> usize {
    (third_byte & 0b00111111) as usize
}

#[test]
fn encode_simple_string() {
    let content = "Hello World, how are you doing?\n";
    let expected_base64 = "SGVsbG8gV29ybGQsIGhvdyBhcmUgeW91IGRvaW5nPwo=";

    assert_eq!(expected_base64, base64_encode(content));
}

#[test]
fn encode_three_characters() {
    let content = "abc";
    let expected_base64 = "YWJj";

    assert_eq!(expected_base64, base64_encode(content));
}

#[test]
fn encode_four_characters() {
    let content = "abcd";
    let expected_base64 = "YWJjZA==";

    assert_eq!(expected_base64, base64_encode(content));
}

#[test]
fn encode_five_characters() {
    let content = "abcde";
    let expected_base64 = "YWJjZGU=";

    assert_eq!(expected_base64, base64_encode(content));
}

#[test]
fn encode_six_characters() {
    let content = "abcabc";
    let expected_base64 = "YWJjYWJj";

    assert_eq!(expected_base64, base64_encode(content));
}
