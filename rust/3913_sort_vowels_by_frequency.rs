/// LeetCode #3913 - Sort Vowels by Frequency
fn sort_vowels(s: String) -> String {
    let vowels = b"aeiou";
    let bytes = s.as_bytes();
    let mut freq = [0i32; 26];
    let mut order = Vec::new();
    let mut seen = [false; 26];
    for &c in bytes {
        if vowels.contains(&c) {
            let i = (c - b'a') as usize;
            if !seen[i] {
                seen[i] = true;
                order.push(c);
            }
            freq[i] += 1;
        }
    }
    let mut first = [usize::MAX; 26];
    for (i, &c) in bytes.iter().enumerate() {
        if vowels.contains(&c) {
            let j = (c - b'a') as usize;
            if first[j] == usize::MAX {
                first[j] = i;
            }
        }
    }
    order.sort_by(|&a, &b| {
        let fa = freq[(a - b'a') as usize];
        let fb = freq[(b - b'a') as usize];
        fb.cmp(&fa)
            .then(first[(a - b'a') as usize].cmp(&first[(b - b'a') as usize]))
    });
    let mut ans = bytes.to_vec();
    let mut idx = 0usize;
    for i in 0..ans.len() {
        if vowels.contains(&ans[i]) {
            let c = order[idx];
            ans[i] = c;
            let j = (c - b'a') as usize;
            freq[j] -= 1;
            if freq[j] == 0 {
                idx += 1;
            }
        }
    }
    String::from_utf8(ans).unwrap()
}

fn main() {
    println!("{}", sort_vowels("leetcode".into()));
}

#[cfg(test)]
mod tests {
    use super::sort_vowels;

    #[test]
    fn example1() {
        assert_eq!(sort_vowels("leetcode".into()), "leetcedo");
    }

    #[test]
    fn example2() {
        assert_eq!(sort_vowels("aeiaaioooa".into()), "aaaaoooiie");
    }

    #[test]
    fn example3() {
        assert_eq!(sort_vowels("baeiou".into()), "baeiou");
    }
}
