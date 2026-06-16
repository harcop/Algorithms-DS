/// LeetCode #1921 - Eliminate Maximum Number of Monsters
fn eliminate_maximum(dist: Vec<i32>, speed: Vec<i32>) -> i32 {
    let mut times: Vec<i32> = dist
        .iter()
        .zip(speed.iter())
        .map(|(&d, &s)| (d - 1) / s)
        .collect();
    times.sort_unstable();
    for (i, &t) in times.iter().enumerate() {
        if t < i as i32 {
            return i as i32;
        }
    }
    times.len() as i32
}

fn main() {
    println!("{}", eliminate_maximum(vec![1, 3, 4], vec![1, 1, 1]));
}

#[cfg(test)]
mod tests {
    use super::eliminate_maximum;

    #[test]
    fn example_one() {
        assert_eq!(eliminate_maximum(vec![1, 3, 4], vec![1, 1, 1]), 3);
    }

    #[test]
    fn example_two() {
        assert_eq!(eliminate_maximum(vec![1, 1, 2, 3], vec![1, 1, 1, 1]), 1);
    }
}
