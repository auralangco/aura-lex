const GT: char = '>';

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OpState {
    /// The lexeme to accept a `:=` operator.
    Decl(DoubleCharOpState<':', '='>),
    /// The lexeme to accept a `=` operator.
    Eq(SingleCharOpState<'='>),
    /// The lexeme to accept a `+` operator.
    Plus(SingleCharOpState<'+'>),
    /// The lexeme to accept a `-` operator.
    Minus(SingleCharOpState<'-'>),
    /// The lexeme to accept a `*` operator.
    Star(SingleCharOpState<'*'>),
    /// The lexeme to accept a `/` operator.
    Slash(SingleCharOpState<'/'>),
    /// The lexeme to accept a `^` operator.
    Caret(SingleCharOpState<'^'>),
    /// The lexeme to accept a `_` operator.
    UScore(SingleCharOpState<'_'>),
    /// The lexeme to accept a `%` operator.
    Percent(SingleCharOpState<'%'>),
    /// The lexeme to accept a `&` operator.
    And(SingleCharOpState<'&'>),
    /// The lexeme to accept a `&&` operator.
    AndAnd(DoubleCharOpState<'&', '&'>),
    /// The lexeme to accept a `|` operator.
    Or(SingleCharOpState<'|'>),
    /// The lexeme to accept a `||` operator.
    OrOr(DoubleCharOpState<'|', '|'>),
    /// The lexeme to accept a `!` operator.
    Not(SingleCharOpState<'!'>),
    /// The lexeme to accept a `!=` operator.
    NotEq(DoubleCharOpState<'!', '='>),
    /// The lexeme to accept a `==` operator.
    EqEq(DoubleCharOpState<'=', '='>),
    /// The lexeme to accept a `>` operator.
    Gt(SingleCharOpState<GT>),
    /// The lexeme to accept a `>=` operator.
    GtEq(DoubleCharOpState<' ', '='>),
    /// The lexeme to accept a `<` operator.
    Lt(SingleCharOpState<'<'>),
    /// The lexeme to accept a `<=` operator.
    LtEq(DoubleCharOpState<'<', '='>),
    /// The lexeme to accept a `<<` operator.
    LtLt(DoubleCharOpState<'<', '<'>),
    /// The lexeme to accept a `>>` operator.
    GtGt(DoubleCharOpState<' ', GT>),
    /// The lexeme to accept a `->` operator.
    RArw(DoubleCharOpState<'-', GT>),
    /// The lexeme to accept a `=>` operator.
    FatRArw(DoubleCharOpState<'=', GT>),
    /// The lexeme to accept a `~` operator.
    Tilde(SingleCharOpState<'~'>),
    /// The lexeme to accept a `::` operator.
    Join(DoubleCharOpState<':', ':'>),
    /// The lexeme to accept a `\` operator.
    BSlash(SingleCharOpState<'\\'>),
    /// The lexeme to accept a `..` operator.
    Range(DoubleCharOpState<'.', '.'>),
    /// The lexeme to accept a `..=` operator.
    CRange(TripleCharOpState<'.', '.', '='>),
    /// The lexeme to accept a `...` operator.
    Spread(TripleCharOpState<'.', '.', '.'>),
}

impl OpState {
    pub fn acceptable(&self) -> bool {
        match self {
            Self::Decl(state) => state.acceptable(),
            Self::Eq(state) => state.acceptable(),
            Self::Plus(state) => state.acceptable(),
            Self::Minus(state) => state.acceptable(),
            Self::Star(state) => state.acceptable(),
            Self::Slash(state) => state.acceptable(),
            Self::Caret(state) => state.acceptable(),
            Self::UScore(state) => state.acceptable(),
            Self::Percent(state) => state.acceptable(),
            Self::And(state) => state.acceptable(),
            Self::AndAnd(state) => state.acceptable(),
            Self::Or(state) => state.acceptable(),
            Self::OrOr(state) => state.acceptable(),
            Self::Not(state) => state.acceptable(),
            Self::NotEq(state) => state.acceptable(),
            Self::EqEq(state) => state.acceptable(),
            Self::Gt(state) => state.acceptable(),
            Self::GtEq(state) => state.acceptable(),
            Self::Lt(state) => state.acceptable(),
            Self::LtEq(state) => state.acceptable(),
            Self::LtLt(state) => state.acceptable(),
            Self::GtGt(state) => state.acceptable(),
            Self::RArw(state) => state.acceptable(),
            Self::FatRArw(state) => state.acceptable(),
            Self::Tilde(state) => state.acceptable(),
            Self::Join(state) => state.acceptable(),
            Self::BSlash(state) => state.acceptable(),
            Self::Range(state) => state.acceptable(),
            Self::CRange(state) => state.acceptable(),
            Self::Spread(state) => state.acceptable(),
        }
    }

