/// LeetCode #3096 - Minimum Levels to Gain More Points
fn minimum_levels(possible: Vec<i32>) -> i32 {
    let s: i32 = possible.iter().map(|&x| if x == 0 { -1 } else { 1 }).sum();
    let mut t = 0;
    for i in 1..possible.len() {
        t += if possible[i - 1] == 0 { -1 } else { 1 };
        if t > s - t {
            return i as i32;
        }
    }
    -1
}

fn main() {
    println!("{}", minimum_levels(vec![1, 0, 1, 0]));
}

#[cfg(test)]
mod tests {
    use super::minimum_levels;

    #[test]
    fn example1() {
        assert_eq!(minimum_levels(vec![1, 0, 1, 0]), 1);
    }

    #[test]
    fn example2() {
        assert_eq!(minimum_levels(vec![1, 1, 1, 1, 1]), 3);
    }

    #[test]
    fn example3() {
        assert_eq!(minimum_levels(vec![0, 0]), -1);
    }
}
