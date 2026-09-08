/// LeetCode #3632 - Subarrays with XOR at Least K
struct Trie {
    bit_length: usize,
    nodes: Vec<[i32; 2]>,
    cnts: Vec<i32>,
}

impl Trie {
    fn new(bit_length: usize) -> Self {
        let mut t = Trie {
            bit_length,
            nodes: Vec::new(),
            cnts: Vec::new(),
        };
        t.new_node();
        t
    }

    fn new_node(&mut self) -> i32 {
        self.nodes.push([-1, -1]);
        self.cnts.push(0);
        (self.nodes.len() - 1) as i32
    }

    fn add(&mut self, num: i32) {
        let mut curr = 0i32;
        for i in (0..self.bit_length).rev() {
            let x = ((num >> i) & 1) as usize;
            if self.nodes[curr as usize][x] == -1 {
                let nxt = self.new_node();
                self.nodes[curr as usize][x] = nxt;
            }
            curr = self.nodes[curr as usize][x];
            self.cnts[curr as usize] += 1;
        }
    }

    fn query(&self, prefix: i32, k: i32) -> i64 {
        let mut result = 0i64;
        let mut curr = 0i32;
        for i in (0..self.bit_length).rev() {
            if curr == -1 {
                break;
            }
            let t = (k >> i) & 1;
            let x = (prefix >> i) & 1;
            if t == 0 {
                let tmp = self.nodes[curr as usize][(1 ^ x) as usize];
                if tmp != -1 {
                    result += self.cnts[tmp as usize] as i64;
                }
            }
            curr = self.nodes[curr as usize][(t ^ x) as usize];
        }
        if curr != -1 {
            result += self.cnts[curr as usize] as i64;
        }
        result
    }
}

fn count_xor_subarrays(nums: Vec<i32>, k: i32) -> i64 {
    let mx = *nums.iter().max().unwrap_or(&0).max(&k).max(&1);
    let bit_length = (32 - mx.leading_zeros()) as usize;
    let mut trie = Trie::new(bit_length.max(1));
    let mut result = 0i64;
    let mut prefix = 0i32;
    trie.add(prefix);
    for x in nums {
        prefix ^= x;
        result += trie.query(prefix, k);
        trie.add(prefix);
    }
    result
}

fn main() {
    println!("{}", count_xor_subarrays(vec![3, 1, 2, 3], 2));
}

#[cfg(test)]
mod tests {
    use super::count_xor_subarrays;

    #[test]
    fn example1() {
        assert_eq!(count_xor_subarrays(vec![3, 1, 2, 3], 2), 6);
    }

    #[test]
    fn example2() {
        assert_eq!(count_xor_subarrays(vec![0, 0, 0], 0), 6);
    }
}
