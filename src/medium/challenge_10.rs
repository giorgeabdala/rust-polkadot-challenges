/// Simple calculator for demonstrating testing and documentation
#[derive(Debug)]
pub struct Calculator;

/// Errors that can occur during calculations
#[derive(Debug, PartialEq)]
pub enum CalcError {
    DivisionByZero,
    Overflow,
}


impl Calculator {
    /// Creates a new calculator instance
    ///
    /// # Examples
    ///
    /// ```
    /// let calc = Calculator::new();
    /// ```
    pub fn new() -> Self {
        Self
    }

    /// Adds two numbers together
    ///
    /// # Arguments
    ///
    /// * `a` - First number
    /// * `b` - Second number
    ///
    /// # Returns
    ///
    /// * `Ok(sum)` - The sum of a and b
    /// * `Err(CalcError::Overflow)` - If result overflows
    ///
    /// # Examples
    ///
    /// ```
    /// let calc = Calculator::new();
    /// assert_eq!(calc.add(5, 3).unwrap(), 8);
    /// ```
    // TODO: Implement this method
    pub fn add(&self, a: u32, b: u32) -> Result<u32, CalcError> {
        a.checked_add(b).ok_or(CalcError::Overflow)
    }

    /// Divides one number by another
    ///
    /// # Arguments
    ///
    /// * `a` - Dividend
    /// * `b` - Divisor
    ///
    /// # Returns
    ///
    /// * `Ok(quotient)` - The result of a/b
    /// * `Err(CalcError::DivisionByZero)` - If b is zero
    ///
    /// # Examples
    ///
    /// ```
    /// let calc = Calculator::new();
    /// assert_eq!(calc.divide(10, 2).unwrap(), 5);
    /// ```
    // TODO: Implement this method
    pub fn divide(&self, a: u32, b: u32) -> Result<u32, CalcError> {
        if b == 0 {return Err(CalcError::DivisionByZero)}
        Ok(a/b)
    }

    /// Checks if a number is even
    ///
    /// # Arguments
    ///
    /// * `n` - Number to check
    ///
    /// # Returns
    ///
    /// * `true` - If number is even
    /// * `false` - If number is odd
    ///
    /// # Examples
    ///
    /// ```
    /// let calc = Calculator::new();
    /// assert!(calc.is_even(4));
    /// assert!(!calc.is_even(5));
    /// ```
    // TODO: Implement this method
    pub fn is_even(&self, n: u32) -> bool {
        n % 2 == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_success() {
        let calc = Calculator::new();
        assert_eq!(calc.add(2, 2), Ok(4));
        assert_eq!(calc.add(0, 5), Ok(5));
        assert_eq!(calc.add(u32::MAX, 0), Ok(u32::MAX));
    }

    #[test]
    fn test_add_overflow() {
        let calc = Calculator::new();
        assert_eq!(calc.add(u32::MAX, 1), Err(CalcError::Overflow));
        assert_eq!(calc.add(u32::MAX - 5, 6), Err(CalcError::Overflow));
    }

    #[test]
    fn test_divide_success() {
        let calc = Calculator::new();
        assert_eq!(calc.divide(10, 2), Ok(5));
        assert_eq!(calc.divide(9, 3), Ok(3));
        assert_eq!(calc.divide(0, 1), Ok(0));
    }

    #[test]
    fn test_divide_by_zero() {
        let calc = Calculator::new();
        assert_eq!(calc.divide(10, 0), Err(CalcError::DivisionByZero));
    }

    #[test]
    fn test_is_even() {
        let calc = Calculator::new();
        assert!(calc.is_even(0));
        assert!(calc.is_even(2));
        assert!(calc.is_even(100));
        assert!(!calc.is_even(1));
        assert!(!calc.is_even(3));
        assert!(!calc.is_even(101));
    }
}

