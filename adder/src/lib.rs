//
//
//
#[derive(Debug)]
pub struct Rectangle {
    length: u32,
    width: u32,
}

impl Rectangle {
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.length > other.length && self.width > other.width
    }
}

pub fn add_two(a: i32) -> i32 {
    a + 2
}

pub fn greeting(_name: &str) -> String {
    String::from("Hello")
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(val: i32) -> Self {
        if val < 1 || val > 100 {
            panic!("Guess value must be less than or equal to 100, got {}.", val);
        }
        Guess {
            value: val,
        }
    }

    pub fn getter(&mut self) -> i32 {
        self.value
    }
}

pub fn print_and_return_a(a: i32) -> i32 {
    println!("I got the value {}", a);
    a
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    #[should_panic(expected = "Guess value must be less than or equal to 100")]
    fn greater_than_100() {
        Guess::new(200);
    }

    #[test]
    #[ignore]
    fn larger_can_hold_smaller() {
        let larger = Rectangle { length: 8, width: 7};
        let smaller = Rectangle { length: 5, width: 1};
        assert!(larger.can_hold(&smaller));
    }

    #[test]
    #[ignore]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {length: 8, width: 7};
        let smaller = Rectangle {length: 5, width: 1};
        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    fn this_test_will_pass() {
        let value = print_and_return_a(10);
        assert_eq!(10, value);
    }

    #[test]
    #[ignore]
    fn this_test_will_fail() {
        let value = print_and_return_a(8);
        assert_eq!(10, value);
    }

}
