use flex::Flex;
use std::fmt::Debug;

// Tests for using Flex with trait objects
mod trait_objects {
    use super::*;

    #[test]
    fn lend_debug() {
        let value = 42;
        let flex: Flex<dyn Debug> = Flex::from(&value as &dyn Debug);
        assert_eq!(format!("{:?}", &*flex), "42");
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn give_debug() {
        let boxed: Box<dyn Debug> = Box::new(42);
        let flex: Flex<dyn Debug> = Flex::from(boxed);
        assert_eq!(format!("{:?}", &*flex), "42");
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn mixed_usage() {
        fn format_debug(flex: Flex<dyn Debug>) -> String {
            format!("{:?}", &*flex)
        }

        let lend: Flex<dyn Debug> = Flex::from(&42 as &dyn Debug);
        let give: Flex<dyn Debug> = Flex::from(Box::new("hello") as Box<dyn Debug>);

        assert_eq!(format_debug(lend), "42");
        assert_eq!(format_debug(give), "\"hello\"");
    }

    #[test]
    fn with_string() {
        let value = String::from("test");
        let flex: Flex<dyn Debug> = Flex::from(&value as &dyn Debug);
        let debug_str = format!("{:?}", &*flex);
        assert!(debug_str.contains("test"));
    }
}

// Tests with custom trait objects
mod custom_traits {
    use super::*;

    trait CustomTrait {
        fn double(&self) -> i32;
    }

    impl CustomTrait for i32 {
        fn double(&self) -> i32 {
            *self * 2
        }
    }

    #[test]
    fn lend_custom_trait() {
        let value = 21;
        let flex: Flex<dyn CustomTrait> = Flex::from(&value as &dyn CustomTrait);
        assert_eq!(flex.double(), 42);
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn give_custom_trait() {
        let boxed: Box<dyn CustomTrait> = Box::new(21);
        let flex: Flex<dyn CustomTrait> = Flex::from(boxed);
        assert_eq!(flex.double(), 42);
    }
}

// Integration tests demonstrating real-world usage patterns
mod usage_patterns {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn in_hashmap() {
        let mut map = HashMap::new();
        let flex = Flex::Lend(&[1, 2, 3][..]);

        map.insert(flex.clone(), "value");
        assert_eq!(map.get(&[1, 2, 3][..]), Some(&"value"));
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn mixed_hashmap() {
        let mut map = HashMap::new();
        let lend = Flex::Lend(&[1, 2, 3][..]);
        let give = Flex::Give(vec![4, 5, 6].into_boxed_slice());

        map.insert(lend, "lend");
        map.insert(give, "give");

        assert_eq!(map.len(), 2);
        assert_eq!(map.get(&[1, 2, 3][..]), Some(&"lend"));
        assert_eq!(map.get(&[4, 5, 6][..]), Some(&"give"));
    }

    #[test]
    fn api_accepting_flex() {
        fn process_data(data: Flex<[i32]>) -> i32 {
            data.iter().sum()
        }

        let lend = Flex::Lend(&[1, 2, 3][..]);
        assert_eq!(process_data(lend), 6);

        #[cfg(feature = "alloc")]
        {
            let give = Flex::Give(vec![4, 5, 6].into_boxed_slice());
            assert_eq!(process_data(give), 15);
        }
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn conditional_allocation() {
        fn maybe_allocate(should_alloc: bool, data: &[i32]) -> Flex<'_, [i32]> {
            if should_alloc {
                Flex::from(data).claim()
            } else {
                Flex::from(data)
            }
        }

        let data = [1, 2, 3];
        let borrowed = maybe_allocate(false, &data);
        let owned = maybe_allocate(true, &data);

        assert_eq!(&*borrowed, &[1, 2, 3]);
        assert_eq!(&*owned, &[1, 2, 3]);
    }
}
