const arr1 = [1,11,3,0,15,5,2,4,10,7,12,6];
const arr2 = [1,5,2,7,8,3,4,9,11,15];

//time O(NlogN) && space O(1);
function lr (arr) {
    arr.sort((a,b) => a - b);
    let _f = 0;
    let _s = 0;
    let _r = [];
    let _rr = [_f, _s];
    for (let i=1; i<arr.length; i++) {
        let _t = arr[i];
        let _ts = arr[_s];
        if (_t === _ts + 1) {
            _s = i;
            _rr[1] = _s;
        } else {
            _r.push([..._rr]);
            _f = _s = i;
            _rr = [_f, _s];
        }
    }
    _r.push(_rr);
    let _m = 0;
    let _mn = 0;
    for(let r of _r) {
        let temp = _m;
        _m = Math.max(_m, (r[1] - r[0]));
        if (_m > temp) {
            _mn = r[0];
        }
    }
    return [_mn, _m];
}

console.log(lr(arr1));
console.log(lr(arr2));