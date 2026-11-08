package leetcode

// LeetCode #457 - Circular Array Loop
//
// Follow same-direction jumps with a slow and fast cursor, and ignore a cycle that is only one index long.
func circularArrayLoop(nums []int) bool {
	n := len(nums)
	if n == 0 {
		return false
	}
	next := func(i int) int {
		return ((i+nums[i])%n + n) % n
	}
	for i := 0; i < n; i++ {
		if nums[i] == 0 {
			continue
		}
		slow, fast := i, i
		for nums[slow]*nums[next(slow)] > 0 && nums[fast]*nums[next(fast)] > 0 && nums[next(fast)]*nums[next(next(fast))] > 0 {
			slow = next(slow)
			fast = next(next(fast))
			if slow == fast {
				if slow != next(slow) {
					return true
				}
				break
			}
		}
		sign := nums[i]
		for j := i; sign*nums[j] > 0; {
			following := next(j)
			nums[j] = 0
			j = following
		}
	}
	return false
}
