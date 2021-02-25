function longestDigitsPrefix(i: string): string {
    let _a = i.split(''); 
    let _f = [];
    for (let _n of _a) {
        let _s = parseInt(_n);
        if (_s * 0 === 0) {
            _f.push(_n);
            continue;
        }
        return _f.join("");
    }
}

console.log(longestDigitsPrefix('123aa1'));
console.log(ldp('123aa1'));

function ldp(str) {
    return /^\d+/.exec(str)[0]
}
