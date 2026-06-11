/// LeetCode #1823 - Find the Winner of the Circular Game
fn find_the_winner(n: i32, k: i32) -> i32 {
    if n == 1 {
        return 1;
    }
    let ans = (k + find_the_winner(n - 1, k)) % n;
    if ans == 0 {
        n
    } else {
        ans
    }
}

fn main() {
    println!("{}", find_the_winner(5, 2));
}

#[cfg(test)]
mod tests {
    use super::find_the_winner;

    #[test]
    fn example_one() {
        assert_eq!(find_the_winner(5, 2), 3);
    }

    #[test]
    fn example_two() {
        assert_eq!(find_the_winner(6, 5), 1);
    }
}
