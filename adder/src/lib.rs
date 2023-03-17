pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.height > other.height && self.width > other.width
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adds_two_numbers() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn fails_test() {
        panic!("Just testing to fail");
    }

    // data for Rectangle testing
    const LARGER: Rectangle = Rectangle {
        width: 8,
        height: 7
    };
    const SMALLER: Rectangle = Rectangle {
        width: 5,
        height: 1
    };

    #[test]
    fn larger_can_hold_smaller() {
        // test LARGER must hold SMALLER
        assert!(LARGER.can_hold(&SMALLER));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        assert!(!SMALLER.can_hold(&LARGER));
    }
}
