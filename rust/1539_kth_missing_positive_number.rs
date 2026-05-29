/// LeetCode #1539 - Kth Missing Positive Number
fn find_kth_positive(arr: Vec<i32>, k: i32) -> i32 {
    let mut missing = 0;
    let mut expect = 1;
    for &x in &arr {
        while expect < x {
            missing += 1;
            if missing == k {
                return expect;
            }
            expect += 1;
        }
        expect = x + 1;
    }
    expect + (k - missing - 1)
}

fn main() {
    println!("{}", find_kth_positive(vec![2, 3, 4, 7, 11], 5));
}

#[cfg(test)]
mod tests {
    use super::find_kth_positive;

    #[test]
    fn example_one() {
        assert_eq!(find_kth_positive(vec![2, 3, 4, 7, 11], 5), 9);
    }

    #[test]
    fn example_two() {
        assert_eq!(find_kth_positive(vec![1, 2, 3, 4], 2), 6);
    }
}
