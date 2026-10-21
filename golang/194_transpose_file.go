package leetcode

// LeetCode #194 - Transpose File
//
// Collect each column while reading rows, then print the columns.
const transposeFileScript = `awk '
{
  for (i = 1; i <= NF; i++) col[i] = col[i] $i (NR > 1 ? " " : "") (FNR == NR ? "" : "")
}
END {
  for (i = 1; i <= NF; i++) print col[i]
}' file.txt`
