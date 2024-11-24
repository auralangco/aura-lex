use super::Accepter;

const GT: char = '>';

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OpAccepter {
    /// The lexeme to accept a `:=` operator.
    Decl(DoubleCharAccepter<':', '='>),
    /// The lexeme to accept a `=` operator.
    Eq(SingleCharAccepter<'='>),
    /// The lexeme to accept a `+` operator.
    Plus(SingleCharAccepter<'+'>),
    /// The lexeme to accept a `-` operator.
    Minus(SingleCharAccepter<'-'>),
    /// The lexeme to accept a `*` operator.
    Star(SingleCharAccepter<'*'>),
    /// The lexeme to accept a `/` operator.
    Slash(SingleCharAccepter<'/'>),
    /// The lexeme to accept a `^` operator.
    Caret(SingleCharAccepter<'^'>),
    /// The lexeme to accept a `_` operator.
    UScore(SingleCharAccepter<'_'>),
    /// The lexeme to accept a `%` operator.
    Percent(SingleCharAccepter<'%'>),
    /// The lexeme to accept a `&` operator.
    And(SingleCharAccepter<'&'>),
    /// The lexeme to accept a `&&` operator.
    AndAnd(DoubleCharAccepter<'&', '&'>),
    /// The lexeme to accept a `|` operator.
    Or(SingleCharAccepter<'|'>),
    /// The lexeme to accept a `||` operator.
    OrOr(DoubleCharAccepter<'|', '|'>),
    /// The lexeme to accept a `!` operator.
    Not(SingleCharAccepter<'!'>),
    /// The lexeme to accept a `!=` operator.
    NotEq(DoubleCharAccepter<'!', '='>),
    /// The lexeme to accept a `==` operator.
    EqEq(DoubleCharAccepter<'=', '='>),
    /// The lexeme to accept a `>` operator.
    Gt(SingleCharAccepter<GT>),
    /// The lexeme to accept a `>=` operator.
    GtEq(DoubleCharAccepter<GT, '='>),
    /// The lexeme to accept a `<` operator.
    Lt(SingleCharAccepter<'<'>),
    /// The lexeme to accept a `<=` operator.
    LtEq(DoubleCharAccepter<'<', '='>),
    /// The lexeme to accept a `<<` operator.
    LtLt(DoubleCharAccepter<'<', '<'>),
    /// The lexeme to accept a `>>` operator.
    GtGt(DoubleCharAccepter<GT, GT>),
    /// The lexeme to accept a `->` operator.
    RArw(DoubleCharAccepter<'-', GT>),
    /// The lexeme to accept a `=>` operator.
    FatRArw(DoubleCharAccepter<'=', GT>),
    /// The lexeme to accept a `~` operator.
    Tilde(SingleCharAccepter<'~'>),
    /// The lexeme to accept a `::` operator.
    Join(DoubleCharAccepter<':', ':'>),
    /// The lexeme to accept a `\` operator.
    BSlash(SingleCharAccepter<'\\'>),
    /// The lexeme to accept a `..` operator.
    Range(DoubleCharAccepter<'.', '.'>),
    /// The lexeme to accept a `..=` operator.
    CRange(TripleCharOpState<'.', '.', '='>),
    /// The lexeme to accept a `...` operator.
    Spread(TripleCharOpState<'.', '.', '.'>),
}

impl Accepter for OpAccepter {
    type State = Self;

    fn acceptable(&self) -> bool {
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

    fn accept(self, c: char) -> Option<Self::State> {
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
}

impl OpAccepter {
    pub fn stream() -> Vec<Self> {
        use OpAccepter::*;
        vec![
            Decl(DoubleCharAccepter::default()),
            Eq(SingleCharAccepter::default()),
            Plus(SingleCharAccepter::default()),
            Minus(SingleCharAccepter::default()),
            Star(SingleCharAccepter::default()),
            Slash(SingleCharAccepter::default()),
            Caret(SingleCharAccepter::default()),
            UScore(SingleCharAccepter::default()),
            Percent(SingleCharAccepter::default()),
            And(SingleCharAccepter::default()),
            AndAnd(DoubleCharAccepter::default()),
            Or(SingleCharAccepter::default()),
            OrOr(DoubleCharAccepter::default()),
            Not(SingleCharAccepter::default()),
            NotEq(DoubleCharAccepter::default()),
            EqEq(DoubleCharAccepter::default()),
            Gt(SingleCharAccepter::default()),
            GtEq(DoubleCharAccepter::default()),
            Lt(SingleCharAccepter::default()),
            LtEq(DoubleCharAccepter::default()),
            LtLt(DoubleCharAccepter::default()),
            GtGt(DoubleCharAccepter::default()),
            RArw(DoubleCharAccepter::default()),
            FatRArw(DoubleCharAccepter::default()),
            Tilde(SingleCharAccepter::default()),
            Join(DoubleCharAccepter::default()),
            BSlash(SingleCharAccepter::default()),
            Range(DoubleCharAccepter::default()),
            CRange(TripleCharOpState::default()),
            Spread(TripleCharOpState::default()),
        ]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
/// A generic state accepter for a single character operator.
/// 
/// This state is used by lexemes made of a single character namely `OP`.
pub enum SingleCharAccepter<const CH: char> {
    #[default]
    /// The state to accept a single character operator.
    Unset,
    /// The state were the char is read.
    Set,
}

impl<const CH: char> Accepter for SingleCharAccepter<CH> {
    type State = Self;
    
    fn acceptable(&self) -> bool {
        *self == Self::Set
    }

    fn accept(self, c: char) -> Option<Self::State> {
        if self == Self::Unset && c == CH {
            Some(Self::Set)
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
/// A generic state accepter for a double character operator.
/// 
/// This state is used by lexemes made of a double character namely `OP1 OP2`.
pub enum DoubleCharAccepter<const CH1: char, const CH2: char> {
    #[default]
    /// The state to accept a double character operator.
    Unset,
    /// The state were the first char is read.
    First,
    /// The state were the second char is read.
    Second,
}

impl<const CH1: char, const CH2: char> Accepter for DoubleCharAccepter<CH1, CH2> {
    type State = Self;
    
    fn acceptable(&self) -> bool {
        *self == Self::Second
    }

    fn accept(self, c: char) -> Option<Self::State> {
        match self {
            Self::Unset if c == CH1 => Some(Self::First),
            Self::First if c == CH2 => Some(Self::Second),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
/// A generic state accepter for a triple character operator.
/// 
/// This state is used by lexemes made of a triple character namely `OP1 OP2 OP3`.
pub enum TripleCharOpState<const CH1: char, const CH2: char, const CH3: char> {
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

impl<const OP1: char, const OP2: char, const OP3: char> Accepter for TripleCharOpState<OP1, OP2, OP3> {
    type State = Self;
    
    fn acceptable(&self) -> bool {
        *self == Self::Third
    }

    fn accept(self, c: char) -> Option<Self::State> {
        match self {
            Self::Unset if c == OP1 => Some(Self::First),
            Self::First if c == OP2 => Some(Self::Second),
            Self::Second if c == OP3 => Some(Self::Third),
            _ => None,
        }
    }
    
}