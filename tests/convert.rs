use flex::Flex;

// Borrow trait tests
mod borrow {
    use super::*;
    use std::borrow::Borrow;

    #[test]
    fn lend_slice() {
        let flex = Flex::Lend(&[1, 2, 3][..]);
        let borrowed: &[i32] = flex.borrow();
        assert_eq!(borrowed, &[1, 2, 3]);
    }

    #[test]
    fn lend_str() {
        let flex = Flex::Lend("hello");
        let borrowed: &str = flex.borrow();
        assert_eq!(borrowed, "hello");
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn give_slice() {
        let flex = Flex::Give(vec![1, 2, 3].into_boxed_slice());
        let borrowed: &[i32] = flex.borrow();
        assert_eq!(borrowed, &[1, 2, 3]);
    }
}

// AsRef trait tests
mod as_ref {
    use super::*;

    #[test]
    fn lend_slice() {
        let data = [1, 2, 3];
        let flex = Flex::Lend(&data[..]);
        let slice: &[i32] = flex.as_ref();
        assert_eq!(slice, &[1, 2, 3]);
    }

    #[test]
    fn in_generic_function() {
        fn take_as_ref<T: AsRef<[i32]>>(value: T) -> usize {
            value.as_ref().len()
        }

        let data = [1, 2, 3];
        let flex = Flex::Lend(&data[..]);
        assert_eq!(take_as_ref(&*flex), 3);
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn give_as_ref() {
        // Vec<T> is Sized and implements AsRef<[T]>
        let flex: Flex<Vec<i32>> = Flex::Give(Box::new(vec![1, 2, 3]));
        let slice: &[i32] = flex.as_ref();
        assert_eq!(slice, &[1, 2, 3]);
    }
}

// From trait tests
mod from {
    use super::*;

    #[test]
    fn from_ref() {
        let data = [1, 2, 3];
        let flex = Flex::from(&data[..]);
        assert_eq!(&*flex, &[1, 2, 3]);
    }

    #[test]
    fn from_mut_ref() {
        let mut data = [1, 2, 3];
        let flex = Flex::from(&mut data[..]);
        assert_eq!(&*flex, &[1, 2, 3]);
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn from_box() {
        let boxed: Box<[i32]> = vec![1, 2, 3].into_boxed_slice();
        let flex = Flex::from(boxed);
        assert_eq!(&*flex, &[1, 2, 3]);
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn from_cow_borrowed() {
        use std::borrow::Cow;
        let cow: Cow<[i32]> = Cow::Borrowed(&[1, 2, 3]);
        let flex = Flex::from(cow);
        assert_eq!(&*flex, &[1, 2, 3]);
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn from_cow_owned() {
        use std::borrow::Cow;
        let cow: Cow<[i32]> = Cow::Owned(vec![1, 2, 3]);
        let flex = Flex::from(cow);
        assert_eq!(&*flex, &[1, 2, 3]);
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn into_cow_from_lend() {
        use std::borrow::Cow;
        let flex = Flex::Lend(&[1, 2, 3][..]);
        let cow: Cow<[i32]> = flex.into();
        assert!(matches!(cow, Cow::Borrowed(_)));
        assert_eq!(&*cow, &[1, 2, 3]);
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn into_cow_from_give() {
        use std::borrow::Cow;
        let flex = Flex::Give(vec![1, 2, 3].into_boxed_slice());
        let cow: Cow<[i32]> = flex.into();
        assert!(matches!(cow, Cow::Owned(_)));
        assert_eq!(&*cow, &[1, 2, 3]);
    }
}
