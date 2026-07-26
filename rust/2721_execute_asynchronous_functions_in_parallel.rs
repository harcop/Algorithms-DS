/// LeetCode #2721 - Execute Asynchronous Functions in Parallel (JS; Rust Result analogue)
fn promise_all(results: Vec<Result<i32, String>>) -> Result<Vec<i32>, String> {
    let mut out = Vec::with_capacity(results.len());
    for r in results {
        match r {
            Ok(v) => out.push(v),
            Err(e) => return Err(e),
        }
    }
    Ok(out)
}

fn main() {
    println!("{:?}", promise_all(vec![Ok(5)]));
}

#[cfg(test)]
mod tests {
    use super::promise_all;

    #[test]
    fn example_one() {
        assert_eq!(promise_all(vec![Ok(5)]), Ok(vec![5]));
    }

    #[test]
    fn example_two() {
        assert_eq!(
            promise_all(vec![Ok(1), Err("Error".into())]),
            Err("Error".into())
        );
    }

    #[test]
    fn example_three() {
        assert_eq!(
            promise_all(vec![Ok(4), Ok(10), Ok(16)]),
            Ok(vec![4, 10, 16])
        );
    }
}
