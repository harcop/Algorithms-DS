/// LeetCode #2726 - Calculator with Method Chaining
struct Calculator {
    x: f64,
}

impl Calculator {
    fn new(value: f64) -> Self {
        Calculator { x: value }
    }

    fn add(mut self, value: f64) -> Self {
        self.x += value;
        self
    }

    fn subtract(mut self, value: f64) -> Self {
        self.x -= value;
        self
    }

    fn multiply(mut self, value: f64) -> Self {
        self.x *= value;
        self
    }

    fn divide(mut self, value: f64) -> Result<Self, String> {
        if value == 0.0 {
            return Err("Division by zero is not allowed".into());
        }
        self.x /= value;
        Ok(self)
    }

    fn power(mut self, value: f64) -> Self {
        self.x = self.x.powf(value);
        self
    }

    fn get_result(&self) -> f64 {
        self.x
    }
}

fn main() {
    println!("{}", Calculator::new(10.0).add(5.0).subtract(7.0).get_result());
}

#[cfg(test)]
mod tests {
    use super::Calculator;

    #[test]
    fn example_one() {
        assert_eq!(
            Calculator::new(10.0).add(5.0).subtract(7.0).get_result(),
            8.0
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            Calculator::new(2.0).multiply(5.0).power(2.0).get_result(),
            100.0
        );
    }

    #[test]
    fn example_three() {
        assert_eq!(
            Calculator::new(20.0).divide(0.0).err(),
            Some("Division by zero is not allowed".into())
        );
    }
}
