package leetcode

import "strconv"

// LeetCode #592 - Fraction Addition and Subtraction
//
// Add each fraction into a running numerator and denominator, then cancel their gcd.
func fractionAddition(expression string) string {
	numerator, denominator := 0, 1
	for i := 0; i < len(expression); {
		sign := 1
		if expression[i] == '+' || expression[i] == '-' {
			if expression[i] == '-' {
				sign = -1
			}
			i++
		}
		value := 0
		for expression[i] != '/' {
			value = value*10 + int(expression[i]-'0')
			i++
		}
		i++
		bottom := 0
		for i < len(expression) && expression[i] != '+' && expression[i] != '-' {
			bottom = bottom*10 + int(expression[i]-'0')
			i++
		}
		numerator = numerator*bottom + sign*value*denominator
		denominator *= bottom
		divisor := gcd(absInt(numerator), denominator)
		numerator /= divisor
		denominator /= divisor
	}
	return strconv.Itoa(numerator) + "/" + strconv.Itoa(denominator)
}

func gcd(a, b int) int {
	for b != 0 {
		a, b = b, a%b
	}
	return a
}
