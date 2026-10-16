package leetcode

import "strings"

// LeetCode #71 - Simplify Path
//
// Split on slashes, ignore empty and "." parts, and pop the stack for "..".
func simplifyPath(path string) string {
	var stack []string
	for _, part := range strings.Split(path, "/") {
		switch part {
		case "", ".":
		case "..":
			if len(stack) > 0 {
				stack = stack[:len(stack)-1]
			}
		default:
			stack = append(stack, part)
		}
	}
	if len(stack) == 0 {
		return "/"
	}
	return "/" + strings.Join(stack, "/")
}
