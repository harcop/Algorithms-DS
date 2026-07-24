/// LeetCode #2655 - Find Maximal Uncovered Ranges
fn find_maximal_uncovered_ranges(n: i32, mut ranges: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    ranges.sort_by_key(|x| x[0]);
    let mut last = -1;
    let mut ans = Vec::new();
    for range in ranges {
        let l = range[0];
        let r = range[1];
        if last + 1 < l {
            ans.push(vec![last + 1, l - 1]);
        }
        last = last.max(r);
    }
    if last + 1 < n {
        ans.push(vec![last + 1, n - 1]);
    }
    ans
}

fn main() {
    println!(
        "{:?}",
        find_maximal_uncovered_ranges(10, vec![vec![3, 5], vec![7, 8]])
    );
}

#[cfg(test)]
mod tests {
    use super::find_maximal_uncovered_ranges;

    #[test]
    fn example_one() {
        assert_eq!(
            find_maximal_uncovered_ranges(10, vec![vec![3, 5], vec![7, 8]]),
            vec![vec![0, 2], vec![6, 6], vec![9, 9]]
        );
    }

    #[test]
    fn example_two() {
        assert!(find_maximal_uncovered_ranges(3, vec![vec![0, 2]]).is_empty());
    }

    #[test]
    fn example_three() {
        assert_eq!(
            find_maximal_uncovered_ranges(7, vec![vec![2, 4], vec![0, 3]]),
            vec![vec![5, 6]]
        );
    }
}
