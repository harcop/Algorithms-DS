package leetcode

import "strconv"

// LeetCode #411 - Minimum Unique Word Abbreviation
//
// A kept letter is a bit. The shortest mask that still keeps one differing letter from every dictionary word wins.
func minAbbreviation(target string, dictionary []string) string {
	n := len(target)
	diffs := make([]int, 0, len(dictionary))
	for _, word := range dictionary {
		if len(word) != n || word == target {
			continue
		}
		mask := 0
		for i := 0; i < n; i++ {
			if word[i] != target[i] {
				mask |= 1 << i
			}
		}
		diffs = append(diffs, mask)
	}
	best := target
	for mask := 0; mask < 1<<n; mask++ {
		if !abbrSeparates(mask, diffs) {
			continue
		}
		cand := abbreviateKept(target, mask)
		if len(cand) < len(best) || (len(cand) == len(best) && cand < best) {
			best = cand
		}
	}
	return best
}

func abbrSeparates(mask int, diffs []int) bool {
	for _, diff := range diffs {
		if mask&diff == 0 {
			return false
		}
	}
	return true
}

func abbreviateKept(target string, mask int) string {
	var out []byte
	i := 0
	for i < len(target) {
		if mask&(1<<i) != 0 {
			out = append(out, target[i])
			i++
			continue
		}
		j := i
		for j < len(target) && mask&(1<<j) == 0 {
			j++
		}
		out = append(out, strconv.Itoa(j-i)...)
		i = j
	}
	return string(out)
}
