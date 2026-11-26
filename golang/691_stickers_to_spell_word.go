package leetcode

// LeetCode #691 - Stickers to Spell Word
//
// Each sticker covers some still-missing letters of the target, and the mask of covered positions is remembered.
func minStickers(stickers []string, target string) int {
	n := len(target)
	const inf = 1 << 20
	dp := make([]int, 1<<n)
	for i := range dp {
		dp[i] = inf
	}
	dp[0] = 0
	for state := 0; state < 1<<n; state++ {
		if dp[state] >= inf {
			continue
		}
		for _, sticker := range stickers {
			counts := [26]int{}
			for i := 0; i < len(sticker); i++ {
				counts[sticker[i]-'a']++
			}
			next := state
			for i := 0; i < n; i++ {
				if next>>i&1 == 1 {
					continue
				}
				letter := target[i] - 'a'
				if counts[letter] == 0 {
					continue
				}
				counts[letter]--
				next |= 1 << i
			}
			dp[next] = min(dp[next], dp[state]+1)
		}
	}
	if dp[1<<n-1] >= inf {
		return -1
	}
	return dp[1<<n-1]
}
