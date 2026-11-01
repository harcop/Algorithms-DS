package leetcode

// LeetCode #327 - Count of Range Sum
//
// Merge-sort the prefix sums and count how many right-half values fall inside each left window.
func countRangeSum(nums []int, lower, upper int) int {
	n := len(nums)
	if n == 0 {
		return 0
	}
	prefix := make([]int64, n+1)
	for i := 0; i < n; i++ {
		prefix[i+1] = prefix[i] + int64(nums[i])
	}
	tmp := make([]int64, n+1)
	low, high := int64(lower), int64(upper)
	var countBetween func(lo, hi int) int
	countBetween = func(lo, hi int) int {
		if lo == hi {
			return 0
		}
		mid := (lo + hi) / 2
		count := countBetween(lo, mid) + countBetween(mid+1, hi)
		j, k := mid+1, mid+1
		for i := lo; i <= mid; i++ {
			for j <= hi && prefix[j]-prefix[i] < low {
				j++
			}
			for k <= hi && prefix[k]-prefix[i] <= high {
				k++
			}
			count += k - j
		}
		p, q, t := lo, mid+1, lo
		for p <= mid && q <= hi {
			if prefix[p] <= prefix[q] {
				tmp[t] = prefix[p]
				p++
			} else {
				tmp[t] = prefix[q]
				q++
			}
			t++
		}
		for p <= mid {
			tmp[t] = prefix[p]
			p++
			t++
		}
		for q <= hi {
			tmp[t] = prefix[q]
			q++
			t++
		}
		copy(prefix[lo:hi+1], tmp[lo:hi+1])
		return count
	}
	return countBetween(0, n)
}
