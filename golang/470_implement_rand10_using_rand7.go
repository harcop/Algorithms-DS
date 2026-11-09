package leetcode

// rand7Draws is the sequence produced by the provided rand7.
var rand7Draws []int

// rand7At is the next index to read from rand7Draws.
var rand7At int

func rand7() int {
	value := rand7Draws[rand7At%len(rand7Draws)]
	rand7At++
	return value
}

// LeetCode #470 - Implement Rand10() Using Rand7()
//
// Two rand7 calls make a number from 0 to 48, and the values below 40 map evenly onto 1 through 10.
func rand10() int {
	for {
		value := (rand7()-1)*7 + (rand7() - 1)
		if value < 40 {
			return value%10 + 1
		}
	}
}
