use super::Flex;

use core::fmt::*;
use core::ops::Deref;

impl<'a, T: ?Sized + Display> Display for Flex<'a, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        self.deref().fmt(f)
    }
}

impl<'a, T: ?Sized + Binary> Binary for Flex<'a, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        self.deref().fmt(f)
    }
}

impl<'a, T: ?Sized + Octal> Octal for Flex<'a, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        self.deref().fmt(f)
    }
}

impl<'a, T: ?Sized + LowerHex> LowerHex for Flex<'a, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        self.deref().fmt(f)
    }
}

impl<'a, T: ?Sized + UpperHex> UpperHex for Flex<'a, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        self.deref().fmt(f)
    }
}

impl<'a, T: ?Sized + LowerExp> LowerExp for Flex<'a, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        self.deref().fmt(f)
    }
}

impl<'a, T: ?Sized + UpperExp> UpperExp for Flex<'a, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        self.deref().fmt(f)
    }
}

impl<'a, T: ?Sized> Pointer for Flex<'a, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        self.deref().fmt(f)
    }
}
