package leetcode

// LeetCode #608 - Tree Node
//
// A missing parent is the root, a parent of someone else is inner, and the rest are leaves.
const treeNodeTypeSQL = `SELECT id,
    CASE
        WHEN p_id IS NULL THEN 'Root'
        WHEN id IN (SELECT p_id FROM Tree) THEN 'Inner'
        ELSE 'Leaf'
    END AS type
FROM Tree`
