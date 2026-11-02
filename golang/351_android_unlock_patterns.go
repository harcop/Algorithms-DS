package leetcode

// LeetCode #351 - Android Unlock Patterns
//
// Count paths of length m through n, crossing a key only after it has been used.
func numberOfPatterns(m, n int) int {
	var skip [10][10]int
	jumps := [][3]int{
		{1, 3, 2}, {1, 7, 4}, {3, 9, 6}, {7, 9, 8},
		{1, 9, 5}, {3, 7, 5}, {2, 8, 5}, {4, 6, 5},
	}
	for _, jump := range jumps {
		skip[jump[0]][jump[1]] = jump[2]
		skip[jump[1]][jump[0]] = jump[2]
	}

	var dfs func(curr, remain, visited int) int
	dfs = func(curr, remain, visited int) int {
		if remain == 0 {
			return 1
		}
		total := 0
		for next := 1; next <= 9; next++ {
			bit := 1 << next
			if visited&bit != 0 {
				continue
			}
			mid := skip[curr][next]
			if mid != 0 && visited&(1<<mid) == 0 {
				continue
			}
			total += dfs(next, remain-1, visited|bit)
		}
		return total
	}

	countFrom := func(start, length int) int {
		return dfs(start, length-1, 1<<start)
	}

	total := 0
	for length := m; length <= n; length++ {
		total += countFrom(1, length)*4 + countFrom(2, length)*4 + countFrom(5, length)
	}
	return total
}
