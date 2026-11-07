package leetcode

// LeetCode #423 - Reconstruct Original Digits from English
//
// Unique letters pin down 0, 2, 4, 6, and 8; the remaining digits fall out by subtraction.
func originalDigits(s string) string {
	var count [26]int
	for i := 0; i < len(s); i++ {
		count[s[i]-'a']++
	}
	var digit [10]int
	digit[0] = count['z'-'a']
	digit[2] = count['w'-'a']
	digit[4] = count['u'-'a']
	digit[6] = count['x'-'a']
	digit[8] = count['g'-'a']
	digit[3] = count['h'-'a'] - digit[8]
	digit[5] = count['f'-'a'] - digit[4]
	digit[7] = count['s'-'a'] - digit[6]
	digit[1] = count['o'-'a'] - digit[0] - digit[2] - digit[4]
	digit[9] = count['i'-'a'] - digit[5] - digit[6] - digit[8]

	out := make([]byte, 0, len(s)/3)
	for d := 0; d < 10; d++ {
		for n := 0; n < digit[d]; n++ {
			out = append(out, byte('0'+d))
		}
	}
	return string(out)
}
