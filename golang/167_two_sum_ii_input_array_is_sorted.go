package leetcode

// LeetCode #167 - Two Sum II - Input Array Is Sorted
//
// LeetCode names this twoSum; that name already belongs to the unsorted version (#1).
// The array is sorted, so move inward from both ends until the pair sums to the target.
func twoSumII(numbers []int, target int) []int {
	lo, hi := 0, len(numbers)-1
	for lo < hi {
		sum := numbers[lo] + numbers[hi]
		if sum == target {
			return []int{lo + 1, hi + 1}
		}
		if sum < target {
			lo++
		} else {
			hi--
		}
	}
	return nil
}
