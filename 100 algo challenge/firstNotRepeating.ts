function firstNotRepeatingCharacter(s: string): string {
    let _s = s.split("");
    let _a = [];
    let _f = {};
    _s.forEach(ele => {
        if (_a.includes(ele)) {
            delete _f[ele]
            return;
        }
        _a.push(ele);
        _f[ele] = true;
    })
    let _g = Object.getOwnPropertyNames(_f);
    return _g[0] ? _g[0] : '-';
}

console.log(firstNotRepeatingCharacter('abacabad'));
console.log(firstNotRepeatingCharacter('abacabaabacaba'));
