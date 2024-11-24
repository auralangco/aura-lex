use super::{op::SingleCharOpState, Accepter};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PtState {
    Dot(SingleCharOpState<'.'>),
    Comma(SingleCharOpState<','>),
    Colon(SingleCharOpState<':'>),
    Semi(SingleCharOpState<';'>),
}

impl Accepter for PtState {
    type State = Self;

    fn acceptable(&self) -> bool {
        match self {
            Self::Dot(state) => state.acceptable(),
            Self::Comma(state) => state.acceptable(),
            Self::Colon(state) => state.acceptable(),
            Self::Semi(state) => state.acceptable(),
        }
    }

    fn accept(self, c: char) -> Option<Self::State> {
        match self {
            Self::Dot(state) => state.accept(c).map(Self::Dot),
            Self::Comma(state) => state.accept(c).map(Self::Comma),
            Self::Colon(state) => state.accept(c).map(Self::Colon),
            Self::Semi(state) => state.accept(c).map(Self::Semi),
        }
    }
}

impl PtState {
    pub fn stream() -> Vec<Self> {
        use PtState::*;
        vec![
            Dot(SingleCharOpState::default()),
            Comma(SingleCharOpState::default()),
            Colon(SingleCharOpState::default()),
            Semi(SingleCharOpState::default()),
        ]
    }
}