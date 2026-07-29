/// LeetCode #2761 - Prime Pairs With Target Sum
fn find_prime_pairs(n: i32) -> Vec<Vec<i32>> {
    let n = n as usize;
    let mut is_prime = vec![true; n + 1];
    is_prime[0] = false;
    if n >= 1 { is_prime[1] = false; }
    let mut i = 2;
    while i * i <= n {
        if is_prime[i] {
            let mut j = i * i;
            while j <= n {
                is_prime[j] = false;
                j += i;
            }
        }
        i += 1;
    }
    let mut ans = Vec::new();
    for x in 2..=n / 2 {
        let y = n - x;
        if is_prime[x] && is_prime[y] {
            ans.push(vec![x as i32, y as i32]);
        }
    }
    ans
}

fn main() {
    println!("{:?}", find_prime_pairs(10));
}

#[cfg(test)]
mod tests {
    use super::find_prime_pairs;

    #[test]
    fn example_one() {
        assert_eq!(find_prime_pairs(10), vec![vec![3, 7], vec![5, 5]]);
    }

    #[test]
    fn example_two() {
        assert_eq!(find_prime_pairs(2), Vec::<Vec<i32>>::new());
    }

    #[test]
    fn example_seven() {
        assert_eq!(find_prime_pairs(7), vec![vec![2, 5]]);
    }
}
