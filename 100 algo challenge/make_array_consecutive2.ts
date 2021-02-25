function makeArrayConsecutive2(s: number[]): number {
    s.sort((a,b) => a-b);
    let _s = s[0];
    let _c = 0; 
    for (let i = 1; i < s.length; i++) {
        let _d = s[i] - _s;
        if (_d > 1) {
            _c += (_d - 1);
        }
        _s = s[i];
    }
    return _c;
}

console.log(makeArrayConsecutive2([6, 2, 3, 8]));