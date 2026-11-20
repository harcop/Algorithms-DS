package leetcode

// LeetCode #605 - Can Place Flowers
//
// An empty plot can take a flower when both neighbors are empty, and each placement spends one.
func canPlaceFlowers(flowerbed []int, n int) bool {
	for i := 0; i < len(flowerbed) && n > 0; i++ {
		if flowerbed[i] == 1 {
			continue
		}
		leftEmpty := i == 0 || flowerbed[i-1] == 0
		rightEmpty := i == len(flowerbed)-1 || flowerbed[i+1] == 0
		if leftEmpty && rightEmpty {
			flowerbed[i] = 1
			n--
		}
	}
	return n <= 0
}
