package leetcode

import "sort"

// LeetCode #267 - Palindrome Permutation II
//
// Put one odd character in the middle and permute the first half of the rest.
func generatePalindromes(s string) []string {
	freq := map[byte]int{}
	for i := 0; i < len(s); i++ {
		freq[s[i]]++
	}
	var mid []byte
	var half []byte
	for ch, count := range freq {
		if count%2 == 1 {
			if len(mid) == 1 {
				return []string{}
			}
			mid = []byte{ch}
		}
		for i := 0; i < count/2; i++ {
			half = append(half, ch)
		}
	}
	sort.Slice(half, func(i, j int) bool { return half[i] < half[j] })
	used := make([]bool, len(half))
	var path []byte
	var out []string
	var permute func()
	permute = func() {
		if len(path) == len(half) {
			left := string(path)
			right := make([]byte, len(path))
			for i := range path {
				right[i] = path[len(path)-1-i]
			}
			out = append(out, left+string(mid)+string(right))
			return
		}
		for i := range half {
			if used[i] || (i > 0 && half[i] == half[i-1] && !used[i-1]) {
				continue
			}
			used[i] = true
			path = append(path, half[i])
			permute()
			path = path[:len(path)-1]
			used[i] = false
		}
	}
	permute()
	return out
}
