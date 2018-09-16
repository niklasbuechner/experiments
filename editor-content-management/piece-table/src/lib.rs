mod piece_table;

pub use piece_table::PieceTable;

pub trait ContentManager {
    fn insert(&mut self, line: u64, character: u64, insert: &mut String);
    fn delete(&mut self, start_line: u64, start_character:u64, end_line: u64, end_character: u64);
    // fn get_at(&self, line: u64, character: u64) -> char;
}
