const arr = [1,2,3,4];
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



// console.log(per(arr));

function per2 (arr) {
    let perm = [];
    for(let i=0; i<arr.length; i++) {
        let _a = Array(...arr);
        swap(0,i,_a);
        let _ad = Array(..._a)
        perm.push(_ad);
        for(let j=1; j<_a.length; j++) {
            if (_a[j+1]) {
                swap(j,j+1,_a);
            } else {
                swap(1,j, _a);
            }
            _ad = Array(..._a)
            perm.push(_ad);
        }
    }
    return perm;
}
function swap (a, b, arr) {
    [arr[a], arr[b]] = [arr[b], arr[a]];
}

console.log(per2(arr));