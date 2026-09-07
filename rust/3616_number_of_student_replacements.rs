/// LeetCode #3616 - Number of Student Replacements
fn total_replacements(ranks: Vec<i32>) -> i32 {
    let mut ans = 0;
    let mut cur = ranks[0];
    for x in ranks {
        if x < cur {
            cur = x;
            ans += 1;
        }
    }
    ans
}

fn main() {
    println!("{}", total_replacements(vec![4, 1, 2]));
}

#[cfg(test)]
mod tests {
    use super::total_replacements;

    #[test]
    fn example1() {
        assert_eq!(total_replacements(vec![4, 1, 2]), 1);
    }

    #[test]
    fn example2() {
        assert_eq!(total_replacements(vec![2, 2, 3]), 0);
    }
}
