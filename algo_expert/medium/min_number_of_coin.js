const arr = [1,2,4];
const k = 6;

// time o(n^2) && space O(n) // dynamic programming
function mn(arr, k) {
    let _n = Array(k+1);
    _n.fill(Infinity,0,k+1);
    for(let _a of arr) {
        for(let i=_a;i<=k; i++) {
            if (_a === i) {
                _n[i] = 1;
                continue;
            };
            let _r = i - _a;
            let _rc = _n[_r];
            let _ac = _n[_a];
            let cc = _rc + _ac;
            _n[i] = min(cc, _n[i]);
        }
    }
    return _n[k];
}
function min(a, b) {
    if (a<b) {
        return a;
    }
    return b;
}

// time O(n) && space O(1) // looping backward
function minC(arr, k) {
    let _t = 0;
    let _gr = k;
    for (let i = arr.length-1; i>=0; i--) {
        let _a = arr[i];
        if (_a > k) {
            continue;
        }
        _t += Math.floor(_gr/_a);
        _gr = _gr%_a;
        if (_gr === 0) {
            return _t;
        }
    }
    return false;
}

console.log(mn(arr, k));
console.log(minC(arr, k));