/// LeetCode #781 - Rabbits in Forest
use std::collections::HashMap;

fn num_rabbits(answers: Vec<i32>) -> i32 {
    let mut cnt: HashMap<i32, i32> = HashMap::new();
    for a in answers {
        *cnt.entry(a).or_insert(0) += 1;
    }
    let mut ans = 0i32;
    for (&k, &c) in &cnt {
        let group = (k + 1) as i32;
        ans += ((c + group - 1) / group) * group;
    }
    ans
}

fn main() {
    println!("{}", num_rabbits(vec![1, 1, 2]));
}

#[cfg(test)]
mod tests {
    use super::num_rabbits;

    #[test]
    fn example_one() {
        assert_eq!(num_rabbits(vec![1, 1, 2]), 5);
    }

    #[test]
    fn example_two() {
        assert_eq!(num_rabbits(vec![10, 10, 10]), 11);
    }
}
