package leetcode

import "strconv"

// LeetCode #282 - Expression Add Operators
//
// Try +, -, and * between the digits, and undo the previous term when multiplying.
func addOperators(num string, target int) []string {
	var out []string
	var search func(i int, path string, cur, prev int)
	search = func(i int, path string, cur, prev int) {
		if i == len(num) {
			if cur == target {
				out = append(out, path)
			}
			return
		}
		for j := i; j < len(num); j++ {
			if j > i && num[i] == '0' {
				break
			}
			slice := num[i : j+1]
			value, _ := strconv.Atoi(slice)
			if i == 0 {
				search(j+1, slice, value, value)
				continue
			}
			search(j+1, path+"+"+slice, cur+value, value)
			search(j+1, path+"-"+slice, cur-value, -value)
			search(j+1, path+"*"+slice, cur-prev+prev*value, prev*value)
		}
	}
	search(0, "", 0, 0)
	return out
}
