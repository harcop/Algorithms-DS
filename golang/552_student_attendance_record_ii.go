package leetcode

// LeetCode #552 - Student Attendance Record II
//
// LeetCode names this checkRecord; that name already belongs to the string check (#551).
// Each day is present, absent, or late, and the count keeps the absences used and the current late streak.
func checkRecordII(n int) int {
	const mod = 1_000_000_007
	var ways [2][3]int
	ways[0][0] = 1
	for day := 0; day < n; day++ {
		var next [2][3]int
		for absences := 0; absences < 2; absences++ {
			for lates := 0; lates < 3; lates++ {
				count := ways[absences][lates]
				if count == 0 {
					continue
				}
				next[absences][0] = (next[absences][0] + count) % mod
				if absences+1 < 2 {
					next[absences+1][0] = (next[absences+1][0] + count) % mod
				}
				if lates+1 < 3 {
					next[absences][lates+1] = (next[absences][lates+1] + count) % mod
				}
			}
		}
		ways = next
	}
	total := 0
	for absences := 0; absences < 2; absences++ {
		for lates := 0; lates < 3; lates++ {
			total = (total + ways[absences][lates]) % mod
		}
	}
	return total
}
