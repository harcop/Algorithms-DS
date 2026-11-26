package leetcode

// LeetCode #681 - Next Closest Time
//
// Every valid clock built from the current digits is tried, and the smallest forward gap wins.
func nextClosestTime(time string) string {
	digits := []byte{time[0], time[1], time[3], time[4]}
	now := (int(time[0]-'0')*10+int(time[1]-'0'))*60 + int(time[3]-'0')*10 + int(time[4]-'0')
	best := 24 * 60
	answer := time
	for _, h1 := range digits {
		for _, h2 := range digits {
			hour := int(h1-'0')*10 + int(h2-'0')
			if hour > 23 {
				continue
			}
			for _, m1 := range digits {
				for _, m2 := range digits {
					minute := int(m1-'0')*10 + int(m2-'0')
					if minute > 59 {
						continue
					}
					gap := hour*60 + minute - now
					if gap <= 0 {
						gap += 24 * 60
					}
					if gap < best {
						best = gap
						answer = string([]byte{h1, h2, ':', m1, m2})
					}
				}
			}
		}
	}
	return answer
}
