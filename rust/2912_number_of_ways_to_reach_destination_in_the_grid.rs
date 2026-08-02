/// LeetCode #2912 - Number of Ways to Reach Destination in the Grid
fn number_of_ways(n: i32, m: i32, k: i32, source: Vec<i32>, dest: Vec<i32>) -> i32 {
    const MOD: i64 = 1_000_000_007;
    let n = n as i64;
    let m = m as i64;
    let mut a = 1i64;
    let mut b = 0i64;
    let mut c = 0i64;
    let mut d = 0i64;

    for _ in 0..k {
        let aa = ((n - 1) * b + (m - 1) * c) % MOD;
        let bb = (a + (n - 2) * b + (m - 1) * d) % MOD;
        let cc = (a + (m - 2) * c + (n - 1) * d) % MOD;
        let dd = (b + c + (n - 2) * d + (m - 2) * d) % MOD;
        a = aa;
        b = bb;
        c = cc;
        d = dd;
    }

    if source[0] == dest[0] {
        if source[1] == dest[1] {
            a as i32
        } else {
            c as i32
        }
    } else if source[1] == dest[1] {
        b as i32
    } else {
        d as i32
    }
}

fn main() {
    println!("{}", number_of_ways(3, 2, 2, vec![1, 1], vec![2, 2]));
}

#[cfg(test)]
mod tests {
    use super::number_of_ways;

    #[test]
    fn example_one() {
        assert_eq!(number_of_ways(3, 2, 2, vec![1, 1], vec![2, 2]), 2);
    }

    #[test]
    fn example_two() {
        assert_eq!(number_of_ways(3, 4, 3, vec![1, 2], vec![2, 3]), 9);
    }
}
