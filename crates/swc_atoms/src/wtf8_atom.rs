use std::{
    fmt::{self, Formatter},
    ops::Deref,
};

use hstr::wtf8::{Wtf8, Wtf8Buf};
use rkyv::{vec::ArchivedVec, Archived, Deserialize, DeserializeUnsized};
use serde::Serializer;

/// Clone-on-write WTF-8 string.
///
///
/// See [tendril] for more details.
#[derive(Clone, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "rkyv-impl", derive(bytecheck::CheckBytes))]
#[cfg_attr(feature = "rkyv-impl", repr(C))]
pub struct Wtf8Atom(pub(super) hstr::Wtf8Atom);

#[cfg(feature = "arbitrary")]
#[cfg_attr(docsrs, doc(cfg(feature = "arbitrary")))]
impl<'a> arbitrary::Arbitrary<'a> for Wtf8Atom {
    fn arbitrary(u: &mut arbitrary::Unstructured<'_>) -> arbitrary::Result<Self> {
        let sym = u.arbitrary::<Vec<u8>>()?;
        if sym.is_empty() {
            return Err(arbitrary::Error::NotEnoughData);
        }
        Ok(Self(hstr::Wtf8Atom::from(sym)))
    }
}

fn _asserts() {
    // let _static_assert_size_eq = std::mem::transmute::<Atom, [usize; 1]>;

    fn _assert_send<T: Send>() {}
    fn _assert_sync<T: Sync>() {}

    _assert_sync::<Wtf8Atom>();
    _assert_send::<Wtf8Atom>();
}

impl Wtf8Atom {
    /// Creates a new [Wtf8Atom] from a string.
    #[inline(always)]
    pub fn new<S>(s: S) -> Self
    where
        hstr::Wtf8Atom: From<S>,
    {
        Wtf8Atom(hstr::Wtf8Atom::from(s))
    }

    pub fn as_wtf8(&self) -> &Wtf8 {
        &self.0
    }
}

impl Deref for Wtf8Atom {
    type Target = Wtf8;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl fmt::Debug for Wtf8Atom {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&**self, f)
    }
}

// impl Display for Wtf8Atom {
//     fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
//         Display::fmt(&**self, f)
//     }
// }

impl PartialOrd for Wtf8Atom {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Wtf8Atom {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.as_wtf8().cmp(other.as_wtf8())
    }
}

impl From<Wtf8Atom> for hstr::Wtf8Atom {
    fn from(s: Wtf8Atom) -> Self {
        s.0
    }
}

impl serde::ser::Serialize for Wtf8Atom {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_bytes(self.as_bytes())
    }
}

impl<'de> serde::de::Deserialize<'de> for Wtf8Atom {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer).map(Self::new)
    }
}

/// NOT A PUBLIC API
#[cfg(feature = "rkyv-impl")]
impl rkyv::Archive for Wtf8Atom {
    type Archived = rkyv::vec::ArchivedVec<u8>;
    type Resolver = rkyv::vec::VecResolver;

    #[allow(clippy::unit_arg)]
    fn resolve(&self, resolver: Self::Resolver, out: rkyv::Place<Self::Archived>) {
        rkyv::vec::ArchivedVec::<Archived<u8>>::resolve_from_slice(self.as_bytes(), resolver, out)
    }
}

/// NOT A PUBLIC API
#[cfg(feature = "rkyv-impl")]
impl<S: rancor::Fallible + rkyv::ser::Writer + rkyv::ser::Allocator + ?Sized> rkyv::Serialize<S>
    for Wtf8Atom
where
    <S as rancor::Fallible>::Error: rancor::Source,
{
    fn serialize(&self, serializer: &mut S) -> Result<Self::Resolver, S::Error> {
        rkyv::vec::ArchivedVec::<u8>::serialize_from_slice(self.as_bytes(), serializer)
    }
}

/// NOT A PUBLIC API
#[cfg(feature = "rkyv-impl")]
impl<D> rkyv::Deserialize<Wtf8Atom, D> for rkyv::vec::ArchivedVec<u8>
where
    D: ?Sized + rancor::Fallible,
    <D as rancor::Fallible>::Error: rancor::Source,
{
    fn deserialize(
        &self,
        deserializer: &mut D,
    ) -> Result<Wtf8Atom, <D as rancor::Fallible>::Error> {
        let s: Vec<u8> = self.deserialize(deserializer)?;

        Ok(Wtf8Atom::new(s))
    }
}

/// noop
#[cfg(feature = "shrink-to-fit")]
impl shrink_to_fit::ShrinkToFit for Wtf8Atom {
    #[inline(always)]
    fn shrink_to_fit(&mut self) {}
}
