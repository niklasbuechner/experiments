use std::char::ParseCharError;
use std::fmt::{Display, Formatter, Result as DisplayResult};
use std::str::FromStr;
use ContentManager;

#[derive(Debug)]
pub struct PieceTable {
    origin: Vec<char>,
    additions: Vec<char>,
    pieces: Vec<Option<Piece>>,
}
impl PieceTable {
    fn add_piece(&mut self, mut piece: Piece) -> usize {
        for index in 0..self.pieces.len() {
            match self.pieces[index] {
                None => {
                    piece.id = index;
                    self.pieces[index] = Some(piece);
                    return index;
                }
                _ => {}
            }
        }

        self.pieces.reserve_exact(100);

        self.add_piece(piece)
    }
}
impl ContentManager for PieceTable {
    // TODO Refactor, Remove empty pieces
    fn insert(&mut self, line: u64, character: u64, insert: &mut String) {
        let free_capacity = self.additions.capacity() - self.additions.len();
        if free_capacity < insert.len() {
            self.additions.reserve(100);
        }

        let (parent_piece_id, parent_offset) = match &self.pieces[0] {
            Some(piece) => piece.get_insertion_location(self, line, character, 0, 0),
            None => panic!("Piece Table initialized incorrectly. It is empty!"),
        };

        println!("Parent: {}, offset: {}", parent_piece_id, parent_offset);

        let insertion_offset = self.additions.len();
        let insertion_piece_index = self.add_piece(Piece {
            id: 0,
            source: Source::Additions,
            offset: insertion_offset,
            length: insert.len(),
            child_id: None,
        });

        let mut split_piece = Piece {
            id: 0,
            source: Source::Additions,
            offset: 0,
            length: 0,
            child_id: None,
        };
        let parent_piece_text_length;
        let parent_piece_child_id;
        match &mut self.pieces[parent_piece_id] {
            Some(piece) => {
                split_piece.source = piece.source;
                split_piece.offset = piece.offset + parent_offset;
                split_piece.length = piece.length - parent_offset;
                split_piece.child_id = piece.child_id;
                parent_piece_text_length = piece.length;
                parent_piece_child_id = piece.child_id;
                piece.length = piece.length - (piece.length - parent_offset);
            }
            None => panic!("A non existent piece in the piece table was requested"),
        };

        let second_parent_piece_index = self.add_piece(split_piece);

        match &mut self.pieces[insertion_piece_index] {
            Some(piece) => {
                piece.child_id = if parent_piece_text_length == parent_offset {
                    parent_piece_child_id
                } else {
                    Some(second_parent_piece_index)
                }
            }
            _ => panic!("This should not happen"),
        };

        match &mut self.pieces[parent_piece_id] {
            Some(piece) => piece.child_id = Some(insertion_piece_index),
            None => panic!("The parent piece we uses earlier seems to have disappered right not"),
        }

        self.additions.append(&mut insert.chars().collect());
        println!("{:?}", self);
    }

    fn delete(&mut self, start_line: u64, start_character:u64, end_line: u64, end_character: u64) {
        let (mut delete_start_piece_id, mut delete_start_offset) = match &self.pieces[0] {
            Some(piece) => piece.get_insertion_location(self, start_line, start_character, 0, 0),
            None => panic!("Piece Table initialized incorrectly. It is empty!"),
        };
        let (delete_end_piece_id, delete_end_offset) = match &self.pieces[0] {
            Some(piece) => piece.get_insertion_location(self, end_line, end_character, 0, 0),
            None => panic!("Piece Table initialized incorrectly. It is empty!"),
        };

        println!("start_piece_id: {}", delete_start_piece_id);
        println!("start_piece_offset: {}", delete_start_offset);
        println!("end_piece_id: {}", delete_end_piece_id);
        println!("end_piece_offset: {}", delete_end_offset);

        while delete_start_piece_id != delete_end_piece_id {
            match &mut self.pieces[delete_start_piece_id] {
                Some(piece) => {
                    piece.length = delete_start_offset;

                    delete_start_piece_id = piece.child_id.unwrap();
                    delete_start_offset = 0;
                },
                None => panic!("Request none existing piece."),
            }
        }

        let new_piece;
        match &mut self.pieces[delete_start_piece_id] {
            Some(piece) => {
                let length = piece.length;
                piece.length = delete_start_offset;

                let new_piece_length = if length as i64 - delete_end_offset as i64 - 1 < 0 {
                    0
                } else {
                    length - delete_end_offset - 1
                };

                new_piece = Piece {
                    id: 0,
                    source: piece.source,
                    offset: piece.offset + delete_end_offset + 1,
                    length: new_piece_length,
                    child_id: piece.child_id,
                };
            },
            None => panic!("Well, the piece still does not exist"),
        }

        let new_piece_id = self.add_piece(new_piece);
        match &mut self.pieces[delete_start_piece_id] {
            Some(piece) => piece.child_id = Some(new_piece_id),
            None => panic!("The piece was just there, how did it vanish?"),
        }
    }
}
impl FromStr for PieceTable {
    type Err = ParseCharError;

