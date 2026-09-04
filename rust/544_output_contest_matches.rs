/// LeetCode #544 - Output Contest Matches
fn find_contest_match(n: i32) -> String {
    let mut teams: Vec<String> = (1..=n).map(|i| i.to_string()).collect();
    while teams.len() > 1 {
        let m = teams.len();
        let mut next = Vec::with_capacity(m / 2);
        for i in 0..m / 2 {
            next.push(format!("({},{})", teams[i], teams[m - 1 - i]));
        }
        teams = next;
    }
    teams.pop().unwrap()
}

fn main() {
    println!("{}", find_contest_match(8));
}

#[cfg(test)]
mod tests {
    use super::find_contest_match;

    #[test]
    fn example_one() {
        assert_eq!(find_contest_match(2), "(1,2)");
    }

    #[test]
    fn example_two() {
        assert_eq!(find_contest_match(4), "((1,4),(2,3))");
    }

    #[test]
    fn example_three() {
        assert_eq!(
            find_contest_match(8),
            "(((1,8),(4,5)),((2,7),(3,6)))"
        );
    }
}
