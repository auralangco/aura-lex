use super::Accepter;

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
    type Accepter = Self;

    fn acceptable(&self) -> bool {
        *self == Self::Set
    }

    fn accept(self, c: char) -> Option<Self::Accepter> {
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
    type Accepter = Self;

    fn acceptable(&self) -> bool {
        *self == Self::Second
    }

    fn accept(self, c: char) -> Option<Self::Accepter> {
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
pub enum TripleCharAccepter<const CH1: char, const CH2: char, const CH3: char> {
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

impl<const OP1: char, const OP2: char, const OP3: char> Accepter
    for TripleCharAccepter<OP1, OP2, OP3>
{
    type Accepter = Self;

    fn acceptable(&self) -> bool {
        *self == Self::Third
    }

    fn accept(self, c: char) -> Option<Self::Accepter> {
        match self {
            Self::Unset if c == OP1 => Some(Self::First),
            Self::First if c == OP2 => Some(Self::Second),
            Self::Second if c == OP3 => Some(Self::Third),
            _ => None,
        }
    }
}
