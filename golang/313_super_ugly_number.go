package leetcode

import "math"

// LeetCode #313 - Super Ugly Number
//
// Each next value is the smallest prime times an earlier ugly number, skipping duplicates.
func nthSuperUglyNumber(n int, primes []int) int {
	ugly := make([]int64, n)
	ugly[0] = 1
	index := make([]int, len(primes))
	for i := 1; i < n; i++ {
		next := int64(math.MaxInt64)
		for j, prime := range primes {
			if candidate := ugly[index[j]] * int64(prime); candidate < next {
				next = candidate
			}
		}
		ugly[i] = next
		for j, prime := range primes {
			if ugly[index[j]]*int64(prime) == next {
				index[j]++
			}
		}
	}
	return int(ugly[n-1])
}
