package leetcode

// LeetCode #192 - Word Frequency
//
// Split on spaces, then count the sorted words.
const wordFrequencyScript = `cat words.txt | tr -s ' ' '\n' | sort | uniq -c | sort -nr | awk '{print $2, $1}'`
