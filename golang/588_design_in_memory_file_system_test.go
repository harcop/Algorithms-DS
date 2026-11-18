package leetcode

import "testing"

func TestFileSystemExample(t *testing.T) {
	fs := NewFileSystem()
	if got := fs.Ls("/"); len(got) != 0 {
		t.Fatalf("Ls(/) = %v, want none", got)
	}
	fs.Mkdir("/a/b/c")
	fs.AddContentToFile("/a/b/c/d", "hello")
	if !sameStrings(fs.Ls("/"), []string{"a"}) {
		t.Fatalf("Ls(/) = %v, want [a]", fs.Ls("/"))
	}
	if !sameStrings(fs.Ls("/a/b/c/d"), []string{"d"}) {
		t.Fatalf("Ls(file) = %v, want [d]", fs.Ls("/a/b/c/d"))
	}
	if got := fs.ReadContentFromFile("/a/b/c/d"); got != "hello" {
		t.Fatalf("ReadContentFromFile = %q, want hello", got)
	}
	fs.AddContentToFile("/a/b/c/d", " world")
	if got := fs.ReadContentFromFile("/a/b/c/d"); got != "hello world" {
		t.Fatalf("ReadContentFromFile = %q, want hello world", got)
	}
}
