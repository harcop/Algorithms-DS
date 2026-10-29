package leetcode

import "sort"

// LeetCode #301 - Remove Invalid Parentheses
//
// Remove one parenthesis at a time and keep the shortest strings whose parentheses balance.
func removeInvalidParentheses(s string) []string {
	balanced := func(text string) bool {
		open := 0
		for i := 0; i < len(text); i++ {
			switch text[i] {
			case '(':
				open++
			case ')':
				open--
				if open < 0 {
					return false
				}
			}
		}
		return open == 0
	}
	seen := map[string]struct{}{s: {}}
	queue := []string{s}
	var out []string
	found := false
	for head := 0; head < len(queue); head++ {
		current := queue[head]
		if balanced(current) {
			out = append(out, current)
			found = true
		}
		if found {
			continue
		}
		for i := 0; i < len(current); i++ {
			if current[i] != '(' && current[i] != ')' {
				continue
			}
			next := current[:i] + current[i+1:]
			if _, ok := seen[next]; ok {
				continue
			}
			seen[next] = struct{}{}
			queue = append(queue, next)
		}
	}
	sort.Strings(out)
	return out
}
