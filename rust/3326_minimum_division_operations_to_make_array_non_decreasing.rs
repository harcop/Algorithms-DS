/// LeetCode #3326 - Minimum Division Operations to Make Array Non Decreasing
fn min_operations(mut nums: Vec<i32>) -> i32 {
    let mx = *nums.iter().max().unwrap() as usize;
    let mut lpf = vec![0usize; mx + 1];
    for i in 2..=mx {
        if lpf[i] == 0 {
            let mut j = i;
            while j <= mx {
                if lpf[j] == 0 {
                    lpf[j] = i;
                }
                j += i;
            }
        }
    }
    let mut ans = 0;
    for i in (0..nums.len() - 1).rev() {
        if nums[i] > nums[i + 1] {
            nums[i] = lpf[nums[i] as usize] as i32;
            if nums[i] > nums[i + 1] {
                return -1;
            }
            ans += 1;
        }
    }
    ans
}

fn main() {
    println!("{}", min_operations(vec![25, 7]));
}

#[cfg(test)]
mod tests {
    use super::min_operations;

    #[test]
    fn example1() {
        assert_eq!(min_operations(vec![25, 7]), 1);
    }

    #[test]
    fn example2() {
        assert_eq!(min_operations(vec![7, 7, 6]), -1);
    }

    #[test]
    fn example3() {
        assert_eq!(min_operations(vec![1, 1, 1, 1]), 0);
    }
}
