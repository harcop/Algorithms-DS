package leetcode

import "strings"

// LeetCode #468 - Validate IP Address
//
// An IPv4 address is four decimal groups from 0 to 255, and an IPv6 address is eight hexadecimal groups.
func validIPAddress(queryIP string) string {
	if validIPv4(queryIP) {
		return "IPv4"
	}
	if validIPv6(queryIP) {
		return "IPv6"
	}
	return "Neither"
}

func validIPv4(ip string) bool {
	parts := strings.Split(ip, ".")
	if len(parts) != 4 {
		return false
	}
	for _, part := range parts {
		if part == "" || len(part) > 3 {
			return false
		}
		if len(part) > 1 && part[0] == '0' {
			return false
		}
		value := 0
		for i := 0; i < len(part); i++ {
			if part[i] < '0' || part[i] > '9' {
				return false
			}
			value = value*10 + int(part[i]-'0')
		}
		if value > 255 {
			return false
		}
	}
	return true
}

func validIPv6(ip string) bool {
	parts := strings.Split(ip, ":")
	if len(parts) != 8 {
		return false
	}
	for _, part := range parts {
		if part == "" || len(part) > 4 {
			return false
		}
		for i := 0; i < len(part); i++ {
			c := part[i]
			digit := c >= '0' && c <= '9'
			lower := c >= 'a' && c <= 'f'
			upper := c >= 'A' && c <= 'F'
			if !digit && !lower && !upper {
				return false
			}
		}
	}
	return true
}
