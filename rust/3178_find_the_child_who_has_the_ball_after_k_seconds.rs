/// LeetCode #3178 - Find the Child Who Has the Ball After K Seconds
fn number_of_child(n: i32, k: i32) -> i32 {
    let (q, r) = (k / (n - 1), k % (n - 1));
    if q % 2 == 1 {
        n - r - 1
    } else {
        r
    }
}

fn main() {
    println!("{}", number_of_child(3, 5));
}

#[cfg(test)]
mod tests {
    use super::number_of_child;

    #[test]
    fn example1() {
        assert_eq!(number_of_child(3, 5), 1);
    }

    #[test]
    fn example2() {
        assert_eq!(number_of_child(5, 6), 2);
    }

    #[test]
    fn example3() {
        assert_eq!(number_of_child(4, 2), 2);
    }
}
