const arr = [
    [1,2], 
    [4,3], 
    [5,6],
    [6,7]
];
const k = 10;
function ks(arr) {
    let _a = [];
    arr.unshift([0,0]);
    let _g = [];
    for(let w = 0; w <= k; w++) {
        _a.push(w);
    }
    for (let i = 0; i < arr.length; i++) {
        for (let j = 0; j < _a.length; j++) {
            const [_l, _r] = [...arr[i]];
            if(_g[i] === undefined) {
                _g[i] = [];
            }
            if (_r === 0 || j === 0 || _l === 0) {
                _g[i][j] = 0;
                continue;
            }
            if (arr[i][1] <= j) {
                const _d = _g[i-1][j-arr[i][1]] + arr[i][0];
                _g[i].push(_d);
            } else {
                _g[i].push(Math.max(_g[i-1][j], _g[i][j-1]));
            }
        }
    }
    let _m = 0;
    let _f = [];
    for (let i = _g.length-1; i >= 1; i--) {
        for(let j = _g[i].length-1; j >= 1; j--) {
            if(_m < k) {
                if (_g[i-1][j] > _g[i][j-1] && j > 0 && i > 0) {
                    _f.push(i);
                    _m += arr[i][0];
                    break;
                }
            }
        }
    }
    return _f.length;
}

console.log(ks(arr));