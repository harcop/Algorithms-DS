const arr = [ 
    [ 't', 'h', 'i', 's', 'i', 's', 'a' ], 
    [ 's', 'i', 'm', 'p', 'l', 'e', 'x' ], 
    [ 'b', 'x', 'x', 'x', 'x', 'e', 'b' ], 
    [ 'x', 'o', 'g', 'g', 'l', 'x', 'o' ], 
    [ 'x', 'x', 'x', 'D', 'T', 'r', 'a' ], 
    [ 'R', 'E', 'P', 'E', 'A', 'd', 'x' ], 
    [ 'x', 'x', 'x', 'x', 'x', 'x', 'x' ], 
    [ 'N', 'O', 'T', 'R', 'E', '-', 'P' ], 
    [ 'x', 'x', 'D', 'E', 'T', 'A', 'E' ] 
];
const w = arr[0].length-1;
const h = arr.length-1;
let vt = [];

function rs (_n, c) {
    let k = Object.keys(c);
    for (let i=0; i<_n.length; i++) {
        let _x = _n[i].x;
        let _y = _n[i].y;
        let _w = arr[_y][_x]; 
        if (k.includes(_w)) {
            let _m = neigh({ x: _n[i].x, y: _n[i].y})
            rs(_m, c[_w]);
        }
    }
}

const word = [
    "this",
    "is",
    "not",
    "a",
    "simple",
    "boggle",
    "board",
    "test",
    "REPEAT",
    "NOTRE-PEATED"
];

function td () {
    let tree = {};
    word.forEach(ele => {
        const wd = ele.split("");
        if (!tree[wd[0]]) {
            tree[wd[0]] = {};
        }
        let p = tree[wd[0]];
        for (let i = 1; i < wd.length; i++) {
            if (p[wd[i]] === undefined) {
                p[wd[i]] = {}
            }
            p = p[wd[i]];
        }
    })
    return tree;
}
const tree = td();

function tt () {
    arr.forEach((ele, y) => {
        const wd = ele.split('');
        for (let x =0; x<wd.length; x++) { 
            vt = [];
            let _l = wd[x];
            if (tree[_l]) {
                vt.push(`${x}${y}`);
                let c = tree[_l];
               const _n = neigh({ x, y });
               
            }
        }
    })
}

function neigh ({ x, y}) {
    const _n = [];
    if (y>0) {
        let up = { x,y: y-1 };
        let _s = `${x}${y-1}`;
        if (!vt.includes(_s)) {
            _n.push(up);
        }
    } 
    if (y>0 && x < w) {
        let ur = { x: x+1, y: y-1 };
        _n.push(ur);
    } 
    if (x < w) {
        let r = { x: x+1, y };
        _n.push(r);
    } 
    if (y < h && x < w) {
        let dr = { x: x+1, y: y+1 };
        _n.push(dr);
    } 
    if (y < h) {
        let d = { x, y: y+1 };
        _n.push(d);
    } 
    if (y < h && x > 0) {
        let dl = { x: x-1, y: y+1 };
        _n.push(dl);
    } 
    if (x > 0) {
        let l = { x: x-1, y };
        _n.push(l);
    } 
    if (y > 0 && x > 0) {
        let ul = { x: x-1, y: y-1 };
        _n.push(ul);
    } 
    return _n;
}
