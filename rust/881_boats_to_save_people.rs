/// LeetCode #881 - Boats to Save People
fn num_rescue_boats(people: Vec<i32>, limit: i32) -> i32 {
    let mut people = people;
    people.sort_unstable();
    let mut i = 0usize;
    let mut j = people.len();
    let mut boats = 0;
    while i < j {
        j -= 1;
        if people[i] + people[j] <= limit {
            i += 1;
        }
        boats += 1;
    }
    boats
}

fn main() {
    println!("{}", num_rescue_boats(vec![1, 2], 3));
}

#[cfg(test)]
mod tests {
    use super::num_rescue_boats;

    #[test]
    fn example_one() {
        assert_eq!(num_rescue_boats(vec![1, 2], 3), 1);
    }
}
