package leetcode

// LeetCode #204 - Count Primes
//
// Sieve out multiples and count the numbers below n that remain prime.
func countPrimes(n int) int {
	if n <= 2 {
		return 0
	}
	prime := make([]bool, n)
	for i := 2; i < n; i++ {
		prime[i] = true
	}
	count := 0
	for i := 2; i < n; i++ {
		if !prime[i] {
			continue
		}
		count++
		if i > n/i {
			continue
		}
		for j := i * i; j < n; j += i {
			prime[j] = false
		}
	}
	return count
}