    pub fn accept(self, c: char) -> Option<Self> {
        match self {
            Self::Decl(state) => state.accept(c).map(Self::Decl),
            Self::Eq(state) => state.accept(c).map(Self::Eq),
            Self::Plus(state) => state.accept(c).map(Self::Plus),
            Self::Minus(state) => state.accept(c).map(Self::Minus),
            Self::Star(state) => state.accept(c).map(Self::Star),
            Self::Slash(state) => state.accept(c).map(Self::Slash),
            Self::Caret(state) => state.accept(c).map(Self::Caret),
            Self::UScore(state) => state.accept(c).map(Self::UScore),
            Self::Percent(state) => state.accept(c).map(Self::Percent),
            Self::And(state) => state.accept(c).map(Self::And),
            Self::AndAnd(state) => state.accept(c).map(Self::AndAnd),
            Self::Or(state) => state.accept(c).map(Self::Or),
            Self::OrOr(state) => state.accept(c).map(Self::OrOr),
            Self::Not(state) => state.accept(c).map(Self::Not),
            Self::NotEq(state) => state.accept(c).map(Self::NotEq),
            Self::EqEq(state) => state.accept(c).map(Self::EqEq),
            Self::Gt(state) => state.accept(c).map(Self::Gt),
            Self::GtEq(state) => state.accept(c).map(Self::GtEq),
            Self::Lt(state) => state.accept(c).map(Self::Lt),
            Self::LtEq(state) => state.accept(c).map(Self::LtEq),
            Self::LtLt(state) => state.accept(c).map(Self::LtLt),
            Self::GtGt(state) => state.accept(c).map(Self::GtGt),
            Self::RArw(state) => state.accept(c).map(Self::RArw),
            Self::FatRArw(state) => state.accept(c).map(Self::FatRArw),
            Self::Tilde(state) => state.accept(c).map(Self::Tilde),
            Self::Join(state) => state.accept(c).map(Self::Join),
            Self::BSlash(state) => state.accept(c).map(Self::BSlash),
            Self::Range(state) => state.accept(c).map(Self::Range),
            Self::CRange(state) => state.accept(c).map(Self::CRange),
            Self::Spread(state) => state.accept(c).map(Self::Spread),
        }
    }

    pub fn stream() -> Vec<Self> {
        use OpState::*;
        vec![
            Decl(DoubleCharOpState::default()),
            Eq(SingleCharOpState::default()),
            Plus(SingleCharOpState::default()),
            Minus(SingleCharOpState::default()),
            Star(SingleCharOpState::default()),
            Slash(SingleCharOpState::default()),
            Caret(SingleCharOpState::default()),
            UScore(SingleCharOpState::default()),
            Percent(SingleCharOpState::default()),
            And(SingleCharOpState::default()),
            AndAnd(DoubleCharOpState::default()),
            Or(SingleCharOpState::default()),
            OrOr(DoubleCharOpState::default()),
            Not(SingleCharOpState::default()),
            NotEq(DoubleCharOpState::default()),
            EqEq(DoubleCharOpState::default()),
            Gt(SingleCharOpState::default()),
            GtEq(DoubleCharOpState::default()),
            Lt(SingleCharOpState::default()),
            LtEq(DoubleCharOpState::default()),
            LtLt(DoubleCharOpState::default()),
            GtGt(DoubleCharOpState::default()),
            RArw(DoubleCharOpState::default()),
            FatRArw(DoubleCharOpState::default()),
            Tilde(SingleCharOpState::default()),
            Join(DoubleCharOpState::default()),
            BSlash(SingleCharOpState::default()),
            Range(DoubleCharOpState::default()),
            CRange(TripleCharOpState::default()),
            Spread(TripleCharOpState::default()),
        ]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SingleCharOpState<const OP: char> {
    #[default]
    /// The state to accept a single character operator.
    Unset,
    /// The state were the char is read.
    Set,
}

impl<const OP: char> SingleCharOpState<OP> {
    pub fn acceptable(&self) -> bool {
        *self == Self::Set
    }

    pub fn accept(self, c: char) -> Option<Self> {
        if self == Self::Unset && c == OP {
            Some(Self::Set)
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DoubleCharOpState<const OP1: char, const OP2: char> {
    #[default]
    /// The state to accept a double character operator.
    Unset,
    /// The state were the first char is read.
    First,
    /// The state were the second char is read.
    Second,
}

impl<const OP1: char, const OP2: char> DoubleCharOpState<OP1, OP2> {
    pub fn acceptable(&self) -> bool {
        *self == Self::Second
    }

    pub fn accept(self, c: char) -> Option<Self> {
        match self {
            Self::Unset if c == OP1 => Some(Self::First),
            Self::First if c == OP2 => Some(Self::Second),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TripleCharOpState<const OP1: char, const OP2: char, const OP3: char> {
    #[default]
    /// The state to accept a triple character operator.
    Unset,
    /// The state were the first char is read.
    First,
    /// The state were the second char is read.
    Second,
    /// The state were the third char is read.
    Third,
}

impl<const OP1: char, const OP2: char, const OP3: char> TripleCharOpState<OP1, OP2, OP3> {
    pub fn acceptable(&self) -> bool {
        *self == Self::Third
    }

    pub fn accept(self, c: char) -> Option<Self> {
        match self {
            Self::Unset if c == OP1 => Some(Self::First),
            Self::First if c == OP2 => Some(Self::Second),
            Self::Second if c == OP3 => Some(Self::Third),
            _ => None,
        }
    }
}