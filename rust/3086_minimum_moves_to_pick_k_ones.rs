/// LeetCode #3086 - Minimum Moves to Pick K Ones
fn minimum_moves(nums: Vec<i32>, k: i32, max_changes: i32) -> i64 {
    let k = k as i64;
    let max_changes = max_changes as i64;
    let ones: Vec<i64> = nums
        .iter()
        .enumerate()
        .filter(|&(_, &v)| v == 1)
        .map(|(i, _)| i as i64)
        .collect();
    let m = ones.len() as i64;

    // With max_changes, we can create ones adjacent (cost 2 each after first free pick).
    // Optimal often: use as many max_changes as possible (2 moves each), plus gather existing ones.
    if k <= max_changes {
        // pick one free at start (create + move = 2 per one if we create at neighbor)
        // If we stand on a 0, create neighbor and move: 2 moves per one; first can be free if we create then pick.
        // Actually: create at aliceIndex+1 and swap: 2 moves per one. For k ones: 2*k.
        // But if k==1 and we can stand on existing 1: 0. Guaranteed feasible.
        return if k == 1 && m > 0 { 0 } else { 2 * k };
    }

    // Need at least need = k - max_changes existing ones gathered (remaining from max_changes at cost 2 each).
    // Also can use fewer max_changes if gathering more existing is cheaper — but action1 costs 2,
    // gathering far ones costs distance. Standard approach: gather x existing ones to median, plus
    // (k-x) created ones at cost 2 each, with x >= max(0, k-max_changes) and x <= min(k, m).
    // Also special: up to 3 nearby ones can be gathered cheaply when standing at a position...
    //
    // Simpler known solution:
    // Let c = max_changes. Prefer using action1 for (k - gathered) ones costing 2 each.
    // For gathering, use sliding window of size `num` on ones positions, cost = sum |pos - median|.
    // But ones at distance 1 from aliceIndex cost 1 each via adjacent swaps; created ones at
    // neighbors cost 2. Optimal: answer = min over strategies.

    let mut ans = i64::MAX;

    // Strategy: gather `num` existing ones (num from max(0, k-max_changes) ..= min(m, k)),
    // create the rest with max_changes (cost 2 each).
    // Cost of gathering `num` ones to their median (optimal meeting point).
    let mut prefix = vec![0i64; ones.len() + 1];
    for i in 0..ones.len() {
        prefix[i + 1] = prefix[i] + ones[i];
    }

    let min_gather = (k - max_changes).max(0) as usize;
    let max_gather = k.min(m) as usize;

    for num in min_gather..=max_gather {
        let created = k - num as i64;
        if created > max_changes {
            continue;
        }
        // Sliding window of size num
        if num == 0 {
            ans = ans.min(2 * created);
            continue;
        }
        for l in 0..=ones.len() - num {
            let r = l + num - 1;
            let mid = l + num / 2;
            let median = ones[mid];
            // cost left: median * count_left - sum_left
            let left_cnt = (mid - l) as i64;
            let right_cnt = (r - mid) as i64;
            let left_sum = prefix[mid] - prefix[l];
            let right_sum = prefix[r + 1] - prefix[mid + 1];
            let gather_cost = median * left_cnt - left_sum + right_sum - median * right_cnt;
            ans = ans.min(gather_cost + 2 * created);
        }
    }

    // Special optimization often needed: when using max_changes for ones next to alice,
    // gathering at most 1 existing + rest created can be better with local ones.
    // Also handle case k - max_changes <= 0 already covered.
    // Extra: if we only need 1 gather from existing when max_changes is large...
    // The above should cover; but LeetCode often needs:
    // When created ones go to neighbors of aliceIndex, and we pick existing ones at
    // aliceIndex and neighbors with cost 0/1 — alternative formula for small gather.
    //
    // Known trick: for num_gather in {0,1,2,3} with enough max_changes, special small answers:
    if max_changes >= k {
        ans = ans.min(2 * k); // all created (but first pick free reduces? actually 2k if start on 0)
        if m >= 1 {
            ans = ans.min(2 * (k - 1)); // stand on existing 1 free, create rest
        }
    }
    if max_changes >= k - 1 && m >= 1 {
        ans = ans.min(2 * (k - 1));
    }
    if max_changes >= k - 2 && m >= 2 {
        // two existing ones that are adjacent or distance 2: cost at most 1 between them + 2*(k-2)
        let mut best_pair = i64::MAX;
        for w in ones.windows(2) {
            best_pair = best_pair.min(w[1] - w[0]);
        }
        if best_pair < i64::MAX {
            // cost to gather 2 ones to one of them: distance between them (meet at one endpoint)
            // Actually meeting: stand at one, other walks: cost = dist.
            ans = ans.min(best_pair + 2 * (k - 2));
        }
    }
    if max_changes >= k - 3 && m >= 3 {
        let mut best3 = i64::MAX;
        for i in 0..ones.len() - 2 {
            // gather 3 ones ones[i..i+2] to median ones[i+1]
            let cost = ones[i + 1] - ones[i] + ones[i + 2] - ones[i + 1];
            best3 = best3.min(cost);
        }
        if best3 < i64::MAX {
            ans = ans.min(best3 + 2 * (k - 3));
        }
    }

    ans
}

fn main() {
    println!(
        "{}",
        minimum_moves(vec![1, 1, 0, 0, 0, 1, 1, 0, 0, 1], 3, 1)
    );
}

#[cfg(test)]
mod tests {
    use super::minimum_moves;

    #[test]
    fn example1() {
        assert_eq!(
            minimum_moves(vec![1, 1, 0, 0, 0, 1, 1, 0, 0, 1], 3, 1),
            3
        );
    }

    #[test]
    fn example2() {
        assert_eq!(minimum_moves(vec![0, 0, 0, 0], 2, 3), 4);
    }
}
