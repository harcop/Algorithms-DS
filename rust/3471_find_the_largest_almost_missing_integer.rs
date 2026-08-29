/// LeetCode #3471 - Find the Largest Almost Missing Integer
fn largest_integer(nums: Vec<i32>, k: i32) -> i32 {
    let n = nums.len();
    let k = k as usize;
    let mut cnt = vec![0; 51];
    for w in 0..=n - k {
        let mut seen = [false; 51];
        for i in w..w + k {
            let x = nums[i] as usize;
            if !seen[x] {
                cnt[x] += 1;
                seen[x] = true;
            }
        }
    }
    (0..=50)
        .rev()
        .find(|&x| cnt[x] == 1)
        .map(|x| x as i32)
        .unwrap_or(-1)
}

fn main() {
    println!("{}", largest_integer(vec![3, 9, 2, 1, 7], 3));
}

#[cfg(test)]
mod tests {
    use super::largest_integer;

    #[test]
    fn example1() {
        assert_eq!(largest_integer(vec![3, 9, 2, 1, 7], 3), 7);
    }

    #[test]
    fn example2() {
        assert_eq!(largest_integer(vec![3, 9, 7, 2, 1, 7], 4), 3);
    }

    #[test]
    fn example3() {
        assert_eq!(largest_integer(vec![0, 0], 1), -1);
    }
}
