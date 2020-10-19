const tree = {
    value: 1,
    left: {
        value: 2,
        left:{
            value: 4,
            left: null,
            right: null
        },
        right: {
            value: 5,
            left: null,
            right: null
        }
    },
    right: {
        value: 3,
        left: {
            value: 6,
            right: null,
            left: null
        },
        right: {
            value: 7,
            left: null,
            right: null
        }
    }
};

function getH (tree) {
    let current = tree;
    let h = 1;
    while (current.left !== null) {
        current = current.left;
        h++;
    }
    return h;
}

console.log(getH(tree), "treeHeight");

function getArrayOfWidth() {
    let m = getH(tree);
    let s = 2**(m-2); //space
    let arr = [1];
    let sl = 2**(m-1)+1; // second layer
    arr.push(sl);
    m = m-2;
    let n = sl;
    for (let i = 0; i < m; i++) {
        if (i === m-1) {
            console.log(i, "as", m)
            n += 2
        }
        else {
            n += s;
        }
        arr.push(n);
    }
    return arr;
    
}
console.log(getArrayOfWidth(), "Arrray");
let width = 50;
let widthArray = getArrayOfWidth();
function rs(node, stage, side) {
    const { value, left, right } = node;
    const parentClass = `c${stage}`
    let newParent = document.getElementsByClassName(parentClass);
    if (newParent[0] === undefined) {
        newParent = document.createElement("div");
        newParent.setAttribute('class', `${parentClass} pp`);
        let myWidth = width * widthArray[0];
        newParent.setAttribute("style", `width: ${myWidth}px`)
        
        $('.tree').append(newParent);
        widthArray.splice(0,1);
    }
    const newChild = document.createElement("div");
    newChild.setAttribute('class', "box");
    const txtNode = document.createTextNode(`${value}`);
    newChild.append(txtNode);
    $(`.${parentClass}`).append(newChild);
    
    let newStage = stage + 1;
    if (left !== null) {
        rs(left, newStage, "left");
    }
    if (right !== null) {
        rs(right, newStage, "right");
    }
}

rs(tree, 1, "center");
const _n = document.getElementsByClassName("tree");
console.log(_n[0]);