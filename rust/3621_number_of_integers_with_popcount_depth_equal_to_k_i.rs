/// LeetCode #3621 - Number of Integers With Popcount-Depth Equal to K I
fn comb_table() -> [[i64; 64]; 64] {
    let mut c = [[0i64; 64]; 64];
    for i in 0..64 {
        c[i][0] = 1;
        for j in 1..=i {
            c[i][j] = c[i - 1][j - 1] + c[i - 1][j];
        }
    }
    c
}

fn count_le_with_bits(n: i64, bits: i32, c: &[[i64; 64]; 64]) -> i64 {
    if bits < 0 {
        return 0;
    }
    let mut ans = 0i64;
    let mut used = 0i32;
    for i in (0..63).rev() {
        if (n >> i) & 1 == 1 {
            let need = bits - used;
            if need >= 0 && (need as usize) <= i {
                ans += c[i][need as usize];
            }
            used += 1;
            if used > bits {
                break;
            }
        }
    }
    if used == bits {
        ans += 1;
    }
    ans
}

fn popcount_depth(n: i64, k: i32) -> i64 {
    if k == 0 {
        return if n >= 1 { 1 } else { 0 };
    }
    let c = comb_table();
    let mut depth_val = [0i32; 64];
    depth_val[1] = 0;
    for b in 2..64 {
        depth_val[b] = 1 + depth_val[b.count_ones() as usize];
    }
    let mut ans = 0i64;
    for b in 1..64 {
        if depth_val[b] == k - 1 {
            ans += count_le_with_bits(n, b as i32, &c);
        }
    }
    if k == 1 {
        ans -= 1;
    }
    ans
}

fn main() {
    println!("{}", popcount_depth(4, 1));
}

#[cfg(test)]
mod tests {
    use super::popcount_depth;

    #[test]
    fn example1() {
        assert_eq!(popcount_depth(4, 1), 2);
    }

    #[test]
    fn example2() {
        assert_eq!(popcount_depth(7, 2), 3);
    }

    #[test]
    fn k_zero() {
        assert_eq!(popcount_depth(10, 0), 1);
    }
}
