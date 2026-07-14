/// LeetCode #2383 - Minimum Hours of Training to Win a Competition
fn min_number_of_hours(
    mut initial_energy: i32,
    mut initial_experience: i32,
    energy: Vec<i32>,
    experience: Vec<i32>,
) -> i32 {
    let mut ans = 0;
    for (&dx, &dy) in energy.iter().zip(experience.iter()) {
        if initial_energy <= dx {
            ans += dx + 1 - initial_energy;
            initial_energy = dx + 1;
        }
        if initial_experience <= dy {
            ans += dy + 1 - initial_experience;
            initial_experience = dy + 1;
        }
        initial_energy -= dx;
        initial_experience += dy;
    }
    ans
}

fn main() {
    println!(
        "{}",
        min_number_of_hours(5, 3, vec![1, 4, 3, 2], vec![2, 6, 3, 1])
    );
}

#[cfg(test)]
mod tests {
    use super::min_number_of_hours;

    #[test]
    fn example_one() {
        assert_eq!(
            min_number_of_hours(5, 3, vec![1, 4, 3, 2], vec![2, 6, 3, 1]),
            8
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(min_number_of_hours(2, 4, vec![1], vec![3]), 0);
    }
}
