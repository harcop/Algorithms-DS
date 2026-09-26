/// LeetCode #3948 - Lexicographically Maximum MEX Array
fn maximum_mex_array(nums: Vec<i32>) -> Vec<i32> {
    let n = nums.len();
    let mut pos = vec![Vec::new(); n + 1];
    for (i, &x) in nums.iter().enumerate() {
        if (x as usize) <= n {
            pos[x as usize].push(i);
        }
    }
    let mut ptr = vec![0usize; n + 1];
    let mut ans = Vec::new();
    let mut start = 0usize;
    while start < n {
        let mut end = start;
        let mut v = 0usize;
        loop {
            while ptr[v] < pos[v].len() && pos[v][ptr[v]] < start {
                ptr[v] += 1;
            }
            if ptr[v] == pos[v].len() {
                break;
            }
            end = end.max(pos[v][ptr[v]]);
            v += 1;
        }
        ans.push(v as i32);
        start = end + 1;
    }
    ans
}

fn main() {
    println!("{:?}", maximum_mex_array(vec![0, 1, 0]));
}

#[cfg(test)]
mod tests {
    use super::maximum_mex_array;

    #[test]
    fn example1() {
        assert_eq!(maximum_mex_array(vec![0, 1, 0]), vec![2, 1]);
    }

    #[test]
    fn example2() {
        assert_eq!(maximum_mex_array(vec![1, 0, 2]), vec![3]);
    }

    #[test]
    fn example3() {
        assert_eq!(maximum_mex_array(vec![3, 1]), vec![0, 0]);
    }
}
