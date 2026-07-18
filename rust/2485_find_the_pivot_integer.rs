/// LeetCode #2485 - Find the Pivot Integer
fn pivot_integer(n: i32) -> i32 {
    let y = (n * n + n) / 2;
    let x = (y as f64).sqrt() as i32;
    if x * x == y {
        x
    } else {
        -1
    }
}

fn main() {
    println!("{}", pivot_integer(8));
}

#[cfg(test)]
mod tests {
    use super::pivot_integer;

    #[test]
    fn example_one() {
        assert_eq!(pivot_integer(8), 6);
    }

    #[test]
    fn example_two() {
        assert_eq!(pivot_integer(1), 1);
    }

    #[test]
    fn example_three() {
        assert_eq!(pivot_integer(4), -1);
    }
}
