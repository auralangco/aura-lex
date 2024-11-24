#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LexemeState {
    /// The lexeme to accept a `val` keyword.
    KwVal {
        /// Indicates the count of matching characters
        stage: u8,
        /// Indicates that the lexeme is acceptable
        acceptable: bool,
    },
    /// The lexeme to accept a `fn` keyword.
    KwFn {
        /// Indicates the count of matching characters
        stage: u8,
        /// Indicates that the lexeme is acceptable
        acceptable: bool,
    },
    /// The lexeme to accept a `type` keyword.
    KwType {
        /// Indicates the count of matching characters
        stage: u8,
        /// Indicates that the lexeme is acceptable
        acceptable: bool,
    },
    /// The lexeme to accept a `tag` keyword.
    KwTag {
        /// Indicates the count of matching characters
        stage: u8,
        /// Indicates that the lexeme is acceptable
        acceptable: bool,
    },
    /// The lexeme to accept a `main` keyword.
    KwMain {
        /// Indicates the count of matching characters
        stage: u8,
        /// Indicates that the lexeme is acceptable
        acceptable: bool,
    },
    /// The lexeme to accept a `macro` keyword.
    KwMacro {
        /// Indicates the count of matching characters
        stage: u8,
        /// Indicates that the lexeme is acceptable
        acceptable: bool,
    },
    /// The lexeme to accept an `import` keyword.
    KwImport {
        /// Indicates the count of matching characters
        stage: u8,
        /// Indicates that the lexeme is acceptable
        acceptable: bool,
    },
    /// The lexeme to accept an `object` keyword.
    KwObject {
        /// Indicates the count of matching characters
        stage: u8,
        /// Indicates that the lexeme is acceptable
        acceptable: bool,
    },
    /// The lexeme to accept a value identifier.
    /// Regex: [a-z][a-z0-9_]+
    IdentVal{ 
        /// Indicates that the lexeme is acceptable
        acceptable: bool
    },
    /// The lexeme to accept a type identifier.
    /// Regex: [A-Z][a-zA-Z0-9]+
    IdentTy{ 
        /// Indicates that the lexeme is acceptable
        acceptable: bool
    },
    /// The lexeme to accept a tag identifier.
    /// Regex: #[a-z][a-z0-9]*(-[a-z0-9])*
    IdentTag {
        /// Indicates that the `#` was read
        state: IdentTagState,
    },
    /// The lexeme to accept a macro identifier.
    /// Regex: @[a-z][a-z0-9:]*
    IdentMacro {
        /// Indicates that the `@` was read
        at: bool,
        /// Indicates that the lexeme accepts a `:`
        accept_colon: bool,
        /// Indicates that the lexeme accepts numbers
        accept_num: bool,
        /// Indicates that the lexeme is acceptable
        acceptable: bool,
    },
    /// The lexeme to accept a subtype identifier.
    /// Regex: \$[a-zA-Z][a-zA-Z0-9]+
    IdentSty {
        /// Indicates that the `$` was read
        dollar: bool,
        /// Indicates that the first alpha char was read
        alpha: bool,
        /// Indicates that the lexeme is acceptable
        acceptable: bool,
    },
    /// The lexeme to accept a `:=` operator.
    OpDecl{ 
        /// Indicates that the `:` was read
        colon: bool,
        /// Indicates that the lexeme is acceptable
        acceptable: bool
    },
    /// The lexeme to accept a `=` operator.
    OpEq { 
        /// Indicates that the lexeme is acceptable
        acceptable: bool
    },
    /// The lexeme to accept a `+` operator.
    OpPlus { 
        /// Indicates that the lexeme is acceptable
        acceptable: bool
    },
    /// The lexeme to accept a `-` operator.
    OpMinus { 
        /// Indicates that the lexeme is acceptable
        acceptable: bool
    },
    /// The lexeme to accept a `*` operator.
    OpStar { 
        /// Indicates that the lexeme is acceptable
        acceptable: bool
    },
    /// The lexeme to accept a `/` operator.
    OpSlash { 
        /// Indicates that the lexeme is acceptable
        acceptable: bool
    },
    /// The lexeme to accept a `^` operator.
    OpCaret { 
        /// Indicates that the lexeme is acceptable
        acceptable: bool
    },
    /// The lexeme to accept a `_` operator.
    OpUScore { 
        /// Indicates that the lexeme is acceptable
        acceptable: bool
    },
    /// The lexeme to accept a `%` operator.
    OpPercent { 
        /// Indicates that the lexeme is acceptable
        acceptable: bool
    },
    /// The lexeme to accept a `&` operator.
    OpAnd { 
        /// Indicates that the lexeme is acceptable
        acceptable: bool
    },
    /// The lexeme to accept a `&&` operator.
    OpAndAnd {
        /// Indicates that the `&` was read
        and: bool,
        /// Indicates that the lexeme is acceptable
        acceptable: bool,
    },
    /// The lexeme to accept a `|` operator.
    OpOr { 
        /// Indicates that the lexeme is acceptable
        acceptable: bool
    },
    /// The lexeme to accept a `||` operator.
    OpOrOr { 
        /// Indicates that the '|' was read
        or: bool,
        /// Indicates that the lexeme is acceptable
        acceptable: bool
    },
    /// The lexeme to accept a `!` operator.
    OpNot { 
        /// Indicates that the lexeme is acceptable
        acceptable: bool
    },
    /// The lexeme to accept a `!=` operator.
    OpNotEq {
        /// Indicates that the `!` was read
        not: bool,
        /// Indicates that the lexeme is acceptable
        acceptable: bool
    },
    /// The lexeme to accept a `==` operator.
    OpEqEq {
        /// Indicates that the `=` was read
        eq: bool,
        /// Indicates that the lexeme is acceptable
        acceptable: bool
    },
    /// The lexeme to accept a `>` operator.
    OpGt { 
        /// Indicates that the lexeme is acceptable
        acceptable: bool
    },
    /// The lexeme to accept a `>=` operator.
    OpGtEq {
        /// Indicates that the `>` was read
        gt: bool,
        /// Indicates that the lexeme is acceptable
        acceptable: bool
    },
    /// The lexeme to accept a `<` operator.
    OpLt { 
        /// Indicates that the lexeme is acceptable
        acceptable: bool
    },
    /// The lexeme to accept a `<=` operator.
    OpLtEq {
        /// Indicates that the `<` was read
        lt: bool,
        /// Indicates that the lexeme is acceptable
        acceptable: bool
    },
    /// The lexeme to accept a `<<` operator.
    OpLtLt {
        /// Indicates that the `<` was read
        lt: bool,
        /// Indicates that the lexeme is acceptable
        acceptable: bool
    },
    /// The lexeme to accept a `>>` operator.
    OpGtGt {
        /// Indicates that the `>` was read
        gt: bool,
        /// Indicates that the lexeme is acceptable
        acceptable: bool
    },
    /// The lexeme to accept a `->` operator.
    OpRArw {
        /// Indicates that the `-` was read
        dash: bool,
        /// Indicates that the lexeme is acceptable
        acceptable: bool
    },
    /// The lexeme to accept a `=>` operator.
    OpFatRArw {
        /// Indicates that the `=` was read
        eq: bool,
        /// Indicates that the lexeme is acceptable
        acceptable: bool
    },
    /// The lexeme to accept a `~` operator.
    OpTilde { 
        /// Indicates that the lexeme is acceptable
        acceptable: bool
    },
    /// The lexeme to accept a `::` operator.
    OpJoin {
        /// Indicates that the `:` was read
        colon: bool,
        /// Indicates that the lexeme is acceptable
        acceptable: bool
    },
    /// The lexeme to accept a `\` operator.
    OpBSlash { 
        /// Indicates that the lexeme is acceptable
        acceptable: bool
    },
    /// The lexeme to accept a `..` operator.
    OpRange {
        /// Indicates the count of `.` was read
        dot: u8,
        /// Indicates that the lexeme is acceptable
        acceptable: bool
    },
    /// The lexeme to accept a `..=` operator.
    OpCRange {
        /// Indicates the count of `.` was read
        dot: u8,
        /// Indicates that the lexeme is acceptable
        acceptable: bool
    },
    /// The lexeme to accept a `...` operator.
    OpSpread {
        /// Indicates the count of `.` was read
        dot: u8,
        /// Indicates that the lexeme is acceptable
        acceptable: bool
    },
    /// The lexeme to accept a `(` operator.
    DelimOParen { 
        /// Indicates that the lexeme is acceptable
        acceptable: bool
    },
    /// The lexeme to accept a `)` operator.
    DelimCParen { 
        /// Indicates that the lexeme is acceptable
        acceptable: bool
    },
    /// The lexeme to accept a `[` operator.
    DelimOBrack { 
        /// Indicates that the lexeme is acceptable
        acceptable: bool
    },
    /// The lexeme to accept a `]` operator.
    DelimCBrack { 
        /// Indicates that the lexeme is acceptable
        acceptable: bool
    },
    /// The lexeme to accept a `{` operator.
    DelimOBrace { 
        /// Indicates that the lexeme is acceptable
        acceptable: bool
    },
    /// The lexeme to accept a `}` operator.
    DelimCBrace { 
        /// Indicates that the lexeme is acceptable
        acceptable: bool
    },
    /// The lexeme to accept a `int` literal.
    /// Regex: (0|[1-9][0-9_]*)(U|I)(8|16|32|64)?"
    LitIntDec {
        /// Indicates that the state of the lexeme
        state: LitIntDecState,
        /// Indicates that the lexeme is acceptable
        acceptable: bool
    },
    LitIntBin,
    LitIntOct,
    LitIntHex,
    LitFlt,
    LitChr,
    LitStr,
    LitAtom,
    PtDot,
    PtComma,
    PtColon,
    PtSemi,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum IdentTagState {
    #[default]
    /// The lexeme hasn't started yet
    Unset,
    /// Waiting for a [a-z] character
    WaitingAlpha,
    /// Waiting for a [a-z0-9-] character
    WaitingAlphaNumDash,
}

