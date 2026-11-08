package leetcode

import "sort"

// LeetCode #451 - Sort Characters By Frequency
//
// Count each character, then write the counts from largest to smallest.
func frequencySort(s string) string {
	count := map[byte]int{}
	for i := 0; i < len(s); i++ {
		count[s[i]]++
	}
	type charCount struct {
		ch byte
		n  int
	}
	items := make([]charCount, 0, len(count))
	for ch, n := range count {
		items = append(items, charCount{ch, n})
	}
	sort.Slice(items, func(i, j int) bool {
		if items[i].n != items[j].n {
			return items[i].n > items[j].n
		}
		return items[i].ch < items[j].ch
	})
	out := make([]byte, 0, len(s))
	for _, item := range items {
		for k := 0; k < item.n; k++ {
			out = append(out, item.ch)
		}
	}
	return string(out)
}
