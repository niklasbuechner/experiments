use std::char::ParseCharError;
use std::str::FromStr;

pub struct GapBuffer {
    pub content: Vec<char>,
    pub gap_size: usize,
    pub gap_position: usize,
}
impl FromStr for GapBuffer {
    type Err = ParseCharError;

    fn from_str(content_str: &str) -> Result<Self, Self::Err> {
        let mut content = content_str.to_string();
        let content_length = content.len();
        let mut gap_inserted = false;
        let gap_position = content_length / 2;
        let mut last_char = content.pop();

        let mut content_buffer = Vec::with_capacity(content_length + 100);
        content_buffer.resize(content_length + 100,' ');

        while let Some(character) = last_char {
            let mut index = content.len();
            if !gap_inserted {
                if index == gap_position {
                    for i in 0..100 {
                        let gap_index = index + i;
                        content_buffer[gap_index] = 'ä';
                    }

                    gap_inserted = true;
                }
                index += 100;
            }

            content_buffer[index] = character;
            last_char = content.pop();
        }

        Ok(GapBuffer {
            content: content_buffer,
            gap_size: 100,
            gap_position,
        })
    }
}