/// LeetCode #3845 - Maximum Subarray XOR with Bounded Range
use std::collections::VecDeque;

struct Trie {
    ch: Vec<[i32; 2]>,
    cnt: Vec<i32>,
}

impl Trie {
    fn new() -> Self {
        Self {
            ch: vec![[-1, -1]],
            cnt: vec![0],
        }
    }

    fn update(&mut self, value: i32, delta: i32) {
        let mut cur = 0usize;
        for bit in (0..15).rev() {
            let b = ((value >> bit) & 1) as usize;
            if self.ch[cur][b] == -1 {
                self.ch[cur][b] = self.ch.len() as i32;
                self.ch.push([-1, -1]);
                self.cnt.push(0);
            }
            cur = self.ch[cur][b] as usize;
            self.cnt[cur] += delta;
        }
    }

    fn get_max_xor(&self, value: i32) -> i32 {
        let mut cur = 0usize;
        let mut ans = 0;
        for bit in (0..15).rev() {
            let b = ((value >> bit) & 1) as usize;
            let opp = b ^ 1;
            if self.ch[cur][opp] != -1 && self.cnt[self.ch[cur][opp] as usize] > 0 {
                ans |= 1 << bit;
                cur = self.ch[cur][opp] as usize;
            } else {
                cur = self.ch[cur][b] as usize;
            }
        }
        ans
    }
}

fn max_xor(nums: Vec<i32>, k: i32) -> i32 {
    let n = nums.len();
    let mut prefix = vec![0; n + 1];
    for i in 0..n {
        prefix[i + 1] = prefix[i] ^ nums[i];
    }
    let mut trie = Trie::new();
    let mut maxq: VecDeque<usize> = VecDeque::new();
    let mut minq: VecDeque<usize> = VecDeque::new();
    let mut left = 0usize;
    let mut ans = 0;
    trie.update(prefix[0], 1);
    for right in 0..n {
        let x = nums[right];
        while maxq.back().map(|&i| nums[i] <= x).unwrap_or(false) {
            maxq.pop_back();
        }
        while minq.back().map(|&i| nums[i] >= x).unwrap_or(false) {
            minq.pop_back();
        }
        maxq.push_back(right);
        minq.push_back(right);
        while nums[*maxq.front().unwrap()] - nums[*minq.front().unwrap()] > k {
            if *maxq.front().unwrap() == left {
                maxq.pop_front();
            }
            if *minq.front().unwrap() == left {
                minq.pop_front();
            }
            trie.update(prefix[left], -1);
            left += 1;
        }
        ans = ans.max(trie.get_max_xor(prefix[right + 1]));
        trie.update(prefix[right + 1], 1);
    }
    ans
}

fn main() {
    println!("{}", max_xor(vec![5, 4, 5, 6], 2));
}

#[cfg(test)]
mod tests {
    use super::max_xor;

    #[test]
    fn example1() {
        assert_eq!(max_xor(vec![5, 4, 5, 6], 2), 7);
    }

    #[test]
    fn example2() {
        assert_eq!(max_xor(vec![5, 4, 5, 6], 1), 6);
    }
}
