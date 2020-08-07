
const bst = require('./bst_construction');


function inOrderTraverse(tree, array) {
    if (tree !== null) {
        inOrderTraverse(tree.left, array);
        array.push(tree.value);
        inOrderTraverse(tree.right, array);
    }
    return array;
}

function preOrderTraverse(tree, array) {
    if (tree !== null) {
        array.push(tree.value);
        preOrderTraverse(tree.left, array);
        preOrderTraverse(tree.right, array);
    }
    return array;
}

function postOrderTraverse(tree, array) {
    if (tree !== null) {
        postOrderTraverse(tree.left, array);
        postOrderTraverse(tree.right, array);
        array.push(tree.value);
    }
    return array;
}

const arr = inOrderTraverse(bst, []);
// console.log(arr);
const arr2 = preOrderTraverse(bst, []);
// console.log(arr2);
const arr3 = postOrderTraverse(bst, []);
// console.log(arr3);

module.exports = {
    inOrderTraverse,
    preOrderTraverse,
    postOrderTraverse,
    bst
}