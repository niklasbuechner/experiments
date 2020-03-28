use super::BASE64_ALPHABET;

pub fn base64_encode(content: &str) -> String {
    let characters: &[u8] = content.as_bytes();
    let mut base64_output = Vec::with_capacity((characters.len() / 3 + 1) * 4);

    let mut counter = 0;
    while counter + 3 <= characters.len() {
        let first_base64_character = extract_first_character_bits(characters[counter]);
        let second_base64_character =
            extract_second_character_bits(characters[counter], characters[counter + 1]);
        let third_base64_character =
            extract_third_character_bits(characters[counter + 1], characters[counter + 2]);
        let fourth_base64_character = characters[counter + 2] & 0b00111111;

        base64_output.append(&mut vec![
            BASE64_ALPHABET[first_base64_character as usize],
            BASE64_ALPHABET[second_base64_character as usize],
            BASE64_ALPHABET[third_base64_character as usize],
            BASE64_ALPHABET[fourth_base64_character as usize],
        ]);

        counter += 3;
    }

    if counter + 1 == characters.len() {
        let first_base64_character = extract_first_character_bits(characters[counter]);
        let second_base64_character = extract_second_character_bits(characters[counter], 0);

        base64_output.append(&mut vec![
            BASE64_ALPHABET[first_base64_character as usize],
            BASE64_ALPHABET[second_base64_character as usize],
            '=',
            '=',
        ]);
    } else if counter + 2 == characters.len() {
        let first_base64_character = extract_first_character_bits(characters[counter]);
        let second_base64_character =
            extract_second_character_bits(characters[counter], characters[counter + 1]);
        let third_base64_character = extract_third_character_bits(characters[counter + 1], 0);

        base64_output.append(&mut vec![
            BASE64_ALPHABET[first_base64_character as usize],
            BASE64_ALPHABET[second_base64_character as usize],
            BASE64_ALPHABET[third_base64_character as usize],
            '=',
        ]);
    }

    base64_output.into_iter().collect::<String>()
}

fn extract_first_character_bits(first_byte: u8) -> u8 {
    (first_byte & 0b1111100) >> 2
}

fn extract_second_character_bits(first_byte: u8, second_byte: u8) -> u8 {
    (first_byte & 0b00000011) << 4 | ((second_byte & 0b11110000) >> 4)
}

fn extract_third_character_bits(second_byte: u8, third_byte: u8) -> u8 {
    (second_byte & 0b00001111) << 2 | ((third_byte & 0b11000000) >> 6)
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
