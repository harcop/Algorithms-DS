/// LeetCode #4041 - Minimum Operations to Form Subset Sum II
fn min_operations(nums: Vec<i32>, target: i32) -> i32 {
    let target = target as usize;
    let inf = 1_000_000_000i32;
    let mut f = vec![inf; target + 1];
    f[0] = 0;
    let cap = target as i32 / 2;
    for x in nums {
        for w in (0..=target).rev() {
            let mut cost = 0i32;
            let mut y = x;
            while y > 0 && (y as usize) <= w {
                f[w] = f[w].min(f[w - y as usize] + cost);
                if y > cap {
                    break;
                }
                y *= 2;
                cost += 1;
            }
            cost = 1;
            y = x / 2;
            while y > 0 {
                let mut extra = 0i32;
                let mut z = y;
                while z > 0 && (z as usize) <= w {
                    f[w] = f[w].min(f[w - z as usize] + cost + extra);
                    if z > cap {
                        break;
                    }
                    z *= 2;
                    extra += 1;
                }
                cost += 1;
                y /= 2;
            }
        }
    }
    if f[target] >= inf { -1 } else { f[target] }
}

fn main() {
    println!("{}", min_operations(vec![10, 2], 13));
}

#[cfg(test)]
mod tests {
    use super::min_operations;

    #[test]
    fn example1() {
        assert_eq!(min_operations(vec![10, 2], 13), 3);
    }

    #[test]
    fn example2() {
        assert_eq!(min_operations(vec![6, 3], 8), 2);
    }

    #[test]
    fn example3() {
        assert_eq!(min_operations(vec![2, 2], 7), -1);
    }
}
