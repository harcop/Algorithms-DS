package leetcode

// LeetCode #174 - Dungeon Game
//
// Walk backward from the princess and keep the health needed to survive each room.
func calculateMinimumHP(dungeon [][]int) int {
	m, n := len(dungeon), len(dungeon[0])
	need := make([][]int, m)
	for i := range need {
		need[i] = make([]int, n)
	}
	need[m-1][n-1] = max(1-dungeon[m-1][n-1], 1)
	for j := n - 2; j >= 0; j-- {
		need[m-1][j] = max(need[m-1][j+1]-dungeon[m-1][j], 1)
	}
	for i := m - 2; i >= 0; i-- {
		need[i][n-1] = max(need[i+1][n-1]-dungeon[i][n-1], 1)
	}
	for i := m - 2; i >= 0; i-- {
		for j := n - 2; j >= 0; j-- {
			need[i][j] = max(min(need[i+1][j], need[i][j+1])-dungeon[i][j], 1)
		}
	}
	return need[0][0]
}
