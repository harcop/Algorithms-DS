package leetcode

// LeetCode #43 - Multiply Strings
//
// Multiply digit by digit into a result buffer, then fold leftover carries into single digits.
func multiply(num1 string, num2 string) string {
	if num1 == "0" || num2 == "0" {
		return "0"
	}

	res := make([]int, len(num1)+len(num2))
	for i := len(num1) - 1; i >= 0; i-- {
		for j := len(num2) - 1; j >= 0; j-- {
			sum := int(num1[i]-'0')*int(num2[j]-'0') + res[i+j+1]
			res[i+j+1] = sum % 10
			res[i+j] += sum / 10
		}
	}

	for i := len(res) - 1; i > 0; i-- {
		res[i-1] += res[i] / 10
		res[i] %= 10
	}

	start := 0
	for start < len(res)-1 && res[start] == 0 {
		start++
	}

	out := make([]byte, len(res)-start)
	for i := start; i < len(res); i++ {
		out[i-start] = byte(res[i]) + '0'
	}
	return string(out)
}
