package leetcode

import (
	"math/big"
	"strings"
)

// LeetCode #564 - Find the Closest Palindrome
//
// Mirror the first half, and the halves just below and above it, then keep the nearer palindrome.
func nearestPalindromic(n string) string {
	length := len(n)
	number, _ := new(big.Int).SetString(n, 10)
	candidates := []*big.Int{
		new(big.Int).Sub(pow10(length-1), big.NewInt(1)),
		new(big.Int).Add(pow10(length), big.NewInt(1)),
	}
	prefix, _ := new(big.Int).SetString(n[:(length+1)/2], 10)
	for _, delta := range []int64{-1, 0, 1} {
		varied := new(big.Int).Add(new(big.Int).Set(prefix), big.NewInt(delta))
		candidates = append(candidates, mirrorPrefix(varied, length))
	}

	var best *big.Int
	bestGap := new(big.Int)
	for _, candidate := range candidates {
		if candidate.Sign() < 0 || candidate.Cmp(number) == 0 {
			continue
		}
		gap := new(big.Int).Abs(new(big.Int).Sub(candidate, number))
		if best == nil || gap.Cmp(bestGap) < 0 || (gap.Cmp(bestGap) == 0 && candidate.Cmp(best) < 0) {
			best = new(big.Int).Set(candidate)
			bestGap = gap
		}
	}
	return best.String()
}

func pow10(exp int) *big.Int {
	if exp <= 0 {
		return big.NewInt(1)
	}
	return new(big.Int).Exp(big.NewInt(10), big.NewInt(int64(exp)), nil)
}

func mirrorPrefix(prefix *big.Int, length int) *big.Int {
	if prefix.Sign() < 0 {
		return big.NewInt(-1)
	}
	half := prefix.String()
	var pal strings.Builder
	pal.WriteString(half)
	start := len(half) - 1
	if length%2 == 1 {
		start = len(half) - 2
	}
	for i := start; i >= 0; i-- {
		pal.WriteByte(half[i])
	}
	value, ok := new(big.Int).SetString(pal.String(), 10)
	if !ok {
		return big.NewInt(0)
	}
	return value
}
