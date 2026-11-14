package leetcode

import "testing"

func TestTinyURLRoundTrip(t *testing.T) {
	codec := NewTinyURL()
	longURL := "https://example.com/a"
	shortURL := codec.Encode(longURL)
	if got := codec.Decode(shortURL); got != longURL {
		t.Fatalf("Decode() = %q, want %q", got, longURL)
	}
	if again := codec.Encode(longURL); again != shortURL {
		t.Fatalf("Encode() = %q, want the same short URL %q", again, shortURL)
	}
}
