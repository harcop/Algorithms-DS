package leetcode

// LeetCode #420 - Strong Password Checker
//
// Length, missing character classes, and runs of three share the same insertions and replacements.
func strongPasswordChecker(password string) int {
	n := len(password)
	lower, upper, digit := false, false, false
	for i := 0; i < n; i++ {
		switch c := password[i]; {
		case c >= 'a' && c <= 'z':
			lower = true
		case c >= 'A' && c <= 'Z':
			upper = true
		case c >= '0' && c <= '9':
			digit = true
		}
	}
	missing := 0
	if !lower {
		missing++
	}
	if !upper {
		missing++
	}
	if !digit {
		missing++
	}

	replace, one, two := 0, 0, 0
	for i := 2; i < n; {
		if password[i] == password[i-1] && password[i] == password[i-2] {
			length := 2
			for i < n && password[i] == password[i-1] {
				length++
				i++
			}
			replace += length / 3
			switch length % 3 {
			case 0:
				one++
			case 1:
				two++
			}
			continue
		}
		i++
	}

	if n < 6 {
		return max(missing, 6-n)
	}
	if n <= 20 {
		return max(missing, replace)
	}
	extra := n - 20
	replace -= min(extra, one)
	replace -= min(max(extra-one, 0), two*2) / 2
	replace -= max(extra-one-2*two, 0) / 3
	return extra + max(missing, replace)
}
