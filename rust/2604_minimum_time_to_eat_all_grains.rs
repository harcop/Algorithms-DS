/// LeetCode #2604 - Minimum Time to Eat All Grains
fn minimum_time(mut hens: Vec<i32>, mut grains: Vec<i32>) -> i32 {
    hens.sort_unstable();
    grains.sort_unstable();
    let m = grains.len();
    let mut l = 0i32;
    let mut r = (hens[0] - grains[0]).abs() + grains[m - 1] - grains[0];

    let check = |t: i32| -> bool {
        let mut j = 0usize;
        for &x in &hens {
            if j == m {
                return true;
            }
            let y = grains[j];
            if y <= x {
                let d = x - y;
                if d > t {
                    return false;
                }
                while j < m && grains[j] <= x {
                    j += 1;
                }
                while j < m && d.min(grains[j] - x) + grains[j] - y <= t {
                    j += 1;
                }
            } else {
                while j < m && grains[j] - x <= t {
                    j += 1;
                }
            }
        }
        j == m
    };

    while l < r {
        let mid = (l + r) >> 1;
        if check(mid) {
            r = mid;
        } else {
            l = mid + 1;
        }
    }
    l
}

fn main() {
    println!("{}", minimum_time(vec![3, 6, 7], vec![2, 4, 7, 9]));
}

#[cfg(test)]
mod tests {
    use super::minimum_time;

    #[test]
    fn example_one() {
        assert_eq!(minimum_time(vec![3, 6, 7], vec![2, 4, 7, 9]), 2);
    }

    #[test]
    fn example_two() {
        assert_eq!(
            minimum_time(vec![4, 6, 109, 111, 213, 215], vec![5, 110, 214]),
            1
        );
    }
}