    fn from_str(code: &str) -> Result<Self, Self::Err> {
        let mut table = PieceTable {
            origin: code.chars().collect(),
            additions: Vec::with_capacity(100),
            pieces: Vec::with_capacity(100),
        };

        table.pieces.resize(100, Option::None);
        table.pieces[0] = Some(Piece::new_initial_piece(table.origin.len()));

        Ok(table)
    }
}
impl Display for PieceTable {
    fn fmt(&self, formatter: &mut Formatter) -> DisplayResult {
        let mut txt = String::new();
        let mut possible_piece = &self.pieces[0];

        while let Some(piece) = possible_piece {
            match piece.source {
                Source::Origin => {
                    for index in piece.offset..piece.offset + piece.length {
                        txt.push(self.origin[index]);
                    }
                }
                Source::Additions => {
                    for index in piece.offset..piece.offset + piece.length {
                        txt.push(self.additions[index]);
                    }
                }
            }

            match piece.child_id {
                Some(id) => possible_piece = &self.pieces[id],
                None => possible_piece = &None,
            }
        }

        write!(formatter, "{}", txt)
    }
}

#[derive(Clone, Debug)]
struct Piece {
    id: usize,
    source: Source,
    offset: usize,
    length: usize,
    child_id: Option<usize>,
}
impl Piece {
    fn new_initial_piece(length: usize) -> Self {
        Piece {
            id: 0,
            source: Source::Origin,
            offset: 0,
            length,
            child_id: None,
        }
    }

    fn get_insertion_location(
        &self,
        table: &PieceTable,
        line: u64,
        character: u64,
        last_line_count: u64,
        last_character_count: u64,
    ) -> (usize, usize) {
        let source = match self.source {
            Source::Origin => &table.origin,
            Source::Additions => &table.additions,
        };

        let mut index_offset = 0;
        let mut line_count = last_line_count;
        let mut character_count = last_character_count;
        for i in 0..self.length {
            let index = index_offset + i;
            if line_count >= line && character_count == character {
                return (self.id, index);
            } else {
                println!("Line_count {}", line_count);
                println!("character_count {}", character_count);
            }

            character_count += 1;
            let current_character = source[self.offset + index];

            if current_character == '\n' {
                line_count += 1;
                character_count = 0;
            } else if current_character == '\r' {
                line_count += 1;
                character_count = 0;

                if source.len() > index + 1 && source[index + 1] == '\n' {
                    index_offset += 1;
                }
            }
        }

        // Insert after last character
        if line_count >= line && character_count == character {
            return (self.id, self.length);
        } else {
            println!("Line_count {}", line_count);
            println!("character_count {}", character_count);
        }

        let child_id = match self.child_id {
            Some(child_id) => child_id,
            None => return (self.id, self.length - 1),
        };

        match &table.pieces[child_id] {
            Some(child) => {
                child.get_insertion_location(table, line, character, line_count, character_count)
            }
            None => return (self.id, self.length - 1),
        }
    }
}

#[derive(Copy, Clone, Debug)]
enum Source {
    Origin,
    Additions,
}
