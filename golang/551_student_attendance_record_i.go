package leetcode

// LeetCode #551 - Student Attendance Record I
//
// A record is eligible with fewer than two absences and no three lates in a row.
func checkRecord(s string) bool {
	absences, lates := 0, 0
	for _, day := range s {
		switch day {
		case 'A':
			absences++
			lates = 0
			if absences > 1 {
				return false
			}
		case 'L':
			lates++
			if lates > 2 {
				return false
			}
		default:
			lates = 0
		}
	}
	return true
}
