/// LeetCode #911 - Online Election
struct TopVotedCandidate {
    times: Vec<i32>,
    leaders: Vec<i32>,
}

impl TopVotedCandidate {
    fn new(persons: Vec<i32>, times: Vec<i32>) -> Self {
        let mut votes = std::collections::HashMap::new();
        let mut leader = persons[0];
        let mut max_votes = 0;
        let mut leaders = Vec::with_capacity(times.len());
        for &p in &persons {
            let c = votes.entry(p).or_insert(0);
            *c += 1;
            if *c >= max_votes {
                max_votes = *c;
                leader = p;
            }
            leaders.push(leader);
        }
        Self { times, leaders }
    }

    fn q(&self, t: i32) -> i32 {
        let i = self.times.partition_point(|&x0| x0 <= t);
        let idx = if i == 0 { 0 } else { i - 1 };
        self.leaders[idx]
    }
}

fn main() {
    let tv = TopVotedCandidate::new(vec![0, 1, 1, 0, 0, 1, 0, 0], vec![0, 5, 10, 15, 20, 25, 30, 35]);
    println!("{}", tv.q(3));
    println!("{}", tv.q(12));
    println!("{}", tv.q(25));
    println!("{}", tv.q(15));
    println!("{}", tv.q(24));
    println!("{}", tv.q(8));
}

#[cfg(test)]
mod tests {
    use super::TopVotedCandidate;

    #[test]
    fn example() {
        let tv = TopVotedCandidate::new(
            vec![0, 1, 1, 0, 0, 1, 0, 0],
            vec![0, 5, 10, 15, 20, 25, 30, 35],
        );
        assert_eq!(tv.q(3), 0);
        assert_eq!(tv.q(12), 1);
        assert_eq!(tv.q(25), 1);
        assert_eq!(tv.q(15), 0);
        assert_eq!(tv.q(24), 0);
        assert_eq!(tv.q(8), 1);
    }
}
