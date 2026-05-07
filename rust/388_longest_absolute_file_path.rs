/// LeetCode #388 - Longest Absolute File Path (`depth -> prefix length`)
use std::collections::HashMap;

fn length_longest_path(input: String) -> i32 {
    let mut path_len: HashMap<usize, i32> = HashMap::new();
    path_len.insert(0, 0);
    let mut best = 0i32;
    for line in input.split('\n') {
        let name = line.trim_start_matches('\t');
        let depth = line.len() - name.len(); // tabs only prefix
        if name.contains('.') {
            best = best.max(path_len[&depth] + name.len() as i32);
        } else {
            let nl = path_len[&depth] + name.len() as i32 + 1;
            path_len.insert(depth + 1, nl);
        }
    }
    best
}

fn main() {
    println!("{}", length_longest_path("dir\n\tsubdir1".into()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lc() {
        assert_eq!(
            length_longest_path(
                "dir\n\tsubdir1\n\t\tfile1.ext\n\t\tsubsubfolder1\n\tsubdir2\n\t\tsubsubdir2\n\t\t\tfile2.ext".into(),
            ),
            32
        );
    }
}
