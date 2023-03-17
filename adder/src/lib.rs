pub fn add(left: i32, right: i32) -> i32 {
    left + right
}

pub fn add_two(num: i32) -> i32 {
    add(num, 2)
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

pub fn greeting(name: &str) -> String {
    format!("Hello {}", name)
}

pub fn wanna_panic() {
    panic!("Hey just panicked for fun")
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
    fn adds_two_to_a_numer() {
        assert_eq!(add_two(4), 6);
    }

    #[test]
    #[ignore]
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

    #[test]
    fn greeting_contains_name() {
        let name = "Victor";
        let result = greeting(name);
        assert!(
            result.contains(name),
            "Greeting didn't contain name, value was {}",
            result
        );
    }

    #[test]
    #[should_panic(expected="fun")]
    fn testing_if_panic_panics() {
        wanna_panic()
    }
}
