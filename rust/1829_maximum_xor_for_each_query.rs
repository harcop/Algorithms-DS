/// LeetCode #1829 - Maximum XOR for Each Query
fn get_maximum_xor(nums: Vec<i32>, maximum_bit: i32) -> Vec<i32> {
    let mut xs = 0i32;
    for &x in &nums {
        xs ^= x;
    }
    let mut ans = Vec::with_capacity(nums.len());
    for &x in nums.iter().rev() {
        let mut k = 0i32;
        for i in (0..maximum_bit).rev() {
            if (xs >> i) & 1 == 0 {
                k |= 1 << i;
            }
        }
        ans.push(k);
        xs ^= x;
    }
    ans
}

fn main() {
    println!("{:?}", get_maximum_xor(vec![0, 1, 1, 3], 2));
}

#[cfg(test)]
mod tests {
    use super::get_maximum_xor;

    #[test]
    fn example_one() {
        assert_eq!(get_maximum_xor(vec![0, 1, 1, 3], 2), vec![0, 3, 2, 3]);
    }

    #[test]
    fn example_two() {
        assert_eq!(get_maximum_xor(vec![2, 3, 4, 7], 3), vec![5, 2, 6, 5]);
    }
}
