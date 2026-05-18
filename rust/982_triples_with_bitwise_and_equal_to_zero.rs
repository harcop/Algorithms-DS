/// LeetCode #982 - Triples with Bitwise AND Equal To Zero
fn triples_with_bitwise_and_zero(nums: Vec<i32>) -> i32 {
    use std::collections::HashMap;
    let mut pair_cnt: HashMap<i32, i32> = HashMap::new();
    for i in 0..nums.len() {
        for j in 0..nums.len() {
            let key = nums[i] & nums[j];
            *pair_cnt.entry(key).or_default() += 1;
        }
    }
    let mut ans = 0i32;
    for k in 0..nums.len() {
        for (and_val, cnt) in &pair_cnt {
            if (*and_val & nums[k]) == 0 {
                ans += cnt;
            }
        }
    }
    ans
}

fn main() {
    println!("{}", triples_with_bitwise_and_zero(vec![2, 1, 3]));
}

#[cfg(test)]
mod tests {
    use super::triples_with_bitwise_and_zero;

    #[test]
    fn example_one() {
        assert_eq!(triples_with_bitwise_and_zero(vec![2, 1, 3]), 12);
    }

    #[test]
    fn example_two() {
        assert_eq!(triples_with_bitwise_and_zero(vec![1, 2, 3, 4, 5]), 0);
    }
}
