package leetcode

import "strconv"

// LeetCode #535 - Encode and Decode TinyURL
//
// LeetCode names these encode and decode; those names already belong to the length-prefixed codec (#271).
// Each new address gets the next integer key, and that key is stored both ways.
type TinyURL struct {
	urlToKey map[string]string
	keyToURL map[string]string
	next     int
}

func NewTinyURL() *TinyURL {
	return &TinyURL{
		urlToKey: map[string]string{},
		keyToURL: map[string]string{},
	}
}

func (codec *TinyURL) Encode(longURL string) string {
	if key, ok := codec.urlToKey[longURL]; ok {
		return "http://tinyurl.com/" + key
	}
	key := strconv.Itoa(codec.next)
	codec.next++
	codec.urlToKey[longURL] = key
	codec.keyToURL[key] = longURL
	return "http://tinyurl.com/" + key
}

func (codec *TinyURL) Decode(shortURL string) string {
	const prefix = "http://tinyurl.com/"
	key := shortURL
	if len(shortURL) >= len(prefix) && shortURL[:len(prefix)] == prefix {
		key = shortURL[len(prefix):]
	}
	return codec.keyToURL[key]
}
