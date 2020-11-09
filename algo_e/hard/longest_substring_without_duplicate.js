
// time O(n) && space O(1) worst case O(n);
function ld (str) {
    let _lw = '';
    let _lwIdx = 0;
    let _ht = {};
    str.split('').forEach((ele,index) => {
        if (_ht[ele] === undefined) {
            let _nl = str.substr(_lwIdx, index-_lwIdx+1);
            if (_nl.length > _lw.length) {
                _lw = _nl;
            } 
            _ht[ele] = index;
        }else {
            _lwIdx = _ht[ele] + 1;
            _ht = {};
        }
    });
    return _lw;
}

console.log(ld('clementisacap'))
console.log(ld('Toluwalope'))