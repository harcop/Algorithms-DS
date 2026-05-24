/// LeetCode #1276 - Number of Burgers with No Waste of Ingredients
fn num_of_burgers(tomato_slices: i32, cheese_slices: i32) -> Vec<i32> {
    let diff = tomato_slices - 2 * cheese_slices;
    if diff < 0 || diff % 2 != 0 {
        return vec![];
    }
    let jumbo = diff / 2;
    let small = cheese_slices - jumbo;
    if small >= 0 {
        vec![jumbo, small]
    } else {
        vec![]
    }
}

fn main() {
    println!("{:?}", num_of_burgers(16, 7));
}

#[cfg(test)]
mod tests {
    use super::num_of_burgers;

    #[test]
    fn example_one() {
        assert_eq!(num_of_burgers(16, 7), vec![1, 6]);
    }

    #[test]
    fn example_two() {
        assert_eq!(num_of_burgers(17, 4), Vec::<i32>::new());
    }

    #[test]
    fn example_three() {
        assert_eq!(num_of_burgers(4, 1), vec![0, 1]);
    }
}
