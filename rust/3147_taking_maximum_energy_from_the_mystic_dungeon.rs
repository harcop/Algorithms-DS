/// LeetCode #3147 - Taking Maximum Energy From the Mystic Dungeon
fn maximum_energy(energy: Vec<i32>, k: i32) -> i32 {
    let k = k as usize;
    let n = energy.len();
    let mut ans = i32::MIN;
    for i in (n - k)..n {
        let mut j = i as isize;
        let mut s = 0;
        while j >= 0 {
            s += energy[j as usize];
            ans = ans.max(s);
            j -= k as isize;
        }
    }
    ans
}

fn main() {
    println!("{}", maximum_energy(vec![5, 2, -10, -5, 1], 3));
}

#[cfg(test)]
mod tests {
    use super::maximum_energy;

    #[test]
    fn example1() {
        assert_eq!(maximum_energy(vec![5, 2, -10, -5, 1], 3), 3);
    }

    #[test]
    fn example2() {
        assert_eq!(maximum_energy(vec![-2, -3, -1], 2), -1);
    }
}
