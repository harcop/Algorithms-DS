function BST(value:Number) {
    this.value = value;
    this.left = null;
    this.right = null;

    this.insert = (val) => {
        let current = this;
        let falser = true;
        while (falser) {
            if (val < current.value) {
                if (current.left === null) {
                    current.left = new BST(val);
                    break;
                }
                else {
                    current = current.left;
                }
            }
            else if (val > current.value) {
                if (current.right === null) {
                    current.right = new BST(val);
                    break;
                }
                else {
                    current = current.right;
                }
            }
        }
        return this;
    }

    this.summer = (sum = [], cur = 0) => {
        let current = this;
        cur += current.value;
        if (current.left !== null) {
            current = current.left;
            current.summer(sum, cur);
            current = this;
        }
        if (current.right !== null) {
            current = current.right;
            current.summer(sum, cur);
            current = this;
        }
        if (current.left === null && current.right === null) {
            sum.push(cur);
        }
        return sum;
    }
}

const bst = new BST(10);
bst.insert(5).insert(4).insert(40);
bst.insert(20).insert(16).insert(21);
bst.insert(3).insert(6);
console.log(bst.summer());