/// LeetCode #1362 - Closest Divisors

fn closest_divisors(num: i32) -> Vec<i32> {
    let mut best = (1, num);
    for candidate in [num + 1, num + 2] {
        let mut d = 1i32;
        while d * d <= candidate {
            if candidate % d == 0 {
                let q = candidate / d;
                if q - d < best.1 - best.0 {
                    best = (d, q);
                }
            }
            d += 1;
        }
    }
    vec![best.0, best.1]
}

fn main() {
    println!("{:?}", closest_divisors(11));
}

#[cfg(test)]
mod tests {
    use super::closest_divisors;

    #[test]
    fn example_one() {
        assert_eq!(closest_divisors(11), vec![3, 4]);
    }

    #[test]
    fn example_two() {
        assert_eq!(closest_divisors(27), vec![4, 7]);
    }
}
