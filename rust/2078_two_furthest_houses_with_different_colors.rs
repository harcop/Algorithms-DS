/// LeetCode #2078 - Two Furthest Houses With Different Colors
fn max_distance(colors: Vec<i32>) -> i32 {
    let n = colors.len();
    let mut ans = 0usize;

    for i in 0..n {
        for j in i + 1..n {
            if colors[i] != colors[j] {
                ans = ans.max(j - i);
            }
        }
    }

    ans as i32
}

fn main() {
    println!("{}", max_distance(vec![1, 1, 1, 6, 1, 1, 1]));
}

#[cfg(test)]
mod tests {
    use super::max_distance;

    #[test]
    fn example_one() {
        assert_eq!(max_distance(vec![1, 1, 1, 6, 1, 1, 1]), 3);
    }

    #[test]
    fn example_two() {
        assert_eq!(max_distance(vec![1, 8, 3, 8, 3]), 4);
    }

    #[test]
    fn example_three() {
        assert_eq!(max_distance(vec![0, 1]), 1);
    }
}
