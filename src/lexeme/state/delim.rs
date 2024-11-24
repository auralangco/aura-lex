use super::BasicState;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DelimState {
    /// The lexeme to accept a `(` operator.
    OParen(BasicState),
    /// The lexeme to accept a `)` operator.
    CParen(BasicState),
    /// The lexeme to accept a `[` operator.
    OBrack(BasicState),
    /// The lexeme to accept a `]` operator.
    CBrack(BasicState),
    /// The lexeme to accept a `{` operator.
    OBrace(BasicState),
    /// The lexeme to accept a `}` operator.
    CBrace(BasicState),
}

impl DelimState {
    pub fn acceptable(&self) -> bool {
        match self {
            Self::OParen(state)
            | Self::CParen(state)
            | Self::OBrack(state)
            | Self::CBrack(state)
            | Self::OBrace(state)
            | Self::CBrace(state) => state.acceptable(),
        }
    }

    pub fn accept(self, c: char) -> Option<Self> {
        match self {
            Self::OParen(BasicState::Unset) if c == '(' => Self::OParen(BasicState::Acceptable),
            Self::CParen(BasicState::Unset) if c == ')' => Self::CParen(BasicState::Acceptable),
            Self::OBrack(BasicState::Unset) if c == '[' => Self::OBrack(BasicState::Acceptable),
            Self::CBrack(BasicState::Unset) if c == ']' => Self::CBrack(BasicState::Acceptable),
            Self::OBrace(BasicState::Unset) if c == '{' => Self::OBrace(BasicState::Acceptable),
            Self::CBrace(BasicState::Unset) if c == '}' => Self::CBrace(BasicState::Acceptable),
            _ => return None,
        }.into()
    }

    pub fn stream() -> Vec<Self> {
        use DelimState::*;
        vec![
            OParen(BasicState::default()),
            CParen(BasicState::default()),
            OBrack(BasicState::default()),
            CBrack(BasicState::default()),
            OBrace(BasicState::default()),
            CBrace(BasicState::default()),
        ]
    }
}