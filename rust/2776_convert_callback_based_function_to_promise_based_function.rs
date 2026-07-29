/// LeetCode #2776 - Convert Callback Based Function to Promise Based Function (JS problem; Rust analogue)
fn promisify<F>(f: F) -> impl Fn(Vec<i32>) -> Result<i32, String>
where
    F: Fn(Box<dyn FnOnce(i32, Option<&str>)>, Vec<i32>),
{
    move |args| {
        let (tx, rx) = std::sync::mpsc::channel();
        f(
            Box::new(move |data, err| {
                let _ = tx.send((data, err.map(|s| s.to_string())));
            }),
            args,
        );
        let (data, err) = rx.recv().unwrap();
        if let Some(e) = err {
            Err(e)
        } else {
            Ok(data)
        }
    }
}

fn main() {
    let sum = promisify(|cb, args| {
        let (a, b) = (args[0], args[1]);
        if a < 0 || b < 0 {
            cb(0, Some("a and b must be positive"));
        } else {
            cb(a + b, None);
        }
    });
    println!("{:?}", sum(vec![1, 2]));
}

#[cfg(test)]
mod tests {
    use super::promisify;

    #[test]
    fn example_one() {
        let mul = promisify(|cb, args| cb(args[0] * args[1] * args[2], None));
        assert_eq!(mul(vec![1, 2, 3]), Ok(6));
    }

    #[test]
    fn example_two() {
        let fail = promisify(|cb, _args| cb(0, Some("Promise Rejected")));
        assert_eq!(fail(vec![4, 5, 6]), Err("Promise Rejected".into()));
    }
}
