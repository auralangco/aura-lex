use super::{generics::{DoubleCharAccepter, SingleCharAccepter, TripleCharAccepter}, Accepter};

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
    CRange(TripleCharAccepter<'.', '.', '='>),
    /// The lexeme to accept a `...` operator.
    Spread(TripleCharAccepter<'.', '.', '.'>),
    /// The lexeme to accept a `$$` operator.
    DollarDollar(DoubleCharAccepter<'$', '$'>),
}

impl Accepter for OpAccepter {
    type Accepter = Self;

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
            Self::DollarDollar(state) => state.acceptable(),
        }
    }

    fn accept(self, c: char) -> Option<Self::Accepter> {
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
            Self::DollarDollar(state) => state.accept(c).map(Self::DollarDollar),
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
            CRange(TripleCharAccepter::default()),
            Spread(TripleCharAccepter::default()),
        ]
    }
}
