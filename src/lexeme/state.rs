pub mod kw;
pub mod ident;
pub mod op;
pub mod delim;
pub mod lit;
pub mod pt;

/// A generic state for lexemes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum BasicState {
    #[default]
    /// The lexeme hasn't started yet
    Unset,
    /// The lexeme is acceptable
    Acceptable,
}

impl BasicState {
    pub fn acceptable(&self) -> bool {
        *self == Self::Acceptable
    }
}

/// The state of a lexeme with defines if the current lexeme accept the next character or if it is a valid lexeme.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LexemeState {
    Kw(kw::KwState),
    Ident(ident::IdentState),
    Op(op::OpState),
    Delim(delim::DelimState),
    Lit(lit::LitState),
    Pt(pt::PtState),
}

impl LexemeState {
    /// Check if the current lexeme accepts the next character.
    /// 
    /// If so, it returns the new state of the lexeme. Otherwise, it returns `None`.
    pub fn accept(self, c: char) -> Option<Self> {
        match self {
            Self::Kw(state) => state.accept(c).map(Self::Kw),
            Self::Ident(state) => state.accept(c).map(Self::Ident),
            Self::Op(state) => state.accept(c).map(Self::Op),
            Self::Delim(state) => state.accept(c).map(Self::Delim),
            Self::Lit(state) => state.accept(c).map(Self::Lit),
            Self::Pt(state) => state.accept(c).map(Self::Pt),
        }
    }

    /// Check if the current lexeme is acceptable. It means that the lexeme is a valid lexeme.
    pub fn acceptable(&self) -> bool {
        match self {
            Self::Kw(state) => state.acceptable(),
            Self::Ident(state) => state.acceptable(),
            Self::Op(state) => state.acceptable(),
            Self::Delim(state) => state.acceptable(),
            Self::Lit(state) => state.acceptable(),
            Self::Pt(state) => state.acceptable(),
        }
    }

    /// Generate a stream of lexeme default states for all possible lexemes.
    pub fn stream() -> Vec<Self> {
        use LexemeState::*;
        vec![
            kw::KwState::stream().into_iter().map(Kw).collect::<Vec<Self>>(),
            ident::IdentState::stream().into_iter().map(Ident).collect::<Vec<Self>>(),
            op::OpState::stream().into_iter().map(Op).collect::<Vec<Self>>(),
            delim::DelimState::stream().into_iter().map(Delim).collect::<Vec<Self>>(),
            lit::LitState::stream().into_iter().map(Lit).collect::<Vec<Self>>(),
            pt::PtState::stream().into_iter().map(Pt).collect::<Vec<Self>>(),
        ]
        .into_iter()
        .flatten()
        .collect()
    }
}

#[cfg(test)]
mod tests {
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
        assert_eq!(stream[0], LexemeState::Op(super::op::OpState::CRange(super::op::TripleCharOpState::Third)));
    }
}