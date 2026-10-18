package leetcode

// LeetCode #137 - Single Number II
//
// LeetCode names this singleNumber; that name already belongs to the twice-repeated version (#136).
// Count each bit modulo three so bits seen three times drop out.
func singleNumberII(nums []int) int {
	ones, twos := 0, 0
	for _, num := range nums {
		twos |= ones & num
		ones ^= num
		threes := ones & twos
		ones &^= threes
		twos &^= threes
	}
	return ones
}
