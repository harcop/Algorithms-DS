package leetcode

// LeetCode #458 - Poor Pigs
//
// Each pig has one more state than the number of tests, so the pigs count in that base until every bucket is covered.
func poorPigs(buckets int, minutesToDie int, minutesToTest int) int {
	states := int64(minutesToTest/minutesToDie + 1)
	pigs := 0
	covered := int64(1)
	limit := int64(buckets)
	for covered < limit {
		covered *= states
		pigs++
	}
	return pigs
}
