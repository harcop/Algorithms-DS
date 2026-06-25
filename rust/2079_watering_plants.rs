/// LeetCode #2079 - Watering Plants
fn watering_plants(plants: Vec<i32>, capacity: i32) -> i32 {
    let mut water = capacity;
    let mut steps = 0;

    for (i, plant) in plants.into_iter().enumerate() {
        if water < plant {
            steps += (i as i32) * 2;
            water = capacity;
        }
        water -= plant;
        steps += 1;
    }

    steps
}

fn main() {
    println!("{}", watering_plants(vec![2, 2, 3, 3], 5));
}

#[cfg(test)]
mod tests {
    use super::watering_plants;

    #[test]
    fn example_one() {
        assert_eq!(watering_plants(vec![2, 2, 3, 3], 5), 14);
    }

    #[test]
    fn example_two() {
        assert_eq!(watering_plants(vec![1, 1, 1, 4, 2, 3], 4), 30);
    }

    #[test]
    fn example_three() {
        assert_eq!(watering_plants(vec![7, 7, 7, 7, 7, 7, 7], 8), 49);
    }
}
