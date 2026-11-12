package leetcode

// LeetCode #514 - Freedom Trail
//
// For each key letter, the cost is the shorter rotation from the previous ring position plus the button press.
func findRotateSteps(ring string, key string) int {
	n := len(ring)
	positions := make([][]int, 26)
	for i := 0; i < n; i++ {
		positions[ring[i]-'a'] = append(positions[ring[i]-'a'], i)
	}
	const inf = int(^uint(0) >> 1)
	dp := make([]int, n)
	for i := range dp {
		dp[i] = inf
	}
	dp[0] = 0
	for i := 0; i < len(key); i++ {
		next := make([]int, n)
		for j := range next {
			next[j] = inf
		}
		for _, place := range positions[key[i]-'a'] {
			for from := 0; from < n; from++ {
				if dp[from] == inf {
					continue
				}
				diff := from - place
				if diff < 0 {
					diff = -diff
				}
				if other := n - diff; other < diff {
					diff = other
				}
				if cost := dp[from] + diff + 1; cost < next[place] {
					next[place] = cost
				}
			}
		}
		dp = next
	}
	best := inf
	for _, cost := range dp {
		if cost < best {
			best = cost
		}
	}
	return best
}
