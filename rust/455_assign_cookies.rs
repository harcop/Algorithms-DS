/// LeetCode #455 - Assign Cookies
fn find_content_children(mut g: Vec<i32>, mut s: Vec<i32>) -> i32 {
    g.sort_unstable();
    s.sort_unstable();
    let mut i = 0usize;
    let mut j = 0usize;
    while i < g.len() && j < s.len() {
        if s[j] >= g[i] {
            i += 1;
        }
        j += 1;
    }
    i as i32
}

fn main() {
    println!("{}", find_content_children(vec![1, 2, 3], vec![1, 1]));
}

#[cfg(test)]
mod tests {
    use super::find_content_children;

    #[test]
    fn example_one() {
        assert_eq!(find_content_children(vec![1, 2, 3], vec![1, 1]), 1);
    }

    #[test]
    fn example_two() {
        assert_eq!(find_content_children(vec![1, 2], vec![1, 2, 3]), 2);
    }
}
