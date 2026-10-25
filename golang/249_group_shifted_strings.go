package leetcode

// LeetCode #249 - Group Shifted Strings
//
// Two strings match when the gaps between their letters are the same.
func groupStrings(strs []string) [][]string {
	groups := map[string][]string{}
	for _, s := range strs {
		key := make([]byte, len(s))
		base := s[0]
		for i := 0; i < len(s); i++ {
			key[i] = (s[i]-base+26)%26 + 'a'
		}
		groups[string(key)] = append(groups[string(key)], s)
	}
	out := make([][]string, 0, len(groups))
	for _, group := range groups {
		out = append(out, group)
	}
	return out
}
