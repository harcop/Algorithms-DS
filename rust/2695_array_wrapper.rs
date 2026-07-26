/// LeetCode #2695 - Array Wrapper (JS problem; Rust struct analogue)
struct ArrayWrapper {
    nums: Vec<i32>,
    sum: i32,
}

impl ArrayWrapper {
    fn new(nums: Vec<i32>) -> Self {
        let sum = nums.iter().sum();
        ArrayWrapper { nums, sum }
    }

    fn value_of(&self) -> i32 {
        self.sum
    }

    fn to_string_repr(&self) -> String {
        format!(
            "[{}]",
            self.nums
                .iter()
                .map(|n| n.to_string())
                .collect::<Vec<_>>()
                .join(",")
        )
    }
}

fn add_wrappers(a: &ArrayWrapper, b: &ArrayWrapper) -> i32 {
    a.value_of() + b.value_of()
}

fn main() {
    let a = ArrayWrapper::new(vec![1, 2]);
    let b = ArrayWrapper::new(vec![3, 4]);
    println!("{} {}", add_wrappers(&a, &b), a.to_string_repr());
}

#[cfg(test)]
mod tests {
    use super::{add_wrappers, ArrayWrapper};

    #[test]
    fn example_add() {
        let a = ArrayWrapper::new(vec![1, 2]);
        let b = ArrayWrapper::new(vec![3, 4]);
        assert_eq!(add_wrappers(&a, &b), 10);
    }

    #[test]
    fn example_string() {
        let a = ArrayWrapper::new(vec![23, 98, 42, 70]);
        assert_eq!(a.to_string_repr(), "[23,98,42,70]");
    }

    #[test]
    fn example_empty() {
        let a = ArrayWrapper::new(vec![]);
        let b = ArrayWrapper::new(vec![]);
        assert_eq!(add_wrappers(&a, &b), 0);
    }
}
