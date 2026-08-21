/// LeetCode #3348 - Smallest Divisible Digit Product II
const K_FACTOR: [[i32; 4]; 10] = [
    [0, 0, 0, 0],
    [0, 0, 0, 0],
    [1, 0, 0, 0],
    [0, 1, 0, 0],
    [2, 0, 0, 0],
    [0, 0, 1, 0],
    [1, 1, 0, 0],
    [0, 0, 0, 1],
    [3, 0, 0, 0],
    [0, 2, 0, 0],
];

fn get_prime_count(mut t: i64) -> ([i32; 4], bool) {
    let mut count = [0i32; 4];
    let primes = [2i64, 3, 5, 7];
    for (i, &p) in primes.iter().enumerate() {
        while t % p == 0 {
            t /= p;
            count[i] += 1;
        }
    }
    (count, t == 1)
}

fn get_prime_count_from_string(num: &[u8]) -> [i32; 4] {
    let mut count = [0i32; 4];
    for &c in num {
        let d = (c - b'0') as usize;
        for i in 0..4 {
            count[i] += K_FACTOR[d][i];
        }
    }
    count
}

fn get_factor_count(count: [i32; 4]) -> [i32; 10] {
    let mut res = [0i32; 10];
    let count8 = count[0] / 3;
    let remaining2 = count[0] % 3;
    let count9 = count[1] / 2;
    let mut count3 = count[1] % 2;
    let mut count4 = remaining2 / 2;
    let mut count2 = remaining2 % 2;
    let mut count6 = 0;
    if count2 == 1 && count3 == 1 {
        count2 = 0;
        count3 = 0;
        count6 = 1;
    }
    if count3 == 1 && count4 == 1 {
        count2 = 1;
        count6 = 1;
        count3 = 0;
        count4 = 0;
    }
    res[2] = count2;
    res[3] = count3;
    res[4] = count4;
    res[5] = count[2];
    res[6] = count6;
    res[7] = count[3];
    res[8] = count8;
    res[9] = count9;
    res
}

fn construct(factors: [i32; 10]) -> String {
    let mut res = String::new();
    for digit in 2..10 {
        for _ in 0..factors[digit] {
            res.push(char::from(b'0' + digit as u8));
        }
    }
    res
}

fn is_subset(a: [i32; 4], b: [i32; 4]) -> bool {
    (0..4).all(|i| b[i] >= a[i])
}

fn subtract(a: [i32; 4], b: [i32; 4]) -> [i32; 4] {
    [
        (a[0] - b[0]).max(0),
        (a[1] - b[1]).max(0),
        (a[2] - b[2]).max(0),
        (a[3] - b[3]).max(0),
    ]
}

fn sum_values(count: [i32; 10]) -> i32 {
    count.iter().sum()
}

fn smallest_number(num: String, t: i64) -> String {
    let (prime_count, is_divisible) = get_prime_count(t);
    if !is_divisible {
        return "-1".into();
    }
    let factor_count = get_factor_count(prime_count);
    let bytes = num.as_bytes();
    if sum_values(factor_count) > bytes.len() as i32 {
        return construct(factor_count);
    }
    let mut prime_prefix = get_prime_count_from_string(bytes);
    let first_zero = bytes
        .iter()
        .position(|&c| c == b'0')
        .unwrap_or(bytes.len());
    if first_zero == bytes.len() && is_subset(prime_count, prime_prefix) {
        return num;
    }
    for i in (0..bytes.len()).rev() {
        let d = (bytes[i] - b'0') as usize;
        prime_prefix = subtract(prime_prefix, K_FACTOR[d]);
        let space = (bytes.len() - 1 - i) as i32;
        if i > first_zero {
            continue;
        }
        for bigger in (d + 1)..10 {
            let factors = get_factor_count(subtract(
                subtract(prime_count, prime_prefix),
                K_FACTOR[bigger],
            ));
            if sum_values(factors) <= space {
                let fill_ones = space - sum_values(factors);
                let mut ans = String::from_utf8(bytes[..i].to_vec()).unwrap();
                ans.push(char::from(b'0' + bigger as u8));
                for _ in 0..fill_ones {
                    ans.push('1');
                }
                ans.push_str(&construct(factors));
                return ans;
            }
        }
    }
    let factors = get_factor_count(prime_count);
    let fill_ones = bytes.len() as i32 + 1 - sum_values(factors);
    let mut ans = String::new();
    for _ in 0..fill_ones {
        ans.push('1');
    }
    ans.push_str(&construct(factors));
    ans
}

fn main() {
    println!("{}", smallest_number("1234".into(), 256));
}

#[cfg(test)]
mod tests {
    use super::smallest_number;

    #[test]
    fn example1() {
        assert_eq!(smallest_number("1234".into(), 256), "1488");
    }

    #[test]
    fn example2() {
        assert_eq!(smallest_number("12355".into(), 50), "12355");
    }

    #[test]
    fn example3() {
        assert_eq!(smallest_number("11111".into(), 26), "-1");
    }
}
