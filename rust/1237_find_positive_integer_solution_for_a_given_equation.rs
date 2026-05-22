/// LeetCode #1237 - Find Positive Integer Solution for a Given Equation
fn find_solution(f: impl Fn(i32, i32) -> i32, z: i32) -> Vec<i32> {
    for x in 1..=z {
        let mut lo = 1i32;
        let mut hi = z;
        while lo < hi {
            let mid = lo + (hi - lo) / 2;
            if f(x, mid) < z {
                lo = mid + 1;
            } else {
                hi = mid;
            }
        }
        if f(x, lo) == z {
            return vec![x, lo];
        }
    }
    vec![1, 1]
}

fn main() {
    println!("{:?}", find_solution(|x, y| x + y, 5));
}

#[cfg(test)]
mod tests {
    use super::find_solution;

    #[test]
    fn example_one() {
        assert_eq!(find_solution(|x, y| x + y, 5), vec![1, 4]);
    }

    #[test]
    fn example_two() {
        assert_eq!(find_solution(|x, y| x * y, 6), vec![1, 6]);
    }

    #[test]
    fn example_three() {
        assert_eq!(find_solution(|x, y| x * y, 12), vec![1, 12]);
    }
}
