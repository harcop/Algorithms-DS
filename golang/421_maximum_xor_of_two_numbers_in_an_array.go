package leetcode

// LeetCode #421 - Maximum XOR of Two Numbers in an Array
//
// A binary trie stores each number so the opposite bit can be chosen at every position.
func findMaximumXOR(nums []int) int {
	type trieNode struct {
		next [2]*trieNode
	}
	root := &trieNode{}
	insert := func(x int) {
		cur := root
		for bit := 31; bit >= 0; bit-- {
			b := (x >> bit) & 1
			if cur.next[b] == nil {
				cur.next[b] = &trieNode{}
			}
			cur = cur.next[b]
		}
	}
	bestWith := func(x int) int {
		cur := root
		ans := 0
		for bit := 31; bit >= 0; bit-- {
			b := (x >> bit) & 1
			want := b ^ 1
			if cur.next[want] != nil {
				ans |= 1 << bit
				cur = cur.next[want]
			} else {
				cur = cur.next[b]
			}
		}
		return ans
	}
	for _, x := range nums {
		insert(x)
	}
	best := 0
	for _, x := range nums {
		if got := bestWith(x); got > best {
			best = got
		}
	}
	return best
}
