#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![no_std]

#[cfg(feature = "alloc")]
extern crate alloc;

mod convert;
mod derive;
mod fmt;

use core::ops::Index;

#[cfg(feature = "alloc")]
use alloc::boxed::Box;

/// A flexible container that can hold either a borrowed reference or an
/// owned boxed value.
///
/// `Flex` provides a unified interface for working with both borrowed
/// (`&'a T`) and owned (`Box<T>`) data of the same type. This is
/// particularly useful for unsized types where the owned representation
/// is naturally `Box<T>` rather than a separate container type.
///
/// # Variants
///
/// - [`Lend`](Flex::Lend): Holds a borrowed reference `&'a T`
/// - [`Give`](Flex::Give): Holds an owned `Box<T>` (requires `alloc`
///   feature)
///
/// # Comparison with Other Types
///
/// Unlike [`Cow`](std::borrow::Cow), which works with type pairs like
/// `str`/`String` or `[T]`/`Vec<T>`, `Flex` works with a single type in
/// two ownership models. This makes it ideal for trait objects and other
/// unsized types where there isn't a natural "owned" container type.
///
/// # Examples
///
/// ```
/// use flex::Flex;
/// use std::fmt::Debug;
///
/// // Works with trait objects
/// fn print_debug(value: Flex<dyn Debug>) {
///     println!("{:?}", value);
/// }
///
/// let borrowed: Flex<dyn Debug> = Flex::from(&42 as &dyn Debug);
/// print_debug(borrowed);
///
/// # #[cfg(feature = "alloc")] {
/// let owned: Flex<dyn Debug> = Flex::from(Box::new("hello") as Box<dyn Debug>);
/// print_debug(owned);
/// # }
/// ```
///
/// ```
/// use flex::Flex;
///
/// // Works with string slices
/// let s = String::from("hello");
/// let borrowed = Flex::from(s.as_str());
/// assert_eq!(&*borrowed, "hello");
/// ```
#[derive(Debug)]
pub enum Flex<'a, T: ?Sized> {
    /// A borrowed reference to data with lifetime `'a`.
    Lend(&'a T),

    /// An owned, heap-allocated value.
    ///
    /// Only available with the `alloc` feature.
    #[cfg(feature = "alloc")]
    #[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
    Give(Box<T>),
}

impl<'a, T: ?Sized> Default for Flex<'a, T>
where
    &'a T: Default,
{
    fn default() -> Self {
        Flex::Lend(Default::default())
    }
}

impl<'a, T: ?Sized + Index<I>, I> Index<I> for Flex<'a, T> {
    type Output = T::Output;

    fn index(&self, index: I) -> &Self::Output {
        Index::index(&**self, index)
    }
}

impl<'a, T: ?Sized> IntoIterator for &'a Flex<'a, T>
where
    &'a T: IntoIterator,
{
    type Item = <&'a T as IntoIterator>::Item;
    type IntoIter = <&'a T as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        (**self).into_iter()
    }
}

#[cfg(feature = "alloc")]
#[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
impl<'a, T: ?Sized> Flex<'a, T>
where
    Box<T>: From<&'a T>,
{
    /// Converts the `Flex` into a `Box<T>`, consuming the `Flex`.
    ///
    /// For `Lend` variants, this allocates a new `Box<T>` from the borrowed reference.
    /// For `Give` variants, this simply returns the owned `Box<T>`.
    pub fn into_box(self) -> Box<T> {
        match self {
            Flex::Lend(r) => Box::from(r),
            Flex::Give(b) => b,
        }
    }

    /// Claims ownership of the data, converting borrowed data to owned.
    ///
    /// For `Lend` variants, this converts the borrowed data into an owned
    /// `Box<T>` by using `Box<T>::from(&T)`. This typically involves
    /// cloning or allocating the data.
    ///
    /// For `Give` variants, this is a no-op that simply changes the
    /// lifetime bound, as the data is already owned.
    ///
    /// # Examples
    ///
    /// ```
    /// use flex::Flex;
    ///
    /// let borrowed = Flex::from(&[1, 2, 3][..]);
    /// let owned: Flex<'static, [i32]> = borrowed.claim();
    /// assert_eq!(&*owned, &[1, 2, 3]);
    /// ```
    ///
    /// ```
    /// use flex::Flex;
    ///
    /// let s = "hello";
    /// let borrowed = Flex::from(s);
    /// let owned: Flex<'static, str> = borrowed.claim();
    /// assert_eq!(&*owned, "hello");
    /// ```
    pub fn claim<'b>(self) -> Flex<'b, T> {
        Flex::Give(self.into_box())
    }
}
