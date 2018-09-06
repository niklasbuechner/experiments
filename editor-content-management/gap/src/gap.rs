use std::char::ParseCharError;
use std::str::FromStr;
use std::fmt::{Display, Formatter, Result as FormatResult};

pub struct GapBuffer {
    pub content: Vec<char>,
    pub gap_size: usize,
    pub gap_position: usize,
}
impl GapBuffer {
    pub fn visualize_gap(&self, visualizer: char) -> String {
        let mut index = 0;
        let mut content = String::with_capacity(self.content.len() - self.gap_size);

        for i in 0..self.content.len() {
            let character = self.content[i];
            if index >= self.gap_position && index < self.gap_position + self.gap_size {
                content.push(visualizer);
            } else {
                content.push(character);
            }

            index += 1;
        }

        format!("{}--DONE--", content)
    }
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