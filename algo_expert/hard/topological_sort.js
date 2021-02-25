const job = [1,2,3,4];
const task = [
    [4,2],
    [1,2],
    [3,2],
    [4,3],
    [1,3],
];

function ts() {
    const ht = {};
    task.forEach(ele => {
        const [_t, _d] = [ele[0], ele[1]];
        if (ht[_d] === undefined) {
            ht[_d] = {};
        }
        if (ht[_d].parent === undefined) {
            ht[_d].parent = [];
        }
        ht[_d].parent.push(_t);
    })
    console.log(ht);
    let _f = [];
    for(const ele in ht) {
        const { parent } = ht[ele];
        console.log(parent)
        _f = rs(parent, ht, _f);
        if(!_f.includes(parseInt(ele))) {
            _f.push(parseInt(ele));
        }
    }
    return _f;
}

function rs(parent, ht, _a=[]) {
    parent.forEach(p => {
        if (ht[p] === undefined) {
            console.log(p)
            if(!_a.includes(p)) {
                _a.push(p);
            }
        }else {
            console.log(p)
            _a = rs(ht[p].parent, ht, _a);
        }
        if(!_a.includes(p)) {
            _a.push(p);
        }
    })
    console.log(_a)
    return _a;
}

console.log(ts());