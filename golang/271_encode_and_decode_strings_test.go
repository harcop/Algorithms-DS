package leetcode

import "testing"

func TestEncodeDecodeRoundTrip(t *testing.T) {
	want := []string{"lint", "code", "love", "you"}
	got := decode(encode(want))
	if len(got) != len(want) {
		t.Fatalf("decode(encode()) = %v, want %v", got, want)
	}
	for i := range want {
		if got[i] != want[i] {
			t.Fatalf("decode(encode()) = %v, want %v", got, want)
		}
	}
}
