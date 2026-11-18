package leetcode

import "strings"

// LeetCode #591 - Tag Validator
//
// LeetCode names this isValid; that name already belongs to valid parentheses (#20).
// Open tags sit on a stack, CDATA is plain text, and nothing may sit outside the root tag.
func isValidTag(code string) bool {
	if code == "" || code[0] != '<' {
		return false
	}
	var stack []string
	opened := false
	for i := 0; i < len(code); {
		if opened && len(stack) == 0 {
			return false
		}
		if strings.HasPrefix(code[i:], "<![CDATA[") {
			if len(stack) == 0 {
				return false
			}
			end := strings.Index(code[i+9:], "]]>")
			if end < 0 {
				return false
			}
			i += 9 + end + 3
			continue
		}
		if code[i] != '<' {
			i++
			continue
		}
		closing := i+1 < len(code) && code[i+1] == '/'
		start := i + 1
		if closing {
			start = i + 2
		}
		end := strings.IndexByte(code[start:], '>')
		if end < 0 || !validTagName(code[start:start+end]) {
			return false
		}
		name := code[start : start+end]
		if closing {
			if len(stack) == 0 || stack[len(stack)-1] != name {
				return false
			}
			stack = stack[:len(stack)-1]
		} else {
			stack = append(stack, name)
			opened = true
		}
		i = start + end + 1
	}
	return opened && len(stack) == 0
}

func validTagName(name string) bool {
	if len(name) < 1 || len(name) > 9 {
		return false
	}
	for i := 0; i < len(name); i++ {
		if name[i] < 'A' || name[i] > 'Z' {
			return false
		}
	}
	return true
}
