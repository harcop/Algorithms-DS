package leetcode

import (
	"strconv"
	"strings"
)

// LeetCode #537 - Complex Number Multiplication
//
// Multiply the real and imaginary parts, then write the product as real+imaginaryi.
func complexNumberMultiply(num1, num2 string) string {
	real1, imag1 := parseComplex(num1)
	real2, imag2 := parseComplex(num2)
	real := real1*real2 - imag1*imag2
	imag := real1*imag2 + imag1*real2
	return strconv.Itoa(real) + "+" + strconv.Itoa(imag) + "i"
}

func parseComplex(s string) (int, int) {
	s = strings.TrimSuffix(s, "i")
	plus := strings.IndexByte(s, '+')
	real, _ := strconv.Atoi(s[:plus])
	imag, _ := strconv.Atoi(s[plus+1:])
	return real, imag
}
