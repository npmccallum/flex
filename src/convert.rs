use core::borrow::Borrow;

use super::Flex;

#[cfg(feature = "alloc")]
use alloc::{
    borrow::{Cow, ToOwned},
    boxed::Box,
};

impl<'a, T: ?Sized> Borrow<T> for Flex<'a, T> {
    fn borrow(&self) -> &T {
        self
    }
}

impl<'a, T: ?Sized + AsRef<U>, U: ?Sized> AsRef<U> for Flex<'a, T> {
    fn as_ref(&self) -> &U {
        match self {
            Flex::Lend(r) => r.as_ref(),

            #[cfg(feature = "alloc")]
            Flex::Give(b) => (**b).as_ref(),
        }
    }
}

impl<'a, T: ?Sized> From<&'a T> for Flex<'a, T> {
    fn from(r: &'a T) -> Self {
        Flex::Lend(r)
    }
}

impl<'a, T: ?Sized> From<&'a mut T> for Flex<'a, T> {
    fn from(r: &'a mut T) -> Self {
        Flex::Lend(r)
    }
}

#[cfg(feature = "alloc")]
#[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
impl<T: ?Sized> From<Box<T>> for Flex<'_, T> {
    fn from(b: Box<T>) -> Self {
        Flex::Give(b)
    }
}

#[cfg(feature = "alloc")]
#[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
impl<'a, T: ?Sized + ToOwned> From<Cow<'a, T>> for Flex<'a, T>
where
    T::Owned: Into<Box<T>>,
{
    fn from(c: Cow<'a, T>) -> Self {
        match c {
            Cow::Borrowed(r) => Flex::Lend(r),
            Cow::Owned(b) => Flex::Give(b.into()),
        }
    }
}

#[cfg(feature = "alloc")]
#[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
impl<'a, T: ?Sized + ToOwned> From<Flex<'a, T>> for Cow<'a, T>
where
    T::Owned: From<Box<T>>,
{
    fn from(c: Flex<'a, T>) -> Self {
        match c {
            Flex::Lend(r) => Cow::Borrowed(r),
            Flex::Give(b) => Cow::Owned(b.into()),
        }
    }
}
