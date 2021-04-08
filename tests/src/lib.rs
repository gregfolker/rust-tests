#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    #[allow(dead_code)]
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.width
    }
}

pub fn greeting(name: &str) -> String {
    format!("Hello {}!", name)
}

#[allow(dead_code)]
pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Expected a value between 1 and 100, got {}", value);
        }
        Guess { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn force_pass_test() {
        assert_eq!(2 + 2, 4);
    }

    // #[test]
    // fn force_fail_test() {
    //    panic!("Make this test fail");
    // }

    #[test]
    fn larger_can_hold_smaller() {
        let larger_rect = Rectangle {
            width: 8,
            height: 7,
        };

        let smaller_rect = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(larger_rect.can_hold(&smaller_rect));
    }

    #[test]
    fn smaller_can_hold_larger() {
        let larger_rect = Rectangle {
            width: 8,
            height: 7,
        };

        let smaller_rect = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(!smaller_rect.can_hold(&larger_rect));
    }

    // Failure messages can be customized as well using the macro `assert!()`
    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(
            result.contains("Carol"),
            "Greeting {} did not contain expected name",
            result
        );
    }

    // The `#[should_panic]` attribute makes it such that the test will
    // pass if the code inside the test function panics. Otherwise, the
    // test will fail
    #[test]
    #[should_panic]
    fn greater_than_100() {
        Guess::new(200);
    }

    // Tests can also be written that use `Result<T, E>`
    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("Somehow, two plus two does not equal four"))
        }
    }
}
