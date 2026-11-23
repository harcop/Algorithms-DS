package leetcode

// LeetCode #659 - Split Array into Consecutive Subsequences
//
// Each number extends a chain that already ends just before it, or it opens a new chain of three.
func isPossible(nums []int) bool {
	freq := map[int]int{}
	for _, num := range nums {
		freq[num]++
	}
	ending := map[int]int{}
	for _, num := range nums {
		if freq[num] == 0 {
			continue
		}
		freq[num]--
		if ending[num-1] > 0 {
			ending[num-1]--
			ending[num]++
			continue
		}
		if freq[num+1] > 0 && freq[num+2] > 0 {
			freq[num+1]--
			freq[num+2]--
			ending[num+2]++
			continue
		}
		return false
	}
	return true
}
