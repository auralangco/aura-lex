use super::Accepter;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CommentState {
    Line(LineState),
    Block(BlockState),
}

impl Accepter for CommentState {
    type State = Self;

    fn acceptable(&self) -> bool {
        match self {
            Self::Line(state) => state.acceptable(),
            Self::Block(state) => state.acceptable(),
        }
    }

    fn accept(self, c: char) -> Option<Self> {
        match self {
            Self::Line(state) => state.accept(c).map(Self::Line),
            Self::Block(state) => state.accept(c).map(Self::Block),
        }
    }
}

impl CommentState {
    pub fn stream() -> Vec<Self> {
        use CommentState::*;
        vec![
            Line(LineState::default()),
            Block(BlockState::default()),
        ]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum LineState {
    #[default]
    Unset,
    FirstSlash,
    Acceptable,
}

impl Accepter for LineState {
    type State = Self;

    fn acceptable(&self) -> bool {
        *self == Self::Acceptable
    }

    fn accept(self, c: char) -> Option<Self> {
        match self {
            Self::Unset if c == '/' => Some(Self::FirstSlash),
            Self::FirstSlash if c == '/' => Some(Self::Acceptable),
            Self::Acceptable if c != '\n' => Some(Self::Acceptable),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum BlockState {
    #[default]
    Unset,
    FirstSlash,
    FirstStar,
    WaitingEndStar,
    WaitingEndSlash,
    Acceptable,
}

impl Accepter for BlockState {
    type State = Self;

    fn acceptable(&self) -> bool {
        *self == Self::Acceptable
    }

    fn accept(self, c: char) -> Option<Self> {
        match self {
            Self::Unset if c == '/' => Some(Self::FirstSlash),
            Self::FirstSlash if c == '*' => Some(Self::FirstStar),
            Self::FirstStar if c != '*' => Some(Self::FirstStar),
            Self::FirstStar if c == '*' => Some(Self::WaitingEndStar),
            Self::WaitingEndStar if c == '*' => Some(Self::WaitingEndStar),
            Self::WaitingEndStar if c == '/' => Some(Self::Acceptable),
            Self::WaitingEndStar if c != '*' => Some(Self::FirstStar),
            _ => None,
        }
    }
}