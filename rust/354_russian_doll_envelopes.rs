/// LeetCode #354 - Russian Doll Envelopes
fn max_envelopes(mut envelopes: Vec<Vec<i32>>) -> i32 {
    envelopes.sort_unstable_by(|a,b| if a[0]==b[0] { b[1].cmp(&a[1]) } else { a[0].cmp(&b[0]) });
    let mut lis: Vec<i32> = vec![];
    for e in envelopes {
        let h = e[1];
        match lis.binary_search(&h) {
            Ok(i) => lis[i] = h,
            Err(i) => {
                if i == lis.len() { lis.push(h); } else { lis[i]=h; }
            }
        }
    }
    lis.len() as i32
}

fn main() {
    println!("{}", max_envelopes(vec![vec![5,4],vec![6,4],vec![6,7],vec![2,3]]));
}

#[cfg(test)]
mod tests {
    use super::max_envelopes;

    #[test]
    fn example_one() {
        assert_eq!(max_envelopes(vec![vec![5,4],vec![6,4],vec![6,7],vec![2,3]]), 3);
    }
}
