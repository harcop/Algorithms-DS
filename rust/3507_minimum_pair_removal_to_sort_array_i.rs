/// LeetCode #3507 - Minimum Pair Removal to Sort Array I
fn is_non_decreasing(a: &[i32]) -> bool {
    a.windows(2).all(|w| w[0] <= w[1])
}

fn minimum_pair_removal(nums: Vec<i32>) -> i32 {
    let mut arr = nums;
    let mut ans = 0;
    while !is_non_decreasing(&arr) {
        let mut k = 0;
        let mut s = arr[0] + arr[1];
        for i in 1..arr.len() - 1 {
            let t = arr[i] + arr[i + 1];
            if s > t {
                s = t;
                k = i;
            }
        }
        arr[k] = s;
        arr.remove(k + 1);
        ans += 1;
    }
    ans
}

fn main() {
    println!("{}", minimum_pair_removal(vec![5, 2, 3, 1]));
}

#[cfg(test)]
mod tests {
    use super::minimum_pair_removal;

    #[test]
    fn example1() {
        assert_eq!(minimum_pair_removal(vec![5, 2, 3, 1]), 2);
    }

    #[test]
    fn example2() {
        assert_eq!(minimum_pair_removal(vec![1, 2, 2]), 0);
    }
}
