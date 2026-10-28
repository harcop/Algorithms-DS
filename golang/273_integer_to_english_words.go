package leetcode

import "strings"

// LeetCode #273 - Integer to English Words
//
// Spell each group of three digits and attach Thousand, Million, or Billion.
func numberToWords(num int) string {
	if num == 0 {
		return "Zero"
	}
	below20 := []string{
		"", "One", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight", "Nine", "Ten",
		"Eleven", "Twelve", "Thirteen", "Fourteen", "Fifteen", "Sixteen", "Seventeen",
		"Eighteen", "Nineteen",
	}
	tens := []string{"", "Ten", "Twenty", "Thirty", "Forty", "Fifty", "Sixty", "Seventy", "Eighty", "Ninety"}
	scales := []string{"", "Thousand", "Million", "Billion"}
	var parts []string
	for i := 0; num > 0; i++ {
		if chunk := num % 1000; chunk != 0 {
			part := spellChunk(chunk, below20, tens)
			if scales[i] != "" {
				part += " " + scales[i]
			}
			parts = append(parts, part)
		}
		num /= 1000
	}
	for i, j := 0, len(parts)-1; i < j; i, j = i+1, j-1 {
		parts[i], parts[j] = parts[j], parts[i]
	}
	return strings.TrimSpace(strings.Join(parts, " "))
}

func spellChunk(n int, below20, tens []string) string {
	var words []string
	if n >= 100 {
		words = append(words, below20[n/100], "Hundred")
		n %= 100
	}
	if n >= 20 {
		words = append(words, tens[n/10])
		n %= 10
	}
	if n > 0 {
		words = append(words, below20[n])
	}
	return strings.Join(words, " ")
}
