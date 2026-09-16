/// LeetCode #3780 - Maximum Sum of Three Numbers Divisible by Three
fn maximum_sum(mut nums: Vec<i32>) -> i32 {
    nums.sort_unstable();
    let mut g = vec![Vec::new(); 3];
    for x in nums {
        g[(x % 3) as usize].push(x);
    }
    let mut ans = 0;
    for a in 0..3 {
        if g[a].is_empty() {
            continue;
        }
        let x = g[a].pop().unwrap();
        for b in 0..3 {
            if g[b].is_empty() {
                continue;
            }
            let y = g[b].pop().unwrap();
            let c = (3 - (a + b) % 3) % 3;
            if let Some(&z) = g[c].last() {
                ans = ans.max(x + y + z);
            }
            g[b].push(y);
        }
        g[a].push(x);
    }
    ans
}

fn main() {
    println!("{}", maximum_sum(vec![4, 2, 3, 1]));
}

#[cfg(test)]
mod tests {
    use super::maximum_sum;

    #[test]
    fn example1() {
        assert_eq!(maximum_sum(vec![4, 2, 3, 1]), 9);
    }

    #[test]
    fn example2() {
        assert_eq!(maximum_sum(vec![2, 1, 5]), 0);
    }
}
