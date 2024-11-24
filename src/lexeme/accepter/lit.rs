use super::Accepter;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LitAccepter {
    /// The lexeme to accept a `int` literal.
    /// Regex: (0|[1-9][0-9_]*)(U|I)(8|16|32|64)?"
    IntDec(IntDecAccepter),
    IntBin(IntBinAccepter),
    IntOct(IntOctAccepter),
    IntHex(IntHexAccepter),
    Flt(FltAccepter),
    Chr(CharAccepter),
    Str(StrAccepter),
    Atom(AtomAccepter),
}

impl Accepter for LitAccepter {
    type Accepter = Self;

    fn acceptable(&self) -> bool {
        match self {
            Self::IntDec(acp) => acp.acceptable(),
            Self::IntBin(acp) => acp.acceptable(),
            Self::IntOct(acp) => acp.acceptable(),
            Self::IntHex(acp) => acp.acceptable(),
            Self::Flt(acp) => acp.acceptable(),
            Self::Chr(acp) => acp.acceptable(),
            Self::Str(acp) => acp.acceptable(),
            Self::Atom(acp) => acp.acceptable(),
        }
    }

    fn accept(self, c: char) -> Option<Self::Accepter> {
        match self {
            Self::IntDec(acp) => acp.accept(c).map(Self::IntDec),
            Self::IntBin(acp) => acp.accept(c).map(Self::IntBin),
            Self::IntOct(acp) => acp.accept(c).map(Self::IntOct),
            Self::IntHex(acp) => acp.accept(c).map(Self::IntHex),
            Self::Flt(acp) => acp.accept(c).map(Self::Flt),
            Self::Chr(acp) => acp.accept(c).map(Self::Chr),
            Self::Str(acp) => acp.accept(c).map(Self::Str),
            Self::Atom(acp) => acp.accept(c).map(Self::Atom),
        }
    }
}

impl LitAccepter {
    pub fn stream() -> Vec<Self> {
        use LitAccepter::*;
        vec![
            IntDec(IntDecAccepter::default()),
            IntBin(IntBinAccepter::default()),
            IntOct(IntOctAccepter::default()),
            IntHex(IntHexAccepter::default()),
            Flt(FltAccepter::default()),
            Chr(CharAccepter::default()),
            Str(StrAccepter::default()),
            Atom(AtomAccepter::default()),
        ]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum IntDecAccepter {
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

impl Accepter for IntDecAccepter {
    type Accepter = Self;

    fn acceptable(&self) -> bool {
        match self {
            Self::LeadingZero | Self::LeadingNonZero | Self::WithBitWidth(8) | Self::WithBitWidth(16) | Self::WithBitWidth(32) | Self::WithBitWidth(64) => true,
            _ => false,
        }
    }

    fn accept(self, c: char) -> Option<Self> {
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
pub enum IntBinAccepter {
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

impl Accepter for IntBinAccepter {
    type Accepter = Self;

    fn acceptable(&self) -> bool {
        *self == Self::Valid
    }

    fn accept(self, c: char) -> Option<Self> {
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
pub enum IntOctAccepter {
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

impl Accepter for IntOctAccepter {
    type Accepter = Self;

    fn acceptable(&self) -> bool {
        *self == Self::Valid
    }

    fn accept(self, c: char) -> Option<Self> {
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
pub enum IntHexAccepter {
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

impl Accepter for IntHexAccepter {
    type Accepter = Self;
    
    fn acceptable(&self) -> bool {
        *self == Self::Acceptable
    }

    fn accept(self, c: char) -> Option<Self> {
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
pub enum FltAccepter {
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

impl Accepter for FltAccepter {
    type Accepter = Self;

    fn acceptable(&self) -> bool {
        *self == Self::Acceptable
    }

    fn accept(self, c: char) -> Option<Self> {
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
pub enum CharAccepter {
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
    

impl Accepter for CharAccepter {
    type Accepter = Self;
    
    fn acceptable(&self) -> bool {
        *self == Self::Acceptable
    }

    fn accept(self, c: char) -> Option<Self> {
        match self {
            Self::Unset if c == '\'' => Some(Self::LeadingSingleQuote),
            Self::LeadingSingleQuote if c.is_ascii_alphanumeric() || c.is_ascii_punctuation() || c == ' ' => Some(Self::SingleChar),
            Self::SingleChar if c == '\'' => Some(Self::Acceptable),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum StrAccepter {
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

impl StrAccepter {
    pub fn acceptable(self) -> bool {
        self == Self::Acceptable
    }

    pub fn accept(self, c: char) -> Option<Self> {
        match self {
            Self::Unset if c == '"' => Some(Self::LeadingDoubleQuote),
            Self::LeadingDoubleQuote if c != '\\' => Some(Self::Any),
            acp if c == '\\' && acp != Self::EscapeNext => Some(Self::EscapeNext),
            Self::Any if c != '"' => Some(Self::Any),
            Self::Any if c == '"' => Some(Self::Acceptable),
            Self::EscapeNext => Some(Self::Any),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AtomAccepter {
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

impl Accepter for AtomAccepter {
    type Accepter = Self;
    
    fn acceptable(&self) -> bool {
        *self == Self::WaitingAlphaNumDash
    }

    fn accept(self, c: char) -> Option<Self> {
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

        let mut acp = StrAccepter::default();
        assert_eq!(acp, StrAccepter::Unset);
        assert_eq!(acp.acceptable(), false);
        acp = acp.accept('"').unwrap();
        assert_eq!(acp.acceptable(), false);
        acp = acp.accept('a').unwrap();
        assert_eq!(acp.acceptable(), false);
        acp = acp.accept('b').unwrap();
        assert_eq!(acp.acceptable(), false);
        acp = acp.accept('"').unwrap();
        assert_eq!(acp.acceptable(), true);
        assert_eq!(acp.accept('c'), None);
    }
}