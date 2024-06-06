/// An abstract CSV token type.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Token {
    pub idx: usize,
    pub kind: Kind,
}

/// An abstract CSV token type.
#[derive(Clone, Debug, Eq, PartialEq, strum::EnumIs, strum::EnumTryAs)]
pub enum Kind {
    Field(String),
    Delimiter,
    LineBreak,
}

impl Token {
    #[inline]
    #[must_use]
    pub const fn new(idx: usize, kind: Kind) -> Self {
        Self { idx, kind }
    }

    #[inline]
    #[must_use]
    pub const fn idx(&self) -> usize {
        self.idx
    }

    #[inline]
    pub fn idx_mut(&mut self) -> &mut usize {
        &mut self.idx
    }

    #[inline]
    pub fn set_idx(&mut self, idx: usize) {
        self.idx = idx;
    }

    #[inline]
    #[must_use]
    pub const fn kind(&self) -> &Kind {
        &self.kind
    }

    #[inline]
    pub fn kind_mut(&mut self) -> &mut Kind {
        &mut self.kind
    }

    #[inline]
    pub fn set_kind(&mut self, kind: Kind) {
        self.kind = kind;
    }

    #[inline]
    #[must_use]
    pub const fn is_field(&self) -> bool {
        self.kind.is_field()
    }

    #[inline]
    #[must_use]
    pub const fn is_delimiter(&self) -> bool {
        self.kind.is_delimiter()
    }

    #[inline]
    #[must_use]
    pub const fn is_line_break(&self) -> bool {
        self.kind.is_line_break()
    }
}
