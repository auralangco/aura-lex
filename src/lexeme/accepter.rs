pub mod generics;
pub mod comment;
pub mod kw;
pub mod ident;
pub mod op;
pub mod delim;
pub mod lit;
pub mod pt;
pub mod ws;


/// A trait for functions that check if a lexeme accepts a character and if it is in a valid state.
pub trait Accepter {
    /// The accepter produced by the lexeme when it accepts a character.
    type Accepter;

    /// Check if the current lexeme is acceptable so the parser knows 
    /// that it is in a valid state.
    fn acceptable(&self) -> bool;

    /// Check if the lexeme accepts the next character.
    /// If it does, it returns the new state of the lexeme. 
    /// Otherwise, it returns `None`.
    fn accept(self, c: char) -> Option<Self::Accepter>;
}

/// The state of a lexeme with defines if the current lexeme accept the next character or if it is a valid lexeme.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LexemeState {
    Kw(kw::KwAccepter),
    Ident(ident::IdentAccepter),
    Op(op::OpAccepter),
    Delim(delim::DelimAccepter),
    Lit(lit::LitAccepter),
    Pt(pt::PtState),
    Ws(ws::WhitespaceAccepter),
    Comment(comment::CommentAccepter),
}

impl Accepter for LexemeState {
    type Accepter = Self;

    /// Check if the current lexeme accepts the next character.
    /// 
    /// If so, it returns the new state of the lexeme. Otherwise, it returns `None`.
    fn accept(self, c: char) -> Option<Self> {
        match self {
            Self::Kw(state) => state.accept(c).map(Self::Kw),
            Self::Ident(state) => state.accept(c).map(Self::Ident),
            Self::Op(state) => state.accept(c).map(Self::Op),
            Self::Delim(state) => state.accept(c).map(Self::Delim),
            Self::Lit(state) => state.accept(c).map(Self::Lit),
            Self::Pt(state) => state.accept(c).map(Self::Pt),
            Self::Ws(state) => state.accept(c).map(Self::Ws),
            Self::Comment(state) => state.accept(c).map(Self::Comment),
        }
    }

    /// Check if the current lexeme is acceptable. It means that the lexeme is a valid lexeme.
    fn acceptable(&self) -> bool {
        match self {
            Self::Kw(state) => state.acceptable(),
            Self::Ident(state) => state.acceptable(),
            Self::Op(state) => state.acceptable(),
            Self::Delim(state) => state.acceptable(),
            Self::Lit(state) => state.acceptable(),
            Self::Pt(state) => state.acceptable(),
            Self::Ws(state) => state.acceptable(),
            Self::Comment(state) => state.acceptable(),
        }
    }
}

impl LexemeState {
    /// Generate a stream of lexeme default states for all possible lexemes.
    pub fn stream() -> Vec<Self> {
        use LexemeState::*;
        vec![
            kw::KwAccepter::stream().into_iter().map(Kw).collect::<Vec<Self>>(),
            ident::IdentAccepter::stream().into_iter().map(Ident).collect::<Vec<Self>>(),
            op::OpAccepter::stream().into_iter().map(Op).collect::<Vec<Self>>(),
            delim::DelimAccepter::stream().into_iter().map(Delim).collect::<Vec<Self>>(),
            lit::LitAccepter::stream().into_iter().map(Lit).collect::<Vec<Self>>(),
            pt::PtState::stream().into_iter().map(Pt).collect::<Vec<Self>>(),
            vec![Ws(ws::WhitespaceAccepter::default())],
            comment::CommentAccepter::stream().into_iter().map(Comment).collect::<Vec<Self>>(),
        ]
        .into_iter()
        .flatten()
        .collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::lexeme::accepter::{generics, op, Accepter};

    #[test]
    fn stream_check() {
        use super::LexemeState;
        let stream = LexemeState::stream();
        dbg!(&stream);
    }

    #[test]
    fn simulate_lexeme_acceptance() {
        use super::LexemeState;
        let mut stream = LexemeState::stream();
        for c in "..=".chars() {
            stream = stream.into_iter().filter_map(|s| s.accept(c)).collect();
        }
        assert_eq!(stream.len(), 1);
        assert!(stream[0].acceptable());
        assert_eq!(stream[0], LexemeState::Op(op::OpAccepter::CRange(generics::TripleCharAccepter::Third)));
    }
}