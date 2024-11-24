use super::Accepter;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LitState {
    /// The lexeme to accept a `int` literal.
    /// Regex: (0|[1-9][0-9_]*)(U|I)(8|16|32|64)?"
    IntDec(IntDecState),
    IntBin(IntBinState),
    IntOct(IntOctState),
    IntHex(IntHexState),
    Flt(FltState),
    Chr(CharState),
    Str(StrState),
    Atom(AtomState),
}

impl Accepter for LitState {
    type State = Self;

    fn acceptable(&self) -> bool {
        match self {
            Self::IntDec(state) => state.acceptable(),
            Self::IntBin(state) => state.acceptable(),
            Self::IntOct(state) => state.acceptable(),
            Self::IntHex(state) => state.acceptable(),
            Self::Flt(state) => state.acceptable(),
            Self::Chr(state) => state.acceptable(),
            Self::Str(state) => state.acceptable(),
            Self::Atom(state) => state.acceptable(),
        }
    }

    fn accept(self, c: char) -> Option<Self::State> {
        match self {
            Self::IntDec(state) => state.accept(c).map(Self::IntDec),
            Self::IntBin(state) => state.accept(c).map(Self::IntBin),
            Self::IntOct(state) => state.accept(c).map(Self::IntOct),
            Self::IntHex(state) => state.accept(c).map(Self::IntHex),
            Self::Flt(state) => state.accept(c).map(Self::Flt),
            Self::Chr(state) => state.accept(c).map(Self::Chr),
            Self::Str(state) => state.accept(c).map(Self::Str),
            Self::Atom(state) => state.accept(c).map(Self::Atom),
        }
    }
}

