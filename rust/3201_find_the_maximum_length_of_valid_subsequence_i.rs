/// LeetCode #3201 - Find the Maximum Length of Valid Subsequence I
fn maximum_length(nums: Vec<i32>) -> i32 {
    let mut f = [[0i32; 2]; 2];
    let mut ans = 0;
    for x in nums {
        let x = (x % 2) as usize;
        for j in 0..2 {
            let y = (j + 2 - x) % 2;
            f[x][y] = f[y][x] + 1;
            ans = ans.max(f[x][y]);
        }
    }
    ans
}

fn main() {
    println!("{}", maximum_length(vec![1, 2, 3, 4]));
}

#[cfg(test)]
mod tests {
    use super::maximum_length;

    #[test]
    fn example1() {
        assert_eq!(maximum_length(vec![1, 2, 3, 4]), 4);
    }

    #[test]
    fn example2() {
        assert_eq!(maximum_length(vec![1, 2, 1, 1, 2, 1, 2]), 6);
    }

    #[test]
    fn example3() {
        assert_eq!(maximum_length(vec![1, 3]), 2);
    }
}
