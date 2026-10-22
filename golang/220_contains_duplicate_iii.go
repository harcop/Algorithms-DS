package leetcode

// LeetCode #220 - Contains Duplicate III
//
// Bucket values by the allowed difference so a nearby almost-duplicate shares a bucket or a neighbor.
func containsNearbyAlmostDuplicate(nums []int, indexDiff int, valueDiff int) bool {
	if valueDiff < 0 || indexDiff <= 0 {
		return false
	}
	width := int64(valueDiff) + 1
	buckets := map[int64]int64{}
	bucketID := func(value int64) int64 {
		if value < 0 {
			return (value+1)/width - 1
		}
		return value / width
	}
	for i, num := range nums {
		value := int64(num)
		id := bucketID(value)
		if _, ok := buckets[id]; ok {
			return true
		}
		if neighbor, ok := buckets[id-1]; ok && absInt64(value-neighbor) <= int64(valueDiff) {
			return true
		}
		if neighbor, ok := buckets[id+1]; ok && absInt64(value-neighbor) <= int64(valueDiff) {
			return true
		}
		buckets[id] = value
		if i >= indexDiff {
			delete(buckets, bucketID(int64(nums[i-indexDiff])))
		}
	}
	return false
}
