/// LeetCode #2092 - Find All People With Secret
use std::collections::HashSet;

struct Dsu {
    p: Vec<usize>,
}

impl Dsu {
    fn new(n: usize) -> Self {
        Dsu {
            p: (0..n).collect(),
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.p[x] != x {
            self.p[x] = self.find(self.p[x]);
        }
        self.p[x]
    }

    fn union(&mut self, a: usize, b: usize) {
        let pa = self.find(a);
        let pb = self.find(b);
        if pa != pb {
            self.p[pb] = pa;
        }
    }

    fn reset(&mut self, x: usize) {
        self.p[x] = x;
    }
}

fn find_all_people(n: i32, mut meetings: Vec<Vec<i32>>, first_person: i32) -> Vec<i32> {
    let n = n as usize;
    meetings.sort_by_key(|m| m[2]);

    let mut dsu = Dsu::new(n);
    dsu.union(0, first_person as usize);

    let mut i = 0usize;
    while i < meetings.len() {
        let time = meetings[i][2];
        let mut people = HashSet::new();
        while i < meetings.len() && meetings[i][2] == time {
            let a = meetings[i][0] as usize;
            let b = meetings[i][1] as usize;
            dsu.union(a, b);
            people.insert(a);
            people.insert(b);
            i += 1;
        }

        for person in people {
            if dsu.find(person) != dsu.find(0) {
                dsu.reset(person);
            }
        }
    }

    (0..n)
        .filter(|&person| dsu.find(person) == dsu.find(0))
        .map(|person| person as i32)
        .collect()
}

fn main() {
    println!(
        "{:?}",
        find_all_people(
            6,
            vec![vec![1, 2, 5], vec![2, 3, 8], vec![1, 5, 10]],
            1,
        )
    );
}

#[cfg(test)]
mod tests {
    use super::find_all_people;

    #[test]
    fn example_one() {
        assert_eq!(
            find_all_people(
                6,
                vec![vec![1, 2, 5], vec![2, 3, 8], vec![1, 5, 10]],
                1,
            ),
            vec![0, 1, 2, 3, 5]
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            find_all_people(
                4,
                vec![vec![3, 1, 3], vec![1, 2, 2], vec![0, 3, 3]],
                3,
            ),
            vec![0, 1, 3]
        );
    }

    #[test]
    fn example_three() {
        assert_eq!(
            find_all_people(
                5,
                vec![vec![3, 4, 2], vec![1, 2, 1], vec![2, 3, 1]],
                1,
            ),
            vec![0, 1, 2, 3, 4]
        );
    }
}
