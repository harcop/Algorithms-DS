/// LeetCode #2145 - Count the Hidden Sequences
fn number_of_arrays(differences: Vec<i32>, lower: i32, upper: i32) -> i32 {
    let mut cur = 0i64;
    let mut min_v = 0i64;
    let mut max_v = 0i64;

    for diff in differences {
        cur += diff as i64;
        min_v = min_v.min(cur);
        max_v = max_v.max(cur);
    }

    let span = max_v - min_v;
    let range = upper as i64 - lower as i64;
    if range < span {
        return 0;
    }

    (range - span + 1) as i32
}

fn main() {
    println!("{}", number_of_arrays(vec![1, -3, 4], 1, 6));
}

#[cfg(test)]
mod tests {
    use super::number_of_arrays;

    #[test]
    fn example_one() {
        assert_eq!(number_of_arrays(vec![1, -3, 4], 1, 6), 2);
    }

    #[test]
    fn example_two() {
        assert_eq!(number_of_arrays(vec![3, -4, 5, 1, -2], -4, 5), 4);
    }

    #[test]
    fn example_three() {
        assert_eq!(number_of_arrays(vec![4, -7, 2], 3, 6), 0);
    }
}
