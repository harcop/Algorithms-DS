/// LeetCode #3587 - Minimum Adjacent Swaps to Alternate Parity
fn min_swaps(nums: Vec<i32>) -> i32 {
    let mut pos = [Vec::new(), Vec::new()];
    for (i, &x) in nums.iter().enumerate() {
        pos[(x & 1) as usize].push(i as i32);
    }
    if (pos[0].len() as i32 - pos[1].len() as i32).abs() > 1 {
        return -1;
    }
    let calc = |k: usize| -> i32 {
        let mut res = 0;
        let mut i = 0;
        let mut t = 0;
        while i < nums.len() {
            res += (pos[k][t] - i as i32).abs();
            t += 1;
            i += 2;
        }
        res
    };
    if pos[0].len() > pos[1].len() {
        calc(0)
    } else if pos[0].len() < pos[1].len() {
        calc(1)
    } else {
        calc(0).min(calc(1))
    }
}

fn main() {
    println!("{}", min_swaps(vec![2, 4, 6, 5, 7]));
}

#[cfg(test)]
mod tests {
    use super::min_swaps;

    #[test]
    fn example1() {
        assert_eq!(min_swaps(vec![2, 4, 6, 5, 7]), 3);
    }

    #[test]
    fn example2() {
        assert_eq!(min_swaps(vec![2, 4, 5, 7]), 1);
    }

    #[test]
    fn example3() {
        assert_eq!(min_swaps(vec![1, 2, 3]), 0);
    }

    #[test]
    fn example4() {
        assert_eq!(min_swaps(vec![4, 5, 6, 8]), -1);
    }
}
