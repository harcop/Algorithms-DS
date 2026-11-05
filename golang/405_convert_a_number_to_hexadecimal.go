package leetcode

// LeetCode #405 - Convert a Number to Hexadecimal
//
// Read the number as an unsigned 32-bit value and emit its nibbles from low to high.
func toHex(num int) string {
	if num == 0 {
		return "0"
	}
	n := uint32(num)
	digits := "0123456789abcdef"
	out := make([]byte, 0, 8)
	for n != 0 {
		out = append(out, digits[n&0xF])
		n >>= 4
	}
	for i, j := 0, len(out)-1; i < j; i, j = i+1, j-1 {
		out[i], out[j] = out[j], out[i]
	}
	return string(out)
}
