/// LeetCode #134 - Gas Station
fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
    let n = gas.len();
    let mut total = 0;
    let mut tank = 0;
    let mut start = 0i32;
    for i in 0..n {
        let diff = gas[i] - cost[i];
        total += diff;
        tank += diff;
        if tank < 0 {
            start = (i + 1) as i32;
            tank = 0;
        }
    }
    if total < 0 {
        -1
    } else {
        start
    }
}

fn main() {
    println!(
        "{}",
        can_complete_circuit(vec![1, 2, 3, 4, 5], vec![3, 4, 5, 1, 2])
    );
}

#[cfg(test)]
mod tests {
    use super::can_complete_circuit;

    #[test]
    fn example_one() {
        assert_eq!(
            can_complete_circuit(vec![1, 2, 3, 4, 5], vec![3, 4, 5, 1, 2]),
            3
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            can_complete_circuit(vec![2, 3, 4], vec![3, 4, 3]),
            -1
        );
    }
}
