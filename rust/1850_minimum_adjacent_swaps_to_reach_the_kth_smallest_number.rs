/// LeetCode #1850 - Minimum Adjacent Swaps to Reach the Kth Smallest Number
fn get_min_swaps(num: String, k: i32) -> i32 {
    fn next_permutation(nums: &mut [char]) -> bool {
        let n = nums.len();
        let mut i = n.saturating_sub(2);
        while nums[i] >= nums[i + 1] {
            if i == 0 {
                return false;
            }
            i -= 1;
        }
        let mut j = n - 1;
        while nums[j] <= nums[i] {
            j -= 1;
        }
        nums.swap(i, j);
        nums[i + 1..].reverse();
        true
    }

    let mut s: Vec<char> = num.chars().collect();
    for _ in 0..k {
        next_permutation(&mut s);
    }

    let mut d: Vec<Vec<usize>> = vec![Vec::new(); 10];
    let mut idx = [0usize; 10];
    let n = s.len();
    for (i, c) in num.chars().enumerate() {
        let j = (c as u8 - b'0') as usize;
        d[j].push(i);
    }

    let mut arr = vec![0usize; n];
    for (i, c) in s.iter().enumerate() {
        let j = (*c as u8 - b'0') as usize;
        arr[i] = d[j][idx[j]];
        idx[j] += 1;
    }

    let mut ans = 0i32;
    for i in 0..n {
        for j in 0..i {
            if arr[j] > arr[i] {
                ans += 1;
            }
        }
    }
    ans
}

fn main() {
    println!("{}", get_min_swaps("5489355142".into(), 4));
}

#[cfg(test)]
mod tests {
    use super::get_min_swaps;

    #[test]
    fn example_one() {
        assert_eq!(get_min_swaps("5489355142".into(), 4), 2);
    }

    #[test]
    fn example_two() {
        assert_eq!(get_min_swaps("11112".into(), 4), 4);
    }
}
