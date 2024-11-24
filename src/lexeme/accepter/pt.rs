use super::{op::SingleCharAccepter, Accepter};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PtState {
    Dot(SingleCharAccepter<'.'>),
    Comma(SingleCharAccepter<','>),
    Colon(SingleCharAccepter<':'>),
    Semi(SingleCharAccepter<';'>),
}

impl Accepter for PtState {
    type State = Self;

    fn acceptable(&self) -> bool {
        match self {
            Self::Dot(acp) => acp.acceptable(),
            Self::Comma(acp) => acp.acceptable(),
            Self::Colon(acp) => acp.acceptable(),
            Self::Semi(acp) => acp.acceptable(),
        }
    }

    fn accept(self, c: char) -> Option<Self::State> {
        match self {
            Self::Dot(acp) => acp.accept(c).map(Self::Dot),
            Self::Comma(acp) => acp.accept(c).map(Self::Comma),
            Self::Colon(acp) => acp.accept(c).map(Self::Colon),
            Self::Semi(acp) => acp.accept(c).map(Self::Semi),
        }
    }
}

impl PtState {
    pub fn stream() -> Vec<Self> {
        use PtState::*;
        vec![
            Dot(SingleCharAccepter::default()),
            Comma(SingleCharAccepter::default()),
            Colon(SingleCharAccepter::default()),
            Semi(SingleCharAccepter::default()),
        ]
    }
}