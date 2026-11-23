package leetcode

// LeetCode #656 - Coin Path
//
// The cheapest jump into each index is kept, and a tie keeps the smaller index so the path stays lexicographically first.
func cheapestJump(coins []int, maxJump int) []int {
	n := len(coins)
	if n == 0 || coins[n-1] < 0 {
		return nil
	}
	const inf = int(1e9)
	cost := make([]int, n)
	next := make([]int, n)
	for i := range cost {
		cost[i] = inf
		next[i] = -1
	}
	cost[n-1] = coins[n-1]
	for i := n - 2; i >= 0; i-- {
		if coins[i] < 0 {
			continue
		}
		for j := i + 1; j < n && j <= i+maxJump; j++ {
			if cost[j] == inf {
				continue
			}
			if candidate := coins[i] + cost[j]; candidate < cost[i] {
				cost[i] = candidate
				next[i] = j
			}
		}
	}
	if cost[0] == inf {
		return nil
	}
	path := []int{}
	for i := 0; i != -1; i = next[i] {
		path = append(path, i+1)
		if i == n-1 {
			break
		}
	}
	if path[len(path)-1] != n {
		return nil
	}
	return path
}
