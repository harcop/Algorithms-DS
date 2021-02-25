const arr = [3, 5, -9, 1, 3, -2, 3, 4, 7, 2, -9, 6, 3, 1, -5, 4];

// time O(n) && space O(1)
function ka (arr) {
    let _m = 0;
    let _ma = _m;
    for (const ele of arr) {
        let _t = _m + ele; 
        _m = max(_t, ele);
        _ma = max(_ma, _m);
    }
    return _ma;
}

function max(a, b) {
    if (a > b) {
        return a;
    }
    return b;
}

console.log(ka(arr));