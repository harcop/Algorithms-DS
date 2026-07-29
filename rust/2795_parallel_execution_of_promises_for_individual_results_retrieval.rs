/// LeetCode #2795 - Parallel Execution of Promises for Individual Results Retrieval (JS problem; Rust analogue)
#[derive(Debug, PartialEq, Eq)]
enum Settled<T> {
    Fulfilled(T),
    Rejected(String),
}

fn promise_all_settled(tasks: Vec<fn() -> Result<i32, &'static str>>) -> Vec<Settled<i32>> {
    tasks
        .into_iter()
        .map(|task| match task() {
            Ok(value) => Settled::Fulfilled(value),
            Err(reason) => Settled::Rejected(reason.to_string()),
        })
        .collect()
}

fn main() {
    fn t1() -> Result<i32, &'static str> {
        Ok(15)
    }
    fn t2() -> Result<i32, &'static str> {
        Ok(20)
    }
    println!("{:?}", promise_all_settled(vec![t1, t2]));
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ok15() -> Result<i32, &'static str> {
        Ok(15)
    }
    fn ok20() -> Result<i32, &'static str> {
        Ok(20)
    }
    fn ok30() -> Result<i32, &'static str> {
        Ok(30)
    }
    fn err() -> Result<i32, &'static str> {
        Err("Error")
    }

    #[test]
    fn example_one() {
        assert_eq!(promise_all_settled(vec![ok15]), vec![Settled::Fulfilled(15)]);
    }

    #[test]
    fn example_two() {
        assert_eq!(
            promise_all_settled(vec![ok20, ok15]),
            vec![Settled::Fulfilled(20), Settled::Fulfilled(15)]
        );
    }

    #[test]
    fn example_three() {
        assert_eq!(
            promise_all_settled(vec![ok30, err]),
            vec![
                Settled::Fulfilled(30),
                Settled::Rejected("Error".into())
            ]
        );
    }
}
