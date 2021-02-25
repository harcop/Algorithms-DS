function palindromeRearranging(str: string): boolean {
    let _s = str.split("");
    let _l = {};
    _s.forEach(ele => {
        if (!_l[ele]) {
            _l[ele] = 0;
        }
        _l[ele]++;
    });
    let _w = 0;
    for(let n in _l) {
        let _m = _l[n];
        if( _m%2 === 1) {
            _w++;
        }
    }
    if (_w>1) {
        return false;
    }
    return true;
}

console.log(palindromeRearranging('aabb'));
console.log(palindromeRearranging('aabcdcbaa'));
console.log(palindromeRearranging('aadddfaa'));