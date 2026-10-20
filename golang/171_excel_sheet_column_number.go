package leetcode

// LeetCode #171 - Excel Sheet Column Number
//
// Treat the title as a base-26 number whose digits run from A to Z.
func titleToNumber(columnTitle string) int {
	n := 0
	for i := 0; i < len(columnTitle); i++ {
		n = n*26 + int(columnTitle[i]-'A'+1)
	}
	return n
}
