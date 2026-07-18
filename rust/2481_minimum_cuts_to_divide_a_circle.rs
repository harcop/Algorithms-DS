/// LeetCode #2481 - Minimum Cuts to Divide a Circle
fn number_of_cuts(n: i32) -> i32 {
    if n == 1 {
        0
    } else if n % 2 == 0 {
        n / 2
    } else {
        n
    }
}

fn main() {
    println!("{}", number_of_cuts(4));
}

#[cfg(test)]
mod tests {
    use super::number_of_cuts;

    #[test]
    fn example_one() {
        assert_eq!(number_of_cuts(4), 2);
    }

    #[test]
    fn example_two() {
        assert_eq!(number_of_cuts(3), 3);
    }

    #[test]
    fn single_piece() {
        assert_eq!(number_of_cuts(1), 0);
    }
}
