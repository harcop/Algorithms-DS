package leetcode

// LeetCode #393 - UTF-8 Validation
//
// A leading byte sets how many continuation bytes must follow.
func validUtf8(data []int) bool {
	need := 0
	for _, value := range data {
		b := value & 0xFF
		if need == 0 {
			switch {
			case b>>7 == 0:
				continue
			case b>>5 == 0b110:
				need = 1
			case b>>4 == 0b1110:
				need = 2
			case b>>3 == 0b11110:
				need = 3
			default:
				return false
			}
			continue
		}
		if b>>6 != 0b10 {
			return false
		}
		need--
	}
	return need == 0
}
