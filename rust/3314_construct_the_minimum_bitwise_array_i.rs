/// LeetCode #3314 - Construct the Minimum Bitwise Array I
fn min_bitwise_array(nums: Vec<i32>) -> Vec<i32> {
    let mut ans = Vec::with_capacity(nums.len());
    for x in nums {
        if x == 2 {
            ans.push(-1);
        } else {
            for i in 1..32 {
                if ((x >> i) & 1) == 0 {
                    ans.push(x ^ (1 << (i - 1)));
                    break;
                }
            }
        }
    }
    ans
}

fn main() {
    println!("{:?}", min_bitwise_array(vec![2, 3, 5, 7]));
}

#[cfg(test)]
mod tests {
    use super::min_bitwise_array;

    #[test]
    fn example1() {
        assert_eq!(min_bitwise_array(vec![2, 3, 5, 7]), vec![-1, 1, 4, 3]);
    }

    #[test]
    fn example2() {
        assert_eq!(min_bitwise_array(vec![11, 13, 31]), vec![9, 12, 15]);
    }
}
