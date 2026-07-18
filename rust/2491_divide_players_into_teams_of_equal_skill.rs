/// LeetCode #2491 - Divide Players Into Teams of Equal Skill
use std::collections::HashMap;

fn divide_players(skill: Vec<i32>) -> i64 {
    let n = skill.len();
    let total: i64 = skill.iter().map(|&s| s as i64).sum();
    if total % (n as i64 / 2) != 0 {
        return -1;
    }
    let team_skill = (total / (n as i64 / 2)) as i32;
    let mut count = HashMap::new();
    for s in skill {
        *count.entry(s).or_insert(0) += 1;
    }

    let mut answer = 0i64;
    for (&s, &freq) in &count {
        let required = team_skill - s;
        if count.get(&required).copied().unwrap_or(0) != freq {
            return -1;
        }
        answer += s as i64 * required as i64 * freq as i64;
    }

    answer / 2
}

fn main() {
    println!("{}", divide_players(vec![3, 2, 5, 1, 3, 4]));
}

#[cfg(test)]
mod tests {
    use super::divide_players;

    #[test]
    fn example_one() {
        assert_eq!(divide_players(vec![3, 2, 5, 1, 3, 4]), 22);
    }

    #[test]
    fn example_two() {
        assert_eq!(divide_players(vec![3, 4]), 12);
    }

    #[test]
    fn example_three() {
        assert_eq!(divide_players(vec![1, 1, 2, 3]), -1);
    }
}
