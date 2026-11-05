package leetcode

// LeetCode #415 - Add Strings
//
// Add the digits from the right and carry the tens place forward.
func addStrings(num1 string, num2 string) string {
	i, j := len(num1), len(num2)
	carry := 0
	out := make([]byte, 0, max(i, j)+1)
	for i > 0 || j > 0 || carry > 0 {
		sum := carry
		if i > 0 {
			i--
			sum += int(num1[i] - '0')
		}
		if j > 0 {
			j--
			sum += int(num2[j] - '0')
		}
		out = append(out, byte('0'+sum%10))
		carry = sum / 10
	}
	for left, right := 0, len(out)-1; left < right; left, right = left+1, right-1 {
		out[left], out[right] = out[right], out[left]
	}
	return string(out)
}
