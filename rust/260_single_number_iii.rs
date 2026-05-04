/// LeetCode #260 - Single Number III
fn single_number(nums: Vec<i32>) -> Vec<i32> {
    let xor: i32 = nums.iter().fold(0, |a, &b| a ^ b);
    let diff = xor & -xor;
    let mut a = 0;
    let mut b = 0;
    for x in nums {
        if x & diff != 0 {
            a ^= x;
        } else {
            b ^= x;
        }
    }
    vec![a.min(b), a.max(b)]
}

fn main() {
    println!("{:?}", single_number(vec![1, 2, 1, 3, 2, 5]));
}

#[cfg(test)]
mod tests {
    use super::single_number;

    #[test]
    fn example_one() {
        let mut v = single_number(vec![1, 2, 1, 3, 2, 5]);
        v.sort();
        assert_eq!(v, vec![3, 5]);
    }

    #[test]
    fn example_two() {
        let mut v = single_number(vec![-1, 0]);
        v.sort();
        assert_eq!(v, vec![-1, 0]);
    }
}
