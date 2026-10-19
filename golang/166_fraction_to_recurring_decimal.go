package leetcode

import (
	"strconv"
	"strings"
)

// LeetCode #166 - Fraction to Recurring Decimal
//
// Divide, and wrap the digits that repeat once a remainder shows up again.
func fractionToDecimal(numerator int, denominator int) string {
	if numerator == 0 {
		return "0"
	}
	var out strings.Builder
	if (numerator < 0) != (denominator < 0) {
		out.WriteByte('-')
	}
	n := absInt64(int64(numerator))
	d := absInt64(int64(denominator))
	out.WriteString(strconv.FormatInt(n/d, 10))
	n %= d
	if n == 0 {
		return out.String()
	}
	out.WriteByte('.')
	seen := map[int64]int{}
	for n != 0 {
		if index, ok := seen[n]; ok {
			text := out.String()
			return text[:index] + "(" + text[index:] + ")"
		}
		seen[n] = out.Len()
		n *= 10
		out.WriteString(strconv.FormatInt(n/d, 10))
		n %= d
	}
	return out.String()
}

func absInt64(value int64) int64 {
	if value < 0 {
		return -value
	}
	return value
}
