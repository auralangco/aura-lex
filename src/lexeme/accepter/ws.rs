use super::Accepter;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum WhitespaceAccepter {
    #[default]
    Unset,
    Acceptable,
}

impl Accepter for WhitespaceAccepter {
    type Accepter = Self;

    fn acceptable(&self) -> bool {
        *self == Self::Acceptable
    }

    fn accept(self, c: char) -> Option<Self> {
        if c.is_whitespace() {
            Some(Self::Acceptable)
        } else {
            None
        }
    }
}
