/// LeetCode #2021 - Brightest Position on Street
use std::collections::HashMap;

fn brightest_position(lights: Vec<Vec<i32>>) -> i32 {
    let mut diff = HashMap::new();
    for light in lights {
        let (i, j) = (light[0], light[1]);
        let l = i - j;
        let r = i + j + 1;
        *diff.entry(l).or_insert(0) += 1;
        *diff.entry(r).or_insert(0) -= 1;
    }

    let mut keys: Vec<i32> = diff.keys().copied().collect();
    keys.sort_unstable();

    let mut ans = 0;
    let mut s = 0;
    let mut mx = 0;
    for k in keys {
        s += diff.get(&k).copied().unwrap_or(0);
        if s > mx {
            mx = s;
            ans = k;
        }
    }
    ans
}

fn main() {
    println!("{}", brightest_position(vec![vec![-3, 2], vec![1, 2], vec![3, 3]]));
}

#[cfg(test)]
mod tests {
    use super::brightest_position;

    #[test]
    fn example_one() {
        assert_eq!(
            brightest_position(vec![vec![-3, 2], vec![1, 2], vec![3, 3]]),
            -1
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(brightest_position(vec![vec![1, 0], vec![0, 1]]), 1);
    }

    #[test]
    fn example_three() {
        assert_eq!(brightest_position(vec![vec![1, 2]]), -1);
    }
}
