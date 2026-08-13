/// LeetCode #3181 - Maximum Total Reward Using Operations II
fn max_total_reward(mut reward_values: Vec<i32>) -> i32 {
    reward_values.sort_unstable();
    reward_values.dedup();
    let mx = *reward_values.last().unwrap() as usize;
    let nbits = mx * 2 + 2;
    let nwords = (nbits + 63) / 64;
    let mut f = vec![0u64; nwords];
    f[0] = 1;
    for &v in &reward_values {
        let v = v as usize;
        let mut add = vec![0u64; nwords];
        for i in 0..nwords {
            let start = i * 64;
            if start >= v {
                break;
            }
            let mut w = f[i];
            if start + 64 > v {
                let keep = v - start;
                w = if keep == 0 { 0 } else { w & ((1u64 << keep) - 1) };
            }
            if w == 0 {
                continue;
            }
            let ni = i + v / 64;
            let sh = v % 64;
            if ni < nwords {
                add[ni] |= w << sh;
            }
            if sh != 0 && ni + 1 < nwords {
                add[ni + 1] |= w >> (64 - sh);
            }
        }
        for i in 0..nwords {
            f[i] |= add[i];
        }
    }
    for i in (0..nwords).rev() {
        if f[i] != 0 {
            return (i * 64 + 63 - f[i].leading_zeros() as usize) as i32;
        }
    }
    0
}

fn main() {
    println!("{}", max_total_reward(vec![1, 1, 3, 3]));
}

#[cfg(test)]
mod tests {
    use super::max_total_reward;

    #[test]
    fn example1() {
        assert_eq!(max_total_reward(vec![1, 1, 3, 3]), 4);
    }

    #[test]
    fn example2() {
        assert_eq!(max_total_reward(vec![1, 6, 4, 3, 2]), 11);
    }
}
