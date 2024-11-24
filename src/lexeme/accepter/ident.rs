use super::Accepter;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IdentAccepter {
    /// The lexeme to accept a value identifier.
    /// Regex: [a-z][a-z0-9_]+
    Val(ValAccepter),
    /// The lexeme to accept a type identifier.
    /// Regex: [A-Z][a-zA-Z0-9]+
    Type(TypeAccepter),
    /// The lexeme to accept a tag identifier.
    /// Regex: #[a-z][a-z0-9]*(-[a-z0-9])*
    Tag(TagAccepter),
    /// The lexeme to accept a macro identifier.
    /// Regex: @([a-z][a-z0-9:]*)+
    Macro(MacroAccepter),
    /// The lexeme to accept a subtype identifier.
    /// Regex: \$[a-zA-Z][a-zA-Z0-9]+
    Subtype(SubtypeAccepter),
}

impl Accepter for IdentAccepter {
    type State = Self;

    fn acceptable(&self) -> bool {
        match self {
            Self::Val(state) => state.acceptable(),
            Self::Type(state) => state.acceptable(),
            Self::Tag(state) => state.acceptable(),
            Self::Macro(state) => state.acceptable(),
            Self::Subtype(state) => state.acceptable(),
        }
    }

    fn accept(self, c: char) -> Option<Self::State> {
        match self {
            Self::Val(state) => state.accept(c).map(Self::Val),
            Self::Type(state) => state.accept(c).map(Self::Type),
            Self::Tag(state) => state.accept(c).map(Self::Tag),
            Self::Macro(state) => state.accept(c).map(Self::Macro),
            Self::Subtype(state) => state.accept(c).map(Self::Subtype),
        }
    }
}

impl IdentAccepter {
    pub fn stream() -> Vec<Self> {
        use IdentAccepter::*;
        vec![
            Val(ValAccepter::default()),
            Type(TypeAccepter::default()),
            Tag(TagAccepter::default()),
            Macro(MacroAccepter::default()),
            Subtype(SubtypeAccepter::default()),
        ]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ValAccepter {
    #[default]
    /// The lexeme hasn't started yet
    Unset,
    /// The value identifier lexeme is acceptable
    Acceptable,
}

impl Accepter for ValAccepter {
    type State = Self;

    fn acceptable(&self) -> bool {
        *self == Self::Acceptable
    }

    fn accept(self, c: char) -> Option<Self> {
        match self {
            Self::Unset if c.is_ascii_lowercase() => Some(Self::Acceptable),
            Self::Acceptable if c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_' => Some(Self::Acceptable),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TypeAccepter {
    #[default]
    /// The lexeme hasn't started yet
    Unset,
    /// The type identifier lexeme is acceptable
    Acceptable,
}

impl Accepter for TypeAccepter {
    type State = Self;

    fn acceptable(&self) -> bool {
        *self == Self::Acceptable
    }

    fn accept(self, c: char) -> Option<Self> {
        match self {
            Self::Unset if c.is_ascii_uppercase() => Some(Self::Acceptable),
            Self::Acceptable if c.is_ascii_alphanumeric() => Some(Self::Acceptable),
            _ => None,
        }
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TagAccepter {
    #[default]
    /// The lexeme hasn't started yet
    Unset,
    /// Waiting for a [a-z] character
    WaitingAlpha,
    /// Waiting for a [a-z0-9-] character
    WaitingAlphaNumDash,
}

impl Accepter for TagAccepter {
    type State = Self;

    fn acceptable(&self) -> bool {
        *self == Self::WaitingAlphaNumDash
    }

    fn accept(self, c: char) -> Option<Self> {
        match self {
            Self::Unset if c == '#' => Some(Self::WaitingAlpha),
            Self::WaitingAlpha if c.is_ascii_lowercase() => Some(Self::WaitingAlphaNumDash),
            Self::WaitingAlphaNumDash if c.is_ascii_lowercase() || c.is_ascii_digit() => Some(Self::WaitingAlphaNumDash),
            Self::WaitingAlphaNumDash if c == '-' => Some(Self::WaitingAlpha),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MacroAccepter {
    #[default]
    /// The lexeme hasn't started yet
    Unset,
    /// Waiting for a leading [a-z] character
    WaitingLeadingAlpha,
    /// Waiting for a [a-z] character
    WaitingAlpha,
    /// Waiting for a [a-z0-9:] character
    WaitingAlphaNumColon,
}

impl Accepter for MacroAccepter {
    type State = Self;

    fn acceptable(&self) -> bool {
        *self == Self::WaitingAlphaNumColon || *self == Self::WaitingAlpha
    }

    fn accept(self, c: char) -> Option<Self> {
        match self {
            Self::Unset if c == '@' => Some(Self::WaitingLeadingAlpha),
            Self::WaitingLeadingAlpha if c.is_ascii_lowercase() => Some(Self::WaitingAlphaNumColon),
            Self::WaitingAlpha if c.is_ascii_lowercase() => Some(Self::WaitingAlphaNumColon),
            Self::WaitingAlphaNumColon if c.is_ascii_lowercase() || c.is_ascii_digit() || c == ':' => Some(Self::WaitingAlphaNumColon),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SubtypeAccepter {
    #[default]
    /// The lexeme hasn't started yet
    Unset,
    /// Waiting for a `[a-zA-Z]` character
    WaitingAlpha,
    /// Waiting for a `[a-zA-Z0-9]` character
    Any,
}

impl Accepter for SubtypeAccepter {
    type State = Self;

    fn acceptable(&self) -> bool {
        *self == Self::Any
    }

    fn accept(self, c: char) -> Option<Self> {
        match self {
            Self::Unset if c == '$' => Some(Self::WaitingAlpha),
            Self::WaitingAlpha if c.is_ascii_alphabetic() => Some(Self::Any),
            Self::Any if c.is_ascii_alphanumeric() => Some(Self::Any),
            _ => None,
        }
    }
}