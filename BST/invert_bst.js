const bst = require('./bst_construction');
// console.log(bst);

// o(n) time | o(d) space (recursive calls)
function invertBst(tree) {
    if (tree.left !== null ) {
        invertBst(tree.left);
    }
    if (tree.right !== null ) {
        invertBst(tree.right);
    }
    const temp = tree.right;
    tree.right = tree.left;
    tree.left = temp;
}


// o(n) time | o(n) space
function invertBst2(tree) {
    const queue = [tree];
    while (queue.length) {
        const current = queue.pop();
        if (current == null) {
            continue;
        }
        swapLEFT_RIGHT(current);
        queue.push(current.left);
        queue.push(current.right);
    }
}

function swapLEFT_RIGHT(current) {
    [current.left, current.right] = [current.right, current.left];
}

invertBst2(bst);
console.log(bst);