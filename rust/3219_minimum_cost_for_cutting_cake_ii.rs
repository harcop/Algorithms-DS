/// LeetCode #3219 - Minimum Cost for Cutting Cake II
fn minimum_cost(m: i32, n: i32, mut horizontal_cut: Vec<i32>, mut vertical_cut: Vec<i32>) -> i64 {
    horizontal_cut.sort();
    vertical_cut.sort();
    let (mut i, mut j) = ((m - 2) as isize, (n - 2) as isize);
    let (mut h, mut v) = (1_i64, 1_i64);
    let mut ans: i64 = 0;

    while i >= 0 || j >= 0 {
        if j < 0 || (i >= 0 && horizontal_cut[i as usize] > vertical_cut[j as usize]) {
            ans += horizontal_cut[i as usize] as i64 * v;
            i -= 1;
            h += 1;
        } else {
            ans += vertical_cut[j as usize] as i64 * h;
            j -= 1;
            v += 1;
        }
    }

    ans
}

fn main() {
    println!("{}", minimum_cost(3, 2, vec![1, 3], vec![5]));
}

#[cfg(test)]
mod tests {
    use super::minimum_cost;

    #[test]
    fn example1() {
        assert_eq!(minimum_cost(3, 2, vec![1, 3], vec![5]), 13);
    }

    #[test]
    fn example2() {
        assert_eq!(minimum_cost(2, 2, vec![7], vec![4]), 15);
    }
}
