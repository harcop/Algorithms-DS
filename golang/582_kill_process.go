package leetcode

// LeetCode #582 - Kill Process
//
// Children hang off each parent, and killing a process kills that whole subtree.
func killProcess(pid []int, ppid []int, kill int) []int {
	children := map[int][]int{}
	for i, id := range pid {
		children[ppid[i]] = append(children[ppid[i]], id)
	}
	killed := make([]int, 0)
	stack := []int{kill}
	for len(stack) > 0 {
		id := stack[len(stack)-1]
		stack = stack[:len(stack)-1]
		killed = append(killed, id)
		stack = append(stack, children[id]...)
	}
	return killed
}
