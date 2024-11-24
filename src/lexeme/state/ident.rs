#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IdentState {
    /// The lexeme to accept a value identifier.
    /// Regex: [a-z][a-z0-9_]+
    Val(ValState),
    /// The lexeme to accept a type identifier.
    /// Regex: [A-Z][a-zA-Z0-9]+
    Type(TypeState),
    /// The lexeme to accept a tag identifier.
    /// Regex: #[a-z][a-z0-9]*(-[a-z0-9])*
    Tag(TagState),
    /// The lexeme to accept a macro identifier.
    /// Regex: @([a-z][a-z0-9:]*)+
    Macro(MacroState),
    /// The lexeme to accept a subtype identifier.
    /// Regex: \$[a-zA-Z][a-zA-Z0-9]+
    Subtype(SubtypeState),
}

impl IdentState {
    pub fn acceptable(&self) -> bool {
        match self {
            Self::Val(state) => state.acceptable(),
            Self::Type(state) => state.acceptable(),
            Self::Tag(state) => state.acceptable(),
            Self::Macro(state) => state.acceptable(),
            Self::Subtype(state) => state.acceptable(),
        }
    }

    pub fn accept(self, c: char) -> Option<Self> {
        match self {
            Self::Val(state) => state.accept(c).map(Self::Val),
            Self::Type(state) => state.accept(c).map(Self::Type),
            Self::Tag(state) => state.accept(c).map(Self::Tag),
            Self::Macro(state) => state.accept(c).map(Self::Macro),
            Self::Subtype(state) => state.accept(c).map(Self::Subtype),
        }
    }

    pub fn stream() -> Vec<Self> {
        use IdentState::*;
        vec![
            Val(ValState::default()),
            Type(TypeState::default()),
            Tag(TagState::default()),
            Macro(MacroState::default()),
            Subtype(SubtypeState::default()),
        ]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ValState {
    #[default]
    /// The lexeme hasn't started yet
    Unset,
    /// The value identifier lexeme is acceptable
    Acceptable,
}

impl ValState {
    pub fn acceptable(&self) -> bool {
        *self == Self::Acceptable
    }

    pub fn accept(self, c: char) -> Option<Self> {
        match self {
            Self::Unset if c.is_ascii_lowercase() => Some(Self::Acceptable),
            Self::Acceptable if c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_' => Some(Self::Acceptable),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TypeState {
    #[default]
    /// The lexeme hasn't started yet
    Unset,
    /// The type identifier lexeme is acceptable
    Acceptable,
}

impl TypeState {
    pub fn acceptable(&self) -> bool {
        *self == Self::Acceptable
    }

    pub fn accept(self, c: char) -> Option<Self> {
        match self {
            Self::Unset if c.is_ascii_uppercase() => Some(Self::Acceptable),
            Self::Acceptable if c.is_ascii_alphanumeric() => Some(Self::Acceptable),
            _ => None,
        }
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TagState {
    #[default]
    /// The lexeme hasn't started yet
    Unset,
    /// Waiting for a [a-z] character
    WaitingAlpha,
    /// Waiting for a [a-z0-9-] character
    WaitingAlphaNumDash,
}

impl TagState {
    pub fn acceptable(&self) -> bool {
        *self == Self::WaitingAlphaNumDash
    }

    pub fn accept(self, c: char) -> Option<Self> {
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
pub enum MacroState {
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

impl MacroState {
    pub fn acceptable(&self) -> bool {
        *self == Self::WaitingAlphaNumColon || *self == Self::WaitingAlpha
    }

    pub fn accept(self, c: char) -> Option<Self> {
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
pub enum SubtypeState {
    #[default]
    /// The lexeme hasn't started yet
    Unset,
    /// Waiting for a `[a-zA-Z]` character
    WaitingAlpha,
    /// Waiting for a `[a-zA-Z0-9]` character
    Any,
}

impl SubtypeState {
    pub fn acceptable(&self) -> bool {
        *self == Self::Any
    }

    pub fn accept(self, c: char) -> Option<Self> {
        match self {
            Self::Unset if c == '$' => Some(Self::WaitingAlpha),
            Self::WaitingAlpha if c.is_ascii_alphabetic() => Some(Self::Any),
            Self::Any if c.is_ascii_alphanumeric() => Some(Self::Any),
            _ => None,
        }
    }
}