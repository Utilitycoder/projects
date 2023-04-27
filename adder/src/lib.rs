pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn it_adds_two(left: usize) -> usize {
    add(left, 2)
}

pub fn greeting(name: &str) -> String {
    format!("Hello {}!", name)
}

#[derive(PartialEq, Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn prints_and_returns_10(a: i32) -> i32 {
    println!("I got the value {}", a);
    10
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exploration() {
        assert_eq!(4, add(2, 2), "This is what we get from add function: {}", add(2, 6));
    }

    const NORMAL: Rectangle = Rectangle { width: 30, height: 50 };
    const SMALLER: Rectangle = Rectangle { width: 10, height: 40 };
    static LARGER: Rectangle = Rectangle { width: 60, height: 45 };

    #[test]
    fn normal_can_hold_smaller() {
        assert!(NORMAL.can_hold(&SMALLER)); // true
    }

    #[test]
    fn normal_cannot_hold_larger() {
        assert!(!NORMAL.can_hold(&LARGER)); // false
    }

    #[test]
    fn same_rectangle() {
        assert_eq!(NORMAL, NORMAL);
    }

    #[test]
    fn different_rectangles() {
        assert_ne!(NORMAL, SMALLER);
    }

    #[test]
    #[ignore]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(
            result.contains("Carol"),
            "Greeting did not contain name, value was `{}`", result
        );
    }

    #[test]
    fn this_test_will_pass() {
        let value = prints_and_returns_10(4);
        assert_eq!(10, value);
    }

    #[test]
    fn this_test_will_fail() {
        let value = prints_and_returns_10(8);
        assert_ne!(5, value);
    }
}
