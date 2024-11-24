use super::Accepter;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KwAccepter {
    Val(ValAccepter),
    Fn(FnAccepter),
    Type(TypeAccepter),
    Tag(TagAccepter),
    Main(MainAccepter),
    Macro(MacroAccepter),
    Import(ImportAccepter),
    Object(ObjectAccepter),
}

impl Accepter for KwAccepter {
    type State = Self;

    fn acceptable(&self) -> bool {
        match self {
            Self::Val(acp) => acp.acceptable(),
            Self::Fn(acp) => acp.acceptable(),
            Self::Type(acp) => acp.acceptable(),
            Self::Tag(acp) => acp.acceptable(),
            Self::Main(acp) => acp.acceptable(),
            Self::Macro(acp) => acp.acceptable(),
            Self::Import(acp) => acp.acceptable(),
            Self::Object(acp) => acp.acceptable(),
        }
    }

    fn accept(self, c: char) -> Option<Self::State> {
        match self {
            Self::Val(acp) => acp.accept(c).map(Self::Val),
            Self::Fn(acp) => acp.accept(c).map(Self::Fn),
            Self::Type(acp) => acp.accept(c).map(Self::Type),
            Self::Tag(acp) => acp.accept(c).map(Self::Tag),
            Self::Main(acp) => acp.accept(c).map(Self::Main),
            Self::Macro(acp) => acp.accept(c).map(Self::Macro),
            Self::Import(acp) => acp.accept(c).map(Self::Import),
            Self::Object(acp) => acp.accept(c).map(Self::Object),
        }
    }
}
impl KwAccepter {
    pub fn stream() -> Vec<Self> {
        use KwAccepter::*;
        vec![
            Val(ValAccepter::default()),
            Fn(FnAccepter::default()),
            Type(TypeAccepter::default()),
            Tag(TagAccepter::default()),
            Main(MainAccepter::default()),
            Macro(MacroAccepter::default()),
            Import(ImportAccepter::default()),
            Object(ObjectAccepter::default()),
        ]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ValAccepter {
    #[default]
    Unset,
    V,
    Va,
    Val,
}

impl Accepter for ValAccepter {
    type State = Self;

    fn acceptable(&self) -> bool {
        *self == Self::Val
    }

    fn accept(self, c: char) -> Option<Self> {
        match self {
            Self::Unset if c == 'v' => Some(Self::V),
            Self::V if c == 'a' => Some(Self::Va),
            Self::Va if c == 'l' => Some(Self::Val),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum FnAccepter {
    #[default]
    Unset,
    F,
    Fn,
}

impl Accepter for FnAccepter {
    type State = Self;

    fn acceptable(&self) -> bool {
        *self == Self::Fn
    }

    fn accept(self, c: char) -> Option<Self> {
        match self {
            Self::Unset if c == 'f' => Some(Self::F),
            Self::F if c == 'n' => Some(Self::Fn),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TypeAccepter {
    #[default]
    Unset,
    T,
    Ty,
    Typ,
    Type,
}

impl Accepter for TypeAccepter {
    type State = Self;

    fn acceptable(&self) -> bool {
        *self == Self::Type
    }

    fn accept(self, c: char) -> Option<Self> {
        match self {
            Self::Unset if c == 't' => Some(Self::T),
            Self::T if c == 'y' => Some(Self::Ty),
            Self::Ty if c == 'p' => Some(Self::Typ),
            Self::Typ if c == 'e' => Some(Self::Type),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TagAccepter {
    #[default]
    Unset,
    T,
    Ta,
    Tag,
}

impl Accepter for TagAccepter {
    type State = Self;

    fn acceptable(&self) -> bool {
        *self == Self::Tag
    }

    fn accept(self, c: char) -> Option<Self> {
        match self {
            Self::Unset if c == 't' => Some(Self::T),
            Self::T if c == 'a' => Some(Self::Ta),
            Self::Ta if c == 'g' => Some(Self::Tag),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MainAccepter {
    #[default]
    Unset,
    M,
    Ma,
    Mai,
    Main,
}

impl Accepter for MainAccepter {
    type State = Self;

    fn acceptable(&self) -> bool {
        *self == Self::Main
    }

    fn accept(self, c: char) -> Option<Self> {
        match self {
            Self::Unset if c == 'm' => Some(Self::M),
            Self::M if c == 'a' => Some(Self::Ma),
            Self::Ma if c == 'i' => Some(Self::Mai),
            Self::Mai if c == 'n' => Some(Self::Main),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MacroAccepter {
    #[default]
    Unset,
    M,
    Ma,
    Mac,
    Macr,
    Macro,
}

impl Accepter for MacroAccepter {
    type State = Self;

    fn acceptable(&self) -> bool {
        *self == Self::Macro
    }

    fn accept(self, c: char) -> Option<Self> {
        match self {
            Self::Unset if c == 'm' => Some(Self::M),
            Self::M if c == 'a' => Some(Self::Ma),
            Self::Ma if c == 'c' => Some(Self::Mac),
            Self::Mac if c == 'r' => Some(Self::Macr),
            Self::Macr if c == 'o' => Some(Self::Macro),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ImportAccepter {
    #[default]
    Unset,
    I,
    Im,
    Imp,
    Impo,
    Impor,
    Import,
}

impl Accepter for ImportAccepter {
    type State = Self;

    fn acceptable(&self) -> bool {
        *self == Self::Import
    }

    fn accept(self, c: char) -> Option<Self> {
        match self {
            Self::Unset if c == 'i' => Some(Self::I),
            Self::I if c == 'm' => Some(Self::Im),
            Self::Im if c == 'p' => Some(Self::Imp),
            Self::Imp if c == 'o' => Some(Self::Impo),
            Self::Impo if c == 'r' => Some(Self::Impor),
            Self::Impor if c == 't' => Some(Self::Import),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ObjectAccepter {
    #[default]
    Unset,
    O,
    Ob,
    Obj,
    Obje,
    Objec,
    Object,
}

impl Accepter for ObjectAccepter {
    type State = Self;

    fn acceptable(&self) -> bool {
        *self == Self::Object
    }

    fn accept(self, c: char) -> Option<Self> {
        match self {
            Self::Unset if c == 'o' => Some(Self::O),
            Self::O if c == 'b' => Some(Self::Ob),
            Self::Ob if c == 'j' => Some(Self::Obj),
            Self::Obj if c == 'e' => Some(Self::Obje),
            Self::Obje if c == 'c' => Some(Self::Objec),
            Self::Objec if c == 't' => Some(Self::Object),
            _ => None,
        }
    }
}