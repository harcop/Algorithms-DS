/// LeetCode #2797 - Partial Function with Placeholders (JS problem; Rust analogue)
fn partial<F>(mut args: Vec<Option<i32>>, f: F) -> impl Fn(Vec<i32>) -> i32
where
    F: Fn(Vec<i32>) -> i32,
{
    move |rest| {
        let mut bound = args.clone();
        let mut i = 0;
        for slot in bound.iter_mut() {
            if slot.is_none() {
                *slot = Some(rest[i]);
                i += 1;
            }
        }
        let mut final_args: Vec<i32> = bound.into_iter().map(|x| x.unwrap()).collect();
        while i < rest.len() {
            final_args.push(rest[i]);
            i += 1;
        }
        f(final_args)
    }
}

fn main() {
    let sum = |args: Vec<i32>| args.iter().sum();
    let p = partial(vec![Some(2), Some(4), Some(6)], sum);
    println!("{}", p(vec![8, 10]));
}

#[cfg(test)]
mod tests {
    use super::partial;

    #[test]
    fn example_one() {
        let identity = |args: Vec<i32>| args.iter().sum();
        let p = partial(vec![Some(2), Some(4), Some(6)], identity);
        assert_eq!(p(vec![8, 10]), 30);
    }

    #[test]
    fn example_two() {
        let identity = |args: Vec<i32>| args.iter().sum();
        let p = partial(
            vec![Some(1), Some(2), None, Some(4), None, Some(6)],
            identity,
        );
        assert_eq!(p(vec![3, 5]), 21);
    }

    #[test]
    fn example_three() {
        let calc = |args: Vec<i32>| {
            let (a, b, c) = (args[0], args[1], args[2]);
            b + a - c
        };
        let p = partial(vec![None, Some(5)], calc);
        assert_eq!(p(vec![5, 20]), -10);
    }
}
