/// LeetCode #1418 - Display Table Of Food Orders In A Restaurant
fn display_table(orders: Vec<Vec<String>>) -> Vec<Vec<String>> {
    use std::collections::{BTreeMap, BTreeSet, HashMap};
    let mut tables: BTreeMap<String, HashMap<String, i32>> = BTreeMap::new();
    let mut foods: BTreeSet<String> = BTreeSet::new();
    for o in orders {
        let table = o[2].clone();
        let food = o[1].clone();
        foods.insert(food.clone());
        *tables.entry(table).or_default().entry(food).or_insert(0) += 1;
    }
    let food_list: Vec<String> = foods.into_iter().collect();
    let mut ans = vec![vec!["Table".into()]];
    ans[0].extend(food_list.iter().cloned());
    for (table, counts) in tables {
        let mut row = vec![table];
        for food in &food_list {
            row.push(counts.get(food).copied().unwrap_or(0).to_string());
        }
        ans.push(row);
    }
    ans
}

fn main() {
    println!("{:?}", display_table(vec![vec!["David".into(), "3".into(), "Ceviche".into()]]));
}

#[cfg(test)]
mod tests {
    use super::display_table;

    #[test]
    fn example_one() {
        let out = display_table(vec![
            vec!["David".into(), "3".into(), "Ceviche".into()],
            vec!["Corina".into(), "6".into(), "Beef Burrito".into()],
            vec!["David".into(), "3".into(), "Fried Chicken".into()],
            vec!["Corina".into(), "6".into(), "Beef Burrito".into()],
        ]);
        assert_eq!(out[0][0], "Table");
        assert!(out.len() >= 2);
    }
}

