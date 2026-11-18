package leetcode

// LeetCode #599 - Minimum Index Sum of Two Lists
//
// A shared name's cost is the sum of its two indexes, and every name with that least cost is kept.
func findRestaurant(list1 []string, list2 []string) []string {
	index := make(map[string]int, len(list1))
	for i, name := range list1 {
		index[name] = i
	}
	best := len(list1) + len(list2)
	var common []string
	for j, name := range list2 {
		i, ok := index[name]
		if !ok {
			continue
		}
		sum := i + j
		if sum < best {
			best = sum
			common = []string{name}
			continue
		}
		if sum == best {
			common = append(common, name)
		}
	}
	return common
}