impl LitState {
    pub fn stream() -> Vec<Self> {
        use LitState::*;
        vec![
            IntDec(IntDecState::default()),
            IntBin(IntBinState::default()),
            IntOct(IntOctState::default()),
            IntHex(IntHexState::default()),
            Flt(FltState::default()),
            Chr(CharState::default()),
            Str(StrState::default()),
            Atom(AtomState::default()),
        ]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum IntDecState {
    #[default]
    /// The lexeme hasn't started yet
    Unset,
    /// The lexeme has a leading zero
    /// Next can only be either `U` or `I` or end
    LeadingZero,
    /// The lexeme has a leading non-zero
    /// Next can be either `0-9`, `_`, `U``, `I` or end
    LeadingNonZero,
    /// The lexeme has a leading non-zero and `_` was just read
    /// Next can be either `0-9`, `U`, `I` or end
    LeadingNonZeroUnderscore,
    /// The lexeme has a leading non-zero and `U` or `I` was read
    /// Next can be either `8`, `1`, `3`, `6` or end
    /// Then respectively end, `6`, `2` and `4`
    WithBitWidth(u8),
}

impl IntDecState {
    pub fn acceptable(self) -> bool {
        match self {
            Self::LeadingZero | Self::LeadingNonZero | Self::WithBitWidth(8) | Self::WithBitWidth(16) | Self::WithBitWidth(32) | Self::WithBitWidth(64) => true,
            _ => false,
        }
    }

    pub fn accept(self, c: char) -> Option<Self> {
        match self {
            Self::Unset if c == '0' => Some(Self::LeadingZero),
            Self::Unset if c.is_digit(10) && c != '0' => Some(Self::LeadingNonZero),
            Self::LeadingZero if c == 'U' || c == 'I' => Some(Self::WithBitWidth(0)),
            Self::LeadingNonZero if c.is_digit(10) => Some(Self::LeadingNonZero),
            Self::LeadingNonZero if c == '_' => Some(Self::LeadingNonZeroUnderscore),
            Self::LeadingNonZero if c == 'U' || c == 'I' => Some(Self::WithBitWidth(0)),
            Self::LeadingNonZeroUnderscore if c.is_digit(10) => Some(Self::LeadingNonZero),
            Self::WithBitWidth(0) if c == '8' => Some(Self::WithBitWidth(8)),
            Self::WithBitWidth(0) if c == '1' => Some(Self::WithBitWidth(1)),
            Self::WithBitWidth(0) if c == '3' => Some(Self::WithBitWidth(3)),
            Self::WithBitWidth(0) if c == '6' => Some(Self::WithBitWidth(6)),
            Self::WithBitWidth(1) if c == '6' => Some(Self::WithBitWidth(16)),
            Self::WithBitWidth(3) if c == '2' => Some(Self::WithBitWidth(32)),
            Self::WithBitWidth(6) if c == '4' => Some(Self::WithBitWidth(64)),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum IntBinState {
    #[default]
    /// The lexeme hasn't started yet
    Unset,
    /// The lexeme has a leading `0`
    /// Next can only be either `b`
    LeadingZero,
    /// The lexeme has a leading `0b`
    /// Next can only be either `0` or `1` or end
    Leading0b,
    /// The lexeme has a leading `0b` and `_` was just read
    /// Next can be either `0` or `1` or end
    TrailingUnderscore,
    /// The lexeme is a valid binary literal
    Valid,
}

impl IntBinState {
    pub fn acceptable(self) -> bool {
        self == Self::Valid
    }

    pub fn accept(self, c: char) -> Option<Self> {
        match self {
            Self::Unset if c == '0' => Some(Self::LeadingZero),
            Self::LeadingZero if c == 'b' => Some(Self::Leading0b),
            Self::Leading0b | Self::Valid if c == '0' || c == '1' => Some(Self::Valid),
            Self::Valid if c == '_' => Some(Self::TrailingUnderscore),
            Self::TrailingUnderscore if c == '0' || c == '1' => Some(Self::Valid),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum IntOctState {
    #[default]
    /// The lexeme hasn't started yet
    Unset,
    /// The lexeme has a leading `0`
    /// Next can only be either `o`
    LeadingZero,
    /// The lexeme has a leading `0o`
    /// Next can only be either `0-7` or end
    Leading0o,
    /// The lexeme has a leading `0o` and `_` was just read
    /// Next can be either `0-7` or end
    TrailingUnderscore,
    /// The lexeme is a valid octal literal
    Valid,
}

impl IntOctState {
    pub fn acceptable(self) -> bool {
        self == Self::Valid
    }

    pub fn accept(self, c: char) -> Option<Self> {
        match self {
            Self::Unset if c == '0' => Some(Self::LeadingZero),
            Self::LeadingZero if c == 'o' => Some(Self::Leading0o),
            Self::Leading0o | Self::Valid if c.is_digit(8) => Some(Self::Valid),
            Self::Valid if c == '_' => Some(Self::TrailingUnderscore),
            Self::TrailingUnderscore if c.is_digit(8) => Some(Self::Valid),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum IntHexState {
    #[default]
    /// The lexeme hasn't started yet
    Unset,
    /// The lexeme has a leading `0`
    /// Next can only be either `x`
    LeadingZero,
    /// The lexeme has a leading `0x`
    /// Next can only be either `0-9`, `a-f`, `A-F` or end
    Leading0x,
    /// The lexeme has a leading `0x` and `_` was just read
    /// Next can be either `0-9`, `a-f`, `A-F` or end
    TrailingUnderscore,
    /// The lexeme is a valid hexadecimal literal
    Acceptable,
}

impl IntHexState {
    pub fn acceptable(self) -> bool {
        self == Self::Acceptable
    }

    pub fn accept(self, c: char) -> Option<Self> {
        match self {
            Self::Unset if c == '0' => Some(Self::LeadingZero),
            Self::LeadingZero if c == 'x' => Some(Self::Leading0x),
            Self::Leading0x | Self::Acceptable if c.is_digit(16) || c.is_ascii_lowercase() || c.is_ascii_uppercase() => Some(Self::Acceptable),
            Self::Acceptable if c == '_' => Some(Self::TrailingUnderscore),
            Self::TrailingUnderscore if c.is_digit(16) || c.is_ascii_lowercase() || c.is_ascii_uppercase() => Some(Self::Acceptable),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum FltState {
    #[default]
    /// The lexeme hasn't started yet
    Unset,
    /// The lexeme has a leading `0`
    /// Next can only be either `.` or end
    LeadingZero,
    /// The lexeme has a leading non-zero
    /// Next can be either `0-9` or `.`
    LeadingNonZero,
    /// The `.` was just read
    /// Next can only be `0-9`
    DecimalPoint,
    /// The lexeme is a valid floating point literal
    /// Next can be either `0-9` or end
    Acceptable,
}

impl FltState {
    pub fn acceptable(self) -> bool {
        self == Self::Acceptable
    }

    pub fn accept(self, c: char) -> Option<Self> {
        match self {
            Self::Unset if c == '0' => Some(Self::LeadingZero),
            Self::Unset if c.is_digit(10) && c != '0' => Some(Self::LeadingNonZero),
            Self::LeadingZero if c == '.' => Some(Self::DecimalPoint),
            Self::LeadingNonZero if c.is_digit(10) => Some(Self::LeadingNonZero),
            Self::LeadingNonZero if c == '.' => Some(Self::DecimalPoint),
            Self::DecimalPoint if c.is_digit(10) => Some(Self::Acceptable),
            Self::Acceptable if c.is_digit(10) => Some(Self::Acceptable),
            _ => None,
        }
    }
}

// TODO: Implement escape sequences and symbols
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CharState {
    #[default]
    /// The lexeme hasn't started yet
    Unset,
    /// The lexeme has a leading `'`
    /// Next can be either `a-z`, `A-Z`, `0-9`, `!`, `@`, `#`, `$`, `%`, `^`, `&`, `*`, `(`, `)`, `-`, `=`, `+`, `[`, `]`, `{`, `}`, `;`, `:`, `'`, `"`, `,`, `.`, `<`, `>`, `/`, `?`, `\`, `|`, `~`, ` ` or end
    LeadingSingleQuote,
    /// The lexeme has a leading `'` and a character was just read
    /// Next can be either `'` or end
    SingleChar,
    /// The lexeme has a leading `'` and a character was just read
    Acceptable,
}
    

impl CharState {
    pub fn acceptable(self) -> bool {
        self == Self::Acceptable
    }

    pub fn accept(self, c: char) -> Option<Self> {
        match self {
            Self::Unset if c == '\'' => Some(Self::LeadingSingleQuote),
            Self::LeadingSingleQuote if c.is_ascii_alphanumeric() || c.is_ascii_punctuation() || c == ' ' => Some(Self::SingleChar),
            Self::SingleChar if c == '\'' => Some(Self::Acceptable),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum StrState {
    #[default]
    /// The lexeme hasn't started yet
    Unset,
    /// The lexeme has a leading `"`
    /// Next can be either `a-z`, `A-Z`, `0-9`, `!`, `@`, `#`, `$`, `%`, `^`, `&`, `*`, `(`, `)`, `-`, `=`, `+`, `[`, `]`, `{`, `}`, `;`, `:`, `'`, `"`, `,`, `.`, `<`, `>`, `/`, `?`, `\`, `|`, `~`, ` ` or end
    LeadingDoubleQuote,
    /// The lexeme has a leading `"` and a character was just read
    /// Next can be either `"` or end
    Any,
    /// A `\` was just read	
    /// Next will be escaped
    EscapeNext,
    /// The lexeme has a trailing '"'"
    Acceptable,
}

impl StrState {
    pub fn acceptable(self) -> bool {
        self == Self::Acceptable
    }

    pub fn accept(self, c: char) -> Option<Self> {
        match self {
            Self::Unset if c == '"' => Some(Self::LeadingDoubleQuote),
            Self::LeadingDoubleQuote if c != '\\' => Some(Self::Any),
            state if c == '\\' && state != Self::EscapeNext => Some(Self::EscapeNext),
            Self::Any if c != '"' => Some(Self::Any),
            Self::Any if c == '"' => Some(Self::Acceptable),
            Self::EscapeNext => Some(Self::Any),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AtomState {
    #[default]
    /// The lexeme hasn't started yet
    Unset,
    /// The lexeme has a leading `'`
    /// Next must be `a-z`
    WaitingLeadingAlpha,
    /// Last char was `-`
    /// Next must be `a-z`
    WaitingAlpha,
    /// The lexeme has a leading `a-z` character
    /// Next can be either `a-z`, `0-9`, `-` or end
    WaitingAlphaNumDash,
}

impl AtomState {
    pub fn acceptable(self) -> bool {
        self == Self::WaitingAlphaNumDash
    }

    pub fn accept(self, c: char) -> Option<Self> {
        match self {
            Self::Unset if c == '\'' => Some(Self::WaitingLeadingAlpha),
            Self::WaitingLeadingAlpha | Self::WaitingAlpha if c.is_ascii_lowercase() => Some(Self::WaitingAlphaNumDash),
            Self::WaitingAlphaNumDash if c.is_ascii_lowercase() || c.is_ascii_digit() => Some(Self::WaitingAlphaNumDash),
            Self::WaitingAlphaNumDash if c == '-' => Some(Self::WaitingAlpha),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn lex_string() {
        use super::*;

        let mut state = StrState::default();
        assert_eq!(state, StrState::Unset);
        assert_eq!(state.acceptable(), false);
        state = state.accept('"').unwrap();
        assert_eq!(state.acceptable(), false);
        state = state.accept('a').unwrap();
        assert_eq!(state.acceptable(), false);
        state = state.accept('b').unwrap();
        assert_eq!(state.acceptable(), false);
        state = state.accept('"').unwrap();
        assert_eq!(state.acceptable(), true);
        assert_eq!(state.accept('c'), None);
    }
}