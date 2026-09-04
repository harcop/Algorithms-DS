/// LeetCode #546 - Remove Boxes
fn remove_boxes(boxes: Vec<i32>) -> i32 {
    let n = boxes.len();
    if n == 0 {
        return 0;
    }
    let mut memo = vec![vec![vec![-1i32; n]; n]; n];
    fn dfs(boxes: &[i32], memo: &mut Vec<Vec<Vec<i32>>>, l: i32, r: i32, k: i32) -> i32 {
        if l > r {
            return 0;
        }
        let (lu, ru, ku) = (l as usize, r as usize, k as usize);
        if memo[lu][ru][ku] != -1 {
            return memo[lu][ru][ku];
        }
        let mut rr = r;
        let mut kk = k;
        while rr > l && boxes[rr as usize - 1] == boxes[rr as usize] {
            rr -= 1;
            kk += 1;
        }
        let mut ans = dfs(boxes, memo, l, rr - 1, 0) + (kk + 1) * (kk + 1);
        for i in l..rr {
            if boxes[i as usize] == boxes[rr as usize]
                && (i == l || boxes[i as usize] != boxes[i as usize - 1])
            {
                ans = ans.max(dfs(boxes, memo, l, i, kk + 1) + dfs(boxes, memo, i + 1, rr - 1, 0));
            }
        }
        memo[lu][ru][ku] = ans;
        ans
    }
    dfs(&boxes, &mut memo, 0, n as i32 - 1, 0)
}

fn main() {
    println!("{}", remove_boxes(vec![1, 3, 2, 2, 2, 3, 4, 3, 1]));
}

#[cfg(test)]
mod tests {
    use super::remove_boxes;

    #[test]
    fn example_one() {
        assert_eq!(remove_boxes(vec![1, 3, 2, 2, 2, 3, 4, 3, 1]), 23);
    }

    #[test]
    fn example_two() {
        assert_eq!(remove_boxes(vec![1, 1, 1]), 9);
    }

    #[test]
    fn example_three() {
        assert_eq!(remove_boxes(vec![1]), 1);
    }
}