impl IdentTagState {
    pub fn acceptable(self) -> bool {
        self == Self::WaitingAlphaNumDash
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum LitIntDecState {
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
    Len(u8),
}

impl LitIntDecState {
    pub fn acceptable(self) -> bool {
        match self {
            Self::LeadingZero | Self::LeadingNonZero | Self::Len(8) | Self::Len(16) | Self::Len(32) | Self::Len(64) => true,
            _ => false,
        }
    }
}

pub struct Lexeme<'src> {
    pub ty: LexemeState,
    pub slice: &'src str,
    pub start: usize,
    pub end: usize,
    pub start_coord: (usize, usize),
    pub end_coord: (usize, usize),
}

impl LexemeState {
    pub fn accept(self, c: char) -> Option<Self> {
        match (c, self) {
            // fn keyword
            ('f', Self::KwFn { stage: 0, .. }) => Self::KwFn { stage: 1, acceptable: false },
            ('n', Self::KwFn { stage: 1, .. }) => Self::KwFn { stage: 2, acceptable: true },
            // tag keyword
            ('t', Self::KwTag { stage: 0, .. }) => Self::KwTag { stage: 1, acceptable: false },
            ('a', Self::KwTag { stage: 1, .. }) => Self::KwTag { stage: 2, acceptable: true },
            ('g', Self::KwTag { stage: 2, .. }) => Self::KwTag { stage: 3, acceptable: true },
            // main keyword
            ('m', Self::KwMain { stage: 0, .. }) => Self::KwMain { stage: 1, acceptable: false },
            ('a', Self::KwMain { stage: 1, .. }) => Self::KwMain { stage: 2, acceptable: true },
            ('i', Self::KwMain { stage: 2, .. }) => Self::KwMain { stage: 3, acceptable: true },
            ('n', Self::KwMain { stage: 3, .. }) => Self::KwMain { stage: 4, acceptable: true },
            // type keyword
            ('t', Self::KwType { stage: 0, .. }) => Self::KwType { stage: 1, acceptable: false },
            ('y', Self::KwType { stage: 1, .. }) => Self::KwType { stage: 2, acceptable: false },
            ('p', Self::KwType { stage: 2, .. }) => Self::KwType { stage: 3, acceptable: true },
            ('e', Self::KwType { stage: 3, .. }) => Self::KwType { stage: 4, acceptable: true },
            // macro keyword
            ('m', Self::KwMacro { stage: 0, .. }) => Self::KwMacro { stage: 1, acceptable: false },
            ('a', Self::KwMacro { stage: 1, .. }) => Self::KwMacro { stage: 2, acceptable: true },
            ('c', Self::KwMacro { stage: 2, .. }) => Self::KwMacro { stage: 3, acceptable: true },
            ('r', Self::KwMacro { stage: 3, .. }) => Self::KwMacro { stage: 4, acceptable: true },
            ('o', Self::KwMacro { stage: 4, .. }) => Self::KwMacro { stage: 5, acceptable: true },
            // import keyword
            ('i', Self::KwImport { stage: 0, .. }) => Self::KwImport { stage: 1, acceptable: false },
            ('m', Self::KwImport { stage: 1, .. }) => Self::KwImport { stage: 2, acceptable: true },
            ('p', Self::KwImport { stage: 2, .. }) => Self::KwImport { stage: 3, acceptable: true },
            ('o', Self::KwImport { stage: 3, .. }) => Self::KwImport { stage: 4, acceptable: true },
            ('r', Self::KwImport { stage: 4, .. }) => Self::KwImport { stage: 5, acceptable: true },
            ('t', Self::KwImport { stage: 5, .. }) => Self::KwImport { stage: 6, acceptable: true },
            // object keyword
            ('o', Self::KwObject { stage: 0, .. }) => Self::KwObject { stage: 1, acceptable: false },
            ('b', Self::KwObject { stage: 1, .. }) => Self::KwObject { stage: 2, acceptable: true },
            ('j', Self::KwObject { stage: 2, .. }) => Self::KwObject { stage: 3, acceptable: true },
            ('e', Self::KwObject { stage: 3, .. }) => Self::KwObject { stage: 4, acceptable: true },
            ('c', Self::KwObject { stage: 4, .. }) => Self::KwObject { stage: 5, acceptable: true },
            ('t', Self::KwObject { stage: 5, .. }) => Self::KwObject { stage: 6, acceptable: true },

            // IdentVal [a-z][a-z0-9_]+
            ('a'..='z', Self::IdentVal { .. }) => Self::IdentVal { acceptable: true },
            ('0'..='9', Self::IdentVal { acceptable: true}) | ('_', Self::IdentVal { acceptable: true }) => self,

            // IdentTy [A-Z][a-zA-Z0-9]+
            ('A'..='Z', Self::IdentTy { .. }) => Self::IdentTy{ acceptable: true },
            ('a'..='z', Self::IdentTy { acceptable: true }) | ('0'..='9', Self::IdentTy { acceptable: true }) => self,

            // IdentTag #[a-z][a-z0-9]*(-[a-z0-9])*
            ('#', Self::IdentTag { state: IdentTagState::Unset, .. }) => Self::IdentTag {
                state: IdentTagState::WaitingAlpha,
            }
            ,
            ('a'..='z', Self::IdentTag { state, .. }) if state != IdentTagState::Unset => Self::IdentTag {
                state: IdentTagState::WaitingAlphaNumDash,
            }
            ,
            ('0'..='9', Self::IdentTag { state: IdentTagState::WaitingAlphaNumDash }) => self,
            ('-', Self::IdentTag { state: IdentTagState::WaitingAlphaNumDash }) => Self::IdentTag {
                state: IdentTagState::WaitingAlpha,
            },

            // IdentMacro @[a-z][a-z0-9:]*
            ('@', Self::IdentMacro { at: false, .. }) => Self::IdentMacro {
                at: true,
                accept_colon: false,
                accept_num: false,
                acceptable: false,
            }
            ,
            ('a'..='z', Self::IdentMacro { at: true, .. }) => Self::IdentMacro {
                at: true,
                accept_colon: true,
                accept_num: true,
                acceptable: true,
            }
            ,
            ('0'..='9', Self::IdentMacro { accept_num: true, .. }) => self,
            (':', Self::IdentMacro { accept_colon: true, .. }) => Self::IdentMacro {
                at: true,
                accept_colon: false,
                accept_num: true,
                acceptable: true,
            }
            ,

            // IdentSty $[a-zA-Z][a-zA-Z0-9]+
            ('$', Self::IdentSty { dollar: false, .. }) => Self::IdentSty {
                dollar: true,
                alpha: false,
                acceptable: false,
            }
            ,
            ('A'..='Z', Self::IdentSty { dollar: true, .. }) | ('a'..='z', Self::IdentSty { dollar: true, .. })  => Self::IdentSty {
                dollar: true,
                alpha: true,
                acceptable: true,
            }
            ,
            ('0'..='9', Self::IdentSty { alpha: true, .. }) => self,

            // OpDecl :=
            (':', Self::OpDecl { colon: false, .. }) => Self::OpDecl {
                colon: true,
                acceptable: false,
            }
            ,
            ('=', Self::OpDecl { colon: true, .. }) => Self::OpDecl {
                colon: true,
                acceptable: true,
            }
            ,

            // OpEq =
            ('=', Self::OpEq { .. }) => Self::OpEq { acceptable: true },

            // OpPlus +
            ('+', Self::OpPlus { .. }) => Self::OpPlus { acceptable: true },

            // OpMinus -
            ('-', Self::OpMinus { .. }) => Self::OpMinus { acceptable: true },

            // OpStar *
            ('*', Self::OpStar { .. }) => Self::OpStar { acceptable: true },

            // OpSlash /
            ('/', Self::OpSlash { .. }) => Self::OpSlash { acceptable: true },

            // OpCaret ^
            ('^', Self::OpCaret { .. }) => Self::OpCaret { acceptable: true },

            // OpUScore _
            ('_', Self::OpUScore { .. }) => Self::OpUScore { acceptable: true },

            // OpPercent %
            ('%', Self::OpPercent { .. }) => Self::OpPercent { acceptable: true },

            // OpAnd &
            ('&', Self::OpAnd { .. }) => Self::OpAnd { acceptable: true },

            // OpAndAnd &&
            ('&', Self::OpAndAnd { and: false, .. }) => Self::OpAndAnd {
                and: true,
                acceptable: false,
            }
            ,
            ('&', Self::OpAndAnd { and: true, .. }) => Self::OpAndAnd {
                and: true,
                acceptable: true,
            }
            ,

            // OpOr |
            ('|', Self::OpOr { .. }) => Self::OpOr { acceptable: true },

            // OpOrOr ||
            ('|', Self::OpOrOr { or: false, .. }) => Self::OpOrOr {
                or: true,
                acceptable: false,
            }
            ,
            ('|', Self::OpOrOr { or: true, .. }) => Self::OpOrOr {
                or: true,
                acceptable: true,
            }
            ,

            // OpNot !
            ('!', Self::OpNot { .. }) => Self::OpNot { acceptable: true },

            // OpNotEq !=
            ('!', Self::OpNotEq { not: false, .. }) => Self::OpNotEq {
                not: true,
                acceptable: false,
            }
            ,
            ('=', Self::OpNotEq { not: true, .. }) => Self::OpNotEq {
                not: true,
                acceptable: true,
            }
            ,

            // OpEqEq ==
            ('=', Self::OpEqEq { eq: false, .. }) => Self::OpEqEq {
                eq: true,
                acceptable: false,
            }
            ,
            ('=', Self::OpEqEq { eq: true, .. }) => Self::OpEqEq {
                eq: true,
                acceptable: true,
            }
            ,

            // OpGt >
            ('>', Self::OpGt { .. }) => Self::OpGt { acceptable: true },

            // OpGtEq >=
            ('>', Self::OpGtEq { gt: false, .. }) => Self::OpGtEq {
                gt: true,
                acceptable: false,
            }
            ,
            ('=', Self::OpGtEq { gt: true, .. }) => Self::OpGtEq {
                gt: true,
                acceptable: true,
            }
            ,

            // OpLt <
            ('<', Self::OpLt { .. }) => Self::OpLt { acceptable: true },

            // OpLtEq <=
            ('<', Self::OpLtEq { lt: false, .. }) => Self::OpLtEq {
                lt: true,
                acceptable: false,
            }
            ,
            ('=', Self::OpLtEq { lt: true, .. }) => Self::OpLtEq {
                lt: true,
                acceptable: true,
            }
            ,

            // OpLtLt <<
            ('<', Self::OpLtLt { lt: false, .. }) => Self::OpLtLt {
                lt: true,
                acceptable: false,
            }
            ,
            ('<', Self::OpLtLt { lt: true, .. }) => Self::OpLtLt {
                lt: true,
                acceptable: true,
            }
            ,

            // OpGtGt >>
            ('>', Self::OpGtGt { gt: false, .. }) => Self::OpGtGt {
                gt: true,
                acceptable: false,
            }
            ,
            ('>', Self::OpGtGt { gt: true, .. }) => Self::OpGtGt {
                gt: true,
                acceptable: true,
            }
            ,

            // OpRArw ->
            ('-', Self::OpRArw { dash: false, .. }) => Self::OpRArw {
                dash: true,
                acceptable: false,
            }
            ,
            ('>', Self::OpRArw { dash: true, .. }) => Self::OpRArw {
                dash: true,
                acceptable: true,
            }
            ,

            // OpFatRArw =>
            ('=', Self::OpFatRArw { eq: false, .. }) => Self::OpFatRArw {
                eq: true,
                acceptable: false,
            }
            ,
            ('>', Self::OpFatRArw { eq: true, .. }) => Self::OpFatRArw {
                eq: true,
                acceptable: true,
            }
            ,

            // OpTilde ~
            ('~', Self::OpTilde { .. }) => Self::OpTilde { acceptable: true },

            // OpJoin ::
            (':', Self::OpJoin { colon: false, .. }) => Self::OpJoin {
                colon: true,
                acceptable: false,
            }
            ,
            (':', Self::OpJoin { colon: true, .. }) => Self::OpJoin {
                colon: true,
                acceptable: true,
            }
            ,

            // OpBSlash \
            ('\\', Self::OpBSlash { .. }) => Self::OpBSlash { acceptable: true },

            // OpRange ..
            ('.', Self::OpRange { dot, .. }) if dot < 2 => Self::OpRange {
                dot: dot + 1,
                acceptable: dot == 2,
            }
            ,

            // OpCRange ..=
            ('.', Self::OpCRange { dot, .. }) if dot < 2 => Self::OpCRange {
                dot: dot + 1,
                acceptable: dot == 2,
            }
            ,
            ('=', Self::OpCRange { dot: 2, .. }) => Self::OpCRange {
                dot: 2,
                acceptable: true,
            }
            ,

            // OpSpread ...
            ('.', Self::OpSpread { dot, .. }) if dot < 3 => Self::OpSpread {
                dot: dot + 1,
                acceptable: dot == 3,
            }
            ,

            // DelimOParen (
            ('(', Self::DelimOParen { .. }) => Self::DelimOParen { acceptable: true },
            // DelimCParen )
            (')', Self::DelimCParen { .. }) => Self::DelimCParen { acceptable: true },
            // DelimOBrack [
            ('[', Self::DelimOBrack { .. }) => Self::DelimOBrack { acceptable: true },
            // DelimCBrack ]
            (']', Self::DelimCBrack { .. }) => Self::DelimCBrack { acceptable: true },
            // DelimOBrace {
            ('{', Self::DelimOBrace { .. }) => Self::DelimOBrace { acceptable: true },
            // DelimCBrace }
            ('}', Self::DelimCBrace { .. }) => Self::DelimCBrace { acceptable: true },

            // LitIntDec (0|[1-9][0-9_]*)(U|I)(8|16|32|64)?
            ('0', Self::LitIntDec { state: LitIntDecState::Unset, .. }) => Self::LitIntDec {
                state: LitIntDecState::LeadingZero,
                acceptable: true,
            }, 
            ('1'..='9', Self::LitIntDec { state: LitIntDecState::Unset, .. }) => Self::LitIntDec {
                state: LitIntDecState::LeadingNonZero,
                acceptable: true,
            },
            ('0'..='9', Self::LitIntDec { state, .. }) if state == LitIntDecState::LeadingNonZero || state == LitIntDecState::LeadingNonZeroUnderscore => self,
            ('_', Self::LitIntDec { state: LitIntDecState::LeadingNonZero, .. }) => Self::LitIntDec {
                state: LitIntDecState::LeadingNonZeroUnderscore,
                acceptable: false,
            },
            ('U' | 'I', Self::LitIntDec { state, .. }) if state == LitIntDecState::LeadingZero || state == LitIntDecState::LeadingNonZero => Self::LitIntDec {
                state: LitIntDecState::Len(0),
                acceptable: false,
            },
            (c, Self::LitIntDec { state: LitIntDecState::Len(0), .. }) if c == '1' || c == '3' || c == '6' || c == '8' => Self::LitIntDec {
                state: LitIntDecState::Len(c.to_digit(10).expect("Char should be one of `1`, `3`, `6` or `8`") as u8), 
                acceptable: c == '8' 
            },
            ('6', Self::LitIntDec { state: LitIntDecState::Len(1), .. }) => Self::LitIntDec { 
                state: LitIntDecState::Len(16),
                acceptable: true 
            },
            ('2', Self::LitIntDec { state: LitIntDecState::Len(3), .. }) => Self::LitIntDec { 
                state: LitIntDecState::Len(32), 
                acceptable: true 
            },
            ('4', Self::LitIntDec { state: LitIntDecState::Len(6), .. }) => Self::LitIntDec { 
                state: LitIntDecState::Len(64),
                acceptable: true 
            },
            

            _ => return None,
        }.into()
    }

