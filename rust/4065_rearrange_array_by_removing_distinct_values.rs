/// LeetCode #4065 - Rearrange Array by Removing Distinct Values
fn rearrange_array(nums: Vec<i32>) -> Vec<i32> {
    let mx = nums.iter().copied().max().unwrap_or(0) as usize;
    let mut cnt = vec![0i32; mx + 1];
    for x in &nums {
        cnt[*x as usize] += 1;
    }
    let mut ans = Vec::with_capacity(nums.len());
    while ans.len() < nums.len() {
        for x in 1..=mx {
            if cnt[x] > 0 {
                ans.push(x as i32);
                cnt[x] -= 1;
            }
        }
    }
    ans
}

fn main() {
    println!("{:?}", rearrange_array(vec![3, 1, 3, 2, 1, 3]));
}

#[cfg(test)]
mod tests {
    use super::rearrange_array;

    #[test]
    fn example1() {
        assert_eq!(rearrange_array(vec![3, 1, 3, 2, 1, 3]), vec![1, 2, 3, 1, 3, 3]);
    }

    #[test]
    fn example2() {
        assert_eq!(rearrange_array(vec![7, 7, 4, 4, 4]), vec![4, 7, 4, 7, 4]);
    }
}
