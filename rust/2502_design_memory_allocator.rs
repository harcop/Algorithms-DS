/// LeetCode #2502 - Design Memory Allocator
struct Allocator {
    memory: Vec<i32>,
}

impl Allocator {
    fn new(n: i32) -> Self {
        Allocator {
            memory: vec![0; n as usize],
        }
    }

    fn allocate(&mut self, size: i32, m_id: i32) -> i32 {
        let size = size as usize;
        let mut cnt = 0usize;
        for i in 0..self.memory.len() {
            if self.memory[i] > 0 {
                cnt = 0;
            } else {
                cnt += 1;
                if cnt == size {
                    let start = i + 1 - size;
                    for j in start..=i {
                        self.memory[j] = m_id;
                    }
                    return start as i32;
                }
            }
        }
        -1
    }

    fn free_memory(&mut self, m_id: i32) -> i32 {
        let mut ans = 0;
        for cell in &mut self.memory {
            if *cell == m_id {
                *cell = 0;
                ans += 1;
            }
        }
        ans
    }
}

fn main() {
    let mut loc = Allocator::new(10);
    println!("{}", loc.allocate(1, 1));
}

#[cfg(test)]
mod tests {
    use super::Allocator;

    #[test]
    fn example_sequence() {
        let mut loc = Allocator::new(10);
        assert_eq!(loc.allocate(1, 1), 0);
        assert_eq!(loc.allocate(1, 2), 1);
        assert_eq!(loc.allocate(1, 3), 2);
        assert_eq!(loc.free_memory(2), 1);
        assert_eq!(loc.allocate(3, 4), 3);
        assert_eq!(loc.allocate(1, 1), 1);
        assert_eq!(loc.allocate(1, 1), 6);
        assert_eq!(loc.free_memory(1), 3);
        assert_eq!(loc.allocate(10, 2), -1);
        assert_eq!(loc.free_memory(7), 0);
    }
}
