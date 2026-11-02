package leetcode

// LeetCode #370 - Range Addition
//
// Each update adds at the start and subtracts just after the end, then a prefix sum builds the array.
func getModifiedArray(length int, updates [][]int) []int {
	diff := make([]int, length+1)
	for _, update := range updates {
		diff[update[0]] += update[2]
		diff[update[1]+1] -= update[2]
	}
	out := make([]int, length)
	running := 0
	for i := 0; i < length; i++ {
		running += diff[i]
		out[i] = running
	}
	return out
}
