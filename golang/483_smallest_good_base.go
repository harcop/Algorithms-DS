package leetcode

import (
	"math/bits"
	"strconv"
)

// LeetCode #483 - Smallest Good Base
//
// Search the longest all-ones representation, and the base is the number whose geometric sum equals n.
func smallestGoodBase(n string) string {
	value, _ := strconv.ParseUint(n, 10, 64)
	for length := bits.Len64(value); length >= 3; length-- {
		lo := uint64(2)
		hi := value - 1
		for lo <= hi {
			base := lo + (hi-lo)/2
			switch compareGoodBase(base, length, value) {
			case 0:
				return strconv.FormatUint(base, 10)
			case -1:
				lo = base + 1
			default:
				if base == 0 {
					hi = 0
					break
				}
				hi = base - 1
			}
		}
	}
	return strconv.FormatUint(value-1, 10)
}

func compareGoodBase(base uint64, length int, target uint64) int {
	sum := uint64(1)
	power := uint64(1)
	for i := 1; i < length; i++ {
		high, next := bits.Mul64(power, base)
		if high != 0 {
			return 1
		}
		power = next
		total, carry := bits.Add64(sum, power, 0)
		if carry != 0 || total > target {
			return 1
		}
		sum = total
	}
	if sum < target {
		return -1
	}
	if sum > target {
		return 1
	}
	return 0
}
