/// LeetCode #2582 - Pass the Pillow
fn pass_the_pillow(n: i32, time: i32) -> i32 {
    let k = time / (n - 1);
    let rem = time % (n - 1);
    if k & 1 == 1 {
        n - rem
    } else {
        rem + 1
    }
}

fn main() {
    println!("{}", pass_the_pillow(4, 5));
}

#[cfg(test)]
mod tests {
    use super::pass_the_pillow;

    #[test]
    fn example_one() {
        assert_eq!(pass_the_pillow(4, 5), 2);
    }

    #[test]
    fn example_two() {
        assert_eq!(pass_the_pillow(3, 2), 3);
    }
}
