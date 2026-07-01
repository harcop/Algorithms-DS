/// LeetCode #2198 - Number of Single Divisor Triplets
fn single_divisor_triplet(nums: Vec<i32>) -> i64 {
    let mut cnt = [0i64; 101];
    for &x in &nums {
        cnt[x as usize] += 1;
    }

    let mut ans = 0i64;
    for a in 1..=100 {
        for b in 1..=100 {
            for c in 1..=100 {
                let s = a + b + c;
                let t = (s % a == 0) as i32 + (s % b == 0) as i32 + (s % c == 0) as i32;
                if t != 1 {
                    continue;
                }
                let x = cnt[a];
                let y = cnt[b];
                let z = cnt[c];
                ans += if a == b {
                    x * (x - 1) * z
                } else if a == c {
                    x * (x - 1) * y
                } else if b == c {
                    x * y * (y - 1)
                } else {
                    x * y * z
                };
            }
        }
    }
    ans
}

fn main() {
    println!("{}", single_divisor_triplet(vec![4, 6, 7, 3, 2]));
}

#[cfg(test)]
mod tests {
    use super::single_divisor_triplet;

    #[test]
    fn example_one() {
        assert_eq!(single_divisor_triplet(vec![4, 6, 7, 3, 2]), 12);
    }

    #[test]
    fn example_two() {
        assert_eq!(single_divisor_triplet(vec![1, 2, 2]), 6);
    }

    #[test]
    fn example_three() {
        assert_eq!(single_divisor_triplet(vec![1, 1, 1]), 0);
    }
}
