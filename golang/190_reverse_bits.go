package leetcode

// LeetCode #190 - Reverse Bits
//
// Shift each of the 32 bits from the low end of the input onto the low end of the result.
func reverseBits(num uint32) uint32 {
	var out uint32
	for i := 0; i < 32; i++ {
		out = (out << 1) | (num & 1)
		num >>= 1
	}
	return out
}
