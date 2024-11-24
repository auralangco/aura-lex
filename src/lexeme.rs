use std::fmt::Display;

use state::LexemeState;

pub mod state;

/// A lexeme in the source code.
/// 
/// A lexeme is a sequence of characters in the source code that is matched by a regular expression.
/// They are categorized by their type, which is represented by a [`LexemeState`]. The lexeme also
/// contains the slice of the source code that it represents, the start and end indices of the slice,
/// and the start and end coordinates of the slice. Those can be used as debugging information for 
/// the parser
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Lexeme<'src> {
    pub ty: LexemeState,
    pub slice: &'src str,
    pub start: usize,
    pub end: usize,
    pub start_coord: Coord,
    pub end_coord: Coord,
}

/// The coordinates of a character in the source code.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash, PartialOrd, Ord)]
pub struct Coord {
    pub line: usize,
    pub col: usize,
}

impl<T: Into<usize>, U: Into<usize>> From<(T, U)> for Coord {
    fn from((line, col): (T, U)) -> Self {
        Self {
            line: line.into(),
            col: col.into(),
        }
    }
}

impl<T: From<usize>, U: From<usize>> Into<(T, U)> for Coord {
    fn into(self) -> (T, U) {
        (self.line.into(), self.col.into())
    }
}

impl Display for Coord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(l: {}, c: {})", self.line, self.col)
    }
}