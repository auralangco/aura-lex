use super::{generics::SingleCharAccepter, Accepter};

const OPAREN: char = '(';
const CPAREN: char = ')';
const OBRACK: char = '[';
const CBRACK: char = ']';
const OBRACE: char = '{';
const CBRACE: char = '}';

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DelimAccepter {
    /// The lexeme to accept a `(` operator.
    OParen(SingleCharAccepter<OPAREN>),
    /// The lexeme to accept a `)` operator.
    CParen(SingleCharAccepter<CPAREN>),
    /// The lexeme to accept a `[` operator.
    OBrack(SingleCharAccepter<OBRACK>),
    /// The lexeme to accept a `]` operator.
    CBrack(SingleCharAccepter<CBRACK>),
    /// The lexeme to accept a `{` operator.
    OBrace(SingleCharAccepter<OBRACE>),
    /// The lexeme to accept a `}` operator.
    CBrace(SingleCharAccepter<CBRACE>),
}

impl Accepter for DelimAccepter {
    type Accepter = Self;

    fn acceptable(&self) -> bool {
        match self {
            Self::OParen(acp) => acp.acceptable(),
            Self::CParen(acp) => acp.acceptable(),
            Self::OBrack(acp) => acp.acceptable(),
            Self::CBrack(acp) => acp.acceptable(),
            Self::OBrace(acp) => acp.acceptable(),
            Self::CBrace(acp) => acp.acceptable(),
        }
    }

    fn accept(self, c: char) -> Option<Self> {
        match self {
            Self::OParen(acp) => acp.accept(c).map(Self::OParen),
            Self::CParen(acp) => acp.accept(c).map(Self::CParen),
            Self::OBrack(acp) => acp.accept(c).map(Self::OBrack),
            Self::CBrack(acp) => acp.accept(c).map(Self::CBrack),
            Self::OBrace(acp) => acp.accept(c).map(Self::OBrace),
            Self::CBrace(acp) => acp.accept(c).map(Self::CBrace),
        }.into()
    }
}

impl DelimAccepter {
    pub fn stream() -> Vec<Self> {
        use DelimAccepter::*;
        vec![
            OParen(SingleCharAccepter::default()),
            CParen(SingleCharAccepter::default()),
            OBrack(SingleCharAccepter::default()),
            CBrack(SingleCharAccepter::default()),
            OBrace(SingleCharAccepter::default()),
            CBrace(SingleCharAccepter::default()),
        ]
    }
}