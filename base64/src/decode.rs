pub fn base64_decode(base64: &str) -> String {
    if base64.len() % 4 != 0 {
        panic!("A base64 string contains a multiple of 4 characters");
    }

    let base64_bits: Vec<u8> = base64
        .chars()
        .map(|character| {
            // The bit value is the character value minus ascii value plus offset in base64 alphabet.
            if character.is_ascii_uppercase() {
                (character as u32) - 65
            } else if character.is_ascii_lowercase() {
                (character as u32) - 97 + 26
            } else if character.is_numeric() {
                (character as u32) - 48 + 52
            } else if character == '=' {
                return 255;
            } else {
                panic!("All base64 characters need to be in [A-Za-z0-9]");
            }
        })
        .map(|character| character as u8)
        .collect::<Vec<u8>>();
    let chunks: Vec<&[u8]> = base64_bits.chunks(4).collect();
    let mut output = String::new();

    // Well it is not very efficient, but good enough for this test implementation.
    for chunk in &chunks {
        let mut character_bits: u32 = 0;
        character_bits |= (chunk[0] as u32) << 18;
        character_bits |= (chunk[1] as u32) << 12;

        let character_bytes;
        if chunk[2] == 255 {
            character_bytes = character_bits.to_be_bytes()[1..2].to_vec();
        } else if chunk[3] == 255 {
            character_bits |= (chunk[2] as u32) << 6;

            character_bytes = character_bits.to_be_bytes()[1..3].to_vec();
        } else {
            character_bits |= (chunk[2] as u32) << 6;
            character_bits |= chunk[3] as u32;

            character_bytes = character_bits.to_be_bytes()[1..4].to_vec();
        }

        let characters = std::str::from_utf8(&character_bytes);
        match characters {
            Ok(characters) => output.push_str(characters),
            Err(_) => panic!("The base64 encoded value does not contain a valid utf8 string."),
        }
    }

    output
}

#[test]
fn decode_simple_string() {
    let expected_content = "Hello World, how are you doing?\n";
    let base64 = "SGVsbG8gV29ybGQsIGhvdyBhcmUgeW91IGRvaW5nPwo=";

    assert_eq!(expected_content, base64_decode(base64));
}

#[test]
fn decode_three_characters() {
    let expected_content = "abc";
    let base64 = "YWJj";

    assert_eq!(expected_content, base64_decode(base64));
}

#[test]
fn decode_four_characters() {
    let expected_content = "abcd";
    let base64 = "YWJjZA==";

    assert_eq!(expected_content, base64_decode(base64));
}

#[test]
fn decode_five_characters() {
    let expected_content = "abcde";
    let base64 = "YWJjZGU=";

    assert_eq!(expected_content, base64_decode(base64));
}

#[test]
fn decode_six_characters() {
    let expected_content = "abcabc";
    let base64 = "YWJjYWJj";

    assert_eq!(expected_content, base64_decode(base64));
}
