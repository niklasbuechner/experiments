use std::char::ParseCharError;
use std::fmt::{Display, Formatter, Result as DisplayResult};
use std::str::FromStr;

pub struct PieceTable {
    origin: Vec<char>,
}
impl FromStr for PieceTable {
    type Err = ParseCharError;

    fn from_str(code: &str) -> Result<Self, Self::Err> {
        return Ok(PieceTable {
            origin: code.chars().collect(),
        });
    }
}
impl Display for PieceTable {
    fn fmt(&self, formatter: &mut Formatter) -> DisplayResult {
        let origin: String = self.origin.iter().collect();
        write!(formatter, "{}", origin)
    }
}
