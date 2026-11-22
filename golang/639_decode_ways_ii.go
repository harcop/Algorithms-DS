package leetcode

// LeetCode #639 - Decode Ways II
//
// LeetCode names this numDecodings; that name already belongs to decode ways (#91).
// A digit or star stands alone, and a pair is counted when it can form 10 through 26.
func numDecodingsII(s string) int {
	const mod = 1_000_000_007
	prev2, prev1 := 1, decodeSingleWays(s[0])
	for i := 1; i < len(s); i++ {
		cur := int64(decodeSingleWays(s[i]))*int64(prev1) + int64(decodePairWays(s[i-1], s[i]))*int64(prev2)
		prev2, prev1 = prev1, int(cur%mod)
	}
	return prev1
}

func decodeSingleWays(ch byte) int {
	if ch == '*' {
		return 9
	}
	if ch == '0' {
		return 0
	}
	return 1
}

func decodePairWays(a, b byte) int {
	if a == '*' && b == '*' {
		return 15
	}
	if a == '*' {
		ways := 1
		if b <= '6' {
			ways++
		}
		return ways
	}
	if b == '*' {
		switch a {
		case '1':
			return 9
		case '2':
			return 6
		default:
			return 0
		}
	}
	if a == '1' || (a == '2' && b <= '6') {
		return 1
	}
	return 0
}
