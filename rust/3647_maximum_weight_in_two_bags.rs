/// LeetCode #3647 - Maximum Weight in Two Bags (premium)
fn max_weight(weights: Vec<i32>, w1: i32, w2: i32) -> i32 {
    let w1 = w1 as usize;
    let w2 = w2 as usize;
    let mut f = vec![vec![0; w2 + 1]; w1 + 1];
    for &x in &weights {
        let x = x as usize;
        for j in (0..=w1).rev() {
            for k in (0..=w2).rev() {
                if x <= j {
                    f[j][k] = f[j][k].max(f[j - x][k] + x as i32);
                }
                if x <= k {
                    f[j][k] = f[j][k].max(f[j][k - x] + x as i32);
                }
            }
        }
    }
    f[w1][w2]
}

fn main() {
    println!("{}", max_weight(vec![1, 4, 3, 2], 5, 4));
}

#[cfg(test)]
mod tests {
    use super::max_weight;

    #[test]
    fn example1() {
        assert_eq!(max_weight(vec![1, 4, 3, 2], 5, 4), 9);
    }

    #[test]
    fn example2() {
        assert_eq!(max_weight(vec![3, 6, 4, 8], 9, 7), 15);
    }

    #[test]
    fn example3() {
        assert_eq!(max_weight(vec![5, 7], 2, 3), 0);
    }
}
