use std::char::ParseCharError;
use std::str::FromStr;
use std::fmt::{Display, Formatter, Result as FormatResult};

/// TODO: Tests inserting
///  - beginning of buffer
///  - end of buffer
///  - middle of buffer
///  - moving the gap forwards
///  - moving the gap backwards
/// TODO: increase gap size if it's to small
pub struct GapBuffer {
    pub content: Vec<char>,
    pub gap_size: usize,
    pub gap_position: usize,
}
impl GapBuffer {
    pub fn visualize_gap(&self) -> String {
        let mut content = String::with_capacity(self.content.len() - self.gap_size);

        for i in 0..self.content.len() {
            let character = self.content[i];
            content.push(character);
        }

        format!("{}--DONE--", content)
    }

    pub fn insert(&mut self, line: u64, character:u64, insert: &mut String) {
        let offset = self.get_offset(line, character);
        self.move_gap(offset);
        self.insert_into_gap(insert);
    }

    pub fn delete(&mut self, start_line: u64, start_character:u64, end_line: u64, end_character: u64) {
        let start_offset = self.get_offset(start_line, start_character);
        let end_offset = self.get_offset(end_line, end_character);

        self.delete_characters(start_offset, end_offset);
    }

    fn get_offset(&self, line: u64, character: u64) -> usize {
        let content_length = self.content.len();
        let mut character_count = 0;
        let mut index = 0;
        let mut line_count = 0;

        while index < content_length {
            if index >= self.gap_position && index < self.gap_position + self.gap_size {
                index = self.gap_position + self.gap_size;

                continue;
            }

            if line_count == line && character_count == character {
                return if index > self.gap_position { index - self.gap_size } else { index };
            }

            character_count += 1;

            if self.content[index] == '\n' {
                character_count = 0;
                line_count += 1;
            }
            if self.content[index] == '\r' {
                character_count = 0;
                line_count += 1;

                if index + 1 < content_length && self.content[index + 1] == '\n' {
                    index += 1;
                }
            }

            index += 1;
        }

        return if index > self.gap_position { index - self.gap_size } else { index };
    }

    fn move_gap(&mut self, offset: usize) {
        if self.gap_position > offset {
            let character_amount_to_copy = self.gap_position - offset;

            for i in 0..character_amount_to_copy {
                let character = self.content[self.gap_position - 1 - i];
                self.content[self.gap_position + self.gap_size - 1 - i] = character;
                self.content[self.gap_position - 1 - i] = 'ä';
            }

            self.gap_position = offset;
        } else {
            let character_amount_to_copy = offset - self.gap_position;

            for i in 0..character_amount_to_copy {
                let character = self.content[self.gap_position + self.gap_size + i];
                self.content[self.gap_position + i] = character;
                self.content[self.gap_position + self.gap_size + i] = 'ä';
            }

            self.gap_position = offset;
        }
    }

    fn insert_into_gap(&mut self, insert: &mut String) {
        let insert_length = insert.len();
        let mut last_char = insert.pop();

        while let Some(character) = last_char {
            self.content[self.gap_position + insert.len()] = character;
            last_char = insert.pop();
        }

        self.gap_position += insert_length;
        self.gap_size -= insert_length;
    }

    fn delete_characters(&mut self, start_offset: usize, end_offset: usize) {
        self.move_gap(end_offset + 1);

        // The number behind the two dots is excluded, therefore increment by one.
        for i in start_offset..end_offset + 1 {
            self.content[i] = 'ä';
        }

        self.gap_position = start_offset;
        self.gap_size += end_offset - start_offset + 1;
    }
}
impl FromStr for GapBuffer {
    type Err = ParseCharError;

    fn from_str(content_str: &str) -> Result<Self, Self::Err> {
        let mut content = content_str.to_string();
        content.push(' ');
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
impl Display for GapBuffer {
    fn fmt(&self, formatter: &mut Formatter) -> FormatResult {
        let mut index = 0;
        let mut content = String::with_capacity(self.content.len() - self.gap_size);

        for i in 0..self.content.len() {
            let character = self.content[i];
            if index < self.gap_position || index >= self.gap_position + self.gap_size {
                content.push(character);
            }

            index += 1;
        }

        write!(formatter, "{}", content)
    }
}