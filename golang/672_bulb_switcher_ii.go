package leetcode

import "math/bits"

// LeetCode #672 - Bulb Switcher II
//
// Presses cancel in pairs, and bulbs repeat every six positions, so sixteen press masks cover every state.
func flipLights(n, presses int) int {
	if n > 6 {
		n = 6
	}
	ops := []int{0b111111, 0b010101, 0b101010, 0b100100}
	seen := map[int]struct{}{}
	for mask := 0; mask < 1<<len(ops); mask++ {
		count := bits.OnesCount(uint(mask))
		if count > presses || count%2 != presses%2 {
			continue
		}
		state := 0
		for i, op := range ops {
			if mask>>i&1 == 1 {
				state ^= op
			}
		}
		state = (state & ((1 << 6) - 1)) >> (6 - n)
		seen[state] = struct{}{}
	}
	return len(seen)
}
