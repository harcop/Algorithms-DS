package leetcode

import (
	"sort"
	"strings"
)

// LeetCode #588 - Design In-Memory File System
//
// Each path segment is a node, a file stores text, and a directory lists its children in order.
type fsNode struct {
	children map[string]*fsNode
	file     bool
	content  string
}

type FileSystem struct {
	root *fsNode
}

func NewFileSystem() *FileSystem {
	return &FileSystem{root: &fsNode{children: map[string]*fsNode{}}}
}

func (fs *FileSystem) descend(path string, create bool) *fsNode {
	node := fs.root
	if path == "/" {
		return node
	}
	for _, name := range strings.Split(path[1:], "/") {
		child, ok := node.children[name]
		if !ok {
			if !create {
				return nil
			}
			child = &fsNode{children: map[string]*fsNode{}}
			node.children[name] = child
		}
		node = child
	}
	return node
}

func (fs *FileSystem) Ls(path string) []string {
	node := fs.descend(path, false)
	if node.file {
		parts := strings.Split(path, "/")
		return []string{parts[len(parts)-1]}
	}
	names := make([]string, 0, len(node.children))
	for name := range node.children {
		names = append(names, name)
	}
	sort.Strings(names)
	return names
}

func (fs *FileSystem) Mkdir(path string) {
	fs.descend(path, true)
}

func (fs *FileSystem) AddContentToFile(filePath string, content string) {
	node := fs.descend(filePath, true)
	node.file = true
	node.content += content
}

func (fs *FileSystem) ReadContentFromFile(filePath string) string {
	return fs.descend(filePath, false).content
}
