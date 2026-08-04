/// LeetCode #2961 - Double Modular Exponentiation
fn mod_pow(mut base: i64, mut exp: i32, m: i64) -> i64 {
    if m == 1 {
        return 0;
    }
    let mut res = 1i64;
    base %= m;
    while exp > 0 {
        if exp & 1 == 1 {
            res = res * base % m;
        }
        base = base * base % m;
        exp >>= 1;
    }
    res
}

fn get_good_indices(variables: Vec<Vec<i32>>, target: i32) -> Vec<i32> {
    variables
        .iter()
        .enumerate()
        .filter_map(|(i, v)| {
            let (a, b, c, m) = (v[0] as i64, v[1], v[2], v[3] as i64);
            let val = mod_pow(mod_pow(a, b, 10), c, m);
            if val == target as i64 {
                Some(i as i32)
            } else {
                None
            }
        })
        .collect()
}

fn main() {
    println!(
        "{:?}",
        get_good_indices(vec![vec![2, 3, 3, 10], vec![3, 3, 3, 1], vec![6, 1, 1, 4]], 2)
    );
}

#[cfg(test)]
mod tests {
    use super::get_good_indices;

    #[test]
    fn example_one() {
        assert_eq!(
            get_good_indices(vec![vec![2, 3, 3, 10], vec![3, 3, 3, 1], vec![6, 1, 1, 4]], 2),
            vec![0, 2]
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            get_good_indices(vec![vec![39, 3, 1000, 1000]], 17),
            Vec::<i32>::new()
        );
    }
}
