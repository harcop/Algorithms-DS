/// LeetCode #1936 - Add Minimum Number of Rungs
fn add_rungs(rungs: Vec<i32>, dist: i32) -> i32 {
    let mut ans = 0i32;
    let mut prev = 0i32;
    for &r in &rungs {
        if r - prev > dist {
            ans += (r - prev - 1) / dist;
        }
        prev = r;
    }
    ans
}

fn main() {
    println!("{}", add_rungs(vec![1, 3, 5, 10], 2));
}

#[cfg(test)]
mod tests {
    use super::add_rungs;

    #[test]
    fn example_one() {
        assert_eq!(add_rungs(vec![1, 3, 5, 10], 2), 2);
    }

    #[test]
    fn example_two() {
        assert_eq!(add_rungs(vec![3, 6, 8, 10], 3), 0);
    }

    #[test]
    fn example_three() {
        assert_eq!(add_rungs(vec![3, 4, 6, 7], 2), 1);
    }
}
