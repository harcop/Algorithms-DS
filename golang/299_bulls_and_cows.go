package leetcode

import "strconv"

// LeetCode #299 - Bulls and Cows
//
// Bulls are exact matches; cows are the leftover digits that appear in both strings.
func getHint(secret string, guess string) string {
	bulls := 0
	var secretCount, guessCount [10]int
	for i := 0; i < len(secret); i++ {
		if secret[i] == guess[i] {
			bulls++
			continue
		}
		secretCount[secret[i]-'0']++
		guessCount[guess[i]-'0']++
	}
	cows := 0
	for digit := 0; digit < 10; digit++ {
		cows += min(secretCount[digit], guessCount[digit])
	}
	return strconv.Itoa(bulls) + "A" + strconv.Itoa(cows) + "B"
}
