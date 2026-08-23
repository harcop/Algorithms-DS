/// LeetCode #3377 - Digit Operations to Make Two Integers Equal
use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn min_operations(n: i32, m: i32) -> i32 {
    let mut sieve = vec![true; 100_000];
    sieve[0] = false;
    sieve[1] = false;
    for i in 2..100_000 {
        if sieve[i] {
            let mut j = i * 2;
            while j < 100_000 {
                sieve[j] = false;
                j += i;
            }
        }
    }
    if sieve[n as usize] || sieve[m as usize] {
        return -1;
    }
    let mut pq = BinaryHeap::new();
    pq.push(Reverse((n, n)));
    let mut visited = vec![false; 100_000];
    while let Some(Reverse((sum, cur))) = pq.pop() {
        if visited[cur as usize] {
            continue;
        }
        visited[cur as usize] = true;
        if cur == m {
            return sum;
        }
        let mut s: Vec<u8> = cur.to_string().into_bytes();
        for i in 0..s.len() {
            let c = s[i];
            if s[i] < b'9' {
                s[i] += 1;
                let next: i32 = std::str::from_utf8(&s).unwrap().parse().unwrap();
                if !sieve[next as usize] && !visited[next as usize] {
                    pq.push(Reverse((sum + next, next)));
                }
                s[i] = c;
            }
            if s[i] > b'0' && !(i == 0 && s[i] == b'1') {
                s[i] -= 1;
                let next: i32 = std::str::from_utf8(&s).unwrap().parse().unwrap();
                if !sieve[next as usize] && !visited[next as usize] {
                    pq.push(Reverse((sum + next, next)));
                }
                s[i] = c;
            }
        }
    }
    -1
}

fn main() {
    println!("{}", min_operations(10, 12));
}

#[cfg(test)]
mod tests {
    use super::min_operations;

    #[test]
    fn example1() {
        assert_eq!(min_operations(10, 12), 85);
    }

    #[test]
    fn example2() {
        assert_eq!(min_operations(4, 8), -1);
    }

    #[test]
    fn example3() {
        assert_eq!(min_operations(6, 2), -1);
    }
}
