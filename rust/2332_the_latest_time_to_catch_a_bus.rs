/// LeetCode #2332 - The Latest Time to Catch a Bus
fn latest_time_catch_the_bus(mut buses: Vec<i32>, mut passengers: Vec<i32>, capacity: i32) -> i32 {
    buses.sort_unstable();
    passengers.sort_unstable();
    let mut j = 0isize;
    let mut c = 0i32;

    for &t in &buses {
        c = capacity;
        while c > 0 && (j as usize) < passengers.len() && passengers[j as usize] <= t {
            c -= 1;
            j += 1;
        }
    }

    j -= 1;
    let mut ans = if c > 0 {
        *buses.last().unwrap()
    } else {
        passengers[j as usize]
    };
    while j >= 0 && passengers[j as usize] == ans {
        ans -= 1;
        j -= 1;
    }
    ans
}

fn main() {
    println!(
        "{}",
        latest_time_catch_the_bus(vec![10, 20], vec![2, 17, 18, 19], 2)
    );
}

#[cfg(test)]
mod tests {
    use super::latest_time_catch_the_bus;

    #[test]
    fn example_one() {
        assert_eq!(
            latest_time_catch_the_bus(vec![10, 20], vec![2, 17, 18, 19], 2),
            16
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            latest_time_catch_the_bus(
                vec![20, 30, 10],
                vec![19, 13, 26, 4, 25, 11, 21],
                2
            ),
            20
        );
    }
}
