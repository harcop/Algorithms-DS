const node = {
    value: -1,
    left: {
        value: 2,
        left: {
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
            left: null,
            right: null
        },
        right: {
            value: -1,
            left: null,
            right: null
        }
    }
}

function mps (node) {
    return Math.max(node.value + rs(node.left) + rs(node.right), rs(node.left), rs(node.right));
}

function rs (l) {
    let _l = l.left;
    let _r = l.right;
    let _l1 = 0;
    let _r1 = 0;
    if (_l !== null) {
         _l1 = rs(_l);
    }
    if (_r !== null) {
        _r1 = rs(_r);
    }
    _l1 = Math.max(_l1, _l1+l.value);
    _r1 = Math.max(_r1, _r1+l.value);
    return Math.max(_l1, _r1);
}

console.log(mps(node))