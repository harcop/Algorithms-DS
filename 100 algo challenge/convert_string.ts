function convertString(s: string, t: string): boolean {
    let _pos = -1;
    let _s = s.split("");
    let _t = t.split("");
    for(let i = 0; i < _t.length; i++) {
        if (_s.includes(_t[i])) {
            _s.splice(0, _s.indexOf(_t[i])+1);
            continue;
        }
        return false;
    }
    return true;
}

console.log(convertString('ceoydefthf5iyg5h5yts', 'codefights'));
console.log(convertString('addbyca', 'abcd'));
