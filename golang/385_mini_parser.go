package leetcode

// LeetCode #385 - Mini Parser
//
// Read a number or a bracketed list, and recurse when the next value is itself a list.
func deserializeNested(s string) NestedInteger {
	i := 0
	var parse func() NestedInteger
	parse = func() NestedInteger {
		if s[i] == '[' {
			i++
			var items []NestedInteger
			for s[i] != ']' {
				if s[i] == ',' {
					i++
					continue
				}
				items = append(items, parse())
			}
			i++
			return nestedList(items...)
		}
		sign := 1
		if s[i] == '-' {
			sign = -1
			i++
		}
		value := 0
		for i < len(s) && s[i] >= '0' && s[i] <= '9' {
			value = value*10 + int(s[i]-'0')
			i++
		}
		return nestedInt(value * sign)
	}
	return parse()
}
