package leetcode

// LeetCode #128 - Longest Consecutive Sequence
//
// A number starts a run when its predecessor is missing. Count forward from each start.
func longestConsecutive(nums []int) int {
	set := make(map[int]struct{}, len(nums))
	for _, num := range nums {
		set[num] = struct{}{}
	}

	best := 0
	for num := range set {
		if _, ok := set[num-1]; ok {
			continue
		}
		length := 1
		for _, ok := set[num+length]; ok; _, ok = set[num+length] {
			length++
		}
		if length > best {
			best = length
		}
	}
	return best
}
