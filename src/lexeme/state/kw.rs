#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KwState {
    Val(ValState),
    Fn(FnState),
    Type(TypeState),
    Tag(TagState),
    Main(MainState),
    Macro(MacroState),
    Import(ImportState),
    Object(ObjectState),
}

impl KwState {
    pub fn acceptable(&self) -> bool {
        match self {
            Self::Val(state) => state.acceptable(),
            Self::Fn(state) => state.acceptable(),
            Self::Type(state) => state.acceptable(),
            Self::Tag(state) => state.acceptable(),
            Self::Main(state) => state.acceptable(),
            Self::Macro(state) => state.acceptable(),
            Self::Import(state) => state.acceptable(),
            Self::Object(state) => state.acceptable(),
        }
    }

    pub fn accept(self, c: char) -> Option<Self> {
        match self {
            Self::Val(state) => state.accept(c).map(Self::Val),
            Self::Fn(state) => state.accept(c).map(Self::Fn),
            Self::Type(state) => state.accept(c).map(Self::Type),
            Self::Tag(state) => state.accept(c).map(Self::Tag),
            Self::Main(state) => state.accept(c).map(Self::Main),
            Self::Macro(state) => state.accept(c).map(Self::Macro),
            Self::Import(state) => state.accept(c).map(Self::Import),
            Self::Object(state) => state.accept(c).map(Self::Object),
        }
    }

    pub fn stream() -> Vec<Self> {
        use KwState::*;
        vec![
            Val(ValState::default()),
            Fn(FnState::default()),
            Type(TypeState::default()),
            Tag(TagState::default()),
            Main(MainState::default()),
            Macro(MacroState::default()),
            Import(ImportState::default()),
            Object(ObjectState::default()),
        ]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ValState {
    #[default]
    Unset,
    V,
    Va,
    Val,
}

impl ValState {
    pub fn acceptable(&self) -> bool {
        *self == Self::Val
    }

    pub fn accept(self, c: char) -> Option<Self> {
        match self {
            Self::Unset if c == 'v' => Some(Self::V),
            Self::V if c == 'a' => Some(Self::Va),
            Self::Va if c == 'l' => Some(Self::Val),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum FnState {
    #[default]
    Unset,
    F,
    Fn,
}

impl FnState {
    pub fn acceptable(&self) -> bool {
        *self == Self::Fn
    }

    pub fn accept(self, c: char) -> Option<Self> {
        match self {
            Self::Unset if c == 'f' => Some(Self::F),
            Self::F if c == 'n' => Some(Self::Fn),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TypeState {
    #[default]
    Unset,
    T,
    Ty,
    Typ,
    Type,
}

impl TypeState {
    pub fn acceptable(&self) -> bool {
        *self == Self::Type
    }

    pub fn accept(self, c: char) -> Option<Self> {
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
pub enum TagState {
    #[default]
    Unset,
    T,
    Ta,
    Tag,
}

impl TagState {
    pub fn acceptable(&self) -> bool {
        *self == Self::Tag
    }

    pub fn accept(self, c: char) -> Option<Self> {
        match self {
            Self::Unset if c == 't' => Some(Self::T),
            Self::T if c == 'a' => Some(Self::Ta),
            Self::Ta if c == 'g' => Some(Self::Tag),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MainState {
    #[default]
    Unset,
    M,
    Ma,
    Mai,
    Main,
}

impl MainState {
    pub fn acceptable(&self) -> bool {
        *self == Self::Main
    }

    pub fn accept(self, c: char) -> Option<Self> {
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
pub enum MacroState {
    #[default]
    Unset,
    M,
    Ma,
    Mac,
    Macr,
    Macro,
}

impl MacroState {
    pub fn acceptable(&self) -> bool {
        *self == Self::Macro
    }

    pub fn accept(self, c: char) -> Option<Self> {
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
pub enum ImportState {
    #[default]
    Unset,
    I,
    Im,
    Imp,
    Impo,
    Impor,
    Import,
}

impl ImportState {
    pub fn acceptable(&self) -> bool {
        *self == Self::Import
    }

    pub fn accept(self, c: char) -> Option<Self> {
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
pub enum ObjectState {
    #[default]
    Unset,
    O,
    Ob,
    Obj,
    Obje,
    Objec,
    Object,
}

impl ObjectState {
    pub fn acceptable(&self) -> bool {
        *self == Self::Object
    }

    pub fn accept(self, c: char) -> Option<Self> {
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