package leetcode

import "strings"

// LeetCode #331 - Verify Preorder Serialization of a Binary Tree
//
// Each node consumes one slot and a non-null node opens two more.
func isValidSerialization(preorder string) bool {
	slots := 1
	for _, token := range strings.Split(preorder, ",") {
		if slots == 0 {
			return false
		}
		slots--
		if token != "#" {
			slots += 2
		}
	}
	return slots == 0
}
