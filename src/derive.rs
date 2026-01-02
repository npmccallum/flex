//! Derive implementations for `Flex`.
//!
//! This module provides implementations of common traits such as `Clone`,
//! `Eq`, `PartialEq`, `Ord`, `PartialOrd`, and `Hash` for the `Flex` type.
//! In normal situations, these traits would be derived. But we need to proxy
//! all the impls through `.derive()`.

use core::hash::{Hash, Hasher};
use core::ops::Deref;

use super::Flex;

#[cfg(feature = "alloc")]
use alloc::boxed::Box;

impl<'a, T: ?Sized> Deref for Flex<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        match self {
            Flex::Lend(r) => r,

            #[cfg(feature = "alloc")]
            Flex::Give(b) => b.deref(),
        }
    }
}

#[cfg(not(feature = "alloc"))]
impl<'a, T: ?Sized> Clone for Flex<'a, T> {
    fn clone(&self) -> Self {
        match self {
            Flex::Lend(r) => Flex::Lend(*r),
        }
    }
}

#[cfg(feature = "alloc")]
impl<'a, T: ?Sized> Clone for Flex<'a, T>
where
    Box<T>: Clone,
{
    fn clone(&self) -> Self {
        match self {
            Flex::Lend(r) => Flex::Lend(*r),
            Flex::Give(b) => Flex::Give(b.clone()),
        }
    }
}

impl<'a, T: ?Sized + Eq> Eq for Flex<'a, T> {}

impl<'a, T: ?Sized + PartialEq> PartialEq for Flex<'a, T> {
    fn eq(&self, other: &Self) -> bool {
        self.deref().eq(other.deref())
    }
}

impl<'a, T: ?Sized + PartialEq> PartialEq<T> for Flex<'a, T> {
    fn eq(&self, other: &T) -> bool {
        self.deref() == other
    }
}

impl<'a, T: ?Sized + PartialEq> PartialEq<&T> for Flex<'a, T> {
    fn eq(&self, other: &&T) -> bool {
        self.deref() == *other
    }
}

#[cfg(feature = "alloc")]
#[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
impl<'a, T: ?Sized + PartialEq> PartialEq<Box<T>> for Flex<'a, T> {
    fn eq(&self, other: &Box<T>) -> bool {
        self.deref() == other.deref()
    }
}

impl<'a, T: ?Sized + Ord> Ord for Flex<'a, T> {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.deref().cmp(other.deref())
    }
}

impl<'a, T: ?Sized + PartialOrd> PartialOrd for Flex<'a, T> {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        self.deref().partial_cmp(other.deref())
    }
}

impl<'a, T: ?Sized + PartialOrd> PartialOrd<T> for Flex<'a, T> {
    fn partial_cmp(&self, other: &T) -> Option<core::cmp::Ordering> {
        self.deref().partial_cmp(other)
    }
}

impl<'a, T: ?Sized + PartialOrd> PartialOrd<&T> for Flex<'a, T> {
    fn partial_cmp(&self, other: &&T) -> Option<core::cmp::Ordering> {
        self.deref().partial_cmp(*other)
    }
}

#[cfg(feature = "alloc")]
#[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
impl<'a, T: ?Sized + PartialOrd> PartialOrd<Box<T>> for Flex<'a, T> {
    fn partial_cmp(&self, other: &Box<T>) -> Option<core::cmp::Ordering> {
        self.deref().partial_cmp(other.deref())
    }
}

impl<'a, T: ?Sized + Hash> Hash for Flex<'a, T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.deref().hash(state)
    }
}
