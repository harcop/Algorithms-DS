package leetcode

// LeetCode #67 - Add Binary
//
// Add both strings from the right, carrying a 1 whenever a column sums to 2 or 3.
func addBinary(a string, b string) string {
	i, j := len(a), len(b)
	carry := 0
	out := make([]byte, 0, max(len(a), len(b))+1)

	for i > 0 || j > 0 || carry > 0 {
		sum := carry
		if i > 0 {
			i--
			sum += int(a[i] - '0')
		}
		if j > 0 {
			j--
			sum += int(b[j] - '0')
		}
		out = append(out, byte(sum%2)+'0')
		carry = sum / 2
	}

	for left, right := 0, len(out)-1; left < right; left, right = left+1, right-1 {
		out[left], out[right] = out[right], out[left]
	}
	return string(out)
}
