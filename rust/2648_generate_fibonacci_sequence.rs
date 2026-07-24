/// LeetCode #2648 - Generate Fibonacci Sequence (JS problem; Rust iterator analogue)
struct FibGenerator {
    a: i64,
    b: i64,
}

impl FibGenerator {
    fn new() -> Self {
        FibGenerator { a: 0, b: 1 }
    }
}

impl Iterator for FibGenerator {
    type Item = i64;

    fn next(&mut self) -> Option<i64> {
        let cur = self.a;
        let next = self.a + self.b;
        self.a = self.b;
        self.b = next;
        Some(cur)
    }
}

fn main() {
    let gen: Vec<_> = FibGenerator::new().take(5).collect();
    println!("{:?}", gen);
}

#[cfg(test)]
mod tests {
    use super::FibGenerator;

    #[test]
    fn example_one() {
        let got: Vec<_> = FibGenerator::new().take(5).collect();
        assert_eq!(got, vec![0, 1, 1, 2, 3]);
    }

    #[test]
    fn example_two() {
        let got: Vec<_> = FibGenerator::new().take(0).collect();
        assert!(got.is_empty());
    }
}
