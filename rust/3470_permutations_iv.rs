/// LeetCode #3470 - Permutations IV
fn permute(n: i32, mut k: i64) -> Vec<i32> {
    let n = n as usize;
    let mut remaining: Vec<i32> = (1..=n as i32).collect();
    let mut ans = Vec::new();
    let mut looking_for_even = true;
    for turn in 0..n {
        let left = n - 1 - turn;
        let mut ways = 1i64;
        let a = (left / 2) as i64;
        let b = ((left + 1) / 2) as i64;
        for x in 2..=a {
            if ways > k / x {
                ways = k + 1;
                break;
            }
            ways *= x;
        }
        if ways <= k {
            for x in 2..=b {
                if ways > k / x {
                    ways = k + 1;
                    break;
                }
                ways *= x;
            }
        }
        let mut found = false;
        let mut idx = 0;
        while idx < remaining.len() {
            let number = remaining[idx];
            if number % 2 != looking_for_even as i32 && (turn > 0 || n % 2 == 1) {
                idx += 1;
                continue;
            }
            if k <= ways {
                ans.push(remaining.remove(idx));
                looking_for_even = ans[ans.len() - 1] % 2 == 0;
                found = true;
                break;
            }
            k -= ways;
            idx += 1;
        }
        if !found {
            return vec![];
        }
    }
    ans
}

fn main() {
    println!("{:?}", permute(4, 6));
}

#[cfg(test)]
mod tests {
    use super::permute;

    #[test]
    fn example1() {
        assert_eq!(permute(4, 6), vec![3, 4, 1, 2]);
    }

    #[test]
    fn example2() {
        assert_eq!(permute(3, 2), vec![3, 2, 1]);
    }

    #[test]
    fn example3() {
        assert_eq!(permute(2, 3), vec![] as Vec<i32>);
    }
}
