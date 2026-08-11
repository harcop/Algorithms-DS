/// LeetCode #3145 - Find Products of Elements of Big Array
const M: usize = 50;

fn build_tables() -> (Vec<i64>, Vec<i64>) {
    let mut cnt = vec![0i64; M + 1];
    let mut s = vec![0i64; M + 1];
    let mut p = 1i64;
    for i in 1..=M {
        cnt[i] = cnt[i - 1] * 2 + p;
        s[i] = s[i - 1] * 2 + p * (i as i64 - 1);
        p *= 2;
    }
    (cnt, s)
}

fn num_idx_and_sum(mut x: i64, cnt: &[i64], s: &[i64]) -> (i64, i64) {
    let mut idx = 0i64;
    let mut total_sum = 0i64;
    while x > 0 {
        let i = (63 - x.leading_zeros()) as usize;
        idx += cnt[i];
        total_sum += s[i];
        x -= 1 << i;
        total_sum += (x + 1) * i as i64;
        idx += x + 1;
    }
    (idx, total_sum)
}

fn f(i: i64, cnt: &[i64], s: &[i64]) -> i64 {
    let mut l = 0i64;
    let mut r = 1i64 << M;
    while l < r {
        let mid = (l + r + 1) >> 1;
        let (idx, _) = num_idx_and_sum(mid, cnt, s);
        if idx < i {
            l = mid;
        } else {
            r = mid - 1;
        }
    }
    let (idx, mut total_sum) = num_idx_and_sum(l, cnt, s);
    let rem = i - idx;
    let mut x = l + 1;
    for _ in 0..rem {
        let y = x & -x;
        total_sum += y.trailing_zeros() as i64;
        x -= y;
    }
    total_sum
}

fn find_products_of_elements(queries: Vec<Vec<i64>>) -> Vec<i32> {
    let (cnt, s) = build_tables();
    queries
        .into_iter()
        .map(|q| {
            let left = q[0];
            let right = q[1];
            let modulus = q[2];
            let exp = f(right + 1, &cnt, &s) - f(left, &cnt, &s);
            mod_pow(2, exp, modulus) as i32
        })
        .collect()
}

fn mod_pow(mut base: i64, mut exp: i64, modulus: i64) -> i64 {
    if modulus == 1 {
        return 0;
    }
    let mut result = 1i64;
    base %= modulus;
    while exp > 0 {
        if exp & 1 == 1 {
            result = result * base % modulus;
        }
        base = base * base % modulus;
        exp >>= 1;
    }
    result
}

fn main() {
    println!("{:?}", find_products_of_elements(vec![vec![1, 3, 7]]));
}

#[cfg(test)]
mod tests {
    use super::find_products_of_elements;

    #[test]
    fn example1() {
        assert_eq!(find_products_of_elements(vec![vec![1, 3, 7]]), vec![4]);
    }

    #[test]
    fn example2() {
        assert_eq!(
            find_products_of_elements(vec![vec![2, 5, 3], vec![7, 7, 4]]),
            vec![2, 2]
        );
    }
}
