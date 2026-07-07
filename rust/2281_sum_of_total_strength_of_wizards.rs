/// LeetCode #2281 - Sum of Total Strength of Wizards
const MOD: i64 = 1_000_000_007;

fn total_strength(strength: Vec<i32>) -> i32 {
    let n = strength.len();
    let mut left = vec![-1i32; n];
    let mut right = vec![n as i32; n];
    let mut stack: Vec<usize> = Vec::new();

    for i in (0..n).rev() {
        while let Some(&top) = stack.last() {
            if strength[top] >= strength[i] {
                left[top] = i as i32;
                stack.pop();
            } else {
                break;
            }
        }
        stack.push(i);
    }

    stack.clear();
    for i in 0..n {
        while let Some(&top) = stack.last() {
            if strength[top] > strength[i] {
                right[top] = i as i32;
                stack.pop();
            } else {
                break;
            }
        }
        stack.push(i);
    }

    let mut prefix = vec![0i64; n];
    for i in 0..n {
        prefix[i] = if i == 0 {
            strength[0] as i64
        } else {
            (strength[i] as i64 + prefix[i - 1]) % MOD
        };
    }

    let mut prefix_of_prefix = vec![0i64; n + 1];
    for i in 0..n {
        prefix_of_prefix[i + 1] = (prefix_of_prefix[i] + prefix[i]) % MOD;
    }

    let mut ans = 0i64;
    for i in 0..n {
        let l = left[i];
        let r = right[i];
        let left_sum = (prefix_of_prefix[i as usize]
            - prefix_of_prefix[0.max(l) as usize]
            + MOD)
            % MOD;
        let right_sum = (prefix_of_prefix[r as usize] - prefix_of_prefix[i] + MOD) % MOD;
        let left_len = (i as i32 - l) as i64;
        let right_len = (r - i as i32) as i64;
        let term = (right_sum * left_len % MOD - left_sum * right_len % MOD + MOD) % MOD;
        ans = (ans + strength[i] as i64 * term % MOD) % MOD;
    }

    ans as i32
}

fn main() {
    println!("{}", total_strength(vec![1, 3, 1, 2]));
}

#[cfg(test)]
mod tests {
    use super::total_strength;

    #[test]
    fn example_one() {
        assert_eq!(total_strength(vec![1, 3, 1, 2]), 44);
    }

    #[test]
    fn example_two() {
        assert_eq!(total_strength(vec![5, 4, 6]), 213);
    }
}
