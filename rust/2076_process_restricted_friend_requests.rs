/// LeetCode #2076 - Process Restricted Friend Requests
fn friend_requests(n: i32, restrictions: Vec<Vec<i32>>, requests: Vec<Vec<i32>>) -> Vec<bool> {
    let mut dsu = Dsu::new(n as usize);
    let mut ans = Vec::with_capacity(requests.len());

    for request in requests {
        let a = dsu.find(request[0] as usize);
        let b = dsu.find(request[1] as usize);
        if a == b {
            ans.push(true);
            continue;
        }

        let blocked = restrictions.iter().any(|r| {
            let x = dsu.find(r[0] as usize);
            let y = dsu.find(r[1] as usize);
            (x == a && y == b) || (x == b && y == a)
        });

        if blocked {
            ans.push(false);
        } else {
            dsu.union(a, b);
            ans.push(true);
        }
    }

    ans
}

struct Dsu {
    parent: Vec<usize>,
    size: Vec<usize>,
}

impl Dsu {
    fn new(n: usize) -> Self {
        Dsu {
            parent: (0..n).collect(),
            size: vec![1; n],
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    fn union(&mut self, a: usize, b: usize) {
        let mut ra = self.find(a);
        let mut rb = self.find(b);
        if ra == rb {
            return;
        }
        if self.size[ra] < self.size[rb] {
            std::mem::swap(&mut ra, &mut rb);
        }
        self.parent[rb] = ra;
        self.size[ra] += self.size[rb];
    }
}

fn main() {
    println!(
        "{:?}",
        friend_requests(3, vec![vec![0, 1]], vec![vec![0, 2], vec![2, 1]])
    );
}

#[cfg(test)]
mod tests {
    use super::friend_requests;

    #[test]
    fn example_one() {
        assert_eq!(
            friend_requests(3, vec![vec![0, 1]], vec![vec![0, 2], vec![2, 1]]),
            vec![true, false]
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            friend_requests(
                3,
                vec![vec![0, 1]],
                vec![vec![1, 2], vec![0, 2]]
            ),
            vec![true, false]
        );
    }

    #[test]
    fn example_three() {
        assert_eq!(
            friend_requests(
                5,
                vec![vec![0, 1], vec![1, 2], vec![2, 3]],
                vec![vec![0, 4], vec![1, 2], vec![3, 1], vec![3, 4]]
            ),
            vec![true, false, true, false]
        );
    }
}
