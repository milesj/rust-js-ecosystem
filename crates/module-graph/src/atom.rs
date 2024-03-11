// Copied from rolldown: https://github.com/rolldown/rolldown/tree/main/crates/rolldown_AtomStr

use oxc::span::{Atom, CompactString};
use std::{fmt, ops::Deref};

#[derive(Debug, Clone, Hash, PartialEq, Eq, Default)]
pub struct AtomStr(CompactString);

impl AtomStr {
    pub fn from(value: &str) -> Self {
        Self(CompactString::from(value))
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }

    pub fn to_oxc_atom(&self) -> Atom<'static> {
        Atom::Compact(self.0.clone())
    }
}

impl Deref for AtomStr {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.as_str()
    }
}

impl fmt::Display for AtomStr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.as_str().fmt(f)
    }
}

pub trait ToAtomStr {
    fn to_atom_str(&self) -> AtomStr;
}

impl ToAtomStr for Atom<'_> {
    fn to_atom_str(&self) -> AtomStr {
        match self {
            Atom::Arena(s) => AtomStr((*s).to_string().into()),
            Atom::Compact(s) => AtomStr(s.clone()),
        }
    }
}

impl ToAtomStr for CompactString {
    fn to_atom_str(&self) -> AtomStr {
        AtomStr(self.clone())
    }
}

impl From<&str> for AtomStr {
    fn from(s: &str) -> Self {
        Self(s.into())
    }
}

impl From<String> for AtomStr {
    fn from(s: String) -> Self {
        Self(s.into())
    }
}
