/// LeetCode #3767 - Maximize Points After Choosing K Tasks
fn max_points(technique1: Vec<i32>, technique2: Vec<i32>, k: i32) -> i64 {
    let n = technique1.len();
    let mut idx: Vec<usize> = (0..n).collect();
    idx.sort_by_key(|&i| technique2[i] - technique1[i]);
    let mut ans: i64 = technique2.iter().map(|&x| x as i64).sum();
    let k = k as usize;
    for &i in &idx[..k] {
        ans += (technique1[i] - technique2[i]) as i64;
    }
    for &i in &idx[k..] {
        if technique1[i] >= technique2[i] {
            ans += (technique1[i] - technique2[i]) as i64;
        }
    }
    ans
}

fn main() {
    println!("{}", max_points(vec![5, 2, 10], vec![10, 3, 8], 2));
}

#[cfg(test)]
mod tests {
    use super::max_points;

    #[test]
    fn example1() {
        assert_eq!(max_points(vec![5, 2, 10], vec![10, 3, 8], 2), 22);
    }

    #[test]
    fn example2() {
        assert_eq!(max_points(vec![10, 20, 30], vec![5, 15, 25], 2), 60);
    }

    #[test]
    fn example3() {
        assert_eq!(max_points(vec![1, 2, 3], vec![4, 5, 6], 0), 15);
    }
}
