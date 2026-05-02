/// LeetCode #163 - Missing Ranges
fn find_missing_ranges(nums: Vec<i32>, lower: i32, upper: i32) -> Vec<String> {
    let mut out = Vec::new();
    let mut prev = lower - 1;
    let mut push = |lo: i32, hi: i32| {
        if lo > hi {
            return;
        }
        if lo == hi {
            out.push(lo.to_string());
        } else {
            out.push(format!("{}->{}", lo, hi));
        }
    };
    for &x in &nums {
        if x as i64 > prev as i64 + 1 {
            push(prev + 1, x - 1);
        }
        prev = x;
    }
    if upper as i64 > prev as i64 {
        push(prev + 1, upper);
    }
    out
}

fn main() {
    println!("{:?}", find_missing_ranges(vec![0, 1, 3, 50, 75], 0, 99));
}

#[cfg(test)]
mod tests {
    use super::find_missing_ranges;

    #[test]
    fn example_one() {
        assert_eq!(
            find_missing_ranges(vec![0, 1, 3, 50, 75], 0, 99),
            vec!["2", "4->49", "51->74", "76->99"]
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(find_missing_ranges(vec![], 1, 1), vec!["1"]);
    }
}
