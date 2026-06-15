/// LeetCode #1862 - Sum of Floored Pairs
const MOD: i64 = 1_000_000_007;

fn sum_of_floored_pairs(nums: Vec<i32>) -> i32 {
    let mx = *nums.iter().max().unwrap() as usize;
    let mut cnt = vec![0i64; mx + 1];
    for &x in &nums {
        cnt[x as usize] += 1;
    }
    let mut s = vec![0i64; mx + 1];
    for i in 1..=mx {
        s[i] = s[i - 1] + cnt[i];
    }
    let mut ans = 0i64;
    for y in 1..=mx {
        if cnt[y] == 0 {
            continue;
        }
        let mut d = 1i64;
        while (d as usize) * y <= mx {
            let start = d as usize * y;
            let end = (start + y - 1).min(mx);
            ans += cnt[y] * d * (s[end] - s[start - 1]);
            ans %= MOD;
            d += 1;
        }
    }
    ans as i32
}

fn main() {
    println!("{}", sum_of_floored_pairs(vec![2, 5, 9]));
}

#[cfg(test)]
mod tests {
    use super::sum_of_floored_pairs;

    #[test]
    fn example_one() {
        assert_eq!(sum_of_floored_pairs(vec![2, 5, 9]), 10);
    }
}
