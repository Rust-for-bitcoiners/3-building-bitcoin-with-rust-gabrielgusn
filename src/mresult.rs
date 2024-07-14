#![allow(unused)]

use core::panic;

pub enum MResult<T, E> {
    Ok(T),
    Err(E),
}

impl<T, E> MResult<T, E> {
    // Function to create an Ok variant
    pub fn ok(value: T) -> Self {
        MResult::Ok(value)
    }
    // Function to create an Err variant
    pub fn err(error: E) -> Self {
        MResult::Err(error)
    }

    // Method to check if it's an Ok variant
    pub fn is_ok(&self) -> bool {
        // checks if the current instance is a variant of MResult::Ok by using the macro "matches!", by comparing self with MResult::Ok(_). In Rust, _ is a wildcard for any value
        matches!(self, MResult::Ok(_))
    }

    // Method to check if it's an Err variant
    pub fn is_err(&self) -> bool {
        // uses the same approach as is_ok(&self) but matching against MResult::Err
        matches!(self, MResult::Err(_))
    }

    // Method to unwrap the Ok value, panics if it's an Err
    pub fn unwrap(self) -> T {
        match self {
            MResult::Ok(value) => value,
            MResult::Err(_) => panic!("Called `unwrap()` on an `MResult::Err` value"),
        }
    }

    // Method to unwrap the Err value, panics if it's an Ok
    pub fn unwrap_err(self) -> E {
        match self {
            MResult::Err(error) => error,
            MResult::Ok(_) => panic!("Called `unwrap_err()` on an `MResult::Ok` value"),
        }
    }
}

// Add unit tests below

#[cfg(test)]
mod tests {
    use super::MResult;

    #[test]
    fn test_ok_creation() {
        let result: MResult<i32, &str> = MResult::ok(42);
        // asserts the variant is Ok
        assert!(result.is_ok());
        // asserts the variant is not Err
        assert!(!result.is_err());
        // asserts if the unwraped value is 42
        assert_eq!(result.unwrap(), 42);
    }

    #[test]
    fn test_err_creation() {
        let result: MResult<i32, &str> = MResult::err("error");
        // asserts if the variant is not Ok
        assert!(!result.is_ok());
        // asserts if the variant is Err
        assert!(result.is_err());
        // matches the unwraping_err with the value of MResult::err
        assert_eq!(result.unwrap_err(), "error");
    }

    #[test]
    #[should_panic(expected = "Called `unwrap()` on an `MResult::Err` value")]
    fn test_unwrap_err() {
        let result: MResult<i32, &str> = MResult::err("error");
        // this function should raise a panic! due to the result being unwrapped id not Ok
        result.unwrap();
    }

    #[test]
    #[should_panic(expected = "Called `unwrap_err()` on an `MResult::Ok` value")]
    fn test_unwrap_ok() {
        let result: MResult<i32, &str> = MResult::ok(42);
        // this function should raise a panic! due to result being unwraped is not Err
        result.unwrap_err();
    }
}