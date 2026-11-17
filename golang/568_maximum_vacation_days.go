package leetcode

// LeetCode #568 - Maximum Vacation Days
//
// Each week, stay or fly once, and keep the best vacation total that can reach each city.
func maxVacationDays(flights [][]int, days [][]int) int {
	if len(days) == 0 || len(days[0]) == 0 {
		return 0
	}
	cities, weeks := len(days), len(days[0])
	const unreachable = -1 << 30
	reach := make([]int, cities)
	for city := range reach {
		reach[city] = unreachable
	}
	reach[0] = 0
	for week := 0; week < weeks; week++ {
		next := make([]int, cities)
		for city := range next {
			next[city] = unreachable
		}
		for city := 0; city < cities; city++ {
			for from := 0; from < cities; from++ {
				if reach[from] == unreachable || (from != city && flights[from][city] == 0) {
					continue
				}
				next[city] = max(next[city], reach[from]+days[city][week])
			}
		}
		reach = next
	}
	best := 0
	for _, total := range reach {
		best = max(best, total)
	}
	return best
}
