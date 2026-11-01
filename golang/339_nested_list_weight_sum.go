package leetcode

// NestedInteger is either a single integer or a list of nested integers.
type NestedInteger struct {
	value *int
	list  []NestedInteger
}

func nestedInt(v int) NestedInteger {
	return NestedInteger{value: &v}
}

func nestedList(items ...NestedInteger) NestedInteger {
	return NestedInteger{list: items}
}

// LeetCode #339 - Nested List Weight Sum
//
// An integer contributes its value times how deep it sits.
func depthSum(nested []NestedInteger) int {
	var walk func([]NestedInteger, int) int
	walk = func(items []NestedInteger, depth int) int {
		sum := 0
		for _, item := range items {
			if item.value != nil {
				sum += *item.value * depth
			} else {
				sum += walk(item.list, depth+1)
			}
		}
		return sum
	}
	return walk(nested, 1)
}
