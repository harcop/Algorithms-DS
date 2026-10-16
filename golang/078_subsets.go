package leetcode

// LeetCode #78 - Subsets
//
// For each number, append a copy of every subset that already exists with that number added.
func subsets(nums []int) [][]int {
	out := [][]int{{}}
	for _, num := range nums {
		next := make([][]int, 0, len(out))
		for _, subset := range out {
			s := append(append([]int(nil), subset...), num)
			next = append(next, s)
		}
		out = append(out, next...)
	}
	return out
}
