/// LeetCode #3519 - Count Numbers with Non-Decreasing Digits
const MOD: i32 = 1_000_000_007;

fn decrement(s: &str) -> String {
    let mut d: Vec<u8> = s.bytes().collect();
    let mut i = d.len();
    while i > 0 {
        i -= 1;
        if d[i] > b'0' {
            d[i] -= 1;
            break;
        }
        d[i] = b'9';
    }
    let start = d.iter().position(|&c| c != b'0').unwrap_or(d.len() - 1);
    String::from_utf8(d[start..].to_vec()).unwrap()
}

fn to_base(s: &str, b: i32) -> Vec<i32> {
    let mut digits: Vec<i32> = s.bytes().map(|c| (c - b'0') as i32).collect();
    let mut result = Vec::new();
    while !digits.is_empty() && !(digits.len() == 1 && digits[0] == 0) {
        let mut rem = 0i32;
        let mut new_digits = Vec::new();
        for &d in &digits {
            let cur = rem * 10 + d;
            let q = cur / b;
            rem = cur % b;
            if !new_digits.is_empty() || q != 0 {
                new_digits.push(q);
            }
        }
        result.push(rem);
        digits = new_digits;
    }
    result.reverse();
    if result.is_empty() {
        result.push(0);
    }
    result
}

fn count_upto(digits: &[i32], b: i32) -> i32 {
    let n = digits.len();
    let mut mem = vec![vec![vec![-1i32; 11]; 2]; n + 1];
    fn dp(
        pos: usize,
        last: usize,
        tight: usize,
        digits: &[i32],
        b: i32,
        mem: &mut [Vec<Vec<i32>>],
    ) -> i32 {
        if pos == digits.len() {
            return 1;
        }
        if mem[pos][tight][last] != -1 {
            return mem[pos][tight][last];
        }
        let limit = if tight == 1 { digits[pos] } else { b - 1 };
        let mut res = 0i32;
        for d in last as i32..=limit {
            let nt = if tight == 1 && d == limit { 1 } else { 0 };
            res = (res + dp(pos + 1, d as usize, nt, digits, b, mem)) % MOD;
        }
        mem[pos][tight][last] = res;
        res
    }
    dp(0, 0, 1, digits, b, &mut mem)
}

fn count_numbers(l: String, r: String, b: i32) -> i32 {
    let r_digits = to_base(&r, b);
    let l_minus = decrement(&l);
    let l_digits = to_base(&l_minus, b);
    let ans = (count_upto(&r_digits, b) - count_upto(&l_digits, b) + MOD) % MOD;
    ans
}

fn main() {
    println!("{}", count_numbers("23".into(), "28".into(), 8));
}

#[cfg(test)]
mod tests {
    use super::count_numbers;

    #[test]
    fn example1() {
        assert_eq!(count_numbers("23".into(), "28".into(), 8), 3);
    }

    #[test]
    fn example2() {
        assert_eq!(count_numbers("2".into(), "7".into(), 2), 2);
    }
}
