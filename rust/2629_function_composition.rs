/// LeetCode #2629 - Function Composition (JS problem; Rust analogue)
fn compose(functions: Vec<fn(i32) -> i32>) -> impl Fn(i32) -> i32 {
    move |x| functions.iter().rev().fold(x, |acc, f| f(acc))
}

fn main() {
    let fn_ = compose(vec![|x| x + 1, |x| x * x, |x| 2 * x]);
    println!("{}", fn_(4));
}

#[cfg(test)]
mod tests {
    use super::compose;

    #[test]
    fn example_one() {
        let fn_ = compose(vec![|x| x + 1, |x| x * x, |x| 2 * x]);
        assert_eq!(fn_(4), 65);
    }

    #[test]
    fn example_two() {
        let fn_ = compose(vec![|x| 10 * x, |x| 10 * x, |x| 10 * x]);
        assert_eq!(fn_(1), 1000);
    }

    #[test]
    fn example_three() {
        let fn_ = compose(vec![]);
        assert_eq!(fn_(42), 42);
    }
}
