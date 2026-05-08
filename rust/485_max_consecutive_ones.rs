/// LeetCode #485 - Max Consecutive Ones
fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
    let mut best = 0i32;
    let mut cur = 0i32;
    for x in nums {
        if x == 1 {
            cur += 1;
            best = best.max(cur);
        } else {
            cur = 0;
        }
    }
    best
}

fn main() {
    println!("{}", find_max_consecutive_ones(vec![1, 1, 0, 1, 1, 1]));
}

#[cfg(test)]
mod tests {
    use super::find_max_consecutive_ones;

    #[test]
    fn example_one() {
        assert_eq!(find_max_consecutive_ones(vec![1, 1, 0, 1, 1, 1]), 3);
    }
}