        pub fn acceptable(&self) -> bool {
            match self {
                Self::KwVal { acceptable, .. }
                | Self::KwFn { acceptable, .. }
                | Self::KwType { acceptable, .. }
                | Self::KwTag { acceptable, .. }
                | Self::KwMain { acceptable, .. }
                | Self::KwMacro { acceptable, .. }
                | Self::KwImport { acceptable, .. }
                | Self::KwObject { acceptable, .. }
                | Self::IdentVal { acceptable }
                | Self::IdentTy { acceptable }
                | Self::IdentMacro { acceptable, .. }
                | Self::IdentSty { acceptable, .. }
                | Self::OpDecl { acceptable, .. }
                | Self::OpEq { acceptable }
                | Self::OpPlus { acceptable }
                | Self::OpMinus { acceptable }
                | Self::OpStar { acceptable }
                | Self::OpSlash { acceptable }
                | Self::OpCaret { acceptable }
                | Self::OpUScore { acceptable }
                | Self::OpPercent { acceptable }
                | Self::OpAnd { acceptable }
                | Self::OpAndAnd { acceptable, .. }
                | Self::OpOr { acceptable }
                | Self::OpOrOr { acceptable, .. }
                | Self::OpNot { acceptable }
                | Self::OpNotEq { acceptable, .. }
                | Self::OpEqEq { acceptable, .. }
                | Self::OpGt { acceptable }
                | Self::OpGtEq { acceptable, .. }
                | Self::OpLt { acceptable }
                | Self::OpLtEq { acceptable, .. }
                | Self::OpLtLt { acceptable, .. }
                | Self::OpGtGt { acceptable, .. }
                | Self::OpRArw { acceptable, .. }
                | Self::OpFatRArw { acceptable, .. }
                | Self::OpTilde { acceptable }
                | Self::OpJoin { acceptable, .. }
                | Self::OpBSlash { acceptable }
                | Self::OpRange { acceptable, .. }
                | Self::OpCRange { acceptable, .. }
                | Self::OpSpread { acceptable, .. }
                | Self::DelimOParen { acceptable }
                | Self::DelimCParen { acceptable }
                | Self::DelimOBrack { acceptable }
                | Self::DelimCBrack { acceptable }
                | Self::DelimOBrace { acceptable }
                | Self::DelimCBrace { acceptable } => *acceptable,
                Self::IdentTag { state } => state.acceptable(),
                _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn lex_fn() {
        let state = LexemeState::KwFn { stage: 0, acceptable: false };
        let state = state.accept('f');
        assert_eq!(state, Some(LexemeState::KwFn { stage: 1, acceptable: false }));
        let state = state.unwrap().accept('n');
        assert_eq!(state, Some(LexemeState::KwFn { stage: 2, acceptable: true }));
        let state = state.unwrap().accept(' ');
        assert_eq!(state, None);
    }

    #[test]
    fn lex_tag() {
        let state = LexemeState::IdentTag { state: IdentTagState::Unset };
        let state = state.accept('#');
        assert_eq!(state, Some(LexemeState::IdentTag { state: IdentTagState::WaitingAlpha }));
        let state = state.unwrap().accept('a');
        assert_eq!(state, Some(LexemeState::IdentTag { state: IdentTagState::WaitingAlphaNumDash }));
        let state = state.unwrap().accept('g');
        assert_eq!(state, Some(LexemeState::IdentTag { state: IdentTagState::WaitingAlphaNumDash }));
        let state = state.unwrap().accept('-');
        assert_eq!(state, Some(LexemeState::IdentTag { state: IdentTagState::WaitingAlpha }));
        let state = state.unwrap().accept(' ');
        assert_eq!(state, None);
    }
}