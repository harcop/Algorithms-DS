/// LeetCode #3927 - Minimize Array Sum Using Divisible Replacements
fn minimize_array_sum(nums: Vec<i32>) -> i64 {
    const MAX: usize = 100_000;
    let mut present = vec![false; MAX + 1];
    for &x in &nums {
        present[x as usize] = true;
    }
    let mut best = vec![0i32; MAX + 1];
    for d in 1..=MAX {
        if !present[d] {
            continue;
        }
        let mut m = d;
        while m <= MAX {
            if best[m] == 0 {
                best[m] = d as i32;
            }
            m += d;
        }
    }
    nums.iter().map(|&x| best[x as usize] as i64).sum()
}

fn main() {
    println!("{}", minimize_array_sum(vec![3, 6, 2]));
}

#[cfg(test)]
mod tests {
    use super::minimize_array_sum;

    #[test]
    fn example1() {
        assert_eq!(minimize_array_sum(vec![3, 6, 2]), 7);
    }

    #[test]
    fn example2() {
        assert_eq!(minimize_array_sum(vec![4, 2, 8, 3]), 9);
    }

    #[test]
    fn example3() {
        assert_eq!(minimize_array_sum(vec![7, 5, 9]), 21);
    }
}
