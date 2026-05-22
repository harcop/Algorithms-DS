/// LeetCode #1198 - Smallest Common Element in All Rows
fn smallest_common_element(mat: Vec<Vec<i32>>) -> i32 {
    let m = mat.len();
    let n = mat[0].len();
    let mut lo = mat[0][0];
    let mut hi = mat[0][n - 1];
    for row in &mat {
        lo = lo.min(row[0]);
        hi = hi.max(row[n - 1]);
    }
    let mut ans = -1;
    while lo <= hi {
        let mid = (lo + hi) / 2;
        let mut ok = true;
        for row in &mat {
            let mut found = false;
            let mut l = 0;
            let mut r = n;
            while l < r {
                let c = (l + r) / 2;
                if row[c] >= mid {
                    r = c;
                } else {
                    l = c + 1;
                }
            }
            if l < n && row[l] == mid {
                found = true;
            }
            if !found {
                ok = false;
                break;
            }
        }
        if ok {
            ans = mid;
            hi = mid - 1;
        } else {
            lo = mid + 1;
        }
    }
    ans
}

fn main() {
    println!(
        "{}",
        smallest_common_element(vec![vec![1, 2, 3], vec![2, 3, 4], vec![2, 3, 5]])
    );
}

#[cfg(test)]
mod tests {
    use super::smallest_common_element;

    #[test]
    fn example_one() {
        assert_eq!(
            smallest_common_element(vec![vec![1, 2, 3], vec![2, 3, 4], vec![2, 3, 5]]),
            2
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            smallest_common_element(vec![vec![1, 2, 3], vec![2, 3, 4], vec![3, 4, 5]]),
            3
        );
    }
}
