package leetcode

// LeetCode #193 - Valid Phone Numbers
//
// Match either (xxx) xxx-xxxx or xxx-xxx-xxxx on a whole line.
const validPhoneNumbersScript = `grep -E '^(\([0-9]{3}\) [0-9]{3}-[0-9]{4}|[0-9]{3}-[0-9]{3}-[0-9]{4})$' file.txt`
