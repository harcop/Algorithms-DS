/// LeetCode #2620 - Counter (JS problem; Rust closure analogue)
fn create_counter(n: i32) -> impl FnMut() -> i32 {
    let mut i = n;
    move || {
        let v = i;
        i += 1;
        v
    }
}

fn main() {
    let mut counter = create_counter(10);
    println!("{} {} {}", counter(), counter(), counter());
}

#[cfg(test)]
mod tests {
    use super::create_counter;

    #[test]
    fn example_one() {
        let mut counter = create_counter(10);
        assert_eq!(counter(), 10);
        assert_eq!(counter(), 11);
        assert_eq!(counter(), 12);
    }

    #[test]
    fn example_two() {
        let mut counter = create_counter(-2);
        assert_eq!(counter(), -2);
        assert_eq!(counter(), -1);
        assert_eq!(counter(), 0);
        assert_eq!(counter(), 1);
        assert_eq!(counter(), 2);
    }
}
