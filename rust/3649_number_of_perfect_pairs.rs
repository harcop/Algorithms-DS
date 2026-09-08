/// LeetCode #3649 - Number of Perfect Pairs
fn perfect_pairs(nums: Vec<i32>) -> i64 {
    let mut a: Vec<i64> = nums.into_iter().map(|x| (x as i64).abs()).collect();
    a.sort_unstable();
    let n = a.len();
    let mut res = 0i64;
    let mut j = 1usize;
    for i in 0..n {
        j = j.max(i + 1);
        while j < n && a[j] <= 2 * a[i] {
            j += 1;
        }
        res += (j - i - 1) as i64;
    }
    res
}

fn main() {
    println!("{}", perfect_pairs(vec![0, 1, 2, 3]));
}

#[cfg(test)]
mod tests {
    use super::perfect_pairs;

    #[test]
    fn example1() {
        assert_eq!(perfect_pairs(vec![0, 1, 2, 3]), 2);
    }

    #[test]
    fn example2() {
        assert_eq!(perfect_pairs(vec![-3, 2, -1, 4]), 4);
    }

    #[test]
    fn example3() {
        assert_eq!(perfect_pairs(vec![1, 10, 100, 1000]), 0);
    }
}
