function Node(name, ancestor) {
    this.name = name
    this.ancestor = ancestor;
    this.children = [];

    this.insert = (name) => {
        const node = new Node(name, this);
        this.children.push(node);
        return this;
    }
    //time O(d)
    this.level = () => {
        let current = this;
        let level = 0;
        let anc = [];
        while(current !== null) {
            level += 1;
            anc.push(current.name);
            current = current.ancestor;
        }
        return { 
            anc,
            level: level - 1,
        };
    }
}

//faster method because we keep track of the ancestor and level
//time O(d)
function yca(a, b) {
    let _la = a.level();
    let _lb = b.level();
    _ancA = _la.anc;
    _ancB = _lb.anc;
    console.log(_ancA, _ancB);
    let cma =  null;
    for(let ele of _ancA) {
        if (_ancB.includes(ele)) {
            cma = ele;
            break;
        }
    }
    return cma;
}
// using backtracking O(d);
function yca2(a, b) {
    let _la = a.level();
    let _lb = b.level();
    _lv1 = _la.level;
    _lv2 = _lb.level;
    console.log(_lv1, _lv2)
    anc = null;
    let diff = _lv2 - _lv1;
    if (diff > 0) {
        console.log(diff)
        anc = bk(a, b, diff);
    } else {
        console.log(diff)
        anc = bk(b, a, Math.abs(diff));
    }
    return anc;
}

function bk(h, l, diff) {
    let current = l;
    console.log(h.name, l.name)
    while(diff>0) {
        current = current.ancestor;
        diff -= 1;
    }
    console.log(current.name)
    let ld = current;
    while(ld !== h) {
        ld = ld.ancestor;
        h = h.ancestor;
    }
    return ld.name;
}
const node = new Node("A", null);
node.insert("B").insert("C").insert("D");
node.children[0].insert('E').insert('F');
node.children[2].insert('G').insert('H');
node.children[2].children[0].insert('I');
node.children[2].children[1].insert('J');
node.children[2].children[1].children[0].insert('K');
node.children[2].children[1].children[0].insert('L');
let a = node.children[2].children[0];
let b = node.children[2].children[0].children[0];
let c = node.children[2].children[1].children[0].children[1];
console.log(a.name, b.name, c.name)
console.log(yca2(a, b))
console.log(yca2(a, c))