function BST(value) {
    this.value = value
    this.left = null
    this.right = null;
    
    // Average (O(log(n) Time complexity) and O(1) space complexity
    //worst (O(n) Time complexity) and O(1) space complexity
    this.insert = function (value) {
        var currentNode = this;
        var falser = true;
        while (falser) {
            if (value < currentNode.value) {
                if (currentNode.left === null) {
                    currentNode.left = new BST(value);
                    break;
                }
                else 
                    currentNode = currentNode.left;
            }
            else if (value >= currentNode.value) {
                if (currentNode.right === null) {
                    currentNode.right = new BST(value);
                    break;
                }
                else {
                    currentNode = currentNode.right;
                }
            }
        }
        return this;
    }

    // Average (O(log(n) Time complexity) and O(1) space complexity
    //worst (O(n) Time complexity) and O(1) space complexity
    this.contain = function (value) {
        var currentNode = this;
        while (currentNode !== null) {
            if (value < currentNode.value) {
                currentNode = currentNode.left;
            }
            else if (value > currentNode.value) {
                currentNode = currentNode.right;
            }
            else {
                return true;
            }
        }
        return false;
    }

    this.remove = function (value, parentNode = null) {
        currentNode = this;
        while(currentNode !== null) {
            if (value < currentNode.value) {
                parentNode = currentNode;
                currentNode = currentNode.left;
            }
            else if (value > currentNode.value) {
                parentNode = currentNode;
                currentNode = currentNode.right;
            }
            else {
                if (currentNode.left !== null && currentNode.right !== null) {
                    currentNode.value = currentNode.right.getMinValue();
                    currentNode.right.remove(value)
                }

            }

        }
        return false;
    }

    this.getMinValue = function () {
        currentNode = this;
        while (currentNode.left !== null) {
            currentNode = currentNode.left;
        }
        return currentNode.value;
    }


}

const bst = new BST(10);
bst.insert(20).insert(5).insert(40);
module.exports = bst;

// console.log(bst);
// console.log(bst.contain(20));
// console.log(bst.contain(21));