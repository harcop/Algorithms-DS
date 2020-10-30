function Node (value) {
    this.value = value;
    this.children = {}
}

function BST (value, bst) {
    this.value = value;
    this.children = {}
    this.parent = bst;
    this.insert = (newValue, newBst) => {
        const node = new BST(newValue, newBst);
        this.children[`${newValue}`] = node;
        return this;
    }
}

const bst = new BST("A", null);
bst.insert("B", bst).insert("C", bst).insert("D", bst).insert("D", bst).insert("E", bst).insert("F", bst);
const b = bst.children["B"];
b.insert("G", b).insert("H", b).insert("I", b);
const h = b.children["H"];
h.insert("O", h).insert("P", h).insert("R", h).insert("S", h);
const p = h.children["P"];
p.insert("T", p).insert("U", p);
const r = h.children["R"];
r.insert("V", r);
const v = r.children["V"];
v.insert("W", v).insert("X", v).insert("Y", v);
const x = v.children["X"];
x.insert("Z", x);
const c = bst.children["C"];
c.insert("J", c);
const d = bst.children["D"];
d.insert("K", d).insert("L", d);
const f = bst.children["F"];
f.insert("M", f).insert("N", f);

const _f = {};
function lcm(bst, v1, v2) {
    //dfs
    let current = bst;
    if (!Object.keys(bst.children).length) {
        if (bst.value !== v1 || bst.value !== v2) {
            return 0;
        }
        let oj = bst.value; 
        while(current !== null) {
            if (_f[oj] === undefined) {
                _f[oj] = [];
            }
            _f[oj].push(current.value);
            current = current.parent;
        }
    }
    current = bst;
    for(const ele in current.children) {
        if (ele === v1 || ele === v2) {
            while(current !== null) {
                if (_f[ele] === undefined) {
                    _f[ele] = [];
                }
                _f[ele].push(current.value);
                current = current.parent;
            }
        }
        current = bst;
        lcm(current.children[ele], v1, v2);
    }
    return _f;
}
console.log(lcm(bst, 'W', 'V'));
function fc () {
    let cc = _f[Object.keys(_f)[0]];
    delete _f[Object.keys(_f)[0]];
    let _c = 0;
    for(const ele in _f) {
        const el = _f[ele];
        for(let i = el.length-1; i>=0; i--) {
            let l = el[i];
            if (cc.includes(l)) {
                _c = l;
            }
        }
    }
    return _c;
}

console.log(fc());

