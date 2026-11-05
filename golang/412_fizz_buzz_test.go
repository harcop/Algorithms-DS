package leetcode

import "testing"

func TestFizzBuzzExample(t *testing.T) {
	got := fizzBuzz(5)
	want := []string{"1", "2", "Fizz", "4", "Buzz"}
	if len(got) != len(want) {
		t.Fatalf("fizzBuzz(5) = %v, want %v", got, want)
	}
	for i := range want {
		if got[i] != want[i] {
			t.Fatalf("fizzBuzz(5) = %v, want %v", got, want)
		}
	}
	got = fizzBuzz(15)
	if got[14] != "FizzBuzz" || got[2] != "Fizz" || got[4] != "Buzz" {
		t.Fatalf("fizzBuzz(15) = %v", got)
	}
}
