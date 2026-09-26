/// LeetCode #3953 - Maximum Score with Co-Prime Element
fn distinct_primes(mut x: usize, spf: &[usize]) -> Vec<usize> {
    let mut s = Vec::new();
    while x > 1 {
        let p = spf[x];
        s.push(p);
        while x % p == 0 {
            x /= p;
        }
    }
    s
}

fn not_coprime(v: usize, cnt: &[i32], spf: &[usize]) -> i32 {
    let pf = distinct_primes(v, spf);
    let sz = pf.len();
    let mut total = 0i32;
    for mask in 1..(1usize << sz) {
        let mut prod = 1usize;
        let mut bits = 0i32;
        for i in 0..sz {
            if (mask >> i) & 1 == 1 {
                prod *= pf[i];
                bits += 1;
            }
        }
        if bits % 2 == 1 {
            total += cnt[prod];
        } else {
            total -= cnt[prod];
        }
    }
    total
}

fn max_score(nums: Vec<i32>, max_val: i32) -> i32 {
    let limit = nums.iter().copied().max().unwrap().max(max_val) as usize;
    let n = limit + 1;
    let mut spf: Vec<usize> = (0..n).collect();
    let mut i = 2usize;
    while i * i < n {
        if spf[i] == i {
            let mut j = i * i;
            while j < n {
                if spf[j] == j {
                    spf[j] = i;
                }
                j += i;
            }
        }
        i += 1;
    }
    let mut cnt = vec![0i32; n];
    let mut present = vec![0i32; n];
    for &x0 in &nums {
        let x = x0 as usize;
        present[x] += 1;
        let pf = distinct_primes(x, &spf);
        let sz = pf.len();
        for mask in 1..(1usize << sz) {
            let mut prod = 1usize;
            for i in 0..sz {
                if (mask >> i) & 1 == 1 {
                    prod *= pf[i];
                }
            }
            cnt[prod] += 1;
        }
    }
    let mut res = i32::MIN;
    if present[1] > 0 {
        res = res.max(1);
    } else if max_val >= 1 {
        res = res.max(0);
    }
    for v in 2..=max_val as usize {
        let c = not_coprime(v, &cnt, &spf);
        let cost = if present[v] > 0 { c - 1 } else { c.max(1) };
        res = res.max(v as i32 - cost);
    }
    let mut seen = vec![false; n];
    for &x0 in &nums {
        let x = x0 as usize;
        if x <= max_val as usize || seen[x] {
            continue;
        }
        seen[x] = true;
        let c = not_coprime(x, &cnt, &spf);
        res = res.max(x as i32 - (c - 1));
    }
    res
}

fn main() {
    println!("{}", max_score(vec![3, 4, 6], 5));
}

#[cfg(test)]
mod tests {
    use super::max_score;

    #[test]
    fn example1() {
        assert_eq!(max_score(vec![3, 4, 6], 5), 4);
    }

    #[test]
    fn example2() {
        assert_eq!(max_score(vec![1, 2, 3], 4), 3);
    }

    #[test]
    fn example3() {
        assert_eq!(max_score(vec![2, 2], 1), 1);
    }
}
