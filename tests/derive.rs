use flex::Flex;

// Deref is implicitly tested throughout, but we can test it explicitly
mod deref {
    use super::*;

    #[test]
    fn lend_deref() {
        let flex = Flex::Lend("hello");
        assert_eq!(&*flex, "hello");
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn give_deref() {
        let flex = Flex::Give(Box::new(42));
        assert_eq!(*flex, 42);
    }
}

// Clone trait tests
mod clone {
    use super::*;

    #[test]
    fn lend_clone() {
        let flex1 = Flex::Lend("hello");
        let flex2 = flex1.clone();
        assert_eq!(&*flex1, &*flex2);
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn give_clone() {
        let flex1 = Flex::Give(vec![1, 2, 3].into_boxed_slice());
        let flex2 = flex1.clone();
        assert_eq!(&*flex1, &*flex2);
    }
}

// Debug trait tests
mod debug {
    use super::*;

    #[test]
    fn lend_debug() {
        let flex = Flex::Lend(&[1, 2, 3][..]);
        let debug_str = format!("{:?}", flex);
        assert!(debug_str.contains("Lend"));
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn give_debug() {
        let flex = Flex::Give(vec![1, 2, 3].into_boxed_slice());
        let debug_str = format!("{:?}", flex);
        assert!(debug_str.contains("Give"));
    }
}

// Eq and PartialEq trait tests
mod equality {
    use super::*;

    #[test]
    fn lend_eq_lend() {
        let flex1 = Flex::Lend(&[1, 2, 3][..]);
        let flex2 = Flex::Lend(&[1, 2, 3][..]);
        assert_eq!(flex1, flex2);
    }

    #[test]
    fn lend_ne_lend() {
        let flex1 = Flex::Lend(&[1, 2, 3][..]);
        let flex2 = Flex::Lend(&[4, 5, 6][..]);
        assert_ne!(flex1, flex2);
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn give_eq_give() {
        let flex1 = Flex::Give(vec![1, 2, 3].into_boxed_slice());
        let flex2 = Flex::Give(vec![1, 2, 3].into_boxed_slice());
        assert_eq!(flex1, flex2);
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn lend_eq_give() {
        let lend = Flex::Lend(&[1, 2, 3][..]);
        let give = Flex::Give(vec![1, 2, 3].into_boxed_slice());
        assert_eq!(lend, give);
    }

    #[test]
    fn eq_with_slice_ref() {
        let flex = Flex::Lend(&[1, 2, 3][..]);
        assert_eq!(flex, &[1, 2, 3][..]);
    }

    #[test]
    fn eq_with_value() {
        let value = 42;
        let flex = Flex::Lend(&value);
        assert_eq!(flex, 42);
    }

    #[test]
    fn eq_with_value_ref() {
        let value = 42;
        let flex = Flex::Lend(&value);
        let other = 42;
        assert_eq!(flex, &other);
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn eq_with_box() {
        let flex = Flex::Lend(&[1, 2, 3][..]);
        let boxed = vec![1, 2, 3].into_boxed_slice();
        assert_eq!(flex, boxed);
    }
}

// Ord and PartialOrd trait tests
mod ordering {
    use super::*;

    #[test]
    fn lend_cmp_lend() {
        let flex1 = Flex::Lend(&[1, 2, 3][..]);
        let flex2 = Flex::Lend(&[1, 2, 4][..]);
        assert!(flex1 < flex2);
    }

    #[test]
    fn lend_cmp_using_cmp_method() {
        use core::cmp::Ordering;
        let flex1 = Flex::Lend(&[1, 2, 3][..]);
        let flex2 = Flex::Lend(&[1, 2, 4][..]);
        assert_eq!(flex1.cmp(&flex2), Ordering::Less);
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn give_cmp_give() {
        let flex1 = Flex::Give(vec![1, 2, 3].into_boxed_slice());
        let flex2 = Flex::Give(vec![1, 2, 4].into_boxed_slice());
        assert!(flex1 < flex2);
    }

    #[test]
    fn cmp_with_slice_ref() {
        let flex = Flex::Lend(&[1, 2, 3][..]);
        assert!(flex < [1, 2, 4][..]);
    }

    #[test]
    fn cmp_with_value() {
        let flex = Flex::Lend("abc");
        assert!(flex < "xyz");
    }

    #[test]
    fn cmp_with_value_ref() {
        let flex = Flex::Lend("abc");
        let other = "xyz";
        assert!(flex < other);
    }

    #[test]
    fn cmp_with_double_ref() {
        let flex = Flex::Lend("abc");
        let other = "xyz";
        let other_ref = &other;
        assert!(flex < *other_ref);
    }

    #[test]
    fn str_ordering() {
        let flex1 = Flex::Lend("abc");
        let flex2 = Flex::Lend("xyz");
        assert!(flex1 < flex2);
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn cmp_with_box() {
        let flex = Flex::Lend(&[1, 2, 3][..]);
        let boxed = vec![1, 2, 4].into_boxed_slice();
        assert!(flex < boxed);
    }
}

// Hash trait tests
mod hash {
    use super::*;
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    fn hash_value<T: Hash>(value: &T) -> u64 {
        let mut hasher = DefaultHasher::new();
        value.hash(&mut hasher);
        hasher.finish()
    }

    #[test]
    fn lend_hash_equal() {
        let flex1 = Flex::Lend(&[1, 2, 3][..]);
        let flex2 = Flex::Lend(&[1, 2, 3][..]);
        assert_eq!(hash_value(&flex1), hash_value(&flex2));
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn lend_give_hash_equal() {
        let lend = Flex::Lend(&[1, 2, 3][..]);
        let give = Flex::Give(vec![1, 2, 3].into_boxed_slice());
        assert_eq!(hash_value(&lend), hash_value(&give));
    }
}
