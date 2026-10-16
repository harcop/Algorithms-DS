package leetcode

import "strconv"

// LeetCode #93 - Restore IP Addresses
//
// Place three more dots, rejecting any segment outside 0..255 or with a leading zero.
func restoreIpAddresses(s string) []string {
	var out []string
	var parts []string

	var dfs func(pos, segment int)
	dfs = func(pos, segment int) {
		if segment == 4 {
			if pos == len(s) {
				out = append(out, joinIP(parts))
			}
			return
		}
		val := 0
		for i := pos; i < len(s) && i < pos+3; i++ {
			val = val*10 + int(s[i]-'0')
			if val > 255 {
				break
			}
			if i != pos && s[pos] == '0' {
				break
			}
			parts = append(parts, strconv.Itoa(val))
			dfs(i+1, segment+1)
			parts = parts[:len(parts)-1]
			if val == 0 {
				break
			}
		}
	}

	dfs(0, 0)
	return out
}

func joinIP(parts []string) string {
	return parts[0] + "." + parts[1] + "." + parts[2] + "." + parts[3]
}
