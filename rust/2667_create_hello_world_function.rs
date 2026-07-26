/// LeetCode #2667 - Create Hello World Function (JS problem; Rust closure analogue)
fn create_hello_world() -> impl Fn() -> &'static str {
    || "Hello World"
}

fn main() {
    let f = create_hello_world();
    println!("{}", f());
}

#[cfg(test)]
mod tests {
    use super::create_hello_world;

    #[test]
    fn example_one() {
        let f = create_hello_world();
        assert_eq!(f(), "Hello World");
    }

    #[test]
    fn example_two() {
        let f = create_hello_world();
        assert_eq!(f(), "Hello World");
        assert_eq!(f(), "Hello World");
    }
}
