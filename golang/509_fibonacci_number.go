package leetcode

// LeetCode #509 - Fibonacci Number
//
// Two running values produce the next Fibonacci number without storing the whole sequence.
func fib(n int) int {
	if n < 2 {
		return n
	}
	a, b := 0, 1
	for i := 2; i <= n; i++ {
		a, b = b, a+b
	}
	return b
}
