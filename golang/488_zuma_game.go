package leetcode

// LeetCode #488 - Zuma Game
//
// Insert the fewest balls that finish a color group, then clear every run of three or more.
func findMinStep(board string, hand string) int {
	var counts [26]int
	for i := 0; i < len(hand); i++ {
		counts[hand[i]-'A']++
	}
	memo := map[zumaState]int{}
	steps := searchZuma(collapseZuma(board), counts, memo)
	if steps >= 6 {
		return -1
	}
	return steps
}

type zumaState struct {
	board string
	hand  [26]int
}

func searchZuma(board string, hand [26]int, memo map[zumaState]int) int {
	if board == "" {
		return 0
	}
	state := zumaState{board, hand}
	if steps, ok := memo[state]; ok {
		return steps
	}
	best := 6
	for i := 0; i < len(board); {
		j := i
		for j < len(board) && board[j] == board[i] {
			j++
		}
		color := board[i] - 'A'
		need := 3 - (j - i)
		if need < 1 {
			need = 1
		}
		if hand[color] >= need {
			hand[color] -= need
			next := collapseZuma(board[:i] + board[j:])
			best = min(best, need+searchZuma(next, hand, memo))
			hand[color] += need
		}
		i = j
	}
	memo[state] = best
	return best
}

func collapseZuma(board string) string {
	current := board
	for {
		if current == "" {
			return current
		}
		var out []byte
		changed := false
		for i := 0; i < len(current); {
			j := i
			for j < len(current) && current[j] == current[i] {
				j++
			}
			if j-i >= 3 {
				changed = true
			} else {
				out = append(out, current[i:j]...)
			}
			i = j
		}
		current = string(out)
		if !changed {
			return current
		}
	}
}
