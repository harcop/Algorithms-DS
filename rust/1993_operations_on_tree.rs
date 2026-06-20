/// LeetCode #1993 - Operations on Tree
pub struct LockingTree {
    locked: Vec<i32>,
    parent: Vec<i32>,
    children: Vec<Vec<usize>>,
}

impl LockingTree {
    fn new(parent: Vec<i32>) -> Self {
        let n = parent.len();
        let mut children = vec![Vec::new(); n];
        for (son, &fa) in parent.iter().enumerate().skip(1) {
            children[fa as usize].push(son);
        }
        Self {
            locked: vec![-1; n],
            parent,
            children,
        }
    }

    fn lock(&mut self, num: i32, user: i32) -> bool {
        let num = num as usize;
        if self.locked[num] == -1 {
            self.locked[num] = user;
            return true;
        }
        false
    }

    fn unlock(&mut self, num: i32, user: i32) -> bool {
        let num = num as usize;
        if self.locked[num] == user {
            self.locked[num] = -1;
            return true;
        }
        false
    }

    fn upgrade(&mut self, num: i32, user: i32) -> bool {
        let num = num as usize;
        let mut x = num as i32;
        while x != -1 {
            if self.locked[x as usize] != -1 {
                return false;
            }
            x = self.parent[x as usize];
        }

        let mut find = false;
        fn dfs(node: usize, locked: &mut [i32], children: &[Vec<usize>], find: &mut bool) {
            for &y in &children[node] {
                if locked[y] != -1 {
                    locked[y] = -1;
                    *find = true;
                }
                dfs(y, locked, children, find);
            }
        }
        dfs(num, &mut self.locked, &self.children, &mut find);
        if !find {
            return false;
        }
        self.locked[num] = user;
        true
    }
}

fn main() {
    let mut tree = LockingTree::new(vec![-1, 0, 0, 1, 1, 2, 2]);
    println!("{}", tree.lock(2, 2));
}

#[cfg(test)]
mod tests {
    use super::LockingTree;

    #[test]
    fn example_one() {
        let mut tree = LockingTree::new(vec![-1, 0, 0, 1, 1, 2, 2]);
        assert!(tree.lock(2, 2));
        assert!(!tree.unlock(2, 3));
        assert!(tree.unlock(2, 2));
        assert!(tree.lock(4, 5));
        assert!(tree.upgrade(0, 1));
        assert!(!tree.lock(0, 1));
    }
}
