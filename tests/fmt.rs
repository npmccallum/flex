use flex::Flex;

// Display trait test
mod display {
    use super::*;

    #[test]
    fn str() {
        let flex = Flex::Lend("hello world");
        assert_eq!(format!("{}", flex), "hello world");
    }

    #[test]
    fn number() {
        let value = 42;
        let flex = Flex::Lend(&value);
        assert_eq!(format!("{}", flex), "42");
    }
}

// Binary, Octal, Hex traits
mod integer_formatting {
    use super::*;

    #[test]
    fn binary() {
        let value = 42;
        let flex = Flex::Lend(&value);
        assert_eq!(format!("{:b}", flex), "101010");
    }

    #[test]
    fn octal() {
        let value = 64;
        let flex = Flex::Lend(&value);
        assert_eq!(format!("{:o}", flex), "100");
    }

    #[test]
    fn lower_hex() {
        let value = 255;
        let flex = Flex::Lend(&value);
        assert_eq!(format!("{:x}", flex), "ff");
    }

    #[test]
    fn upper_hex() {
        let value = 255;
        let flex = Flex::Lend(&value);
        assert_eq!(format!("{:X}", flex), "FF");
    }
}

// Exponential formatting
mod exponential_formatting {
    use super::*;

    #[test]
    fn lower_exp() {
        let value = 1234.5f64;
        let flex = Flex::Lend(&value);
        let formatted = format!("{:e}", flex);
        assert!(formatted.starts_with("1.2345e"));
    }

    #[test]
    fn upper_exp() {
        let value = 1234.5f64;
        let flex = Flex::Lend(&value);
        let formatted = format!("{:E}", flex);
        assert!(formatted.starts_with("1.2345E"));
    }
}

// Pointer trait test
mod pointer {
    use super::*;

    #[test]
    fn pointer_format() {
        let value = 42;
        let flex = Flex::Lend(&value);
        let formatted = format!("{:p}", flex);
        assert!(formatted.starts_with("0x"));
    }
}
