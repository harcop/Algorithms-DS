/// LeetCode #4066 - Maximum Equal Adjacent Pairs After at Most One Replacement
fn max_equal_adjacent_pairs(nums: Vec<i32>) -> i32 {
    let mut cnt = std::collections::HashMap::<u64, i32>::new();
    let mut equal = 0i32;
    let mut gained = 0i32;
    for w in nums.windows(2) {
        let (mut x, mut y) = (w[0], w[1]);
        if x == y {
            equal += 1;
            continue;
        }
        if x > y {
            std::mem::swap(&mut x, &mut y);
        }
        let key = ((x as u64) << 30) | y as u64;
        let v = cnt.entry(key).or_insert(0);
        *v += 1;
        gained = gained.max(*v);
    }
    equal + gained
}

fn main() {
    println!("{}", max_equal_adjacent_pairs(vec![1, 2, 3, 2]));
}

#[cfg(test)]
mod tests {
    use super::max_equal_adjacent_pairs;

    #[test]
    fn example1() {
        assert_eq!(max_equal_adjacent_pairs(vec![1, 2, 3, 2]), 2);
    }

    #[test]
    fn example2() {
        assert_eq!(max_equal_adjacent_pairs(vec![1, 2, 1, 2, 1]), 4);
    }

    #[test]
    fn example3() {
        assert_eq!(max_equal_adjacent_pairs(vec![1, 1, 1]), 2);
    }
}
