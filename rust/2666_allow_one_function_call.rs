/// LeetCode #2666 - Allow One Function Call (JS problem; Rust closure analogue)
fn once<F, T, R>(mut fn_: F) -> impl FnMut(T) -> Option<R>
where
    F: FnMut(T) -> R,
{
    let mut called = false;
    move |args| {
        if called {
            None
        } else {
            called = true;
            Some(fn_(args))
        }
    }
}

fn main() {
    let mut once_fn = once(|args: (i32, i32, i32)| args.0 + args.1 + args.2);
    println!("{:?} {:?}", once_fn((1, 2, 3)), once_fn((2, 3, 6)));
}

#[cfg(test)]
mod tests {
    use super::once;

    #[test]
    fn example_one() {
        let mut once_fn = once(|args: (i32, i32, i32)| args.0 + args.1 + args.2);
        assert_eq!(once_fn((1, 2, 3)), Some(6));
        assert_eq!(once_fn((2, 3, 6)), None);
    }

    #[test]
    fn example_two() {
        let mut once_fn = once(|args: (i32, i32, i32)| args.0 * args.1 * args.2);
        assert_eq!(once_fn((5, 7, 4)), Some(140));
        assert_eq!(once_fn((2, 3, 6)), None);
        assert_eq!(once_fn((4, 6, 8)), None);
    }
}
