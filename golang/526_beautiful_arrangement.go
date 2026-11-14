package leetcode

// LeetCode #526 - Beautiful Arrangement
//
// Place each unused number where it divides the position or the position divides it.
func countArrangement(n int) int {
	used := make([]bool, n+1)
	var place func(pos int) int
	place = func(pos int) int {
		if pos > n {
			return 1
		}
		count := 0
		for value := 1; value <= n; value++ {
			if used[value] || (value%pos != 0 && pos%value != 0) {
				continue
			}
			used[value] = true
			count += place(pos + 1)
			used[value] = false
		}
		return count
	}
	return place(1)
}
