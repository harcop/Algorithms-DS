package leetcode

// LeetCode #575 - Distribute Candies
//
// She can eat half the candies, so the limit is the smaller of that half and the number of types.
func distributeCandies(candyType []int) int {
	seen := make(map[int]struct{}, len(candyType))
	for _, kind := range candyType {
		seen[kind] = struct{}{}
	}
	return min(len(seen), len(candyType)/2)
}
