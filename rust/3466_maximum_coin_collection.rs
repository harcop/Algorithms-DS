/// LeetCode #3466 - Maximum Coin Collection
fn max_coins(lane1: Vec<i32>, lane2: Vec<i32>) -> i64 {
    const NEG: i64 = i64::MIN / 4;
    let n = lane1.len();
    let mut best = [[NEG; 2]; 3];
    let mut ans = NEG;
    for i in 0..n {
        let v = [lane1[i] as i64, lane2[i] as i64];
        let mut cur = [[NEG; 2]; 3];
        cur[0][0] = v[0];
        cur[1][1] = v[1];
        for sw in 0..3 {
            for lane in 0..2 {
                if best[sw][lane] == NEG {
                    continue;
                }
                cur[sw][lane] = cur[sw][lane].max(best[sw][lane] + v[lane]);
                if sw + 1 < 3 {
                    cur[sw + 1][lane ^ 1] =
                        cur[sw + 1][lane ^ 1].max(best[sw][lane] + v[lane ^ 1]);
                }
            }
        }
        best = cur;
        for sw in 0..3 {
            for lane in 0..2 {
                ans = ans.max(best[sw][lane]);
            }
        }
    }
    ans
}

fn main() {
    println!("{}", max_coins(vec![1, -2, -10, 3], vec![-5, 10, 0, 1]));
}

#[cfg(test)]
mod tests {
    use super::max_coins;

    #[test]
    fn example1() {
        assert_eq!(max_coins(vec![1, -2, -10, 3], vec![-5, 10, 0, 1]), 14);
    }

    #[test]
    fn example2() {
        assert_eq!(max_coins(vec![1, -1, -1, -1], vec![0, 3, 4, -5]), 8);
    }

    #[test]
    fn example3() {
        assert_eq!(max_coins(vec![-5, -4, -3], vec![-1, 2, 3]), 5);
    }

    #[test]
    fn example4() {
        assert_eq!(max_coins(vec![-3, -3, -3], vec![9, -2, 4]), 11);
    }

    #[test]
    fn example5() {
        assert_eq!(max_coins(vec![-10], vec![-2]), -2);
    }
}
