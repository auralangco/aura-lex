use super::{op::SingleCharOpState, Accepter};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PtState {
    PtDot(SingleCharOpState<'.'>),
    PtComma(SingleCharOpState<','>),
    PtColon(SingleCharOpState<':'>),
    PtSemi(SingleCharOpState<';'>),
}

impl Accepter for PtState {
    type State = Self;

    fn acceptable(&self) -> bool {
        match self {
            Self::PtDot(state) => state.acceptable(),
            Self::PtComma(state) => state.acceptable(),
            Self::PtColon(state) => state.acceptable(),
            Self::PtSemi(state) => state.acceptable(),
        }
    }

    fn accept(self, c: char) -> Option<Self::State> {
        match self {
            Self::PtDot(state) => state.accept(c).map(Self::PtDot),
            Self::PtComma(state) => state.accept(c).map(Self::PtComma),
            Self::PtColon(state) => state.accept(c).map(Self::PtColon),
            Self::PtSemi(state) => state.accept(c).map(Self::PtSemi),
        }
    }
}

impl PtState {
    pub fn stream() -> Vec<Self> {
        use PtState::*;
        vec![
            PtDot(SingleCharOpState::default()),
            PtComma(SingleCharOpState::default()),
            PtColon(SingleCharOpState::default()),
            PtSemi(SingleCharOpState::default()),
        ]
    }
}