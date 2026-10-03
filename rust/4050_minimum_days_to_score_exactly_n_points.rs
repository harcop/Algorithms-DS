/// LeetCode #4050 - Minimum Days to Score Exactly N Points
fn min_days(n: i32) -> i32 {
    const MX: usize = 100_001;
    const INF: i32 = 1_000_000_000;
    let mut f = vec![INF; MX];
    f[0] = -1;
    for i in 1..MX {
        let mut j = 1i32;
        loop {
            let s = j * (j + 1) / 2;
            if s > i as i32 {
                break;
            }
            f[i] = f[i].min(f[i - s as usize] + j + 1);
            j += 1;
        }
    }
    f[n as usize]
}

fn main() {
    println!("{}", min_days(2));
}

#[cfg(test)]
mod tests {
    use super::min_days;

    #[test]
    fn example1() {
        assert_eq!(min_days(2), 3);
    }

    #[test]
    fn example2() {
        assert_eq!(min_days(9), 6);
    }

    #[test]
    fn example3() {
        assert_eq!(min_days(12), 7);
    }
}
