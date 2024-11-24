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
    pub start_coord: (usize, usize),
    pub end_coord: (usize, usize),
}
