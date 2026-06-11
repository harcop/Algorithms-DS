/// LeetCode #1826 - Faulty Sensor
fn bad_sensor(sensor1: Vec<i32>, sensor2: Vec<i32>) -> i32 {
    let n = sensor1.len();
    let mut i = 0usize;
    while i < n - 1 && sensor1[i] == sensor2[i] {
        i += 1;
    }
    while i < n - 1 {
        if sensor1[i + 1] != sensor2[i] {
            return 1;
        }
        if sensor1[i] != sensor2[i + 1] {
            return 2;
        }
        i += 1;
    }
    -1
}

fn main() {
    println!("{}", bad_sensor(vec![2, 3, 4, 5], vec![2, 1, 3, 4]));
}

#[cfg(test)]
mod tests {
    use super::bad_sensor;

    #[test]
    fn example_one() {
        assert_eq!(bad_sensor(vec![2, 3, 4, 5], vec![2, 1, 3, 4]), 1);
    }

    #[test]
    fn example_two() {
        assert_eq!(bad_sensor(vec![2, 2, 2, 2, 2], vec![2, 2, 2, 2, 5]), -1);
    }
}
