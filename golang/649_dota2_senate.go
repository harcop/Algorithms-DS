package leetcode

// LeetCode #649 - Dota2 Senate
//
// Each senator bans the next opposing senator, then returns to the back of the line.
func predictPartyVictory(senate string) string {
	var radiant, dire []int
	for i := 0; i < len(senate); i++ {
		if senate[i] == 'R' {
			radiant = append(radiant, i)
		} else {
			dire = append(dire, i)
		}
	}
	ri, di := 0, 0
	for ri < len(radiant) && di < len(dire) {
		if radiant[ri] < dire[di] {
			radiant = append(radiant, radiant[ri]+len(senate))
		} else {
			dire = append(dire, dire[di]+len(senate))
		}
		ri++
		di++
	}
	if ri < len(radiant) {
		return "Radiant"
	}
	return "Dire"
}
