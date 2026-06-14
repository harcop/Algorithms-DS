/// LeetCode #1854 - Maximum Population Year
fn maximum_population(logs: Vec<Vec<i32>>) -> i32 {
    let mut d = vec![0i32; 101];
    let offset = 1950;
    for log in logs {
        let a = (log[0] - offset) as usize;
        let b = (log[1] - offset) as usize;
        d[a] += 1;
        d[b] -= 1;
    }
    let mut s = 0i32;
    let mut mx = 0i32;
    let mut year = 0i32;
    for (i, &x) in d.iter().enumerate() {
        s += x;
        if s > mx {
            mx = s;
            year = i as i32 + offset;
        }
    }
    year
}

fn main() {
    println!("{}", maximum_population(vec![vec![1993, 1999], vec![2000, 2010]]));
}

#[cfg(test)]
mod tests {
    use super::maximum_population;

    #[test]
    fn example_one() {
        assert_eq!(maximum_population(vec![vec![1993, 1999], vec![2000, 2010]]), 1993);
    }

    #[test]
    fn example_two() {
        assert_eq!(
            maximum_population(vec![vec![1950, 1961], vec![1960, 1971], vec![1970, 1981]]),
            1960
        );
    }
}
