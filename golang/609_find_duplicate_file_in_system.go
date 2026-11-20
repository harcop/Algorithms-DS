package leetcode

import "strings"

// LeetCode #609 - Find Duplicate File in System
//
// Group full paths by the text inside the parentheses, and keep groups that share a content.
func findDuplicateFiles(paths []string) [][]string {
	groups := map[string][]string{}
	for _, entry := range paths {
		parts := strings.Split(entry, " ")
		dir := parts[0]
		for _, file := range parts[1:] {
			open := strings.IndexByte(file, '(')
			name := file[:open]
			content := file[open+1 : len(file)-1]
			groups[content] = append(groups[content], dir+"/"+name)
		}
	}
	out := make([][]string, 0)
	for _, files := range groups {
		if len(files) > 1 {
			out = append(out, files)
		}
	}
	return out
}
