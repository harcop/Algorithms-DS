/// LeetCode #2105 - Watering Plants II
fn minimum_refill(plants: Vec<i32>, capacity_a: i32, capacity_b: i32) -> i32 {
    let mut i = 0usize;
    let mut j = plants.len() - 1;
    let mut a = capacity_a;
    let mut b = capacity_b;
    let mut ans = 0;

    while i < j {
        if a < plants[i] {
            ans += 1;
            a = capacity_a;
        }
        a -= plants[i];

        if b < plants[j] {
            ans += 1;
            b = capacity_b;
        }
        b -= plants[j];

        i += 1;
        j -= 1;
    }

    if i == j && a.max(b) < plants[i] {
        ans += 1;
    }
    ans
}

fn main() {
    println!("{}", minimum_refill(vec![2, 2, 3, 3], 5, 5));
}

#[cfg(test)]
mod tests {
    use super::minimum_refill;

    #[test]
    fn example_one() {
        assert_eq!(minimum_refill(vec![2, 2, 3, 3], 5, 5), 1);
    }

    #[test]
    fn example_two() {
        assert_eq!(minimum_refill(vec![2, 2, 3, 3], 3, 4), 2);
    }

    #[test]
    fn example_three() {
        assert_eq!(minimum_refill(vec![5], 10, 8), 0);
    }
}
