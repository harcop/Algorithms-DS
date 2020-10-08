const arr = [1,2,3,4,5];
function Node (value) {
    this.value = value
    this.children = {}
}
function per (arr) {
    let head = new Node('a');
    arr.forEach((child, index) => {
        const node = new Node(child);
        let temp = Array(...arr);
        temp.splice(index, 1);
        head.children[child] = node;
        gen(temp, node); 
    });
    return head.children
}

function gen(child, parent) {
    if (child.length > 1) {
        child.forEach((ele, index) => {
            let node = new Node(ele);
            let temp = Array(...child);
            temp.splice(index, 1);
            if (temp.length > 0) {
                parent.children[ele] = node;
                let cc = gen(temp, node);
                parent.children[ele].children[cc] = new Node(cc);
            }
        })
    }
    return child[0];
}

console.log(per(arr));