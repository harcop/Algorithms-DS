package leetcode

// LeetCode #149 - Max Points on a Line
//
// From each point, group the others by reduced slope and keep the largest group.
func maxPoints(points [][]int) int {
	n := len(points)
	if n <= 2 {
		return n
	}
	best := 0
	for i := 0; i < n; i++ {
		slopes := map[[2]int]int{}
		same := 1
		for j := i + 1; j < n; j++ {
			dx := points[j][0] - points[i][0]
			dy := points[j][1] - points[i][1]
			if dx == 0 && dy == 0 {
				same++
				continue
			}
			g := slopeGCD(absInt(dx), absInt(dy))
			sx, sy := dx/g, dy/g
			if sx < 0 || (sx == 0 && sy < 0) {
				sx, sy = -sx, -sy
			}
			slopes[[2]int{sx, sy}]++
		}
		local := same
		for _, count := range slopes {
			if count+same > local {
				local = count + same
			}
		}
		if local > best {
			best = local
		}
	}
	return best
}

func slopeGCD(a, b int) int {
	for b != 0 {
		a, b = b, a%b
	}
	if a < 1 {
		return 1
	}
	return a
}

func absInt(v int) int {
	if v < 0 {
		return -v
	}
	return v
}
