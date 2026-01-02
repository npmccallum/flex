use flex::Flex;

// Construction and variant tests
mod construction {
    use super::*;

    #[test]
    fn lend_from_str() {
        let flex = Flex::Lend("hello");
        assert_eq!(&*flex, "hello");
    }

    #[test]
    fn lend_from_slice() {
        let data = [1, 2, 3];
        let flex = Flex::Lend(&data[..]);
        assert_eq!(&*flex, &[1, 2, 3]);
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn give_from_box() {
        let flex = Flex::Give(Box::new(42));
        assert_eq!(*flex, 42);
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn give_from_box_slice() {
        let flex = Flex::Give(vec![1, 2, 3].into_boxed_slice());
        assert_eq!(&*flex, &[1, 2, 3]);
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn give_from_box_str() {
        let flex = Flex::Give(String::from("hello").into_boxed_str());
        assert_eq!(&*flex, "hello");
    }
}

// Default trait tests
mod default {
    use super::*;

    #[test]
    fn default_slice() {
        let flex: Flex<[i32]> = Flex::default();
        assert_eq!(&*flex, &[]);
    }

    #[test]
    fn default_str() {
        let flex: Flex<str> = Flex::default();
        assert_eq!(&*flex, "");
    }
}

// Index trait tests
mod index {
    use super::*;

    #[test]
    fn slice_element() {
        let data = [1, 2, 3, 4, 5];
        let flex = Flex::Lend(&data[..]);
        assert_eq!(flex[0], 1);
        assert_eq!(flex[4], 5);
    }

    #[test]
    fn slice_range() {
        let data = [1, 2, 3, 4, 5];
        let flex = Flex::Lend(&data[..]);
        assert_eq!(&flex[1..4], &[2, 3, 4]);
        assert_eq!(&flex[..3], &[1, 2, 3]);
        assert_eq!(&flex[3..], &[4, 5]);
    }

    #[test]
    fn str_range() {
        let flex = Flex::Lend("hello");
        assert_eq!(&flex[0..1], "h");
        assert_eq!(&flex[1..4], "ell");
        assert_eq!(&flex[..], "hello");
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn give_slice_element() {
        let flex = Flex::Give(vec![1, 2, 3, 4, 5].into_boxed_slice());
        assert_eq!(flex[2], 3);
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn give_str_range() {
        let flex = Flex::Give(String::from("hello").into_boxed_str());
        assert_eq!(&flex[1..4], "ell");
    }
}

// IntoIterator trait tests
mod into_iterator {
    use super::*;

    #[test]
    fn lend_collect() {
        let flex = Flex::Lend(&[1, 2, 3][..]);
        let collected: Vec<&i32> = (&flex).into_iter().collect();
        assert_eq!(collected, vec![&1, &2, &3]);
    }

    #[test]
    fn lend_for_loop() {
        let flex = Flex::Lend(&[1, 2, 3][..]);
        let mut sum = 0;
        for &value in &flex {
            sum += value;
        }
        assert_eq!(sum, 6);
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn give_collect() {
        let flex = Flex::Give(vec![1, 2, 3].into_boxed_slice());
        let collected: Vec<&i32> = (&flex).into_iter().collect();
        assert_eq!(collected, vec![&1, &2, &3]);
    }
}

// Alloc-only methods
#[cfg(feature = "alloc")]
mod alloc_methods {
    use super::*;

    #[test]
    fn into_box_from_lend() {
        let flex = Flex::Lend(&[1, 2, 3][..]);
        let boxed = flex.into_box();
        assert_eq!(&*boxed, &[1, 2, 3]);
    }

    #[test]
    fn into_box_from_give() {
        let flex = Flex::Give(vec![1, 2, 3].into_boxed_slice());
        let boxed = flex.into_box();
        assert_eq!(&*boxed, &[1, 2, 3]);
    }

    #[test]
    fn claim_lend_to_static() {
        let borrowed = Flex::Lend(&[1, 2, 3][..]);
        let owned: Flex<'static, [i32]> = borrowed.claim();
        assert_eq!(&*owned, &[1, 2, 3]);
    }

    #[test]
    fn claim_give_to_static() {
        let flex = Flex::Give(vec![1, 2, 3].into_boxed_slice());
        let claimed: Flex<'static, [i32]> = flex.claim();
        assert_eq!(&*claimed, &[1, 2, 3]);
    }

    #[test]
    fn claim_str() {
        let borrowed = Flex::Lend("hello");
        let owned: Flex<'static, str> = borrowed.claim();
        assert_eq!(&*owned, "hello");
    }
}
