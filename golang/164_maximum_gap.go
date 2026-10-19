package leetcode

import "math"

// LeetCode #164 - Maximum Gap
//
// Drop the numbers into n-1 buckets so the largest empty span is a gap between buckets.
func maximumGap(nums []int) int {
	n := len(nums)
	if n < 2 {
		return 0
	}
	minV, maxV := nums[0], nums[0]
	for _, value := range nums[1:] {
		if value < minV {
			minV = value
		}
		if value > maxV {
			maxV = value
		}
	}
	if minV == maxV {
		return 0
	}
	buckets := n - 1
	bucketMin := make([]int, buckets)
	bucketMax := make([]int, buckets)
	for i := range bucketMin {
		bucketMin[i] = math.MaxInt32
		bucketMax[i] = math.MinInt32
	}
	span := int64(maxV - minV)
	for _, value := range nums {
		if value == minV || value == maxV {
			continue
		}
		index := int((int64(value-minV) * int64(buckets)) / span)
		if index >= buckets {
			index = buckets - 1
		}
		if value < bucketMin[index] {
			bucketMin[index] = value
		}
		if value > bucketMax[index] {
			bucketMax[index] = value
		}
	}
	best, prev := 0, minV
	for i := 0; i < buckets; i++ {
		if bucketMin[i] == math.MaxInt32 {
			continue
		}
		if gap := bucketMin[i] - prev; gap > best {
			best = gap
		}
		prev = bucketMax[i]
	}
	if gap := maxV - prev; gap > best {
		best = gap
	}
	return best
}
