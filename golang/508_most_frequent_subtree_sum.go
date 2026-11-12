package leetcode

// LeetCode #508 - Most Frequent Subtree Sum
//
// Each subtree reports its sum, and the sums that occur most often are returned.
func findFrequentTreeSum(root *TreeNode) []int {
	freq := map[int]int{}
	var walk func(*TreeNode) int
	walk = func(node *TreeNode) int {
		if node == nil {
			return 0
		}
		sum := node.Val + walk(node.Left) + walk(node.Right)
		freq[sum]++
		return sum
	}
	walk(root)
	best := 0
	for _, count := range freq {
		if count > best {
			best = count
		}
	}
	var out []int
	for sum, count := range freq {
		if count == best {
			out = append(out, sum)
		}
	}
	return out
}
