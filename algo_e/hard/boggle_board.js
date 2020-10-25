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

console.log(td());