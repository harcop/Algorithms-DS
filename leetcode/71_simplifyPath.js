// Given an absolute path for a Unix-style file system, which begins with a slash '/', transform this path into its simplified canonical path.

// In Unix-style file system context, a single period '.' signifies the current directory, a double period ".." denotes moving up one directory level, and multiple slashes such as "//" are interpreted as a single slash. In this problem, treat sequences of periods not covered by the previous rules (like "...") as valid names for files or directories.

// The simplified canonical path should adhere to the following rules:

// It must start with a single slash '/'.
// Directories within the path should be separated by only one slash '/'.
// It should not end with a slash '/', unless it's the root directory.
// It should exclude any single or double periods used to denote current or parent directories.
// Return the new path.


/**
 * @param {string} path
 * @return {string}
 */
var simplifyPath = function(path) {
    const rr = []
    for(let i = 0; i < path.length; i++) {
        let current = path[i];
        if(current !== '/') {
            let name = ''
            while(current && current !== '/') {
              name += current;
              i++
              current = path[i]
            }
            if(name.length) {
              if(name === '..') {
                if(rr.length) {
                  rr.pop()
                }
              }
              else if(name !== '.') {
                rr.push(name)
              }
              i--
            }
        }
    }
    return '/' + rr.join('/');
};
// no regex

console.log(simplifyPath("/../"))
console.log(simplifyPath("/./"))
console.log(simplifyPath("/home/"))
console.log(simplifyPath("/home//foo/"))
console.log(simplifyPath("/home/user/Documents/../Pictures"))
console.log(simplifyPath("/.../a/../b/c/../d/./"))
console.log(simplifyPath("/a/../../b/../c//.//"))
console.log(simplifyPath("/..hidden")) // "/..hidden"
console.log(simplifyPath("/.hidden")) // "/..hidden"
console.log(simplifyPath("/.....hidden")) // "/..hidden"
console.log(simplifyPath("/a//b////c/d//././/..")) //  "/a/b/c"
