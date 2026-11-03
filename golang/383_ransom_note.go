package leetcode

// LeetCode #383 - Ransom Note
//
// The magazine must contain at least as many of each letter as the note uses.
func canConstruct(ransomNote, magazine string) bool {
	var count [26]int
	for i := 0; i < len(magazine); i++ {
		count[magazine[i]-'a']++
	}
	for i := 0; i < len(ransomNote); i++ {
		count[ransomNote[i]-'a']--
		if count[ransomNote[i]-'a'] < 0 {
			return false
		}
	}
	return true
}
