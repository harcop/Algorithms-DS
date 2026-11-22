package leetcode

import "strconv"

// LeetCode #640 - Solve the Equation
//
// Both sides collapse to an x coefficient and a constant, then the equation is solved for x.
func solveEquation(equation string) string {
	sides := splitEquation(equation)
	leftX, leftC := parseEquationSide(sides[0])
	rightX, rightC := parseEquationSide(sides[1])
	coef := leftX - rightX
	constant := rightC - leftC
	if coef == 0 {
		if constant == 0 {
			return "Infinite solutions"
		}
		return "No solution"
	}
	return "x=" + strconv.Itoa(constant/coef)
}

func splitEquation(equation string) [2]string {
	for i := 0; i < len(equation); i++ {
		if equation[i] == '=' {
			return [2]string{equation[:i], equation[i+1:]}
		}
	}
	return [2]string{equation, ""}
}

func parseEquationSide(side string) (coef, constant int) {
	if side == "" || (side[0] != '+' && side[0] != '-') {
		side = "+" + side
	}
	for i := 0; i < len(side); {
		sign := 1
		if side[i] == '-' {
			sign = -1
		}
		i++
		number := 0
		seenDigit := false
		for i < len(side) && side[i] >= '0' && side[i] <= '9' {
			seenDigit = true
			number = number*10 + int(side[i]-'0')
			i++
		}
		if i < len(side) && side[i] == 'x' {
			if !seenDigit {
				number = 1
			}
			coef += sign * number
			i++
			continue
		}
		constant += sign * number
	}
	return coef, constant
}
